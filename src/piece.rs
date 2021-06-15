use crate::game::Matrix;

use std::{cmp::min, collections::HashMap};
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

#[derive(Clone, Copy, PartialEq)]
pub enum HDirection {
    Left = -1,
    None = 0,
    Right = 1,
}

#[derive(Clone, Copy)]
pub enum VDirection {
    // Up = -1,
    None = 0,
    Down = 1,
}

#[derive(Clone, Copy)]
pub enum RotationAction {
    None,
    RotateCW,
    Rotate180,
    RotateCCW,
}

pub type PieceShape = [Vec<(i8, i8)>; 4];
pub type KickData = [[Vec<(i8, i8)>; 3]; 4];

#[derive(Deserialize)]
pub struct PieceType {
    pub shape: PieceShape,
    pub color: PieceColor,
    pub kick_table: String,
    pub spin_bonus: bool,
}

pub struct Position {
    pub row: i8,
    pub col: i8,
}

pub struct Piece {
    pub position: Position,
    shape: PieceShape,
    pub color: PieceColor,
    kick_table: String,
    rotation: usize,
    pub ghost_position: i8,
    spin_bonus: bool,
    last_move_was_rotation: bool,
}

impl Piece {
    pub fn new(shape: PieceShape, color: PieceColor, kick_table: String, spin_bonus: bool) -> Self {
        Self {
            position: Position {row: 0, col: 3},
            shape,
            color,
            kick_table,
            rotation: 0,
            ghost_position: 0,
            spin_bonus,
            last_move_was_rotation: false,
        }
    }

    /* Check if the given movement and rotation would cause a collision.
        Returns true is a collision would occur and false otherwise.
    */
    fn check_collision(&self, matrix: &Matrix, h_dir: i8, v_dir: i8, rotation: usize) -> bool {
        for (rel_row, rel_col) in self.shape[rotation].iter() {
            let row = (*rel_row + self.position.row + v_dir) as usize;
            let col = (*rel_col + self.position.col + h_dir) as usize;
            // If col is < 0 the the cast to usize makes it large so the first check handles out of bounds both left and right
            if col >= matrix[0].len() || row >= matrix.len() || matrix[row][col] != PieceColor::Empty {
                return true;
            }
        }
        false
    }

    pub fn movement(&mut self, matrix: &Matrix, h_dir: HDirection, v_dir: VDirection) -> bool {
        if self.check_collision(matrix, h_dir as i8, v_dir as i8, self.rotation) {
            return false;
        }
        self.position.row += v_dir as i8;
        self.position.col += h_dir as i8;
        self.update_ghost(&matrix);
        match h_dir {
            HDirection::Left | HDirection::Right => {
                self.last_move_was_rotation = false;
            }
            _ => {}
        }
        true
    }

    pub fn rotate(&mut self, matrix: &Matrix, kick_data: &HashMap<String, KickData>, rotation: RotationAction) -> bool {
        let target_rotation = (self.rotation + rotation as usize) % 4;
        if self.check_collision(matrix, 0, 0, target_rotation) {
            // Rotation causes a collision do wall kicks
            return self.wall_kick(matrix, kick_data, rotation);
        }
        self.rotation = target_rotation;
        self.update_ghost(&matrix);
        self.last_move_was_rotation = true;
        true
    }

    fn wall_kick(&mut self, matrix: &Matrix, kick_data: &HashMap<String, KickData>, rotation: RotationAction) -> bool {
        let target_rotation = (self.rotation + rotation as usize) % 4;

        let kick_movements = &kick_data.get(&self.kick_table).unwrap()[self.rotation][rotation as usize-1];

        for (h, v) in kick_movements {
            if !self.check_collision(matrix, *h, *v, target_rotation) {
                self.rotation = target_rotation;
                self.position.row += *v;
                self.position.col += *h;
                self.update_ghost(&matrix);
                self.last_move_was_rotation = true;
                return true;
            }
        }
        false
    }

    pub fn hard_drop(&mut self) {
        self.position.row = self.ghost_position;
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
            for current_row in matrix.iter().skip(row) {
                if current_row[col] != PieceColor::Empty {
                    break;
                }
                fall_distance += 1;
            }
            min_fall_distance = min(min_fall_distance, fall_distance);
        }
        self.ghost_position = if min_fall_distance > 0 {
            self.position.row + min_fall_distance as i8 - 1
        } else {
            self.position.row
        };
    }

    pub fn is_grounded(&self, matrix: &Matrix) -> bool {
        self.check_collision(matrix, 0, 1, self.rotation)
    }

    pub fn reset_position(&mut self) {
        self.position.col = 3;
        self.position.row = 0;
        self.rotation = 0;
    }

    pub fn check_bonus(&self, matrix: &Matrix) -> bool {
        if !self.spin_bonus || !self.last_move_was_rotation {
            return false;
        }
        let collides_up = self.check_collision(matrix, 0, -1, self.rotation);
        let collides_left = self.check_collision(matrix, -1, 0, self.rotation);
        let collides_right = self.check_collision(matrix, 1, 0, self.rotation);
        collides_up && collides_left && collides_right
    }
}
