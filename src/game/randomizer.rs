use super::piece::{Piece, PieceColor, PieceShape, PieceType, shape_dimensions, shape_top_left};

use std::collections::HashMap;

use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub enum RandomizerStyle {
    NBag, // A randomized list of all pieces once
    DoubleNBag, // A randomized list containing each piece twice
    Minus1Bag, // A randomized list of all piece except for one, remember the excluded piece and don't exclude it twice in a row
    FullRandom, // Fully random list of pieces
    Classic, //Random list of pieces with rerolls on pieces that appear twice in a row
    Streak, // Get a random piece a random number of times in a row
    Chaos, // Randomly generated pieces
}

pub struct Randomizer {
    piece_list: Vec<String>,
    pub style: RandomizerStyle,
    remembered_piece: Option<String>,
}

impl Randomizer {
    pub fn new(piece_list: Vec<String>, style: RandomizerStyle) -> Self {
        Self {
            piece_list,
            style,
            remembered_piece: None,
        }
    }

    pub fn generate_pieces(&mut self, cannot_start_with: &Option<Vec<String>>, piece_data: &HashMap<String, PieceType>) -> Vec<Piece> {
        let mut new_pieces: Vec<String>;
        match self.style {
            RandomizerStyle::NBag => new_pieces = self.n_bag(),
            RandomizerStyle::DoubleNBag => new_pieces = self.double_n_bag(),
            RandomizerStyle::Minus1Bag => new_pieces = self.minus_1_bag(),
            RandomizerStyle::FullRandom => new_pieces = self.full_random(),
            RandomizerStyle::Classic => new_pieces = self.classic(),
            RandomizerStyle::Streak => new_pieces = self.streak(),
            RandomizerStyle::Chaos => return self.chaos(),
        }
        if let Some(disallowed) = cannot_start_with {
            fix_starting_piece(&mut new_pieces, disallowed);
        }
        create_pieces(new_pieces, piece_data)
    }

    fn n_bag(&self) -> Vec<String> {
        let mut bag = self.piece_list.clone();
        randomize(&mut bag);
        bag
    }

    fn double_n_bag(&self) -> Vec<String> {
        let mut bag = self.piece_list.clone();
        bag.append(&mut self.piece_list.clone());
        randomize(&mut bag);
        bag
    }

    fn minus_1_bag(&mut self) -> Vec<String> {
        let mut bag = self.piece_list.clone();
        randomize(&mut bag);
        if bag.len() < 2 {
            return bag;
        }

        // Ensure that the same piece is not removed from 2 bags in a row
        match &self.remembered_piece {
            Some(piece) => {
                let mut removed = bag.remove(bag.len()-1);
                let mut tested = Vec::new();
                while removed == *piece && !bag.is_empty() {
                    tested.push(removed);
                    removed = bag.remove(bag.len()-1);
                }
                self.remembered_piece = Some(removed);
                bag.append(&mut tested);
            }
            None => {
                self.remembered_piece = Some(bag.remove(bag.len()-1));
            }
        }
        bag
    }

    fn full_random(&self) -> Vec<String> {
        let mut pieces = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let r = rng.gen_range(0..self.piece_list.len());
            pieces.push(self.piece_list[r].clone());
        }

        pieces
    }

    fn classic(&mut self) -> Vec<String> {
        let mut pieces = self.full_random();
        let mut rng = rand::thread_rng();

        // If the piece is the same as the previous piece reroll it once
        if let Some(prev) = &self.remembered_piece {
            if pieces[0] == *prev {
                pieces[0] = self.piece_list[rng.gen_range(0..self.piece_list.len())].clone();
            }
        }
        for i in 1..pieces.len() {
            if pieces[i] == pieces[i-1] {
                pieces[i] = self.piece_list[rng.gen_range(0..self.piece_list.len())].clone();
            }
        }
        self.remembered_piece = Some(pieces[pieces.len()-1].clone());
        pieces
    }

    fn streak(&mut self) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let piece = self.piece_list[rng.gen_range(0..self.piece_list.len())].clone();
        let count = rng.gen_range(2..6);
        vec![piece; count]
    }

    fn chaos(&self) -> Vec<Piece> {
        let mut pieces = Vec::new();
        for _ in 0..10 {
            let shape = generate_piece_shape();
            let color: PieceColor = rand::random();
            pieces.push(Piece::new(shape, color, "SRS".to_string(), false));
        }
        pieces
    }
}

fn randomize<T>(bag: &mut [T]) {
    let mut rng = rand::thread_rng();
    let len = bag.len();
    for i in 0..len {
        bag.swap(i, rng.gen_range(i..len));
    }
}

fn create_pieces(piece_names: Vec<String>, piece_data: &HashMap<String, PieceType>) -> Vec<Piece> {
    let mut pieces = Vec::new();
    for name in piece_names {
        let piece = piece_data.get(&name)
            .unwrap_or_else(|| panic!("Tried to get {} from piece_data, but it was not found", name));
        pieces.push(Piece::new(piece.shape.clone(), piece.color, piece.kick_table.clone(), piece.spin_bonus));
    }

    pieces
}

fn fix_starting_piece(list: &mut Vec<String>, disallowed: &[String]) {
    let len = list.len();
    if disallowed.contains(&list[len-1]) {
        for i in (0..len-2).rev() {
            if !disallowed.contains(&list[i]) {
                list.swap(i, len-1);
            }
        }
    }
}

fn generate_piece_shape() -> PieceShape {
    let mut rng = rand::thread_rng();
    let bound = rng.gen_range(3..=4);

    let mut initial_rotation = Vec::new();
    /* Iterate through each space of the bound x bound bounding box */
    let mut prev_row = vec![false; bound];
    for i in 0..bound {
        let mut prev = false;
        for j in 0..bound {
            let r = rng.gen_range(0..100);
            let mut is_filled = initial_rotation.len() < 1 && r < 80;
            is_filled = is_filled || initial_rotation.len() < 4 && (prev_row[j] || prev) && r < 60;
            is_filled = is_filled || (prev_row[j] || prev) && r < 30;
            is_filled = is_filled || r < 5;
            if is_filled {
                initial_rotation.push((j as i8, i as i8));
            }
            prev = is_filled;
            prev_row[j] = is_filled;
        }
    }

    /* If an empty piece is generated, make it a monomino */
    if initial_rotation.is_empty() {
        initial_rotation.push((0,0));
    }

    let new_bound = adjust_bounding_box(&mut initial_rotation);

    let rot_cw = rotate_shape(&initial_rotation, new_bound);
    let rot_180 = rotate_shape(&rot_cw, new_bound);
    let rot_ccw = rotate_shape(&rot_180, new_bound);

    [initial_rotation, rot_cw, rot_180, rot_ccw]
}

/* Rotate the piece CW by making each row (from the top) a column (from the right) */
fn rotate_shape(shape: &[(i8, i8)], bound: i8) -> Vec<(i8, i8)> {
    let mut rotated = Vec::new();
    let columns = bound - 1;
    for (i, j) in shape {
        rotated.push((columns-*j, *i));
    }
    rotated
}

/* Move piece center to the center of a tight bounding box, returns the new bound */
fn adjust_bounding_box(shape: &mut Vec<(i8, i8)>) -> i8 {
    let (leftmost, topmost) = shape_top_left(shape);
    let (width, height) = shape_dimensions(shape);
    let piece_center_x = (leftmost as usize + width/2) as i8;
    let piece_center_y = (topmost as usize + height/2) as i8;
    let new_bound = std::cmp::max(width, height) as i8;
    let centering_offset_x = new_bound / 2 - piece_center_x;
    let centering_offset_y = new_bound / 2 - piece_center_y;
    for (i, j) in shape.iter_mut() {
        *i += centering_offset_x;
        *j += centering_offset_y;
    }
    new_bound
}
