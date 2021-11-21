use super::{SceneAction, SceneTrait};
use crate::{
    game::{Game, render},
    assets::Assets,
    input::{self, GameInput},
};

use std::{collections::HashMap, path::Path};
use enum_map::EnumMap;


pub struct GameScene {
    bindings: HashMap<String, GameInput>,

    game: Game,
    inputs: EnumMap<GameInput, bool>,
    gamemode_name: String,
}

impl GameScene {
    pub fn new(gamemode_name: String) -> Result<Self, String> {
        Ok(Self {
            bindings: crate::load_data(Path::new("config/control_config.toml"))?,
            game: Game::new(&gamemode_name)?,
            inputs: EnumMap::default(),
            gamemode_name,
        })
    }
}

impl SceneTrait for GameScene {
    fn handle_input(&mut self, input_events: Vec<sdl2::event::Event>) {
        for event in input_events {
            input::handle_input_event(&mut self.inputs, event, &self.bindings);
        }
    }

    fn update(&mut self, elapsed: u128) -> SceneAction {
        if self.inputs[GameInput::Reset] {
            *self = GameScene::new(std::mem::take(&mut self.gamemode_name)).expect("Reset Error");
            return SceneAction::Continue;
        }

        self.game.update(&mut self.inputs, elapsed);
        SceneAction::Continue
    }

    fn render(&self, canvas: &mut sdl2::render::WindowCanvas, assets: &mut Assets) -> Result<(), String> {
        render::render(canvas, &self.game, assets.get_game_assets(&self.gamemode_name)?)
    }
}