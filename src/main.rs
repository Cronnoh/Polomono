mod pieces;

use std::{
    cmp::{max, min},
    time::Duration
};

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
}

struct Position {
    row: usize,
    col: usize,
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

    let mut grid: [[usize; MATRIX_WIDTH]; MATRIX_HEIGHT] = [[0; MATRIX_WIDTH]; MATRIX_HEIGHT];
    grid[16][5] = 1;
    grid[15][4] = 2;
    grid[15][5] = 3;
    grid[19][0] = 4;
    grid[0][0] = 5;
    grid[19][9] = 6;
    grid[0][9] = 7;

    let texture_creator = canvas.texture_creator();
    let blocks = texture_creator.load_texture("assets/tet.png")?;

    let mut inputs = Inputs {
        hard_drop: false,
        soft_drop: false,
        left: false,
        right: false
    };

    let piece_list = pieces::PieceList::new();

    let mut current_piece = Piece {
        pos: Position {row: 5, col: 4},
        shape: &piece_list.I_type,
        rotation: 0,
    };

    let mut current_position = Position{row: 0, col: 0};

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
                _ => {}
            }
        }

        // Update
        if inputs.soft_drop {
            current_piece.rotation = (current_piece.rotation + 1) % 4;
        }
        if inputs.right {
            current_piece.pos.col += 1;
        }
        if inputs.left {
            current_piece.pos.col -= 1;
        }
        update(&mut grid, &mut inputs, &mut current_position);


        // Render
        render(&mut canvas, &blocks, &grid, &current_piece)?;
        
        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 15));
    }

    Ok(())
}

fn update(grid: &mut [[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], inputs: &mut Inputs, current_position: &mut Position) {
    if inputs.hard_drop {
        hard_drop(grid, &current_position);
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

        // grid[current_position.row][current_position.col] = 0;
        // let mut new_position = max(current_position.col as i32 + direction, 0);
        // new_position = min(new_position,  grid[0].len() as i32 - 1);
        // current_position.col = new_position as usize;
        // grid[current_position.row][current_position.col] = 6;

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
        let x = ((col + piece.pos.col) as u32 * SCALE) as i32;
        let y = ((row + piece.pos.row) as u32 * SCALE) as i32;
        let  block = Rect::new(0, 0, 16, 16);
        canvas.copy(&texture, block, Rect::new(x, y, SCALE, SCALE))?;
    }

    canvas.present();

    Ok(())
}

fn hard_drop(grid: &mut [[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], current_position: &Position) {
    let row = find_collision_down(&grid, &current_position);
    grid[row][current_position.col] = 6;
}

// Returns farthest open space in direction
fn find_collision_down(grid: &[[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], current_position: &Position) -> usize {
    for i in current_position.row+1..grid.len()-1 {
        if grid[i+1][current_position.col] != 0 {
            return i;
        }
    }
    return grid.len()-1;
}

fn find_collision_right(grid: &[[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], current_position: &Position) -> usize {
    for i in current_position.col+1..grid[0].len()-1 {
        if grid[current_position.row][i+1] != 0 {
            return i;
        }
    }
    return grid[0].len()-1;
}

fn find_collision_left(grid: &[[usize; MATRIX_WIDTH]; MATRIX_HEIGHT], current_position: &Position) -> usize {
    for i in (1..=current_position.col-1).rev() {
        if grid[current_position.row][i-1] != 0 {
            return i;
        }
    }
    return 0;
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

fn get_bag() -> [char; 7] {
    let mut bag = ['I','T','O','J','L','S','Z'];
    let mut rng = rand::thread_rng();
    let len = bag.len();
    for i in 0..len {
        bag.swap(i, rng.gen_range(i..len));
    }
    bag
}

/*
Moving to the left
    find the left-most block in every row of the piece
    find the distance each of these could move without colliding
    move the piece to the minimum of these values 
*/
