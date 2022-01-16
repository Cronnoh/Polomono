pub mod render;
pub mod assets;
use std::path::Path;
use std::cmp::{min, max};

use crate::input::MenuInput;

use enum_map::EnumMap;

const DAS_THRESHOLD: u128 = 250_000;
const ARR_THRESHOLD: u128 = 150_000;

enum MenuTile {
    Gamemode(String),
    Settings,
}

pub enum MenuStatus {
    Exit,
    Continue,
    Game(String),
    Settings,
} 

pub struct Menu {
    selected_index: usize,
    tiles: Vec<MenuTile>,

    prev_movement_input: MenuInput,
    arr_timer: u128,
    das_timer: u128,
}

impl Menu {
    pub fn new() -> Result<Self, String> {
        let gamemode_names: Vec<String> = crate::load_data_ron(Path::new(&"config/menu_config.ron"))?;
        let mut tiles: Vec<MenuTile> = gamemode_names.iter().map(|name| MenuTile::Gamemode(name.to_string())).collect();
        tiles.push(MenuTile::Settings);
        Ok(Self {
            selected_index: 0,
            tiles,

            prev_movement_input: MenuInput::Cancel,
            arr_timer: 0,
            das_timer: 0,
        })
    }

    pub fn update(&mut self, input: &mut EnumMap<MenuInput, bool>, elapsed: u128) -> MenuStatus {
        let (button, mut movement) = self.get_movement(input);
        movement = self.handle_autoshift(button, movement, elapsed);
        self.selected_index = (self.selected_index as i32 + movement).rem_euclid(self.tiles.len() as i32) as usize;

        match (input[MenuInput::Accept], input[MenuInput::Cancel]) {
            (true, false) => {
                input[MenuInput::Accept] = false;
                match &self.tiles[self.selected_index] {
                    MenuTile::Gamemode(name) => MenuStatus::Game(name.clone()),
                    MenuTile::Settings => MenuStatus::Settings,
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

    fn get_movement(&mut self, input: &mut EnumMap<MenuInput, bool>) -> (MenuInput, i32) {
        /* Menu movement 
            Up and Down move one tile
            Left and Right are used to move faster
            Faster movements are prioritized
            Faster movements should 'catch' on the top and bottom of the menu
            Pressing Left at the top of the list should move to the bottom (opposite for Right at bottom) 
        */
        if input[MenuInput::Left] || input[MenuInput::Right] {
            let at_top = self.selected_index == 0;
            let at_bottom = self.selected_index == self.tiles.len() - 1;
            match (input[MenuInput::Left], input[MenuInput::Right], at_top, at_bottom) {
                (true, false, true, _) => {
                    (MenuInput::Left, -1)
                }
                (true, false, false, _) => {
                    (MenuInput::Left, max(-3, -(self.selected_index as i32)))
                }
                (false, true, _, true) => {
                    (MenuInput::Right, 1)
                }
                (false, true, _, false) => {
                    (MenuInput::Right, min(3, (self.tiles.len()-1 - self.selected_index) as i32))
                }
                _ => (MenuInput::Cancel, 0)
            }
        } else {
            match (input[MenuInput::Up], input[MenuInput::Down]) {
                (true, false) => {
                    (MenuInput::Up, -1)
                }
                (false, true) => {
                    (MenuInput::Down, 1)
                }
                _ => (MenuInput::Cancel, 0)
            }
        }
    }

    fn handle_autoshift(&mut self, button: MenuInput, mut movement: i32, elapsed: u128) -> i32 {
        if self.prev_movement_input != button {
            self.arr_timer = 0;
            self.das_timer = 0;
            self.prev_movement_input = button;
        } else if button != MenuInput::Cancel {
            if self.das_timer < DAS_THRESHOLD {
                self.das_timer += elapsed;
                movement = 0;
            } else {
                self.arr_timer += elapsed;
                let repeats = (self.arr_timer / ARR_THRESHOLD) as i32;
                self.arr_timer %= ARR_THRESHOLD;
                movement *= repeats;
            }
        }
        movement
    }
}
