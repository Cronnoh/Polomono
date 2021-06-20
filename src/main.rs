mod piece;
mod game;
mod input;
mod render;
mod randomizer;
use input::GameInput;

use std::{collections::HashMap, path::Path, time::Instant};

use sdl2::{
    event::Event,
    image::InitFlag,
    keyboard::Scancode,
};
use serde::{Deserialize, de::DeserializeOwned};

const OFFSCREEN_ROWS: usize = 5;

#[derive(Deserialize)]
pub struct Config {
    matrix_height: usize,
    matrix_width: usize,

    das: u32,
    arr: u32,
    gravity: u32,
    lock_delay: u32,
    preview_count: usize,

    piece_list: Vec<String>,
    cannot_start_with: Option<Vec<String>>,
    starting_randomizer: Option<randomizer::RandomizerStyle>,
    randomizer: randomizer::RandomizerStyle,
}

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

    let mut assets = render::Assets::new();
    assets.load_block_textures(&texture_creator, Path::new("assets/blocks.png"))?;
    assets.load_font(&ttf_context, &texture_creator, Path::new("assets/Hack-Bold.ttf"))?;
    assets.load_frame(&texture_creator, Path::new("assets/frame.png"))?;

    let config: Config = load_data(Path::new("config.toml"))?;
    let bindings: HashMap<String, GameInput> = load_data(Path::new("control_config.toml"))?;

    let mut input = input::Input::new();

    let mut game = game::Game::new(&config)?;
    let mut current_time = Instant::now();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        let elapsed = current_time.elapsed().as_micros();
        current_time = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown{ repeat: false, ..} | Event::KeyUp{ repeat: false, ..}
                | Event::ControllerButtonDown{..} | Event::ControllerButtonUp{..} => {
                    input::handle_input_event(&mut input, event, &bindings);
                }
                _ => {},
            }
        }

        if input.reset {
            input.reset = false;
            game = game::Game::new(&config)?;
        }

        game.update(&mut input, elapsed);

        render::render(&mut canvas, &game, &mut assets)?;
    }

    Ok(())
}

fn load_data<T: DeserializeOwned>(file_path: &std::path::Path) -> Result<T, String> {
    let data_file = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Error opening {}: {}", file_path.to_str().unwrap(), e.to_string()))?;
    toml::from_str(&data_file)
        .map_err(|e| format!("Error reading {}: {}", file_path.to_str().unwrap(), e.to_string()))
}
