mod piece;
mod game;
mod input;

use std::{time::Duration};

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

    let mut input = input::Input {
        hard_drop: false,
        soft_drop: false,
        left: false,
        right: false,
        rot_cw: false,
        rot_ccw: false,
        rot_180: false,
    };

    let mut game = game::Game::new(MATRIX_WIDTH, MATRIX_HEIGHT);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown{..} | Event::KeyUp{..} => {
                    input::handle_input_event(&mut input, event);
                }
                _ => {},
            }
        }

        // Update
        game.update(&input);


        // Render
        render(&mut canvas, &blocks, &game)?;
        
        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, game: &game::Game) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let canvas_center = Point::new(width as i32 / 2, height as i32 / 2);

    for (i, row) in game.matrix.iter().enumerate() {
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

    for (row, col) in game.piece.get_orientation().iter() {
        let x = (*col as i32 + game.piece.position.col) * SCALE as i32;
        let y = (*row as i32 + game.piece.position.row) * SCALE as i32;
        let  block = Rect::new(0, 0, 16, 16);
        canvas.copy(&texture, block, Rect::new(x, y, SCALE, SCALE))?;
    }

    canvas.present();

    Ok(())
}
