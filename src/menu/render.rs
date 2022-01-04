use super::{Menu, assets::MenuAssets};

use sdl2::{pixels::Color, rect::Rect, render::{WindowCanvas, BlendMode}};

pub fn render(menu: &Menu, canvas: &mut WindowCanvas, assets: &MenuAssets) -> Result<(), String>{
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();
    for (row, i) in (-2..=2).enumerate() {
        let tile_index = menu.selected_index as i32 + i;
        let label = match assets.tile_labels.get(tile_index as usize) {
            Some(x) => x,
            None => continue,
        };

        // Draw Tile
        let query = label.query();
        let x_pos = 32 - 25 * i.abs();
        let y_pos = 9 + 72 * row as i32;
        let tile_dest = Rect::new(x_pos, y_pos, 320, 54);
        // Draw Label
        let label_dest = Rect::new(x_pos + 12, y_pos + 13, query.width, query.height);
        canvas.set_draw_color(assets.tile_colors[tile_index as usize]);
        canvas.fill_rect(tile_dest)?;
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
