use crate::piece::*;
use crate::input::*;
use std::collections::HashMap;

use rand::Rng;

const DAS: u128 = 100000;
const ARR: u128 = 0;
const GRAVITY: u128 = 250000;
const LOCK_DELAY: u128 = 500000;
const PREVIEWS: usize = 5;

pub type Matrix = Vec<Vec<PieceColor>>;

pub struct Stats {
    pub score: u32,
    pub time: u128,
    pub lines_cleared: u32,
    pub pieces_placed: u32,
}

pub struct Game {
    pub matrix: Matrix,
    pub piece: Piece,
    pub held: Option<Piece>,
    pub piece_data: HashMap<String, PieceType>,
    kick_data: HashMap<String, KickData>,
    bag: Vec<String>,

    das: u128,
    arr: u128,
    arr_leftover: u128,
    gravity: u128,
    gravity_timer: u128,
    lock_timer: u128,
    can_hold: bool,
    pub stats: Stats,
}

impl Game {
    pub fn new(matrix_width: usize, matrix_height: usize) -> Result<Self, String> {
        let matrix = vec![vec![PieceColor::Empty; matrix_width]; matrix_height];
        let piece_data = load_piece_data()?;
        let kick_data = load_kick_data()?;
        validate_data(&piece_data, &kick_data)?;

        let mut bag = generate_bag(&piece_data);
        let piece = next_piece(&mut bag, &piece_data, &matrix);

        let stats = Stats {
            score: 0,
            time: 0,
            lines_cleared: 0,
            pieces_placed: 0,
        };

        Ok(Self {
            matrix,
            piece,
            held: None,
            piece_data,
            kick_data,
            bag,

            das: DAS,
            arr: ARR,
            arr_leftover: 0,
            gravity: GRAVITY,
            gravity_timer: 0,
            lock_timer: 0,
            can_hold: true,
            stats,
        })
    }

    pub fn update(&mut self, input: &mut Input, elapsed: u128) {
        self.stats.time += elapsed;
        let (movement_action, rotation_action) = read_inputs(&input);
        let mut placed_piece = false;
        let mut gravity = self.gravity;

        match movement_action {
            MovementAction::HardDrop => {
                input.hard_drop = false;
                self.piece.hard_drop();
                placed_piece = true;
            }
            MovementAction::Left => {
                self.handle_piece_movement(input.left_held, elapsed, HDirection::Left);
            }
            MovementAction::Right => {
                self.handle_piece_movement(input.right_held, elapsed, HDirection::Right);
            }
            _ => {}
        }

        match rotation_action {
            RotationAction::None => {}
            _ => {
                input.rot_cw = false;
                input.rot_180 = false;
                input.rot_ccw = false;
                self.piece.rotate(&self.matrix, &self.kick_data, rotation_action);
            }
        }

        if input.soft_drop {
            gravity /= 4;
            self.gravity_timer = std::cmp::min(self.gravity_timer, gravity);
        }
        self.gravity_timer += elapsed;
        while self.gravity_timer > gravity {
            self.gravity_timer -= gravity;
            if !self.piece.movement(&self.matrix, HDirection::None, VDirection::Down) {
                break;
            }
        }

        if input.hold {
            input.hold = false;
            if self.can_hold {
                self.can_hold = false;
                self.hold_piece();
            }
        }

        if self.piece.is_grounded(&self.matrix) {
            self.lock_timer += elapsed;
            if self.lock_timer >= LOCK_DELAY {
                placed_piece = true;
            }
        } else {
            self.lock_timer = 0;
        }

        if placed_piece {
            let _bonus = self.piece.check_bonus(&self.matrix);
            self.piece.lock(&mut self.matrix);
            self.lock_timer = 0;
            self.gravity_timer = 0;
            self.arr_leftover = 0;
            self.can_hold = true;
            self.stats.pieces_placed += 1;
            let remove = filled_rows(&mut self.matrix);
            if remove.len() > 0 {
                self.stats.lines_cleared += remove.len() as u32;
                remove_rows(&mut self.matrix, remove);
            }
            self.piece = next_piece(&mut self.bag, &self.piece_data, &self.matrix);
        }
    }

    fn handle_piece_movement(&mut self, time_held: u128, elapsed: u128, direction: HDirection) {
        if time_held == elapsed {
            self.piece.movement(&self.matrix, direction, VDirection::None);
            self.arr_leftover = 0;
        }
        if time_held > self.das {
            let mut time = elapsed + self.arr_leftover;
            while time > self.arr {
                if !self.piece.movement(&self.matrix, direction, VDirection::None) {
                    self.arr_leftover = 0;
                    break;
                }
                time -= self.arr;
            }
            self.arr_leftover = time;
        }
    }

    pub fn get_preview_pieces(&self) -> &[String] {
        &self.bag[self.bag.len()-PREVIEWS..]
    }

    fn hold_piece(&mut self) {
        self.piece.reset_position();
        match &mut self.held {
            Some(held) => {
                std::mem::swap(&mut self.piece, held);
            }
            None => {
                let next = next_piece(&mut self.bag, &self.piece_data, &self.matrix);
                self.held = Some(std::mem::replace(&mut self.piece, next));
            }
        }
        self.arr_leftover = 0;
        self.gravity_timer = 0;
        self.lock_timer = 0;
        self.piece.update_ghost(&self.matrix);
    }
}

fn generate_bag(piece_data: &HashMap<String, PieceType>) -> Vec<String> {
    // Get all pieces from the list
    let mut bag: Vec<String> = piece_data.keys().cloned().collect();

    // Suffle the pieces
    let mut rng = rand::thread_rng();
    let len = bag.len();
    for i in 0..len {
        bag.swap(i, rng.gen_range(i..len));
    }
    bag
}

fn next_piece(bag: &mut Vec<String>, piece_data: &HashMap<String, PieceType>, matrix: &Matrix) -> Piece {
    while bag.len() <= PREVIEWS {
        let mut new_bag = generate_bag(&piece_data);
        new_bag.append(bag);
        *bag = new_bag;
    }
    let new_piece = piece_data.get(&bag.pop().unwrap()).unwrap();
    let mut piece = Piece::new(new_piece.shape.clone(), new_piece.color, new_piece.kick_table.clone(), new_piece.spin_bonus);
    piece.update_ghost(&matrix);
    piece
}

fn filled_rows(matrix: &mut Matrix) -> Vec<usize> {
    let mut remove = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        let mut count = 0;
        for value in row.iter() {
            if *value == PieceColor::Empty {
                break;
            }
            count += 1;
        }
        if count == matrix[0].len() {
            remove.push(i);
        }
    }
    remove
}

fn remove_rows(matrix: &mut Matrix, remove: Vec<usize>) {
    for row in remove.iter() {
        // Empty the row
        for col in 0..matrix[0].len() {
            matrix[*row][col] = PieceColor::Empty;
        }
        // Swap the row upward
        for current in (1..=*row).rev() {
            matrix.swap(current, current-1);
        }
    }
}

fn read_inputs(input: &Input) -> (MovementAction, RotationAction) {
    let movement_action = match (input.hard_drop, input.left, input.right) {
        (true, _, _) =>  {
            return (MovementAction::HardDrop, RotationAction::None);
        }
        (_, true, false) => MovementAction::Left,
        (_, false, true) => MovementAction::Right,
        _ => MovementAction::None,
    };

    let rotation_action = match (input.rot_cw, input.rot_ccw, input.rot_180) {
        (true, false, false) => RotationAction::RotateCW,
        (false, true, false) => RotationAction::RotateCCW,
        (false, false, true) => RotationAction::Rotate180,
        _ => RotationAction::None,
    };
    (movement_action, rotation_action)
}

fn validate_data(piece_data: &HashMap<String, PieceType>, wall_kick_data: &HashMap<String, KickData>) -> Result<(), String> {
    for (piece_name, data) in piece_data.iter() {
        match wall_kick_data.get(&data.kick_table) {
            Some(_) => continue,
            None => {
                return Err(
                    format!("Piece {} has kick table {} in piece_data.toml, but that table was not found in wall_kick_data.toml.", piece_name, data.kick_table)
                );
            }
        }
    }
    Ok(())
}
