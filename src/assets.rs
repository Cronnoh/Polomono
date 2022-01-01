use crate::game::assets::GameAssets;
use crate::menu::assets::MenuAssets;

use sdl2::pixels::Color;
use sdl2::ttf::{Sdl2TtfContext, Font};
use sdl2::video::WindowContext;
use sdl2::render::{TextureCreator, Texture};

pub struct Assets<'a, 'b> {
    texture_creator: &'a TextureCreator<WindowContext>,
    ttf_context: &'a Sdl2TtfContext,

    game_assets: Option<GameAssets<'a, 'b>>,
    menu_assets: Option<MenuAssets<'a>>,
}

impl<'a, 'b> Assets<'a, 'b> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Result<Self, String> {
        Ok(Self {
            texture_creator,
            ttf_context,

            game_assets: None,
            menu_assets: None,

        })
    }

    // Load assets if they are not currently loaded, then return them
    pub fn get_game_assets(&mut self, gamemode_name: &str) -> Result<&mut GameAssets<'a, 'b>, String> {
        if self.game_assets.is_none() {
            self.game_assets = Some(GameAssets::new(self.texture_creator, self.ttf_context, &gamemode_name.to_uppercase())?);
        }
        Ok(self.game_assets.as_mut().unwrap())
    }

    pub fn get_menu_assets(&mut self) -> Result<&mut MenuAssets<'a>, String> {
        if self.menu_assets.is_none() {
            self.menu_assets = Some(MenuAssets::new(self.texture_creator, self.ttf_context)?);
        }
        Ok(self.menu_assets.as_mut().unwrap())
    }
}

pub fn create_text_texture<'a, T>(text: &str, color: Color, font: &Font, texture_creator: &'a TextureCreator<T>) -> Result<Texture<'a>, String> {
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
}
