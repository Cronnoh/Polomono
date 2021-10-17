use std::path::Path;

use sdl2::{image::LoadTexture, pixels::Color, rect::{Rect}, render::{Texture, BlendMode, TextureCreator}, ttf::{Font, Sdl2TtfContext}, video::WindowContext};

pub struct GameAssets<'a, 'b> {
    pub block_sheet: Texture<'a>,
    pub block_sprites: Vec<Rect>,
    pub stat_font: Font<'a, 'b>,
    pub stat_labels: Vec<Texture<'a>>,
    pub frame: Texture<'a>,
}

impl<'a, 'b> GameAssets<'a, 'b> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Result<Self, String> {
        let (block_sheet, block_sprites) = load_block_textures(texture_creator, Path::new("assets/blocks.png"))?;
        let (stat_font, stat_labels)  = load_font(texture_creator, ttf_context, Path::new("assets/Hack-Bold.ttf"))?;
        let frame = load_frame(&texture_creator, Path::new("assets/frame.png"))?;

        Ok(Self {
            block_sheet,
            block_sprites,
            stat_font,
            stat_labels,
            frame,
        })
    }

    pub fn create_stat_textures(&self, stats: &super::Stats, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Vec<Texture<'a>>, String> {
        let color = Color::RGB(255, 255, 255);
        let textures = vec![
            create_text_texture(&stats.score.to_string(), color, &self.stat_font, &texture_creator)?,
            create_text_texture(&format_time(stats.time), color, &self.stat_font, &texture_creator)?,
            create_text_texture(&stats.lines_cleared.to_string(), color, &self.stat_font, &texture_creator)?,
            create_text_texture(&stats.pieces_placed.to_string(), color, &self.stat_font, &texture_creator)?,
        ];

        Ok(textures)
    }
}

fn load_block_textures<'a>(texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<(Texture<'a>, Vec<Rect>), String> {
    let mut block_sheet = texture_creator.load_texture(path)?;
    let block_sprites = block_texture_regions(&block_sheet)?;
    block_sheet.set_blend_mode(BlendMode::Blend);
    Ok((block_sheet, block_sprites))
}

fn load_font<'a, 'b>(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext, path: &Path) -> Result<(Font<'a, 'b>, Vec<Texture<'a>>), String> {
    let stat_font = ttf_context.load_font(path, 28)?;
    let label_font = ttf_context.load_font(path, 18)?;
    let label_color = Color::RGB(144, 144, 144);
    let stat_labels = vec![
        create_text_texture("Score", label_color, &label_font, &texture_creator)?,
        create_text_texture("Time", label_color, &label_font, &texture_creator)?,
        create_text_texture("Lines", label_color, &label_font, &texture_creator)?,
        create_text_texture("Pieces", label_color, &label_font, &texture_creator)?,
    ];

    Ok((stat_font, stat_labels))
}

fn load_frame<'a>(texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<Texture<'a>, String> {
    texture_creator.load_texture(path)
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

fn block_texture_regions(texture: &Texture) -> Result<Vec<Rect>, String> {
    let mut regions = Vec::new();
    let query = texture.query();

    for i in 0..super::piece::PieceColor::ColorCount as i32 {
        let offset = i*query.height as i32;
        if offset >= query.width as i32 {
            return Err("Block texture file is not properly formed".to_string());
        }
        regions.push(Rect::new(offset, 0, query.height, query.height));
    }

    Ok(regions)
}
