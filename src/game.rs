use crate::piece::*;
use std::{collections::HashMap};

use rand::Rng;

pub struct Game {
    pub matrix: Vec<Vec<usize>>,
    pub piece: Piece,
    piece_data: HashMap<char, PieceType>,
    bag: Vec<char>,
}

impl Game {
    pub fn new(matrix_width: usize, matrix_height: usize) -> Self {
        let matrix = vec![vec![0; matrix_width]; matrix_height];
        let piece_data = load_piece_data();

        let mut bag = get_bag(&piece_data);
        let first_piece = piece_data.get(&bag.pop().unwrap()).unwrap();
        let piece = Piece::new(first_piece.clone());

        Self {
            matrix,
            piece,
            piece_data,
            bag,
        }
    }

    pub fn update(&mut self, inputs: &crate::Inputs) {
        if inputs.hard_drop {
            self.piece.hard_drop(&mut self.matrix);
            let remove = filled_rows(&mut self.matrix);
            remove_rows(&mut self.matrix, remove);
            self.next_piece();
        } else {
            let mut direction = 0;
            if inputs.left {
                direction += -1;
            }
            if inputs.right {
                direction += 1
            }
            if inputs.rot_ccw {
                self.piece.rotate(&self.matrix, 3);
            }
            if inputs.rot_cw {
                self.piece.rotate(&self.matrix, 1);
            }
            if inputs.rot_180 {
                self.piece.rotate(&self.matrix, 2);
            }
            self.piece.move_h(&self.matrix, direction);

        }
    }

    fn next_piece(&mut self) {
        if self.bag.is_empty() {
            self.bag = get_bag(&self.piece_data);
        }
        self.piece = Piece::new(self.piece_data.get(&self.bag.pop().unwrap()).unwrap().clone());
    }
}

fn get_bag(piece_list: &HashMap<char, PieceType>) -> Vec<char> {
    // Get all pieces from the list
    let mut bag: Vec<char> = piece_list.keys().cloned().collect();

    // Suffle the pieces
    let mut rng = rand::thread_rng();
    let len = bag.len();
    for i in 0..len {
        bag.swap(i, rng.gen_range(i..len));
    }
    bag
}

fn filled_rows(matrix: &mut Vec<Vec<usize>>) -> Vec<usize> {
    let mut remove = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        let mut count = 0;
        for value in row.iter() {
            if *value == 0 {
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

fn remove_rows(matrix: &mut Vec<Vec<usize>>, remove: Vec<usize>) {
    for row in remove.iter() {
        // Empty the row
        for col in 0..matrix[0].len() {
            matrix[*row][col] = 0;
        }
        // Swap the row upward
        for current in (2..=*row).rev() {
            matrix.swap(current, current-1);
        }
    }
}
