use std::{collections::HashMap, path::Path};
use enum_map::EnumMap;

use crate::{input::{self, GameInput}, render::{self, Assets}, scenes::{SceneAction, SceneTrait}};

pub struct GameScene {
    bindings: HashMap<String, GameInput>,

    game: crate::game::Game,
    inputs: EnumMap<GameInput, bool>,
    // piece_data: HashMap<String, PieceType>,
    // ruleset: Ruleset,
}

impl GameScene {
    pub fn new() -> Result<Self, String> {
        let config = crate::load_data(Path::new("config.toml"))?;

        Ok(Self {
            bindings: crate::load_data(Path::new("control_config.toml"))?,
            game: crate::game::Game::new(&config)?,
            inputs: EnumMap::default(),
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
        self.game.update(&mut self.inputs, elapsed);
        SceneAction::Continue
    }

    fn render(&self, canvas: &mut sdl2::render::WindowCanvas, assets: &mut Assets) -> Result<(), String> {
        render::render(canvas, &self.game, assets)
    }
}