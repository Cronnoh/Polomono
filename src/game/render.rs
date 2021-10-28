use super::Game;
use super::assets::GameAssets;
use super::piece::{Piece, shape_dimensions, shape_top_left};
use crate::OFFSCREEN_ROWS;

use sdl2::{
    pixels::Color,
    rect::{Rect, Point},
    render::{WindowCanvas, Texture},
};

const MATRIX_FRAME_WIDTH: usize = 160;
const MATRIX_FRAME_HEIGHT: usize = 320;

pub fn render(canvas: &mut WindowCanvas, game: &Game, assets: &mut GameAssets) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();

    // Scale the grid appropriately based on the size of the matrix
    let grid_square_size = std::cmp::min(MATRIX_FRAME_HEIGHT / (game.matrix.len() - OFFSCREEN_ROWS), MATRIX_FRAME_WIDTH / game.matrix[0].len()) as u32;

    draw_matrix(canvas, &game.matrix, grid_square_size, assets)?;
    draw_piece(canvas, &game.piece, grid_square_size, assets, game.ruleset.ghost_piece_enabled)?;
    draw_preview(canvas, game, assets)?;
    draw_held(canvas, game, assets)?;
    draw_stats(canvas, &game.stats, assets)?;
    draw_frame(canvas, assets)?;

    canvas.present();
    Ok(())
}

fn draw_matrix(canvas: &mut WindowCanvas, matrix: &crate::game::Matrix, grid_square_size: u32, assets: &mut GameAssets) -> Result<(), String> {
    let matrix_offset = Point::new(168, 16);

    assets.block_sheet.set_alpha_mod(255);
    for (i, row) in matrix.iter().skip(OFFSCREEN_ROWS).enumerate() {
        for (j, color) in row.iter().enumerate() {
            let point = Point::new(j as i32, i as i32) * grid_square_size as i32 + matrix_offset;
            canvas.copy(&assets.block_sheet, assets.block_sprites[*color as usize], Rect::new(point.x, point.y, grid_square_size, grid_square_size))?;
        }
    }

    Ok(())
}

fn draw_piece(canvas: &mut WindowCanvas, piece: &Piece, grid_square_size: u32, assets: &mut GameAssets, draw_ghost: bool) -> Result<(), String> {
    let matrix_offset = Point::new(168, 16);

    if draw_ghost {
        /*
        Ghost Piece is drawn transparently over a white background to brighten it up and create an outline.
        The regular piece is draw afterward so that it is on top when it intersects with the ghost piece.
        */

        // Draw ghost piece outline
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        assets.block_sheet.set_alpha_mod(255);
        for (col, row) in piece.get_orientation().iter() {
            if *row as i32 + piece.ghost_position >= OFFSCREEN_ROWS as i32 {
                let pos = get_grid_position(*col as i32 + piece.position.col, *row as i32 + piece.ghost_position - OFFSCREEN_ROWS as i32, grid_square_size, matrix_offset);
                canvas.fill_rect(Rect::new(pos.x-1, pos.y-1, grid_square_size+2, grid_square_size+2))?;
            }
        }

        // Draw ghost piece
        assets.block_sheet.set_alpha_mod(192);
        for (col, row) in piece.get_orientation().iter() {
            if *row as i32 + piece.ghost_position >= OFFSCREEN_ROWS as i32 {
                let pos = get_grid_position(*col as i32 + piece.position.col, *row as i32 + piece.ghost_position - OFFSCREEN_ROWS as i32, grid_square_size, matrix_offset);
                canvas.copy(&assets.block_sheet, assets.block_sprites[piece.color as usize], Rect::new(pos.x, pos.y, grid_square_size, grid_square_size))?;
            }
        }
    }

    // Draw piece
    assets.block_sheet.set_alpha_mod(255);
    for (col, row) in piece.get_orientation().iter() {
        if *row as i32 + piece.position.row >= OFFSCREEN_ROWS as i32 {
            let pos = get_grid_position(*col as i32 + piece.position.col, *row as i32 + piece.position.row - OFFSCREEN_ROWS as i32, grid_square_size, matrix_offset);
            canvas.copy(&assets.block_sheet, assets.block_sprites[piece.color as usize], Rect::new(pos.x, pos.y, grid_square_size, grid_square_size))?;
        }
    }

    Ok(())
}

fn draw_preview(canvas: &mut WindowCanvas, game: &Game, assets: &mut GameAssets) -> Result<(), String> {
    let preview_offset_x = 336;
    let preview_offset_y = 16;
    let preview_piece_box_size = 48;
    let size = 10;

    for (i, piece) in game.get_preview_pieces().iter().rev().enumerate() {
        let offset_y = preview_offset_y + (preview_piece_box_size * i);
        draw_centered_piece(canvas, &assets.block_sheet, &assets.block_sprites[piece.color as usize], &piece.shape[0], preview_offset_x, offset_y, preview_piece_box_size, size)?;
    }
    Ok(())
}

fn draw_held(canvas: &mut WindowCanvas, game: &Game, assets: &mut GameAssets) -> Result<(), String> {
    if let Some(held) = &game.held {
        let hold_offset_x = 112;
        let hold_offset_y = 16;
        let hold_box_size = 48;
        let size = 10;

        draw_centered_piece(canvas, &assets.block_sheet, &assets.block_sprites[held.color as usize], &held.shape[0], hold_offset_x, hold_offset_y, hold_box_size, size)?;
    }
    Ok(())
}

fn draw_stats(canvas: &mut WindowCanvas, stats: &crate::game::Stats, assets: &mut GameAssets) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let mut stat_textures = assets.create_stat_textures(&stats, &texture_creator)?;
    let stats_offset_x = 440;
    let stats_offset_y = 28;
    let label_number_spacing = 18;
    let vertical_stat_spacing = 75;

    for (i, (number, label)) in stat_textures.iter_mut().zip(&assets.stat_labels).enumerate() {
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

fn draw_frame(canvas: &mut WindowCanvas, assets: &GameAssets) -> Result<(), String> {
    let frame_x = 104;
    let frame_y = 8;
    let query = assets.frame.query();
    canvas.copy(&assets.frame, None, Rect::new(frame_x, frame_y, query.width, query.height))
}

fn get_grid_position(column: i32, row: i32, grid_square_size: u32, matrix_offset: Point) -> Point {
    let x = column as i32 * grid_square_size as i32 + matrix_offset.x;
    let y = row as i32 * grid_square_size as i32 + matrix_offset.y;

    Point::new(x, y)
}

fn draw_centered_piece(
    canvas: &mut WindowCanvas,
    block_sheet: &Texture,
    color: &Rect,
    shape: &[(i8, i8)],
    offset_x: usize,
    offset_y: usize, 
    container_size: usize,
    block_size: u32
) -> Result<(), String> {

    let (width, height) = shape_dimensions(shape);
    let (top_left_x, top_left_y) = shape_top_left(shape);
    let centering_offset_x = get_centered_offset(offset_x, container_size, width, block_size as usize);
    let centering_offset_y = get_centered_offset(offset_y, container_size, height, block_size as usize);

    for (col, row) in shape.iter() {
        let x = (*col as i32 - top_left_x) * block_size as i32 + centering_offset_x as i32;
        let y = (*row as i32 - top_left_y) * block_size as i32 + centering_offset_y as i32;
        canvas.copy(block_sheet, *color, Rect::new(x, y, block_size, block_size))?;
    }
    Ok(())
}

fn get_centered_offset(offset: usize, container_size: usize, dimension: usize, size: usize) -> usize {
    offset + container_size / 2 - dimension * size / 2
}
