mod pieces;

use std::{cmp::min, time::Duration};

use rand::Rng;
use sdl2::{
    rect::{Rect, Point},
    render::{WindowCanvas, Texture},
    pixels::Color,
    event::Event,
    keyboard::Scancode,
    image::{InitFlag, LoadTexture},
};

const MATRIX_WIDTH: usize = 10;
const MATRIX_HEIGHT: usize = 20;
const SCALE: u32 = 32;

#[derive(Debug)]
struct Inputs {
    hard_drop: bool,
    soft_drop: bool,
    left: bool,
    right: bool,
    rot_cw: bool,
    rot_ccw: bool,
    rot_180: bool,
}

struct Position {
    row: i32,
    col: i32,
}

struct Piece<'a> {
    pos: Position,
    shape: &'a pieces::PieceType,
    rotation: usize,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window("idk", 1184, 666)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;


    let texture_creator = canvas.texture_creator();
    let blocks = texture_creator.load_texture("assets/tet.png")?;

    let mut inputs = Inputs {
        hard_drop: false,
        soft_drop: false,
        left: false,
        right: false,
        rot_cw: false,
        rot_ccw: false,
        rot_180: false,

    };

    let mut grid: [[usize; MATRIX_WIDTH]; MATRIX_HEIGHT] = [[0; MATRIX_WIDTH]; MATRIX_HEIGHT];
    let piece_list = pieces::create_piece_map();

    let mut current_piece = Piece {
        pos: Position {row: 0, col: 5},
        shape: &piece_list.get(&'T').unwrap(),
        rotation: 0,
    };

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { scancode: Some(Scancode::W), repeat: false, .. } => {
                    inputs.hard_drop = true;
                }
                Event::KeyUp { scancode: Some(Scancode::W), repeat: false, .. } => {
                    inputs.hard_drop = false;
                }
                Event::KeyDown { scancode: Some(Scancode::S), repeat: false, .. } => {
                    inputs.soft_drop = true;
                }
                Event::KeyUp { scancode: Some(Scancode::S), repeat: false, .. } => {
                    inputs.soft_drop = false;
                }
                Event::KeyDown { scancode: Some(Scancode::A), repeat: false, .. } => {
                    inputs.left = true;
                }
                Event::KeyUp { scancode: Some(Scancode::A), repeat: false, .. } => {
                    inputs.left = false;
                }
                Event::KeyDown { scancode: Some(Scancode::D), repeat: false, .. } => {
                    inputs.right = true;
                }
                Event::KeyUp { scancode: Some(Scancode::D), repeat: false, .. } => {
                    inputs.right = false;
                }
                Event::KeyDown { scancode: Some(Scancode::J), repeat: false, .. } => {
                    inputs.rot_ccw = true;
                }
                Event::KeyUp { scancode: Some(Scancode::J), repeat: false, .. } => {
                    inputs.rot_ccw = false;
                }
                Event::KeyDown { scancode: Some(Scancode::K), repeat: false, .. } => {
                    inputs.rot_180 = true;
                }
                Event::KeyUp { scancode: Some(Scancode::K), repeat: false, .. } => {
                    inputs.rot_180 = false;
                }
                Event::KeyDown { scancode: Some(Scancode::L), repeat: false, .. } => {
                    inputs.rot_cw = true;
                }
                Event::KeyUp { scancode: Some(Scancode::L), repeat: false, .. } => {
                    inputs.rot_cw = false;
                }
                _ => {}
            }
        }

        // Update
        update(&mut grid, &mut inputs, &mut current_piece);


        // Render
        render(&mut canvas, &blocks, &grid, &current_piece)?;
        
        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn update(grid: &mut [[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], inputs: &mut Inputs, current_piece: &mut Piece) {
    if inputs.hard_drop {
        hard_drop(current_piece, grid);
        inputs.hard_drop = false;
        let remove = filled_rows(grid);
        remove_rows(grid, remove);
    } else {
        let mut direction = 0;
        if inputs.left {
            inputs.left = false;
            direction += -1;
        }
        if inputs.right {
            inputs.right = false;
            direction += 1
        }
        if inputs.rot_ccw {
            inputs.rot_ccw = false;
            rotate_piece(current_piece, grid, 3);
        }
        if inputs.rot_cw {
            inputs.rot_cw = false;
            rotate_piece(current_piece, grid, 1);
        }
        if inputs.rot_180 {
            inputs.rot_180 = false;
            rotate_piece(current_piece, grid, 2);
        }
        move_piece_h(current_piece, grid, direction);

    }

    // get_bag();
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, grid: &[[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], piece: &Piece) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let canvas_center = Point::new(width as i32 / 2, height as i32 / 2);

    for (i, row) in grid.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value > 0 {
                let x = (j as u32 * SCALE) as i32;
                let y = (i as u32 * SCALE) as i32;
                let block = Rect::new(((*value-1)*16) as i32, 0, 16, 16);
                canvas.copy(&texture, block, Rect::new(x, y, SCALE, SCALE))?;
            }
        }
    }
    canvas.copy(&texture, Rect::new(0, 0, 16, 16), Rect::from_center(canvas_center, 64, 64))?;

    for (row, col) in piece.shape[piece.rotation].iter() {
        let x = (*col as i32 + piece.pos.col) * SCALE as i32;
        let y = (*row as i32 + piece.pos.row) * SCALE as i32;
        let  block = Rect::new(0, 0, 16, 16);
        canvas.copy(&texture, block, Rect::new(x, y, SCALE, SCALE))?;
    }

    canvas.present();

    Ok(())
}

// fn hard_drop(grid: &mut [[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], current_position: &Position) {
//     let row = find_collision_down(&grid, &current_position);
//     grid[row][current_position.col] = 6;
// }

// // Returns farthest open space in direction
// fn find_collision_down(grid: &[[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], current_position: &Position) -> usize {
//     for i in current_position.row+1..grid.len()-1 {
//         if grid[i+1][current_position.col] != 0 {
//             return i;
//         }
//     }
//     return grid.len()-1;
// }

fn hard_drop(piece: &mut Piece, grid: &mut [[usize; MATRIX_WIDTH]; MATRIX_HEIGHT]) {
    let mut min_fall_distance = grid.len();
    for (row, col) in piece.shape[piece.rotation].iter() {
        let new_row = (*row + piece.pos.row) as usize;
        let new_col = (*col + piece.pos.col) as usize;
        let mut fall_distance = 0;
        for i in new_row..grid.len() {
            if grid[i][new_col] != 0 {
                break;
            }
            fall_distance += 1;
        }
        min_fall_distance = min(min_fall_distance, fall_distance);
    }

    for (row, col) in piece.shape[piece.rotation].iter() {
        let new_row = (*row + piece.pos.row) as usize + min_fall_distance - 1;
        let new_col = (*col + piece.pos.col) as usize;
        grid[new_row][new_col] = 6;
    }
}

fn move_piece_h(piece: &mut Piece, grid: &[[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], direction: i32) {
    for (row, col) in piece.shape[piece.rotation].iter() {
        let new_row = (*row + piece.pos.row) as usize;
        let new_col = (*col + piece.pos.col + direction) as usize;
        // If the new_col is < 0 the the cast to usize makes it large so the first check handles out of bounds both left and right
        if new_col >= grid[0].len() || grid[new_row][new_col] != 0 {
            return;
        }
    }
    piece.pos.col += direction;
}

fn rotate_piece(piece: &mut Piece, grid: &[[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], rotation: usize) {
    let target_rotation = (piece.rotation + rotation) % 4;
    for (row, col) in piece.shape[target_rotation].iter() {
        let new_row = (*row + piece.pos.row) as usize;
        let new_col = (*col + piece.pos.col) as usize;
        // If the new_col is < 0 the the cast to usize makes it large so the first check handles out of bounds both left and right
        if new_col >= grid[0].len() || grid[new_row][new_col] != 0 {
            // Rotation causes collision, do wall kicks
            return;
        }
    }
    piece.rotation = target_rotation;
}

fn filled_rows(grid: &mut [[usize; MATRIX_WIDTH]; MATRIX_HEIGHT]) -> Vec<usize> {
    let mut remove = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        let mut count = 0;
        for value in row.iter() {
            if *value == 0 {
                break;
            }
            count += 1;
        }
        if count == grid[0].len() {
            remove.push(i);
        }
    }
    remove
}

fn remove_rows(grid: &mut [[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], remove: Vec<usize>) {
    for row in remove.iter() {
        // Empty the row
        for col in 0..grid[0].len() {
            grid[*row][col] = 0;
        }
        // Swap the row upward
        for current in (2..=*row).rev() {
            grid.swap(current, current-1);
        }
    }
}

// fn get_bag() -> [char; 7] {
//     let mut bag = ['I','T','O','J','L','S','Z'];
//     let mut rng = rand::thread_rng();
//     let len = bag.len();
//     for i in 0..len {
//         bag.swap(i, rng.gen_range(i..len));
//     }
//     bag
// }
