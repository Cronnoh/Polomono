use crate::game::assets::GameAssets;
use crate::menu::assets::MenuAssets;

use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;

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
    pub fn get_game_assets(&mut self) -> Result<&mut GameAssets<'a, 'b>, String> {
        if let None = self.game_assets {
            self.game_assets = Some(GameAssets::new(&self.texture_creator, &self.ttf_context)?);
        }
        Ok(self.game_assets.as_mut().unwrap())
    }

    pub fn get_menu_assets(&mut self) -> Result<&mut MenuAssets<'a>, String> {
        if let None = self.game_assets {
            self.menu_assets = Some(MenuAssets::new(&self.texture_creator)?);
        }
        Ok(self.menu_assets.as_mut().unwrap())
    }
}
