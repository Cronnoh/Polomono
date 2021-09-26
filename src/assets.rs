use std::path::Path;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::{Rect},
    render::{Texture, BlendMode, TextureCreator},
    ttf::Font,
};

pub struct Assets<'a, 'b> {
    block_sheet: Option<Texture<'a>>,
    block_sprites: Option<Vec<Rect>>,
    stat_font: Option<Font<'a, 'b>>,
    stat_labels: Option<Vec<Texture<'a>>>,
    frame: Option<Texture<'a>>,
}

impl<'a, 'b> Assets<'a, 'b> {
    pub fn new() -> Self {
        Self {
            block_sheet: None,
            block_sprites: None,
            stat_font: None,
            stat_labels: None,
            frame: None,
        }
    }

    pub fn load_block_textures(&mut self, texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<(), String> {
        self.block_sheet = Some(texture_creator.load_texture(path)?);
        if let Some(sheet) = &mut self.block_sheet {
            self.block_sprites = Some(crate::game::render::block_texture_regions(&sheet)?);
            sheet.set_blend_mode(BlendMode::Blend);
        }
        Ok(())
    }

    pub fn load_font(&mut self, ttf_context: &'a Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<(), String> {
        self.stat_font = Some(ttf_context.load_font(path, 28)?);
        let label_font = ttf_context.load_font(path, 18)?;
        let label_color = Color::RGB(144, 144, 144);
        self.stat_labels = Some(vec![
            create_text_texture("Score", label_color, &label_font, &texture_creator)?,
            create_text_texture("Time", label_color, &label_font, &texture_creator)?,
            create_text_texture("Lines", label_color, &label_font, &texture_creator)?,
            create_text_texture("Pieces", label_color, &label_font, &texture_creator)?,
        ]);

        Ok(())
    }

    pub fn load_frame(&mut self, texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<(), String> {
        self.frame = Some(texture_creator.load_texture(path)?);
        Ok(())
    }

    pub fn get_block_textures(&mut self) -> Result<(&mut Texture<'a>, &Vec<Rect>), String> {
        let block_sheet = match &mut self.block_sheet {
            Some(x) => x,
            None => return Err("Block Spritesheet not loaded".to_string()),
        };
        let block_sprites = match &self.block_sprites {
            Some(x) => x,
            None => return Err("Block Sprites not loaded".to_string()),
        };

        Ok((block_sheet, block_sprites))
    }

    fn get_font(&self) -> Result<&Font, String> {
        match &self.stat_font {
            Some(x) => Ok(x),
            None => Err("Font is not loaded".to_string()),
        }
    }

    pub fn get_stat_labels(&self) -> Result<&Vec<Texture>, String> {
        match &self.stat_labels {
            Some(x) => Ok(x),
            None => Err("Stat labels not loaded".to_string()),
        }
    }

    pub fn get_frame(&self) -> Result<&Texture, String>{
        match &self.frame {
            Some(x) => Ok(x),
            None => Err("Frame is not loaded".to_string()),
        } 
    }

    pub fn create_stat_textures(&self, stats: &crate::game::Stats, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Vec<Texture<'a>>, String> {
        let font = self.get_font()?;
        let color = Color::RGB(255, 255, 255);
        let textures = vec![
            create_text_texture(&stats.score.to_string(), color, font, &texture_creator)?,
            create_text_texture(&format_time(stats.time), color, font, &texture_creator)?,
            create_text_texture(&stats.lines_cleared.to_string(), color, font, &texture_creator)?,
            create_text_texture(&stats.pieces_placed.to_string(), color, font, &texture_creator)?,
        ];

        Ok(textures)
    }
}

fn create_text_texture<'a, T>(text: &str, color: Color, font: &Font, texture_creator: &'a TextureCreator<T>) -> Result<Texture<'a>, String>{
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
}

fn format_time(microseconds: u128) -> String {
    let hundredths = (microseconds % 1000000) / 10000;
    let total_seconds = microseconds / 1000000;
    let seconds = total_seconds % 60;
    let minutes = total_seconds / 60;

    format!("{:>0width$}:{:>0width$}.{:>0width$}", minutes, seconds, hundredths, width=2)
}
