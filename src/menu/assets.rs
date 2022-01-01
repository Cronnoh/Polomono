use std::path::Path;

use sdl2::{image::LoadTexture, rect::Rect, render::{Texture, TextureCreator}, video::WindowContext, ttf::Sdl2TtfContext, pixels::Color};
use crate::assets::create_text_texture;

pub struct MenuAssets<'a> {
    pub menu_sheet: Texture<'a>,
    pub menu_tile: Rect,
    pub menu_tile_selected: Rect,
    pub menu_page_arrow: Rect,
    pub menu_page_dot: Rect,
    pub menu_page_dot_selected: Rect,
    pub tile_labels: Vec<Texture<'a>>,
}

impl<'a> MenuAssets<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Result<Self, String> {
        let menu_sheet = texture_creator.load_texture("assets/menu.png")?;

        let font = ttf_context.load_font(Path::new("assets/Hack-Bold.ttf"), 28)?;
        let label_color = Color::RGB(255, 255, 255);
        let tile_labels = vec![
            create_text_texture("Marathon", label_color, &font, texture_creator)?,
            create_text_texture("Sprint", label_color, &font, texture_creator)?,
            create_text_texture("Settings", label_color, &font, texture_creator)?,
        ];

        Ok(Self {
            menu_sheet,
            menu_tile: Rect::new(0, 80, 298, 72),
            menu_tile_selected: Rect::new(0, 0, 298, 80),
            menu_page_arrow: Rect::new(0, 152, 6, 10),
            menu_page_dot: Rect::new(6, 152, 10, 10),
            menu_page_dot_selected: Rect::new(16, 152, 10, 10),
            tile_labels,
        })
    }
}