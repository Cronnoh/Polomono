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
pub struct KickData {
    pub data: [[Vec<(i8, i8)>; 3]; 4],
}

#[derive(Deserialize)]
pub struct PieceType {
    pub shape: PieceShape,
    pub color: PieceColor,
    pub kick_table: String,
}

pub struct Position {
    pub row: i8,
    pub col: i8,
}

pub struct Piece {
    pub position: Position,
    shape: PieceShape,
    pub color: PieceColor,
    kick_key: String,
    rotation: usize,
    pub ghost_position: i8,
}

impl Piece {
    pub fn new(shape: PieceShape, color: PieceColor, kick_key: String) -> Self {
        Self {
            position: Position {row: 0, col: 3},
            shape,
            color,
            kick_key,
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

    pub fn rotate(&mut self, matrix: &Matrix, kick_data: &HashMap<String, KickData>, rotation: usize) -> bool {
        let target_rotation = (self.rotation + rotation) % 4;
        if self.check_collision(matrix, 0, 0, target_rotation) {
            // Rotation causes a collision do wall kicks
            return self.wall_kick(matrix, kick_data, rotation);
        }
        self.rotation = target_rotation;
        self.update_ghost(&matrix);
        true
    }

    fn wall_kick(&mut self, matrix: &Matrix, kick_data: &HashMap<String, KickData>, rotation: usize) -> bool {
        let target_rotation = (self.rotation + rotation) % 4;
        
        // let current_offsets = match self.rotation {
        //     0 => vec![(0,0),(0,0),(0,0),(0,0),(0,0)],
        //     1 => vec![(0,0),(1,0),(1,1),(0,-2),(1,-2)],
        //     2 => vec![(0,0),(0,0),(0,0),(0,0),(0,0)],
        //     3 => vec![(0,0),(-1,0),(-1,1),(0,-2),(-1,-2)],
        //     _ => return false,
        // };

        // let target_offsets = match target_rotation {
        //     0 => vec![(0,0),(0,0),(0,0),(0,0),(0,0)],
        //     1 => vec![(0,0),(1,0),(1,1),(0,-2),(1,-2)],
        //     2 => vec![(0,0),(0,0),(0,0),(0,0),(0,0)],
        //     3 => vec![(0,0),(-1,0),(-1,1),(0,-2),(-1,-2)],
        //     _ => return false,
        // };

        // let kick_movements = current_offsets.iter().zip(target_offsets).map(|((a ,b), (c, d))| ((a-c), (b-d)));

        let kick_movements = &kick_data.get(&self.kick_key).unwrap().data[self.rotation][rotation-1];

        for (h, v) in kick_movements {
            if !self.check_collision(matrix, *h, *v, target_rotation) {
                self.rotation = target_rotation;
                self.position.row += *v;
                self.position.col += *h;
                self.update_ghost(&matrix);
                return true;
            }
        }
        false
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

pub fn load_kick_data() -> Result<HashMap<String, KickData>, String> {
    let kick_data_file = fs::read_to_string("wall_kick_data.toml")
        .map_err(|e| format!("Error opening wall_kick_data.toml: {}", e.to_string()))?;
    let kick_data = toml::from_str(&kick_data_file)
        .map_err(|e| format!("Error reading wall_kick_data.toml: {}", e.to_string()))?;
    Ok(kick_data)
}
