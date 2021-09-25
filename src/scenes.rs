use sdl2::render::WindowCanvas;

use crate::{game_scene, menu, render::Assets};

pub trait SceneTrait {
    // fn start();
    fn handle_input(&mut self, input_events: Vec<sdl2::event::Event>);
    fn update(&mut self, elapsed: u128) -> SceneAction;
    fn render(&self, canvas: &mut WindowCanvas, assets: &mut Assets) -> Result<(), String>;
}

pub enum SceneAction {
    Continue,
    Push(Scene),
    Replace(Scene),
    Pop,
}

pub enum Scene {
    Game(game_scene::GameScene),
    Title,
    Settings,
}

pub struct SceneManager {
    stack: Vec<Scene>,
}

impl SceneManager {
    pub fn new(start_scene: Scene) -> Self {
        SceneManager {
            stack: vec![start_scene],
        }
    }

    pub fn handle_scene_action(&mut self, action: SceneAction) {
        match action {
            SceneAction::Continue => {},
            SceneAction::Push(x) => self.stack.push(x),
            SceneAction::Replace(x) => {
                drop(self.stack.pop());
                self.stack.push(x);
            }
            SceneAction::Pop => drop(self.stack.pop()),
        }
    }

    pub fn update(&mut self, canvas: &mut WindowCanvas, assets: &mut Assets, input_events: Vec<sdl2::event::Event>, elapsed: u128) -> Result<(), String> {
        let next = match self.stack.last_mut().unwrap() {
            Scene::Game(game) => SceneManager::run_scene(game, canvas, assets, input_events, elapsed)?,
            Scene::Title => SceneAction::Push(Scene::Game(game_scene::GameScene::new()?)),
            Scene::Settings => SceneAction::Push(Scene::Game(game_scene::GameScene::new()?)),
        };

        self.handle_scene_action(next);
        Ok(())
    }

    fn run_scene<T>(scene: &mut T, canvas: &mut WindowCanvas, assets: &mut Assets, input_events: Vec<sdl2::event::Event>, elapsed: u128) -> Result<SceneAction, String>
    where T: SceneTrait {
        scene.handle_input(input_events);
        scene.update(elapsed);
        scene.render(canvas, assets)?;
        Ok(SceneAction::Continue)
    }
}
