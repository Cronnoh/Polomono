use crate::input::MenuInput;

use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};
use enum_map::EnumMap;

const GRID_COLUMNS: usize = 2;
const GRID_ROWS: usize = 3;

const MENU_GRID: [[MenuTile; GRID_COLUMNS]; GRID_ROWS] = [
    [MenuTile::Option1, MenuTile::Option2],
    [MenuTile::Option3, MenuTile::Option4],
    [MenuTile::Option5, MenuTile::Option6],
];

#[derive(Debug)]
enum MenuTile {
    Option1,
    Option2,
    Option3,
    Option4,
    Option5,
    Option6,
}

pub enum MenuStatus {
    Exit,
    Continue,
    Game,
    Settings,
} 

pub struct Menu {
    pub grid_position: (usize, usize),
}

impl Menu {
    pub fn update(&mut self, input: &mut EnumMap<MenuInput, bool>) -> MenuStatus {
        let movement_h = match (input[MenuInput::Left], input[MenuInput::Right]) {
            (true, false) => {
                input[MenuInput::Left] = false;
                -1
            }
            (false, true) => {
                input[MenuInput::Right] = false;
                1
            }
            _ => 0,
        };

        let movement_v = match (input[MenuInput::Up], input[MenuInput::Down]) {
            (true, false) => {
                input[MenuInput::Up] = false;
                -1
            }
            (false, true) => {
                input[MenuInput::Down] = false;
                1
            }
            _ => 0,
        };

        let new_x = (((self.grid_position.0 as i32 + movement_h) + GRID_COLUMNS as i32) % GRID_COLUMNS as i32) as usize;
        let new_y = (((self.grid_position.1 as i32 + movement_v) + GRID_ROWS as i32) % GRID_ROWS as i32) as usize;
        self.grid_position = (new_x, new_y);

        match (input[MenuInput::Accept], input[MenuInput::Cancel]) {
            (true, false) => {
                input[MenuInput::Accept] = false;
                match MENU_GRID[self.grid_position.1][self.grid_position.0] {
                    MenuTile::Option1 => MenuStatus::Game,
                    MenuTile::Option2 => MenuStatus::Continue,
                    MenuTile::Option3 => MenuStatus::Continue,
                    MenuTile::Option4 => MenuStatus::Continue,
                    MenuTile::Option5 => MenuStatus::Continue,
                    MenuTile::Option6 => MenuStatus::Settings,
                }
            }
            (false, true) => {
                input[MenuInput::Cancel] = false;
                MenuStatus::Exit
            }
            (_, _) => {
                MenuStatus::Continue
            }
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(64, 64, 64));
        for (y, row) in MENU_GRID.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                draw_tile(canvas, tile, (x ,y));
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(self.grid_position.0 as i32 * 270, self.grid_position.1 as i32 * 100, 270, 100)).unwrap();
        canvas.present();
    }
}

fn draw_tile(canvas: &mut WindowCanvas, tile: &MenuTile, position: (usize, usize)) {
    let color = match *tile {
        MenuTile::Option1 => Color::RGB(255, 0, 0),
        MenuTile::Option2 => Color::RGB(255, 255, 0),
        MenuTile::Option3 => Color::RGB(0, 255, 0),
        MenuTile::Option4 => Color::RGB(0, 255, 255),
        MenuTile::Option5 => Color::RGB(0, 0, 255),
        MenuTile::Option6 => Color::RGB(255, 0, 255),
    };
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(position.0 as i32 * 270, position.1 as i32 * 100, 270, 100)).unwrap();
}