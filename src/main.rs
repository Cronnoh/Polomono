mod game;
mod input;
mod assets;
mod menu;
mod scenes;

use std::time::Instant;

use sdl2::{
    event::Event,
    image::InitFlag,
    keyboard::Scancode,
};
use serde::{de::DeserializeOwned};

const OFFSCREEN_ROWS: usize = 5;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let game_controller_subsystem = sdl_context.game_controller()?;
    let _controller = input::open_game_controller(game_controller_subsystem)?;
    let ttf_context = sdl2::ttf::init()
        .map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("idk", 1280, 720)
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_logical_size(640, 360)
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut assets = assets::Assets::new(&texture_creator, &ttf_context)?;

    let mut scene_manager = scenes::SceneManager::new(scenes::Scene::MainMenu(scenes::menu_scene::MenuScene::new()?));

    let mut current_time = Instant::now();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        let elapsed = current_time.elapsed().as_micros();
        current_time = Instant::now();

        let mut input_events = Vec::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown{ repeat: false, ..} | Event::KeyUp{ repeat: false, ..}
                | Event::ControllerButtonDown{..} | Event::ControllerButtonUp{..} => {
                    input_events.push(event);
                }
                _ => {},
            }
        }

        scene_manager.update(&mut canvas, &mut assets, input_events, elapsed)?;
    }

    Ok(())
}

fn load_data<T: DeserializeOwned>(file_path: &std::path::Path) -> Result<T, String> {
    let data_file = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Error opening {}: {}", file_path.to_str().unwrap(), e.to_string()))?;
    toml::from_str(&data_file)
        .map_err(|e| format!("Error reading {}: {}", file_path.to_str().unwrap(), e.to_string()))
}

fn load_data_ron<T: DeserializeOwned>(file_path: &std::path::Path) -> Result<T, String> {
    let data_file = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Error opening {}: {}", file_path.to_str().unwrap(), e.to_string()))?;
    ron::from_str(&data_file)
        .map_err(|e| format!("Error reading {}: {}", file_path.to_str().unwrap(), e.to_string()))
}
