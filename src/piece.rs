use crate::game::Matrix;

use std::{cmp::min, collections::HashMap, fs};
use serde::Deserialize;


#[derive(Copy, Clone, PartialEq, Deserialize)]
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

pub type PieceShape = [Vec<(i8, i8)>; 4];

#[derive(Deserialize)]
pub struct PieceType {
    pub shape: PieceShape,
    pub color: PieceColor,
}

pub struct Position {
    pub row: i8,
    pub col: i8,
}

pub struct Piece {
    pub position: Position,
    shape: PieceShape,
    pub color: PieceColor,
    rotation: usize,
    pub ghost_position: i8,
}

impl Piece {
    pub fn new(shape: PieceShape, color: PieceColor) -> Self {
        Self {
            position: Position {row: 0, col: 3},
            shape,
            color,
            rotation: 0,
            ghost_position: 0,
        }
    }

    /* A funtion that checks for collision given a rotation and a move direction */
    fn check_collision(&self, matrix: &Matrix, h_dir: i8, v_dir: i8, rotation: usize) -> bool {
        for (rel_row, rel_col) in self.shape[rotation].iter() {
            let row = (*rel_row + self.position.row + v_dir) as usize;
            let col = (*rel_col + self.position.col + h_dir) as usize;
            // If col is < 0 the the cast to usize makes it large so the first check handles out of bounds both left and right
            if col >= matrix[0].len() || row >= matrix.len() || matrix[row][col] != PieceColor::Empty {
                return true;
            }
        }
        return false;
    }

    pub fn movement(&mut self, matrix: &Matrix, h_dir: i8, v_dir: i8) -> bool {
        if self.check_collision(matrix, h_dir, v_dir, self.rotation) {
            return false;
        }
        self.position.row += v_dir;
        self.position.col += h_dir;
        self.update_ghost(&matrix);
        true
    }

    pub fn rotate(&mut self, matrix: &Matrix, rotation: usize) -> bool {
        let target_rotation = (self.rotation + rotation) % 4;
        if self.check_collision(matrix, 0, 0, target_rotation) {
            // Rotation causes a collision do wall kicks
            return false;
        }
        self.rotation = target_rotation;
        self.update_ghost(&matrix);
        true
    }

    pub fn hard_drop(&mut self, matrix: &mut Matrix) {
        self.position.row = self.ghost_position;
        self.lock(matrix);
    }

    pub fn get_orientation(&self) -> &Vec<(i8, i8)> {
        &self.shape[self.rotation]
    }

    pub fn lock(&self, matrix: &mut Matrix) {
        for (rel_row, rel_col) in self.shape[self.rotation].iter() {
            let row = (*rel_row + self.position.row) as usize;
            let col = (*rel_col + self.position.col) as usize;
            matrix[row][col] = self.color;
        }
    }

    pub fn update_ghost(&mut self, matrix: &Matrix) {
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
        self.ghost_position = self.position.row + min_fall_distance as i8 - 1;
    }
}

pub fn load_piece_data() -> Result<HashMap<String, PieceType>, String> {
    let piece_data_file = fs::read_to_string("piece_data.toml")
        .map_err(|e| format!("Error opening piece_data.toml: {}", e.to_string()))?;
    let piece_data = toml::from_str(&piece_data_file)
        .map_err(|e| format!("Error reading piece_data.toml: {}", e.to_string()))?;
    Ok(piece_data)
}
