use sdl2::{
    GameControllerSubsystem,
    controller::GameController,
    controller::Button,
    event::Event,
    keyboard::Scancode
};

#[derive(Debug)]
pub struct Input {
    pub hard_drop: bool,
    pub soft_drop: bool,
    pub left: bool,
    pub right: bool,
    pub rot_cw: bool,
    pub rot_ccw: bool,
    pub rot_180: bool,
    pub hold: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            hard_drop: false,
            soft_drop: false,
            left: false,
            right: false,
            rot_cw: false,
            rot_ccw: false,
            rot_180: false,
            hold: false,
        }
    }
}

pub fn handle_input_event(input: &mut Input, event: Event) {
    match event {
        Event::KeyDown { scancode: Some(Scancode::W), repeat: false, .. }
        | Event::ControllerButtonDown { button: Button::DPadUp, ..} => {
            input.hard_drop = true;
        }
        Event::KeyUp { scancode: Some(Scancode::W), repeat: false, .. }
        | Event::ControllerButtonUp { button: Button::DPadUp, ..} => {
            input.hard_drop = false;
        }
        Event::KeyDown { scancode: Some(Scancode::S), repeat: false, .. }
        | Event::ControllerButtonDown { button: Button::DPadDown, ..} => {
            input.soft_drop = true;
        }
        Event::KeyUp { scancode: Some(Scancode::S), repeat: false, .. }
        | Event::ControllerButtonUp { button: Button::DPadDown, ..} => {
            input.soft_drop = false;
        }
        Event::KeyDown { scancode: Some(Scancode::A), repeat: false, .. }
        | Event::ControllerButtonDown { button: Button::DPadLeft, ..} => {
            input.left = true;
        }
        Event::KeyUp { scancode: Some(Scancode::A), repeat: false, .. }
        | Event::ControllerButtonUp { button: Button::DPadLeft, ..} => {
            input.left = false;
        }
        Event::KeyDown { scancode: Some(Scancode::D), repeat: false, .. }
        | Event::ControllerButtonDown { button: Button::DPadRight, ..} => {
            input.right = true;
        }
        Event::KeyUp { scancode: Some(Scancode::D), repeat: false, .. }
        | Event::ControllerButtonUp { button: Button::DPadRight, ..} => {
            input.right = false;
        }
        Event::KeyDown { scancode: Some(Scancode::J), repeat: false, .. }
        | Event::ControllerButtonDown { button: Button::A, ..} => {
            input.rot_ccw = true;
        }
        Event::KeyUp { scancode: Some(Scancode::J), repeat: false, .. }
        | Event::ControllerButtonUp { button: Button::A, ..} => {
            input.rot_ccw = false;
        }
        Event::KeyDown { scancode: Some(Scancode::K), repeat: false, .. }
        | Event::ControllerButtonDown { button: Button::Y, ..} => {
            input.rot_180 = true;
        }
        Event::KeyUp { scancode: Some(Scancode::K), repeat: false, .. }
        | Event::ControllerButtonUp { button: Button::Y, ..} => {
            input.rot_180 = false;
        }
        Event::KeyDown { scancode: Some(Scancode::L), repeat: false, .. }
        | Event::ControllerButtonDown { button: Button::B, ..} => {
            input.rot_cw = true;
        }
        Event::KeyUp { scancode: Some(Scancode::L), repeat: false, .. }
        | Event::ControllerButtonUp { button: Button::B, ..} => {
            input.rot_cw = false;
        }
        Event::KeyDown { scancode: Some(Scancode::LShift), repeat: false, .. }
        | Event::ControllerButtonDown { button: Button::LeftShoulder, ..} => {
            input.hold = true;
        }
        Event::KeyUp { scancode: Some(Scancode::LShift), repeat: false, .. }
        | Event::ControllerButtonUp { button: Button::LeftShoulder, ..} => {
            input.hold = false;
        }
        _ => {}
    }
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
