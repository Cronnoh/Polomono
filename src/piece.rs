use std::{cmp::min, collections::HashMap};

use crate::game::Matrix;

#[derive(Copy, Clone, PartialEq)]
pub enum PieceColor {
    Empty,
    Cyan,
    Magenta,
    Yellow,
    Blue,
    Orange,
    Green,
    Red,
    Gray,
    ColorCount
}

pub type PieceShape = [Vec<(i32, i32)>; 4];

pub struct PieceType {
    pub shape: PieceShape,
    pub color: PieceColor,
}

pub struct Position {
    pub row: i32,
    pub col: i32,
}

pub struct Piece {
    
    pub position: Position,
    shape: PieceShape,
    pub color: PieceColor,
    rotation: usize,
}

impl Piece {
    pub fn new(shape: PieceShape, color: PieceColor) -> Self {
        Self {
            position: Position {row: 0, col: 3},
            shape,
            color,
            rotation: 0,
        }
    }

    /* A funtion that checks for collision given a rotation and a move direction */

    pub fn move_h(&mut self, matrix: &Matrix, direction: i32) -> bool {
        for (rel_row, rel_col) in self.shape[self.rotation].iter() {
            let row = (*rel_row + self.position.row) as usize;
            let col = (*rel_col + self.position.col + direction) as usize;
            // If the new_col is < 0 the the cast to usize makes it large so the first check handles out of bounds both left and right
            if col >= matrix[0].len() || matrix[row][col] != PieceColor::Empty {
                return false;
            }
        }
        self.position.col += direction;
        true
    }

    pub fn rotate(&mut self, matrix: &Matrix, rotation: usize) {
        let target_rotation = (self.rotation + rotation) % 4;
        for (rel_row, rel_col) in self.shape[target_rotation].iter() {
            let row = (*rel_row + self.position.row) as usize;
            let col = (*rel_col + self.position.col) as usize;
            // If the new_col is < 0 the the cast to usize makes it large so the first check handles out of bounds both left and right
            if col >= matrix[0].len() || matrix[row][col] != PieceColor::Empty {
                // Rotation causes collision, do wall kicks
                return;
            }
        }
        self.rotation = target_rotation;
    }

    pub fn hard_drop(&self, matrix: &mut Matrix) {
        let mut min_fall_distance = matrix.len();
        for (rel_row, rel_col) in self.shape[self.rotation].iter() {
            let row = (*rel_row + self.position.row) as usize;
            let col = (*rel_col + self.position.col) as usize;
            let mut fall_distance = 0;
            for i in row..matrix.len() {
                if matrix[i][col] != PieceColor::Empty {
                    break;
                }
                fall_distance += 1;
            }
            min_fall_distance = min(min_fall_distance, fall_distance);
        }

        for (row, col) in self.shape[self.rotation].iter() {
            let new_row = (*row + self.position.row) as usize + min_fall_distance - 1;
            let new_col = (*col + self.position.col) as usize;
            matrix[new_row][new_col] = self.color;
        }
    }

    pub fn get_orientation(&self) -> &Vec<(i32, i32)> {
        &self.shape[self.rotation]
    }
}

pub fn load_piece_data<'a>() -> HashMap<char, PieceType> {
    let mut piece_list = HashMap::new();
    piece_list.insert(
        'I',
        PieceType {
            shape: [
                vec!((0,0), (0,1), (0,2), (0,3)),
                vec!((0,2), (1,2), (2,2), (3,2)),
                vec!((1,0), (1,1), (1,2), (1,3)),
                vec!((0,1), (1,1), (2,1), (3,1)),
            ],
            color: PieceColor::Cyan,
        });

    piece_list.insert(
        'T',
        PieceType {
            shape: [
                vec!((0,1), (1,0), (1,1), (1,2)),
                vec!((0,1), (1,1), (1,2), (2,1)),
                vec!((1,0), (1,1), (1,2), (2,1)),
                vec!((0,1), (1,0), (1,1), (2,1)),
            ],
            color: PieceColor::Magenta,
        });

    piece_list.insert(
        'O',
        PieceType {
            shape: [
                vec!((0,1), (0,2), (1,1), (1,2)),
                vec!((0,1), (0,2), (1,1), (1,2)),
                vec!((0,1), (0,2), (1,1), (1,2)),
                vec!((0,1), (0,2), (1,1), (1,2)),
            ],
            color: PieceColor::Yellow,
        });

    piece_list.insert(
        'J',
        PieceType {
            shape: [
                vec!((0,0), (1,0), (1,1), (1,2)),
                vec!((0,1), (0,2), (1,1), (2,1)),
                vec!((1,0), (1,1), (1,2), (2,2)),
                vec!((0,1), (1,1), (2,0), (2,1)),
            ],
            color: PieceColor::Blue,
        });

    piece_list.insert(
        'L',
        PieceType {
            shape: [
                vec!((0,2), (1,0), (1,1), (1,2)),
                vec!((0,1), (1,1), (2,1), (2,2)),
                vec!((1,0), (1,1), (1,2), (2,0)),
                vec!((0,0), (0,1), (1,1), (2,1)),
            ],
            color: PieceColor::Orange,
        });

    piece_list.insert(
        'S',
        PieceType {
            shape: [
                vec!((0,1), (0,2), (1,0), (1,1)),
                vec!((0,1), (1,1), (1,2), (2,2)),
                vec!((1,1), (1,2), (2,0), (2,1)),
                vec!((0,0), (1,0), (1,1), (2,1)),
            ],
            color: PieceColor::Green,
        });

    piece_list.insert(
        'Z',
        PieceType {
            shape: [
                vec!((0,0), (0,1), (1,1), (1,2)),
                vec!((0,2), (1,1), (1,2), (2,1)),
                vec!((1,0), (1,1), (2,1), (2,2)),
                vec!((0,1), (1,0), (1,1), (2,0)),
            ],
            color: PieceColor::Red,
        });

    piece_list.insert(
        '2',
        PieceType {
            shape: [
                vec!((0,1), (0,2)),
                vec!((0,2), (1,2)),
                vec!((1,1), (1,2)),
                vec!((0,1), (1,1)),
            ],
            color: PieceColor::Gray,
        });


    piece_list
}
