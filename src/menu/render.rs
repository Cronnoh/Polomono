use super::{MENU_GRID, Menu, MenuTile};

use crate::assets::Assets;

use sdl2::{pixels::Color, rect::{Point, Rect}, render::WindowCanvas};

pub fn render(menu: &Menu, canvas: &mut WindowCanvas, assets: &Assets) {
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.clear();
    for (y, row) in MENU_GRID.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let selected = menu.grid_position.0 == x && menu.grid_position.1 == y;
            draw_tile(canvas, tile, selected, assets);
        }
    }
    canvas.present();
}

fn draw_tile(canvas: &mut WindowCanvas, tile: &MenuTile, selected: bool, assets: &Assets) {
    let (mut position, flip_h, flip_v) = match *tile {
        MenuTile::Option1 => (Rect::new(32, 36, 298, 72), false, false),
        MenuTile::Option2 => (Rect::new(320, 36, 298, 72), true, true),
        MenuTile::Option3 => (Rect::new(32, 144, 298, 72), false, false),
        MenuTile::Option4 => (Rect::new(320, 144, 298, 72), true, true),
        MenuTile::Option5 => (Rect::new(32, 252, 298, 72), false, false),
        MenuTile::Option6 => (Rect::new(320, 252, 298, 72), true, true),
    };

    let tile_sprite = if selected {
        match *tile {
            MenuTile::Option1 | MenuTile::Option3 | MenuTile::Option5 => {
                position.x -= 4;
            }
            MenuTile::Option2 | MenuTile::Option4 | MenuTile::Option6 => {
                position.x += 4;
            }
        }
        position.y -= 4;
        position.set_height(80);
        assets.menu_tile_selected
    } else {
        assets.menu_tile
    };

    canvas.copy_ex(&assets.menu_sheet, tile_sprite, position, 0.0, Point::new(0,0), flip_h, flip_v).unwrap();
}