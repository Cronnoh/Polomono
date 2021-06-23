use crate::piece::{PieceColor, shape_dimensions, shape_top_left};
use crate::game::Game;
use crate::OFFSCREEN_ROWS;

use std::path::Path;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::{Rect, Point},
    render::{WindowCanvas, Texture, BlendMode, TextureCreator},
    ttf::Font,
};

pub struct Assets<'a, 'b> {
    block_sheet: Option<Texture<'a>>,
    block_sprites: Option<Vec<Rect>>,
    stat_font: Option<Font<'a, 'b>>,
    stat_labels: Option<Vec<Texture<'a>>>,
    frame: Option<Texture<'a>>,
}

impl<'a, 'b> Assets<'a, 'b> {
    pub fn new() -> Self {
        Self {
            block_sheet: None,
            block_sprites: None,
            stat_font: None,
            stat_labels: None,
            frame: None,
        }
    }

    pub fn load_block_textures(&mut self, texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<(), String> {
        self.block_sheet = Some(texture_creator.load_texture(path)?);
        if let Some(sheet) = &mut self.block_sheet {
            self.block_sprites = Some(block_texture_regions(&sheet)?);
            sheet.set_blend_mode(BlendMode::Blend);
        }
        Ok(())
    }

    pub fn load_font(&mut self, ttf_context: &'a Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<(), String> {
        self.stat_font = Some(ttf_context.load_font(path, 28)?);
        let label_font = ttf_context.load_font(path, 18)?;
        let label_color = Color::RGB(144, 144, 144);
        self.stat_labels = Some(vec![
            create_text_texture("Score", label_color, &label_font, &texture_creator)?,
            create_text_texture("Time", label_color, &label_font, &texture_creator)?,
            create_text_texture("Lines", label_color, &label_font, &texture_creator)?,
            create_text_texture("Pieces", label_color, &label_font, &texture_creator)?,
        ]);

        Ok(())
    }

    pub fn load_frame(&mut self, texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<(), String> {
        self.frame = Some(texture_creator.load_texture(path)?);
        Ok(())
    }

    fn get_block_textures(&mut self) -> Result<(&mut Texture<'a>, &Vec<Rect>), String> {
        let block_sheet = match &mut self.block_sheet {
            Some(x) => x,
            None => return Err("Block Spritesheet not loaded".to_string()),
        };
        let block_sprites = match &self.block_sprites {
            Some(x) => x,
            None => return Err("Block Sprites not loaded".to_string()),
        };

        Ok((block_sheet, block_sprites))
    }

    fn get_font(&self) -> Result<&Font, String> {
        match &self.stat_font {
            Some(x) => Ok(x),
            None => Err("Font is not loaded".to_string()),
        }
    }

    fn get_stat_labels(&self) -> Result<&Vec<Texture>, String> {
        match &self.stat_labels {
            Some(x) => Ok(x),
            None => Err("Stat labels not loaded".to_string()),
        }
    }

    fn get_frame(&self) -> Result<&Texture, String>{
        match &self.frame {
            Some(x) => Ok(x),
            None => Err("Frame is not loaded".to_string()),
        } 
    }

    fn create_stat_textures(&self, stats: &crate::game::Stats, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Vec<Texture<'a>>, String> {
        let font = self.get_font()?;
        let color = Color::RGB(255, 255, 255);
        let textures = vec![
            create_text_texture(&stats.score.to_string(), color, font, &texture_creator)?,
            create_text_texture(&format_time(stats.time), color, font, &texture_creator)?,
            create_text_texture(&stats.lines_cleared.to_string(), color, font, &texture_creator)?,
            create_text_texture(&stats.pieces_placed.to_string(), color, font, &texture_creator)?,
        ];

        Ok(textures)
    }
}

pub fn render(canvas: &mut WindowCanvas, game: &Game, assets: &mut Assets) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();

    // Scale the grid appropriately based on the size of the matrix
    let grid_square_size = std::cmp::min(16 * 20 / (game.matrix.len() - OFFSCREEN_ROWS), 16 * 10 / game.matrix[0].len()) as u32;
    let mut block_assets = assets.get_block_textures()?;

    draw_matrix(canvas, &game.matrix, grid_square_size, &mut block_assets)?;
    draw_piece(canvas, &game.piece, grid_square_size, &mut block_assets)?;
    draw_preview(canvas, game, &block_assets)?;
    draw_held(canvas, game, &block_assets)?;
    draw_stats(canvas, &game.stats, assets)?;
    draw_frame(canvas, assets)?;

    canvas.present();
    Ok(())
}

fn draw_matrix(canvas: &mut WindowCanvas, matrix: &crate::game::Matrix, grid_square_size: u32, assets: &mut (&mut Texture, &Vec<Rect>)) -> Result<(), String> {
    let matrix_offset = Point::new(168, 16);
    let (block_sheet, block_sprites) = assets;

    block_sheet.set_alpha_mod(255);
    for (i, row) in matrix.iter().skip(OFFSCREEN_ROWS).enumerate() {
        for (j, color) in row.iter().enumerate() {
            let point = Point::new(j as i32, i as i32) * grid_square_size as i32 + matrix_offset;
            canvas.copy(block_sheet, block_sprites[*color as usize], Rect::new(point.x, point.y, grid_square_size, grid_square_size))?;
        }
    }

    Ok(())
}

fn draw_piece(canvas: &mut WindowCanvas, piece: &crate::piece::Piece, grid_square_size: u32, assets: &mut (&mut Texture, &Vec<Rect>)) -> Result<(), String> {
    let matrix_offset = Point::new(168, 16);
    let (block_sheet, block_sprites) = assets;

    /*
    Ghost Piece is drawn transparently over a white background to brighten it up and create an outline.
    The regular piece is draw afterward so that it is on top when it intersects with the ghost piece.
    */

    // Draw ghost piece outline
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    block_sheet.set_alpha_mod(255);
    for (col, row) in piece.get_orientation().iter() {
        if *row as i32 + piece.ghost_position >= OFFSCREEN_ROWS as i32 {
            let pos = get_grid_position(*col as i32 + piece.position.col, *row as i32 + piece.ghost_position - OFFSCREEN_ROWS as i32, grid_square_size, matrix_offset);
            canvas.fill_rect(Rect::new(pos.x-1, pos.y-1, grid_square_size+2, grid_square_size+2))?;
        }
    }

    // Draw ghost piece
    block_sheet.set_alpha_mod(192);
    for (col, row) in piece.get_orientation().iter() {
        if *row as i32 + piece.ghost_position >= OFFSCREEN_ROWS as i32 {
            let pos = get_grid_position(*col as i32 + piece.position.col, *row as i32 + piece.ghost_position - OFFSCREEN_ROWS as i32, grid_square_size, matrix_offset);
            canvas.copy(block_sheet, block_sprites[piece.color as usize], Rect::new(pos.x, pos.y, grid_square_size, grid_square_size))?;
        }
    }

    // Draw piece
    block_sheet.set_alpha_mod(255);
    for (col, row) in piece.get_orientation().iter() {
        if *row as i32 + piece.position.row >= OFFSCREEN_ROWS as i32 {
            let pos = get_grid_position(*col as i32 + piece.position.col, *row as i32 + piece.position.row - OFFSCREEN_ROWS as i32, grid_square_size, matrix_offset);
            canvas.copy(block_sheet, block_sprites[piece.color as usize], Rect::new(pos.x, pos.y, grid_square_size, grid_square_size))?;
        }
    }

    Ok(())
}

fn draw_preview(canvas: &mut WindowCanvas, game: &Game, assets: &(&mut Texture, &Vec<Rect>)) -> Result<(), String> {
    let (block_sheet, block_sprites) = assets;
    let preview_offset_x = 336;
    let preview_offset_y = 16;
    let preview_piece_box_size = 48;
    let size = 10;

    for (i, piece) in game.get_preview_pieces().iter().rev().enumerate() {
        let next_piece = game.piece_data.get(piece).unwrap();
        let (width, height) = shape_dimensions(&next_piece.shape[0]);
        let (top_left_x, top_left_y) = shape_top_left(&next_piece.shape[0]);
        let centering_offset_x = get_centered_offset(preview_offset_x, preview_piece_box_size, width, size as usize);
        let centering_offset_y = get_centered_offset(preview_offset_y, preview_piece_box_size, height, size as usize);
        for (col, row) in next_piece.shape[0].iter() {
            let x = (*col as i32 - top_left_x) * size as i32 + centering_offset_x as i32;
            let y = (*row as i32 - top_left_y) * size as i32 + centering_offset_y as i32 + (preview_piece_box_size * i) as i32;
            canvas.copy(block_sheet, block_sprites[next_piece.color as usize], Rect::new(x, y, size, size))?;
        }
    }
    Ok(())
}

fn draw_held(canvas: &mut WindowCanvas, game: &Game, assets: &(&mut Texture, &Vec<Rect>)) -> Result<(), String> {
    if let Some(held) = &game.held {
        let (block_sheet, block_sprites) = assets;
        let hold_offset_x = 112;
        let hold_offset_y = 16;
        let hold_box_size = 48;
        let size = 10;

        let (width, height) = shape_dimensions(&held.shape[0]);
        let (top_left_x, top_left_y) = shape_top_left(&held.shape[0]);
        let centering_offset_x = get_centered_offset(hold_offset_x, hold_box_size, width, size as usize);
        let centering_offset_y = get_centered_offset(hold_offset_y, hold_box_size, height, size as usize);

        for (col, row) in held.get_orientation().iter() {
            let x = (*col as i32 - top_left_x) * size as i32 + centering_offset_x as i32;
            let y = (*row as i32 - top_left_y) * size as i32 + centering_offset_y as i32;
            canvas.copy(block_sheet, block_sprites[held.color as usize], Rect::new(x, y, size, size))?;
        }
    }
    Ok(())
}

fn draw_stats(canvas: &mut WindowCanvas, stats: &crate::game::Stats, assets: &mut Assets) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let mut stat_textures = assets.create_stat_textures(&stats, &texture_creator)?;
    let stats_offset_x = 440;
    let stats_offset_y = 28;
    let label_number_spacing = 18;
    let vertical_stat_spacing = 75;
    let labels = assets.get_stat_labels()?;

    for (i, (number, label)) in stat_textures.iter_mut().zip(labels).enumerate() {
        let label_query = label.query();
        let label_y = stats_offset_y + vertical_stat_spacing * i as i32;
        canvas.copy(&label, None, Rect::new(stats_offset_x, label_y, label_query.width, label_query.height))?;

        let query = number.query();
        let number_y = label_y + label_number_spacing;
        number.set_color_mod(96, 96, 96);
        canvas.copy(&number, None, Rect::new(stats_offset_x+1, number_y+1, query.width, query.height))?;
        number.set_color_mod(255, 255, 255);
        canvas.copy(&number, None, Rect::new(stats_offset_x, number_y, query.width, query.height))?;
    }

    Ok(())
}

fn draw_frame(canvas: &mut WindowCanvas, assets: &Assets) -> Result<(), String> {
    let frame_x = 104;
    let frame_y = 8;
    let texture = assets.get_frame()?;
    let query = texture.query();
    canvas.copy(texture, None, Rect::new(frame_x, frame_y, query.width, query.height))
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

fn create_text_texture<'a, T>(text: &str, color: Color, font: &Font, texture_creator: &'a TextureCreator<T>) -> Result<Texture<'a>, String>{
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
}

fn format_time(microseconds: u128) -> String {
    let hundredths = (microseconds % 1000000) / 10000;
    let total_seconds = microseconds / 1000000;
    let seconds = total_seconds % 60;
    let minutes = total_seconds / 60;

    format!("{:>0width$}:{:>0width$}.{:>0width$}", minutes, seconds, hundredths, width=2)
}

fn get_grid_position(column: i32, row: i32, grid_square_size: u32, matrix_offset: Point) -> Point {
    let x = column as i32 * grid_square_size as i32 + matrix_offset.x;
    let y = row as i32 * grid_square_size as i32 + matrix_offset.y;

    Point::new(x, y)
}

fn get_centered_offset(offset: usize, container_size: usize, dimension: usize, size: usize) -> usize {
    offset + container_size / 2 - dimension * size / 2
}
