use sdl2::{event::Event, keyboard::Scancode};

#[derive(Debug)]
pub struct Input {
    pub hard_drop: bool,
    pub soft_drop: bool,
    pub left: bool,
    pub right: bool,
    pub rot_cw: bool,
    pub rot_ccw: bool,
    pub rot_180: bool,
}

pub fn handle_input_event(input: &mut Input, event: Event) {
    match event {
        Event::KeyDown { scancode: Some(Scancode::W), repeat: false, .. } => {
            input.hard_drop = true;
        }
        Event::KeyUp { scancode: Some(Scancode::W), repeat: false, .. } => {
            input.hard_drop = false;
        }
        Event::KeyDown { scancode: Some(Scancode::S), repeat: false, .. } => {
            input.soft_drop = true;
        }
        Event::KeyUp { scancode: Some(Scancode::S), repeat: false, .. } => {
            input.soft_drop = false;
        }
        Event::KeyDown { scancode: Some(Scancode::A), repeat: false, .. } => {
            input.left = true;
        }
        Event::KeyUp { scancode: Some(Scancode::A), repeat: false, .. } => {
            input.left = false;
        }
        Event::KeyDown { scancode: Some(Scancode::D), repeat: false, .. } => {
            input.right = true;
        }
        Event::KeyUp { scancode: Some(Scancode::D), repeat: false, .. } => {
            input.right = false;
        }
        Event::KeyDown { scancode: Some(Scancode::J), repeat: false, .. } => {
            input.rot_ccw = true;
        }
        Event::KeyUp { scancode: Some(Scancode::J), repeat: false, .. } => {
            input.rot_ccw = false;
        }
        Event::KeyDown { scancode: Some(Scancode::K), repeat: false, .. } => {
            input.rot_180 = true;
        }
        Event::KeyUp { scancode: Some(Scancode::K), repeat: false, .. } => {
            input.rot_180 = false;
        }
        Event::KeyDown { scancode: Some(Scancode::L), repeat: false, .. } => {
            input.rot_cw = true;
        }
        Event::KeyUp { scancode: Some(Scancode::L), repeat: false, .. } => {
            input.rot_cw = false;
        }
        _ => {}
    }
}