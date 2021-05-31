use std::{cmp::min, collections::HashMap};

pub type PieceType = [Vec<(i32, i32)>; 4];

pub struct Position {
    pub row: i32,
    pub col: i32,
}

pub struct Piece {
    
    pub position: Position,
    shape: PieceType,
    rotation: usize,
}

impl Piece {
    pub fn new(shape: PieceType) -> Self {
        Self {
            position: Position {row: 0, col: 4},
            shape,
            rotation: 0,
        }
    }

    pub fn move_h(&mut self, grid: &Vec<Vec<usize>>, direction: i32) {
        for (row, col) in self.shape[self.rotation].iter() {
            let new_row = (*row + self.position.row) as usize;
            let new_col = (*col + self.position.col + direction) as usize;
            // If the new_col is < 0 the the cast to usize makes it large so the first check handles out of bounds both left and right
            if new_col >= grid[0].len() || grid[new_row][new_col] != 0 {
                return;
            }
        }
        self.position.col += direction;
    }

    pub fn rotate(&mut self, grid: &Vec<Vec<usize>>, rotation: usize) {
        let target_rotation = (self.rotation + rotation) % 4;
        for (row, col) in self.shape[target_rotation].iter() {
            let new_row = (*row + self.position.row) as usize;
            let new_col = (*col + self.position.col) as usize;
            // If the new_col is < 0 the the cast to usize makes it large so the first check handles out of bounds both left and right
            if new_col >= grid[0].len() || grid[new_row][new_col] != 0 {
                // Rotation causes collision, do wall kicks
                return;
            }
        }
        self.rotation = target_rotation;
    }

    pub fn hard_drop(&self, matrix: &mut Vec<Vec<usize>>) {
        let mut min_fall_distance = matrix.len();
        for (row, col) in self.shape[self.rotation].iter() {
            let new_row = (*row + self.position.row) as usize;
            let new_col = (*col + self.position.col) as usize;
            let mut fall_distance = 0;
            for i in new_row..matrix.len() {
                if matrix[i][new_col] != 0 {
                    break;
                }
                fall_distance += 1;
            }
            min_fall_distance = min(min_fall_distance, fall_distance);
        }

        for (row, col) in self.shape[self.rotation].iter() {
            let new_row = (*row + self.position.row) as usize + min_fall_distance - 1;
            let new_col = (*col + self.position.col) as usize;
            matrix[new_row][new_col] = 6;
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
        [
            vec!((0,0), (0,1), (0,2), (0,3)),
            vec!((0,2), (1,2), (2,2), (3,2)),
            vec!((1,0), (1,1), (1,2), (1,3)),
            vec!((0,1), (1,1), (2,1), (3,1)),
        ]);

    piece_list.insert(
        'T',
        [
            vec!((0,1), (1,0), (1,1), (1,2)),
            vec!((0,1), (1,1), (1,2), (2,1)),
            vec!((1,0), (1,1), (1,2), (2,1)),
            vec!((0,1), (1,0), (1,1), (2,1)),
        ]);

    piece_list.insert(
        'O',
        [
            vec!((0,1), (0,2), (1,1), (1,2)),
            vec!((0,1), (0,2), (1,1), (1,2)),
            vec!((0,1), (0,2), (1,1), (1,2)),
            vec!((0,1), (0,2), (1,1), (1,2)),
        ]);

    piece_list.insert(
        'J',
        [
            vec!((0,0), (1,0), (1,1), (1,2)),
            vec!((0,1), (0,2), (1,1), (2,1)),
            vec!((1,0), (1,1), (1,2), (2,2)),
            vec!((0,1), (1,1), (2,0), (2,1)),
        ]);

    piece_list.insert(
        'L',
        [
            vec!((0,2), (1,0), (1,1), (1,2)),
            vec!((0,1), (1,1), (2,1), (2,2)),
            vec!((1,0), (1,1), (1,2), (2,0)),
            vec!((0,0), (0,1), (1,1), (2,1)),
        ]);

    piece_list.insert(
        'S',
        [
            vec!((0,1), (0,2), (1,0), (1,1)),
            vec!((0,1), (1,1), (1,2), (2,2)),
            vec!((1,1), (1,2), (2,0), (2,1)),
            vec!((0,0), (1,0), (1,1), (2,1)),
        ]);

    piece_list.insert(
        'Z',
        [
            vec!((0,0), (0,1), (1,1), (1,2)),
            vec!((0,2), (1,1), (1,2), (2,1)),
            vec!((1,0), (1,1), (2,1), (2,2)),
            vec!((0,1), (1,0), (1,1), (2,0)),
        ]);

    piece_list.insert(
        '2',
        [
            vec!((0,0), (0,1)),
            vec!((0,1), (1,1)),
            vec!((1,0), (1,1)),
            vec!((0,0), (1,0)),
        ]);


    piece_list
}
