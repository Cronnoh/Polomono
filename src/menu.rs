pub mod render;
use crate::input::MenuInput;

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

        let new_x = (self.grid_position.0 as i32 + movement_h).rem_euclid(GRID_COLUMNS as i32) as usize;
        let new_y = (self.grid_position.1 as i32 + movement_v).rem_euclid(GRID_ROWS as i32) as usize;
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
}
