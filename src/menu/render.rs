use super::{Menu, assets::MenuAssets};

use sdl2::{pixels::Color, rect::Rect, render::{WindowCanvas, BlendMode}};

pub fn render(menu: &Menu, canvas: &mut WindowCanvas, assets: &MenuAssets) -> Result<(), String>{
    canvas.set_draw_color(Color::RGB(48, 64, 96));
    canvas.clear();
    canvas.copy(&assets.menu_bg, None, None)?;

    let tile_width = 320;
    let tile_height = 64;

    for (row, i) in (-3..=3).enumerate() {
        let tile_index = menu.selected_index as i32 + i;
        let label = match assets.tile_labels.get(tile_index as usize) {
            Some(x) => x,
            None => continue,
        };

        // Draw Tile
        let x_pos = 32 - 23 * i.abs();
        let y_pos = tile_height as i32 * row as i32 - tile_height as i32/2 - 12;
        let tile_dest = Rect::new(x_pos, y_pos, tile_width, tile_height);
        // Draw Label
        let query = label.query();
        let label_dest = Rect::new(x_pos + 12, y_pos + 13, query.width, query.height);
        canvas.set_draw_color(assets.tile_colors[tile_index as usize]);
        canvas.fill_rect(tile_dest)?;
        canvas.copy(&assets.menu_tile_overlay, None, tile_dest)?;
        canvas.copy(label, None, label_dest)?;
        // Darken unselected tiles
        if i != 0 {
            canvas.set_draw_color(Color::RGBA(0, 0, 32, 164));
            canvas.set_blend_mode(BlendMode::Blend);
            canvas.fill_rect(tile_dest)?;
            canvas.set_blend_mode(BlendMode::None);
        }
    }
    canvas.present();
    Ok(())
}
