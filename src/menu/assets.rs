use std::{path::Path, hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use sdl2::{image::LoadTexture, render::{Texture, TextureCreator}, video::WindowContext, ttf::Sdl2TtfContext, pixels::Color};
use crate::assets::create_text_texture;

pub struct MenuAssets<'a> {
    pub menu_bg: Texture<'a>,
    pub menu_tile_overlay: Texture<'a>,
    pub tile_labels: Vec<Texture<'a>>,
    pub tile_colors: Vec<Color>,
}

impl<'a> MenuAssets<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Result<Self, String> {
        let menu_bg = texture_creator.load_texture("assets/menu_bg.png")?;
        let menu_tile_overlay = texture_creator.load_texture("assets/menu_tile_overlay.png")?;

        let font = ttf_context.load_font(Path::new("assets/Hack-Bold.ttf"), 28)?;
        let label_color = Color::RGB(255, 255, 255);
        let mut tile_labels = Vec::new();
        let mut tile_colors = Vec::new();
        let label_text: Vec<String> = crate::load_data_ron(Path::new(&"config/menu_config.ron"))?;
        for text in label_text {
            tile_labels.push(create_text_texture(&text.to_uppercase(), label_color, &font, texture_creator)?);
            tile_colors.push(generate_color(text));
        }
        tile_labels.push(create_text_texture("SETTINGS", label_color, &font, texture_creator)?);
        tile_colors.push(Color::RGB(128, 128, 128));

        Ok(Self {
            menu_bg,
            menu_tile_overlay,
            tile_labels,
            tile_colors,
        })
    }
}

/* Calculate a color based on the hash of the argument lol */
fn generate_color<T: Hash>(thing: T) -> Color {
    let mut s = DefaultHasher::new();
    thing.hash(&mut s);
    let hash = s.finish();
    let r = (hash & 0xFF) as u8;
    let g = (hash >> 8 & 0xFF) as u8;
    let b = (hash >> 16 & 0xFF) as u8;
    Color::RGB(r, g, b)
}
