pub mod render;
pub mod assets;
use std::path::Path;
use std::cmp::{min, max};

use crate::input::MenuInput;

use enum_map::EnumMap;

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
}

impl Menu {
    pub fn new() -> Result<Self, String> {
        let gamemode_names: Vec<String> = crate::load_data_ron(Path::new(&"config/menu_config.ron"))?;
        let mut tiles: Vec<MenuTile> = gamemode_names.iter().map(|name| MenuTile::Gamemode(name.to_string())).collect();
        tiles.push(MenuTile::Settings);
        Ok(Self {
            selected_index: 0,
            tiles,
        })
    }

    pub fn update(&mut self, input: &mut EnumMap<MenuInput, bool>) -> MenuStatus {
        /* Menu movement 
            Up and Down move one tile
            Left and Right are used to move faster
            Faster movements are prioritized
            Faster movements should 'catch' on the top and bottom of the menu
            Pressing Left at the top of the list should move to the bottom (opposite for Right at bottom) 
        */
        let movement = if input[MenuInput::Left] || input[MenuInput::Right] {
            let at_top = self.selected_index == 0;
            let at_bottom = self.selected_index == self.tiles.len() - 1;
            match (input[MenuInput::Left], input[MenuInput::Right], at_top, at_bottom) {
                (true, false, true, _) => {
                    input[MenuInput::Left] = false;
                    -1
                }
                (true, false, false, _) => {
                    input[MenuInput::Left] = false;
                    max(-3, -(self.selected_index as i32))
                }
                (false, true, _, true) => {
                    input[MenuInput::Right] = false;
                    1
                }
                (false, true, _, false) => {
                    input[MenuInput::Right] = false;
                    min(3, (self.tiles.len()-1 - self.selected_index) as i32)
                }
                _ => 0
            }
        } else {
            match (input[MenuInput::Up], input[MenuInput::Down]) {
                (true, false) => {
                    input[MenuInput::Up] = false;
                    -1
                }
                (false, true) => {
                    input[MenuInput::Down] = false;
                    1
                }
                _ => 0,
            }
        };

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
}
