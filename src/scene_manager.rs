use crate::{game, menu};

enum SceneAction {
    Continue,
    Push(Scene),
    Pop,
}

enum Scene {
    Game(game::Game),
    Title(menu::Menu),
    Settings,
}

struct SceneManager {
    stack: Vec<Scene>,
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            stack: Vec::new(),
        }
    }

    pub fn handle_scene_action(&mut self, action: SceneAction) {
        match action {
            SceneAction::Continue => {},
            SceneAction::Push(x) => self.stack.push(x),
            SceneAction::Pop => drop(self.stack.pop()),
        }
    }
}
