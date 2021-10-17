use sdl2::{image::LoadTexture, rect::Rect, render::{Texture, TextureCreator}, video::WindowContext};

pub struct MenuAssets<'a> {
    pub menu_sheet: Texture<'a>,
    pub menu_tile: Rect,
    pub menu_tile_selected: Rect,
    pub menu_page_arrow: Rect,
    pub menu_page_dot: Rect,
    pub menu_page_dot_selected: Rect,
}

impl<'a> MenuAssets<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let menu_sheet = texture_creator.load_texture("assets/menu.png")?;

        Ok(Self {
            menu_sheet,
            menu_tile: Rect::new(0, 80, 298, 72),
            menu_tile_selected: Rect::new(0, 0, 298, 80),
            menu_page_arrow: Rect::new(0, 152, 6, 10),
            menu_page_dot: Rect::new(6, 152, 10, 10),
            menu_page_dot_selected: Rect::new(16, 152, 10, 10),
        })
    }
}