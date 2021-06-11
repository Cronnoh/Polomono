mod piece;
mod game;
mod input;
use piece::PieceColor;

use std::time::Instant;

use sdl2::{
    rect::Rect,
    render::{WindowCanvas, Texture, BlendMode, TextureCreator},
    pixels::Color,
    event::Event,
    keyboard::Scancode,
    image::{InitFlag, LoadTexture},
    ttf::{Font}
};

const MATRIX_WIDTH: usize = 10;
const MATRIX_HEIGHT: usize = 20;
const OFFSCREEN_ROWS: usize = 5;
const SCALE: u32 = 32;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let ttf_context = sdl2::ttf::init()
        .map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("idk", 1184, 666)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut blocks_texture = texture_creator.load_texture("assets/tet.png")?;
    blocks_texture.set_blend_mode(BlendMode::Blend);
    let blocks_regions = block_texture_regions(&blocks_texture)?;

    let font = ttf_context.load_font("assets/Hack-Bold.ttf", 48)?;

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
        hold: false,
    };
    
    let mut game = game::Game::new(MATRIX_WIDTH, MATRIX_HEIGHT+OFFSCREEN_ROWS)?;
    let mut current_time = Instant::now();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        let elapsed = current_time.elapsed().as_micros();
        current_time = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { scancode: Some(Scancode::R), repeat: false, ..} => {
                    game = game::Game::new(MATRIX_WIDTH, MATRIX_HEIGHT+OFFSCREEN_ROWS)?;
                }
                Event::KeyDown{..} | Event::KeyUp{..} => {
                    input::handle_input_event(&mut input, event);
                }
                _ => {},
            }
        }

        input::update_held_times(&mut input, elapsed);

        game.update(&mut input, elapsed);

        let mut stat_textures = create_stat_textures(&game.stats, &font, &texture_creator)?;
        render(&mut canvas, &mut blocks_texture, &blocks_regions, &mut stat_textures, &game)?;
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

fn render(canvas: &mut WindowCanvas, texture: &mut Texture, regions: &Vec<Rect>, stat_textures: &mut Vec<Texture>, game: &game::Game) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();

    // let (width, height) = canvas.output_size()?;
    // let canvas_center = Point::new(width as i32 / 2, height as i32 / 2);

    texture.set_alpha_mod(255);
    for (i, row) in game.matrix.iter().enumerate().skip(OFFSCREEN_ROWS) {
        for (j, color) in row.iter().enumerate() {
            let x = (j as u32 * SCALE) as i32;
            let y = ((i as u32 - OFFSCREEN_ROWS as u32)* SCALE) as i32;
            canvas.copy(&texture, regions[*color as usize], Rect::new(x, y, SCALE, SCALE))?;
        }
    }

    /* 
    Ghost Piece is drawn transparently over a white background to brighten it up and create an outline.
    The regular piece is draw afterward so that it is on top when it intersects with the ghost piece.
    */

    // Draw ghost piece outline
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (row, col) in game.piece.get_orientation().iter() {
        let ghost_x = (*col + game.piece.position.col) as i32 * SCALE as i32;
        let ghost_y = ((*row + game.piece.ghost_position) as i32 - OFFSCREEN_ROWS as i32) * SCALE as i32;
        canvas.fill_rect(Rect::new(ghost_x-2, ghost_y-2, SCALE+4, SCALE+4))?;
    }

    // Draw ghost piece
    texture.set_alpha_mod(192);
    for (row, col) in game.piece.get_orientation().iter() {
        let ghost_x = (*col + game.piece.position.col) as i32 * SCALE as i32;
        let ghost_y = ((*row + game.piece.ghost_position) as i32 - OFFSCREEN_ROWS as i32) * SCALE as i32;
        canvas.copy(&texture, regions[game.piece.color as usize], Rect::new(ghost_x, ghost_y, SCALE, SCALE))?;
    }

    // Draw piece
    texture.set_alpha_mod(255);
    for (row, col) in game.piece.get_orientation().iter() {
        let x = (*col + game.piece.position.col) as i32 * SCALE as i32;
        let y = ((*row + game.piece.position.row) as i32 - OFFSCREEN_ROWS as i32) * SCALE as i32;
        canvas.copy(&texture, regions[game.piece.color as usize], Rect::new(x, y, SCALE, SCALE))?;
    }

    let preview_offset_x = 350;
    let preview_offset_y = 100;
    let preview_piece_seperation = 50;
    let size = (SCALE/2) as i32;

    for (i, piece) in game.get_preview_pieces().iter().rev().enumerate() {
        let next_piece = game.piece_data.get(piece).unwrap();
        for (row, col) in next_piece.shape[0].iter() {
            let x = *col as i32 * size as i32 + preview_offset_x;
            let y = *row as i32 * size as i32 + preview_piece_seperation * i as i32 + preview_offset_y;
            canvas.copy(&texture, regions[next_piece.color as usize], Rect::new(x, y, size as u32, size as u32))?;
        }
    }

    if let Some(held) = &game.held {
        for (row, col) in held.get_orientation().iter() {
            let x = *col as i32 * size as i32 + preview_offset_x;
            let y = *row as i32 * size as i32 + preview_offset_y + 500;
            canvas.copy(&texture, regions[held.color as usize], Rect::new(x, y, size as u32, size as u32))?;
        }
    }

    let vertical_stat_spacing = 60;
    for (i, texture) in stat_textures.iter_mut().enumerate() {
        let query = texture.query();
        let pos_x = 500;
        let pos_y = 300 + vertical_stat_spacing * i as i32;
        texture.set_color_mod(96, 96, 96);
        canvas.copy(&texture, None, Rect::new(pos_x+2, pos_y+2, query.width, query.height))?;
        texture.set_color_mod(255, 255, 255);
        canvas.copy(&texture, None, Rect::new(pos_x, pos_y, query.width, query.height))?;
    }

    canvas.present();

    Ok(())
}

fn format_time(microseconds: u128) -> String {
    let hundredths = (microseconds % 1000000) / 10000;
    let total_seconds = microseconds / 1000000;
    let seconds = total_seconds % 60;
    let minutes = total_seconds / 60;

    format!("{:>0width$}:{:>0width$}.{:>0width$}", minutes, seconds, hundredths, width=2)
}

fn create_stat_textures<'a, T>(stats: &game::Stats, font: &Font, texture_creator: &'a TextureCreator<T>) -> Result<Vec<Texture<'a>>, String> {
    let mut textures = Vec::new();
    let score_surface = font
        .render(&stats.score.to_string())
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;
    textures.push(texture_creator
        .create_texture_from_surface(&score_surface)
        .map_err(|e| e.to_string())?);

    let time_surface = font
        .render(&format_time(stats.time))
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;
    textures.push(texture_creator
        .create_texture_from_surface(&time_surface)
        .map_err(|e| e.to_string())?);

    let lines_cleared_surface = font
        .render(&stats.lines_cleared.to_string())
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;
    textures.push(texture_creator
        .create_texture_from_surface(&lines_cleared_surface)
        .map_err(|e| e.to_string())?);

    let pieces_placed_surface = font
        .render(&stats.pieces_placed.to_string())
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;
    textures.push(texture_creator
        .create_texture_from_surface(&pieces_placed_surface)
        .map_err(|e| e.to_string())?);        

    Ok(textures)
}
