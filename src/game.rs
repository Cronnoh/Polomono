pub mod render;
pub mod assets;
mod randomizer;
mod piece;

use piece::*;
use randomizer::*;
use crate::input::*;

use std::collections::HashMap;
use serde::Deserialize;
use enum_map::EnumMap;

pub type Matrix = [Vec<PieceColor>];

#[derive(Deserialize)]
pub struct Config {
    matrix_height: usize,
    matrix_width: usize,

    das: u32,
    arr: u32,
    gravity: u32,
    lock_delay: u32,
    preview_count: usize,

    piece_list: Vec<String>,
    cannot_start_with: Option<Vec<String>>,
    starting_randomizer: Option<randomizer::RandomizerStyle>,
    randomizer: randomizer::RandomizerStyle,
}

pub enum MovementAction {
    HardDrop,
    InstantDrop,
    Horizontal(HDirection),
    ShiftHorizontal(HDirection),
    None,
}
/* All times are in microseconds (µs) */
pub struct Stats {
    pub score: u32,
    pub time: u128,
    pub lines_cleared: u32,
    pub pieces_placed: u32,
}

pub struct Game {
    pub matrix: Vec<Vec<PieceColor>>,
    pub piece: Piece,
    pub held: Option<Piece>,
    pub piece_data: HashMap<String, PieceType>,
    kick_data: HashMap<String, KickData>,
    piece_queue: Vec<Piece>,
    pub stats: Stats,
    randomizer: Randomizer,

    das: u128, // Delayed Auto-Shift - Time in µs that left/right must be held before auto-shift begins
    arr: u128, // Auto-Repeat Rate - Time in µs the stays in each play during auto-shift
    gravity: u128, // Time in µs the piece stays in each row while falling
    lock_delay: u128, // Time in µs that the piece can stay grounded without locking into place

    /* Timer fields count upward to the above related values */
    das_timer: u128,
    gravity_timer: u128,
    lock_timer: u128,
    arr_leftover: u128, // Remainder of arr time from the previous update, should add to elapsed time

    preview_count: usize,
    can_hold: bool,
    prev_clear_was_fancy: bool,
    prev_direction: HDirection,
    game_over: bool,
}

impl Game {
    pub fn new(config: &Config) -> Result<Self, String> {
        let matrix = vec![vec![PieceColor::Empty; config.matrix_width]; config.matrix_height+crate::OFFSCREEN_ROWS];
        let piece_data = crate::load_data(std::path::Path::new("data/piece_data.toml"))?;
        let kick_data = crate::load_data(std::path::Path::new("data/wall_kick_data.toml"))?;
        validate_data(&piece_data, &kick_data, &config.piece_list)?;

        // Generate the first group of pieces with the initial randomizer style, than change it
        let starting_randomizer = match config.starting_randomizer {
            Some(x) => x,
            None => config.randomizer,
        };
        let mut randomizer = Randomizer::new(config.piece_list.clone(), starting_randomizer);
        let mut piece_queue = randomizer.generate_pieces(&config.cannot_start_with, &piece_data);
        randomizer.style = config.randomizer;
        extend_queue(&mut piece_queue, config.preview_count, &piece_data, &mut randomizer);
        let piece = next_piece(&mut piece_queue, &matrix);

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
            piece_queue,
            stats,
            randomizer,

            // Config values are in milliseconds, must be converted to microseconds
            das: config.das as u128 * 1000,
            arr: config.arr as u128 * 1000,
            gravity: config.gravity as u128 * 1000,
            lock_delay: config.lock_delay as u128 * 1000,

            das_timer: 0,
            gravity_timer: 0,
            lock_timer: 0,
            arr_leftover: 0,

            preview_count: config.preview_count,
            can_hold: true,
            prev_clear_was_fancy: false,
            prev_direction: HDirection::None,
            game_over: false,
        })
    }

    pub fn update(&mut self, input: &mut EnumMap<GameInput, bool>, elapsed: u128) {
        if self.game_over {
            return;
        }
        self.stats.time += elapsed;
        let (movement_action, rotation_action) = read_inputs(&input);
        let mut placed_piece = false;

        match movement_action {
            MovementAction::HardDrop => {
                input[GameInput::HardDrop] = false;
                self.piece.hard_drop();
                placed_piece = true;
            }
            MovementAction::InstantDrop => {
                input[GameInput::InstantDrop] = false;
                self.piece.hard_drop();
            }
            MovementAction::Horizontal(direction) => {
                self.handle_piece_movement(elapsed, direction);
            }
            MovementAction::ShiftHorizontal(direction) => {
                self.direction_change(direction);
                self.auto_shift(direction, std::u128::MAX, 0);
            }
            _ => {
                self.direction_change(HDirection::None);
            }
        }

        match rotation_action {
            RotationAction::None => {}
            _ => {
                input[GameInput::RotateCW] = false;
                input[GameInput::RotateCCW] = false;
                input[GameInput::Rotate180] = false;
                if self.piece.rotate(&self.matrix, &self.kick_data, rotation_action) {
                    /* Reduce the lock timer after a successful rotation to make spins easier */
                    self.lock_timer = std::cmp::max(0, self.lock_timer as i128 - self.lock_delay as i128/4) as u128;
                }
            }
        }

        if input[GameInput::Hold] {
            input[GameInput::Hold] = false;
            self.hold_piece();
        }

        self.gravity(elapsed, input[GameInput::SoftDrop]);

        if self.piece.is_grounded(&self.matrix) {
            self.lock_timer += elapsed;
            placed_piece = placed_piece || self.lock_timer >= self.lock_delay;
        } else {
            self.lock_timer = 0;
        }

        if placed_piece {
            let bonus = self.piece.check_bonus(&self.matrix);
            self.piece.lock(&mut self.matrix);
            self.game_over = self.check_loss();
            self.lock_timer = 0;
            self.gravity_timer = 0;
            self.direction_change(HDirection::None);
            self.can_hold = true;
            self.stats.pieces_placed += 1;
            self.handle_line_clears(bonus);
            extend_queue(&mut self.piece_queue, self.preview_count, &self.piece_data, &mut self.randomizer);
            self.piece = next_piece(&mut self.piece_queue, &self.matrix);
        }
    }

    fn handle_piece_movement(&mut self, elapsed: u128, direction: HDirection) {
        if self.prev_direction != direction {
            self.piece.movement(&self.matrix, direction, VDirection::None);
            self.direction_change(direction);
        } else {
            self.das_timer += elapsed;
            if self.das_timer >= self.das {
                let time = elapsed + self.arr_leftover;
                self.auto_shift(direction, time, self.arr);
            }
        }
    }

    fn auto_shift(&mut self, direction: HDirection, time: u128, arr: u128) {
        let mut leftover = time;
        while leftover > arr {
            if !self.piece.movement(&self.matrix, direction, VDirection::None) {
                self.arr_leftover = 0;
                return;
            }
            leftover -= arr;
        }
        self.arr_leftover = leftover;
    }

    fn gravity(&mut self, elapsed: u128, speed_up: bool) {
        let mut gravity = self.gravity;
        if speed_up {
            gravity /= 4;
            self.gravity_timer = std::cmp::min(self.gravity_timer, gravity);
        }
        self.gravity_timer += elapsed;
        while self.gravity_timer > gravity {
            self.gravity_timer -= gravity;
            if !self.piece.movement(&self.matrix, HDirection::None, VDirection::Down) {
                return;
            }
        }
    }

    fn handle_line_clears(&mut self, bonus: bool) {
        let cleared_lines = filled_rows(&mut self.matrix);
        if !cleared_lines.is_empty() {
            self.update_score(cleared_lines.len() as u32, bonus);
            self.stats.lines_cleared += cleared_lines.len() as u32;
            remove_rows(&mut self.matrix, cleared_lines);
        }
    }

    pub fn get_preview_pieces(&self) -> &[Piece] {
        &self.piece_queue[self.piece_queue.len()-self.preview_count..]
    }

    fn hold_piece(&mut self) {
        if !self.can_hold {
            return;
        }
        self.can_hold = false;
        self.piece.reset_position(&self.matrix);
        match &mut self.held {
            Some(held) => {
                std::mem::swap(&mut self.piece, held);
            }
            None => {
                extend_queue(&mut self.piece_queue, self.preview_count, &self.piece_data, &mut self.randomizer);
                let next = next_piece(&mut self.piece_queue, &self.matrix);
                self.held = Some(std::mem::replace(&mut self.piece, next));
            }
        }
        self.gravity_timer = 0;
        self.lock_timer = 0;
        self.direction_change(HDirection::None);
        self.piece.update_ghost(&self.matrix);
    }

    fn update_score(&mut self, cleared_rows: u32, bonus: bool) {
        let exponent = if bonus {
            cleared_rows + 1
        } else {
            cleared_rows - 1
        };
        let mut points = 100 * u32::pow(2, exponent);

        let fancy = cleared_rows >= 4 || (bonus && cleared_rows >= 2);
        if self.prev_clear_was_fancy && fancy {
            points += (points as f64 * 0.5) as u32;
        }

        self.prev_clear_was_fancy = fancy;
        self.stats.score += points;
    }

    /* Lose is the piece is placed entirely offscreen */
    fn check_loss(&self) -> bool {
        let mut lowest = 0;
        for (_, row) in self.piece.get_orientation().iter() {
            // Max because down is positive
            lowest = std::cmp::max(lowest, *row as i32 + self.piece.position.row);
        }
        lowest < crate::OFFSCREEN_ROWS as i32
    }

    /* Reset timers when changing direction to make movement more consistent */
    fn direction_change(&mut self, direction: HDirection) {
        self.das_timer = 0;
        self.arr_leftover = 0;
        self.prev_direction = direction;
    }
}

fn next_piece(piece_queue: &mut Vec<Piece>, matrix: &Matrix) -> Piece {
    let mut piece = piece_queue.pop()
        .expect("Popped from empty piece queue");
    piece.reset_position(&matrix);
    piece
}

fn extend_queue(piece_queue: &mut Vec<Piece>, preview_count: usize, piece_data: &HashMap<String, PieceType>, randomizer: &mut Randomizer) {
    /* Add more pieces to the queue if it is too small */
    while piece_queue.len() <= preview_count {
        let mut new_bag = randomizer.generate_pieces(&None, piece_data);
        new_bag.append(piece_queue);
        *piece_queue = new_bag;
    }
}

fn filled_rows(matrix: &mut Matrix) -> Vec<usize> {
    let mut cleared = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        let mut count = 0;
        for value in row.iter() {
            if *value == PieceColor::Empty {
                break;
            }
            count += 1;
        }
        if count == matrix[0].len() {
            cleared.push(i);
        }
    }
    cleared
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

fn read_inputs(input: &EnumMap<GameInput, bool>) -> (MovementAction, RotationAction) {
    /* HardDrop and InstantDrop return to disallow rotation with those movements */
    use crate::input::GameInput::*;
    if input[HardDrop] {
        return (MovementAction::HardDrop, RotationAction::None);
    }
    if input[InstantDrop] {
        return (MovementAction::InstantDrop, RotationAction::None);
    }

    let movement_action = match (input[Left], input[Right], input[ShiftLeft], input[ShiftRight]) {
        (_, _, true, false) => MovementAction::ShiftHorizontal(HDirection::Left),
        (_, _, false, true) => MovementAction::ShiftHorizontal(HDirection::Right),
        (true, false, _, _) => MovementAction::Horizontal(HDirection::Left),
        (false, true, _, _) => MovementAction::Horizontal(HDirection::Right),
        _ => MovementAction::None,
    };

    let rotation_action = match (input[RotateCW], input[RotateCCW], input[Rotate180]) {
        (true, false, false) => RotationAction::RotateCW,
        (false, true, false) => RotationAction::RotateCCW,
        (false, false, true) => RotationAction::Rotate180,
        _ => RotationAction::None,
    };
    (movement_action, rotation_action)
}

/* Checks that all kick tables in the piece data are found in the wall kick data */
fn validate_data(piece_data: &HashMap<String, PieceType>, wall_kick_data: &HashMap<String, KickData>, piece_list: &[String]) -> Result<(), String> {
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

    for piece in piece_list.iter() {
        match piece_data.get(piece) {
            Some(_) => continue,
            None => {
                return Err(
                    format!("Piece {} found in config.toml piece_list, be is not defined in piece_data.toml.", piece)
                );
            }
        }
    }
    Ok(())
}
