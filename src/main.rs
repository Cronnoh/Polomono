mod piece;
mod game;
mod input;
mod randomizer;
use input::GameInput;
use piece::{Piece, PieceColor};

use std::{collections::HashMap, path::Path, time::Instant};

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Scancode, pixels::Color,
    rect::{Rect, Point},
    render::{WindowCanvas, Texture, BlendMode, TextureCreator},
    ttf::Font,
};
use serde::{Deserialize, de::DeserializeOwned};

const OFFSCREEN_ROWS: usize = 5;

#[derive(Deserialize)]
pub struct Config {
    matrix_height: usize,
    matrix_width: usize,

    das: u32,
    arr: u32,
    gravity: u32,
    lock_delay: u32,
    preview_count: usize,

    piece_list: Vec<String>,
    cannot_start_with: Option<Vec<String>>,
    starting_randomizer: Option<randomizer::RandomizerStyle>,
    randomizer: randomizer::RandomizerStyle,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let game_controller_subsystem = sdl_context.game_controller()?;
    let _controller = input::open_game_controller(game_controller_subsystem)?;

    let ttf_context = sdl2::ttf::init()
        .map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("idk", 1280, 720)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_logical_size(640, 360)
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut blocks_texture = texture_creator.load_texture("assets/tet.png")?;
    blocks_texture.set_blend_mode(BlendMode::Blend);
    let blocks_regions = block_texture_regions(&blocks_texture)?;

    let font = ttf_context.load_font("assets/Hack-Bold.ttf", 24)?;

    let config: Config = load_data(Path::new("config.toml"))?;
    let bindings: HashMap<String, GameInput> = load_data(Path::new("control_config.toml"))?;

    let mut input = input::Input::new();

    let mut game = game::Game::new(&config)?;
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
                Event::KeyDown{ repeat: false, ..} | Event::KeyUp{ repeat: false, ..}
                | Event::ControllerButtonDown{..} | Event::ControllerButtonUp{..} => {
                    input::handle_input_event(&mut input, event, &bindings);
                }
                _ => {},
            }
        }

        if input.reset {
            input.reset = false;
            game = game::Game::new(&config)?;
        }

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

fn render(canvas: &mut WindowCanvas, texture: &mut Texture, regions: &[Rect], stat_textures: &mut Vec<Texture>, game: &game::Game) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();

    let grid_square_size = 16;
    let matrix_offset = Point::new(168, 16);

    texture.set_alpha_mod(255);
    for (i, row) in game.matrix.iter().skip(OFFSCREEN_ROWS).enumerate() {
        for (j, color) in row.iter().enumerate() {
            let point = Point::new(j as i32, i as i32) * grid_square_size as i32 + matrix_offset;
            canvas.copy(&texture, regions[*color as usize], Rect::new(point.x, point.y, grid_square_size, grid_square_size))?;
        }
    }

    draw_piece(canvas, &game.piece, grid_square_size, matrix_offset, texture, &regions)?;

    canvas.set_draw_color(Color::RGB(96, 96, 96));
    let preview_offset_x = matrix_offset.x + (game.matrix[0].len() as i32 + 1) * grid_square_size as i32;
    let preview_offset_y = 16;
    let preview_piece_seperation = 4 * (grid_square_size/2) as i32;
    let hold_offset_x = 112;
    let size = grid_square_size/2;

    for (i, piece) in game.get_preview_pieces().iter().rev().enumerate() {
        let next_piece = game.piece_data.get(piece).unwrap();
        canvas.set_draw_color(Color::RGB(96, i as u8*20, 96));
        canvas.fill_rect(Rect::new(preview_offset_x, preview_piece_seperation * i as i32 + preview_offset_y, 48, 48))?;
        for (col, row) in next_piece.shape[0].iter() {
            let x = *col as i32 * size as i32 + preview_offset_x;
            let y = *row as i32 * size as i32 + preview_piece_seperation * i as i32 + preview_offset_y;
            canvas.copy(&texture, regions[next_piece.color as usize], Rect::new(x, y, size, size))?;
        }
    }

    canvas.fill_rect(Rect::new(hold_offset_x, preview_offset_y, 48, 48))?;
    if let Some(held) = &game.held {
        for (col, row) in held.get_orientation().iter() {
            let x = *col as i32 * size as i32 + hold_offset_x;
            let y = *row as i32 * size as i32 + preview_offset_y;
            canvas.copy(&texture, regions[held.color as usize], Rect::new(x, y, size, size))?;
        }
    }

    let vertical_stat_spacing = 20;
    for (i, texture) in stat_textures.iter_mut().enumerate() {
        let query = texture.query();
        let pos_x = preview_offset_x;
        let pos_y = vertical_stat_spacing * i as i32 + matrix_offset.y + (15 * grid_square_size as i32);
        texture.set_color_mod(96, 96, 96);
        canvas.copy(&texture, None, Rect::new(pos_x+1, pos_y+1, query.width, query.height))?;
        texture.set_color_mod(255, 255, 255);
        canvas.copy(&texture, None, Rect::new(pos_x, pos_y, query.width, query.height))?;
    }

    canvas.present();

    Ok(())
}

fn get_grid_position(column: i8, row: i8, grid_square_size: u32, matrix_offset: Point) -> Point {
    let x = column as i32 * grid_square_size as i32 + matrix_offset.x;
    let y = row as i32 * grid_square_size as i32 + matrix_offset.y;

    Point::new(x, y)
}

fn draw_piece(canvas: &mut WindowCanvas, piece: &Piece, grid_square_size: u32, matrix_offset: Point, texture: &mut Texture, regions: &[Rect]) -> Result<(), String> {
    /*
    Ghost Piece is drawn transparently over a white background to brighten it up and create an outline.
    The regular piece is draw afterward so that it is on top when it intersects with the ghost piece.
    */

    // Draw ghost piece outline
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    texture.set_alpha_mod(255);
    for (col, row) in piece.get_orientation().iter() {
        let pos = get_grid_position(*col + piece.position.col, *row + piece.ghost_position - OFFSCREEN_ROWS as i8, grid_square_size, matrix_offset);
        canvas.fill_rect(Rect::new(pos.x-1, pos.y-1, grid_square_size+2, grid_square_size+2))?;
    }

    // Draw ghost piece
    texture.set_alpha_mod(192);
    for (col, row) in piece.get_orientation().iter() {
        let pos = get_grid_position(*col + piece.position.col, *row + piece.ghost_position - OFFSCREEN_ROWS as i8, grid_square_size, matrix_offset);
        canvas.copy(&texture, regions[piece.color as usize], Rect::new(pos.x, pos.y, grid_square_size, grid_square_size))?;
    }

    // Draw piece
    texture.set_alpha_mod(255);
    for (col, row) in piece.get_orientation().iter() {
        let pos = get_grid_position(*col + piece.position.col, *row + piece.position.row - OFFSCREEN_ROWS as i8, grid_square_size, matrix_offset);
        canvas.copy(&texture, regions[piece.color as usize], Rect::new(pos.x, pos.y, grid_square_size, grid_square_size))?;
    }

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
    let textures = vec![
        create_text_texture(&stats.score.to_string(), font, texture_creator)?,
        create_text_texture(&format_time(stats.time), font, texture_creator)?,
        create_text_texture(&stats.lines_cleared.to_string(), font, texture_creator)?,
        create_text_texture(&stats.pieces_placed.to_string(), font, texture_creator)?,
    ];

    Ok(textures)
}

fn create_text_texture<'a, T>(text: &str, font: &Font, texture_creator: &'a TextureCreator<T>) -> Result<Texture<'a>, String>{
    let surface = font
        .render(text)
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;
    texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
}

fn load_data<T: DeserializeOwned>(file_path: &std::path::Path) -> Result<T, String> {
    let data_file = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Error opening {}: {}", file_path.to_str().unwrap(), e.to_string()))?;
    toml::from_str(&data_file)
        .map_err(|e| format!("Error reading {}: {}", file_path.to_str().unwrap(), e.to_string()))
}
