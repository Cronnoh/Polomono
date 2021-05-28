use std::time::Duration;

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

    let mut grid = [[0; WIDTH]; HEIGHT];
    grid[5][5] = 1;
    grid[5][4] = 2;
    grid[7][4] = 3;
    grid[19][0] = 4;
    grid[0][0] = 5;
    grid[19][9] = 6;
    grid[0][9] = 7;

    let texture_creator = canvas.texture_creator();
    let blocks = texture_creator.load_texture("assets/tet.png")?;
                    
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
                    grid[9][8] = (grid[9][8] + 1) % 7;
                }
                Event::KeyDown { scancode: Some(Scancode::A), repeat: false, .. } => {
                    grid[8][8] = (grid[8][8] + 1) % 7;
                }
                Event::KeyDown { scancode: Some(Scancode::S), repeat: false, .. } => {
                    grid[7][8] = (grid[7][8] + 1) % 7;
                }
                Event::KeyDown { scancode: Some(Scancode::D), repeat: false, .. } => {
                    grid[6][8] = (grid[6][8] + 1) % 7;
                }
                _ => {}
            }
        }

        // Update

        // Render
        render(&mut canvas, &blocks, &grid)?;
        

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, grid: &[[u32; WIDTH]; HEIGHT]) -> Result<(), String> {
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
