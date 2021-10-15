use super::{Scene, SceneAction, SceneTrait};
use super::{game_scene::GameScene};

use crate::{
    menu::{Menu, MenuStatus},
    assets::Assets,
    input::{self, MenuInput},
};

use std::{collections::HashMap, path::Path};
use enum_map::EnumMap;
use sdl2::render::WindowCanvas;

pub struct MenuScene {
    menu: Menu,
    bindings: HashMap<String, MenuInput>,
    inputs: EnumMap<MenuInput, bool>,
}

impl MenuScene {
    pub fn new() -> Result<Self, String> {

        Ok(Self {
            bindings: crate::load_data(Path::new("config/menu_control_config.toml"))?,
            menu: Menu {grid_position: (0, 0)},
            inputs: EnumMap::default(),
        })
    }
}

impl SceneTrait for MenuScene {
    fn handle_input(&mut self, input_events: Vec<sdl2::event::Event>) {
        for event in input_events {
            input::handle_input_event(&mut self.inputs, event, &self.bindings);
        }
    }

    fn update(&mut self, _elapsed: u128) -> SceneAction {
        match self.menu.update(&mut self.inputs) {
            MenuStatus::Continue => SceneAction::Continue,
            MenuStatus::Game => SceneAction::Push(Scene::Game(GameScene::new().unwrap())),
            MenuStatus::Settings => SceneAction::Continue,
            MenuStatus::Exit => SceneAction::Pop,
        }
    }

    fn render(&self, canvas: &mut WindowCanvas, assets: &mut Assets) -> Result<(), String> {
        crate::menu::render::render(&self.menu, canvas, assets);
        Ok(())
    }
}
