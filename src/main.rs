use std::{
    cmp::{max, min},
    time::Duration
 };

use sdl2::{
    rect::{Rect, Point},
    render::{WindowCanvas, Texture},
    pixels::Color,
    event::Event,
    keyboard::Scancode,
    image::{InitFlag, LoadTexture},
};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const SCALE: u32 = 32;

#[derive(Debug)]
struct Inputs {
    hard_drop: bool,
    soft_drop: bool,
    left: bool,
    right: bool,
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

    let mut grid: [[usize; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
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

    let mut current_position = (0, 0);

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
        let mut direction = 0;
        if inputs.left {
            inputs.left = false;
            direction += -1;
        }
        if inputs.right {
            inputs.right = false;
            direction += 1
        }

        grid[current_position.1][current_position.0] = 0;
        let mut new_position = max(current_position.0 as i32 + direction, 0);
        new_position = min(new_position,  grid[0].len() as i32 - 1);
        current_position.0 = new_position as usize;
        grid[current_position.1][current_position.0] = 6;

        if inputs.hard_drop {
            hard_drop(&mut grid, current_position);
            inputs.hard_drop = false;
        }

        let remove = filled_rows(&mut grid);
        remove_rows(&mut grid, remove);

        // Render
        render(&mut canvas, &blocks, &grid)?;
        

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, grid: &[[usize; WIDTH]; HEIGHT]) -> Result<(), String> {
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

    canvas.present();

    Ok(())
}

fn hard_drop(grid: &mut [[usize; WIDTH]; HEIGHT], current_position: (usize, usize)) {
    for i in current_position.1+1..grid.len()-1 {
        if grid[i+1][current_position.0] != 0 {
            grid[i][current_position.0] = 6;
            return;
        }
    }
    grid[grid.len()-1][current_position.0] = 6;
}

fn filled_rows(grid: &mut [[usize; WIDTH]; HEIGHT]) -> Vec<usize> {
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

fn remove_rows(grid: &mut [[usize; WIDTH]; HEIGHT], remove: Vec<usize>) {
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
