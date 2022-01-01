use super::{Menu, assets::MenuAssets};

use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

pub fn render(menu: &Menu, canvas: &mut WindowCanvas, assets: &MenuAssets) -> Result<(), String>{
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();
    for (row, i) in (-2..=2).enumerate() {
        let tile_index = menu.selected_index as i32 + i;
        let label = match assets.tile_labels.get(tile_index as usize) {
            Some(x) => x,
            None => continue,
        };
        let query = label.query();
        let position = Rect::new(32, 9+72*row as i32, query.width, query.height);
        if i != 0 {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.fill_rect(position)?;
        }
        canvas.copy(label, None, position)?;
    }
    canvas.present();
    Ok(())
}
