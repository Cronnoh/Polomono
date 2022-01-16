use std::collections::HashMap;

use enum_map::EnumMap;
use sdl2::{GameControllerSubsystem, controller::GameController, event::Event};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, enum_map::Enum, Clone, Copy)]
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
    Reset,
}

#[derive(Serialize, Deserialize, enum_map::Enum, Clone, Copy, PartialEq, Eq)]
pub enum MenuInput {
    Up,
    Down,
    Left,
    Right,
    Accept,
    Cancel,
}

pub fn handle_input_event<T>(input: &mut EnumMap<T, bool>, event: Event, bindings: &HashMap<String, T>)
where T: enum_map::Enum<bool> + Copy {
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

    if let Some(x) = bindings.get(&button) {
        input[*x] = state;
    };
}

pub fn open_game_controller(game_controller_subsystem: GameControllerSubsystem) -> Result<Option<GameController>, String> {
    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    // Iterate over all available joysticks and look for game controllers.
    Ok((0..available).find_map(|id| {
        if !game_controller_subsystem.is_game_controller(id) {
            return None;
        }

        match game_controller_subsystem.open(id) {
            Ok(c) => {
                // We managed to find and open a game controller,
                // exit the loop
                Some(c)
            }
            Err(_) => None,
        }
    }))
}
