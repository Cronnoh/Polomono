pub mod render;
pub mod assets;
use std::path::Path;

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

        self.selected_index = (self.selected_index as i32 + movement_v).rem_euclid(self.tiles.len() as i32) as usize;

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
