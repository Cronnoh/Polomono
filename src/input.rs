use std::collections::HashMap;

use sdl2::{
    GameControllerSubsystem,
    controller::GameController,
    event::Event,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Input {
    pub hard_drop: bool,
    pub instant_drop: bool,
    pub soft_drop: bool,
    pub left: bool,
    pub shift_left: bool,
    pub right: bool,
    pub shift_right: bool,
    pub rot_cw: bool,
    pub rot_180: bool,
    pub rot_ccw: bool,
    pub hold: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            hard_drop: false,
            instant_drop: false,
            soft_drop: false,
            left: false,
            shift_left: false,
            right: false,
            shift_right: false,
            rot_cw: false,
            rot_180: false,
            rot_ccw: false,
            hold: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum GameInput {
    HardDrop,
    InstantDrop,
    SoftDrop,
    Left,
    ShiftLeft,
    Right,
    ShiftRight,
    RotateCW,
    Rotate180,
    RotateCCW,
    Hold,
}

pub fn handle_input_event(input: &mut Input, event: Event, bindings: &HashMap<String, GameInput>) {
    let (button, state) = match event {
        Event::KeyDown { keycode: Some(key), ..} => {
            (format!("Key({})", key.to_string()), true)
        }
        Event::ControllerButtonDown { button: btn, ..} => {
            (format!("Btn({})", btn.string()), true)
        }
        Event::KeyUp { keycode: Some(key), ..} => {
            (format!("Key({})", key.to_string()), false)
        }
        Event::ControllerButtonUp { button: btn, ..} => {
            (format!("Btn({})", btn.string()), false)
        }
        _ => { return }
    };

    match bindings.get(&button) {
        Some(x) => match x {
            GameInput::HardDrop => input.hard_drop = state,
            GameInput::InstantDrop => input.instant_drop = state,
            GameInput::SoftDrop => input.soft_drop = state,
            GameInput::Left => input.left = state,
            GameInput::ShiftLeft => input.shift_left = state,
            GameInput::Right => input.right = state,
            GameInput::ShiftRight => input.shift_right = state,
            GameInput::RotateCW => input.rot_cw = state,
            GameInput::Rotate180 => input.rot_180 = state,
            GameInput::RotateCCW => input.rot_ccw = state,
            GameInput::Hold => input.hold = state,
        }
        None => return,
    };
}

pub fn open_game_controller(game_controller_subsystem: GameControllerSubsystem) -> Result<Option<GameController>, String> {
    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    // Iterate over all available joysticks and look for game controllers.
    Ok((0..available)
        .find_map(|id| {
            if !game_controller_subsystem.is_game_controller(id) {
                return None;
            }

            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    Some(c)
                }
                Err(_) => {
                    None
                }
            }
        }))
}
