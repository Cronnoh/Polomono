mod piece;
mod game;
mod input;

use std::time::Instant;

use piece::PieceColor;
use sdl2::{
    rect::Rect,
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
    let blocks_texture = texture_creator.load_texture("assets/tet.png")?;
    let blocks_regions = block_texture_regions(&blocks_texture)?;

    let mut input = input::Input {
        hard_drop: false,
        soft_drop: false,
        left: false,
        left_held: 0,
        right: false,
        right_held: 0,
        rot_cw: false,
        rot_ccw: false,
        rot_180: false,
    };

    let mut game = game::Game::new(MATRIX_WIDTH, MATRIX_HEIGHT);
    let mut current_time = Instant::now();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        let elapsed = current_time.elapsed().as_millis();
        current_time = Instant::now();

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

        input::update_held_times(&mut input, elapsed);

        game.update(&mut input, elapsed);

        render(&mut canvas, &blocks_texture, &blocks_regions, &game)?;

    }

    Ok(())
}

fn block_texture_regions(texture: &Texture) -> Result<Vec<Rect>, String> {
    let mut regions = Vec::new();
    let query = texture.query();

    for i in 0..PieceColor::ColorCount as i32 {
        let offset = i*query.height as i32;
        if offset >= query.width as i32 {
            return Err("Block texture file is not properly formed".to_string());
        }
        regions.push(Rect::new(offset, 0, query.height, query.height));
    }

    Ok(regions)
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, regions: &Vec<Rect>, game: &game::Game) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();

    // let (width, height) = canvas.output_size()?;
    // let canvas_center = Point::new(width as i32 / 2, height as i32 / 2);

    for (i, row) in game.matrix.iter().enumerate() {
        for (j, color) in row.iter().enumerate() {
            let x = (j as u32 * SCALE) as i32;
            let y = (i as u32 * SCALE) as i32;
            canvas.copy(&texture, regions[*color as usize], Rect::new(x, y, SCALE, SCALE))?;
        }
    }

    for (row, col) in game.piece.get_orientation().iter() {
        let x = (*col as i32 + game.piece.position.col) * SCALE as i32;
        let y = (*row as i32 + game.piece.position.row) * SCALE as i32;
        canvas.copy(&texture, regions[game.piece.color as usize], Rect::new(x, y, SCALE, SCALE))?;
    }

    canvas.present();

    Ok(())
}
