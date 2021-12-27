use std::path::Path;

use enum_map::{EnumMap, enum_map};
use sdl2::{image::LoadTexture, pixels::Color, rect::{Rect}, render::{Texture, BlendMode, TextureCreator}, ttf::{Font, Sdl2TtfContext}, video::WindowContext};

use crate::game::configuration::{EndCondition, GameStat};

use super::Stats;

pub struct GameAssets<'a, 'b> {
    pub block_sheet: Texture<'a>,
    pub block_sprites: Vec<Rect>,
    pub stat_font: Font<'a, 'b>,
    pub next_level_font: Font<'a, 'b>,
    pub stat_labels: EnumMap<GameStat, Texture<'a>>,
    pub frame: Texture<'a>,
    pub gamemode_name_texture: Texture<'a>,
}

impl<'a, 'b> GameAssets<'a, 'b> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext, gamemode_name: &str) -> Result<Self, String> {
        let (block_sheet, block_sprites) = load_block_textures(texture_creator, Path::new("assets/blocks.png"))?;
        let stat_font = ttf_context.load_font(Path::new("assets/Hack-Bold.ttf"), 28)?;
        let next_level_font = ttf_context.load_font(Path::new("assets/Hack-Bold.ttf"), 14)?;
        let label_font = ttf_context.load_font(Path::new("assets/Hack-Bold.ttf"), 18)?;
        let stat_labels = load_stat_labels(texture_creator, label_font)?;
        let frame = load_frame(texture_creator, Path::new("assets/frame.png"))?;
        let gamemode_name_texture = load_gamemode_name_texture(texture_creator, ttf_context, Path::new("assets/Hack-Bold.ttf"), gamemode_name)?;

        Ok(Self {
            block_sheet,
            block_sprites,
            stat_font,
            next_level_font,
            stat_labels,
            frame,
            gamemode_name_texture,
        })
    }

    pub fn create_stat_textures<'c>(&self, stats: &super::Stats, level: usize, texture_creator: &'c TextureCreator<WindowContext>) -> Result<EnumMap<GameStat, Texture<'c>>, String> {
        let color = Color::RGB(255, 255, 255);

        let textures = enum_map! {
            GameStat::Score => create_text_texture(&stats.score.to_string(), color, &self.stat_font, texture_creator)?,
            GameStat::Time => create_text_texture(&format_time(stats.time), color, &self.stat_font, texture_creator)?,
            GameStat::Lines => create_text_texture(&stats.lines_cleared.to_string(), color, &self.stat_font, texture_creator)?,
            GameStat::Pieces => create_text_texture(&stats.pieces_placed.to_string(), color, &self.stat_font, texture_creator)?,
            GameStat::Level => create_text_texture(&level.to_string(), color, &self.stat_font, texture_creator)?,
            GameStat::PiecesPerSecond => create_text_texture(&format!("{:.3}", stats.pieces_per_second()), color, &self.stat_font, texture_creator)?,
        };

        Ok(textures)
    }

    pub fn create_next_level_label<'c>(&self, level_up_cond: &EndCondition, level_stats: &Stats, texture_creator: &'c TextureCreator<WindowContext>) -> Result<Texture<'c>, String> {
        let text = match &level_up_cond {
            EndCondition::Time(min, sec) => {
                let microseconds = (min * 60 + sec) as u128 * 1_000_000;
                let remaining = microseconds.saturating_sub(level_stats.time);
                let mut time = format_time(remaining);
                time.truncate(5);
                time
            }
            EndCondition::Score(next) => format!("{}S", next - level_stats.score),
            EndCondition::Lines(next) => format!("{}L", next - level_stats.lines_cleared),
            EndCondition::Pieces(next) => format!("{}P", next - level_stats.pieces_placed),
            EndCondition::Endless => String::new(),
        };

        let color = Color::RGB(192, 192, 192);
        create_text_texture(&text, color, &self.next_level_font, texture_creator)
    }
}

fn load_gamemode_name_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext, path: &Path, gamemode_name: &str) -> Result<Texture<'a>, String> {
    let font = ttf_context.load_font(path, 72)?;
    create_text_texture(gamemode_name, Color::RGBA(0, 0, 0, 64), &font, texture_creator)
}

fn load_block_textures<'a>(texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<(Texture<'a>, Vec<Rect>), String> {
    let mut block_sheet = texture_creator.load_texture(path)?;
    let block_sprites = block_texture_regions(&block_sheet)?;
    block_sheet.set_blend_mode(BlendMode::Blend);
    Ok((block_sheet, block_sprites))
}

fn load_stat_labels<'a>(texture_creator: &'a TextureCreator<WindowContext>, label_font: Font) -> Result<EnumMap<GameStat, Texture<'a>>, String> {
    let label_color = Color::RGB(144, 144, 144);
    let stat_labels = enum_map! {
        GameStat::Score => create_text_texture("Score", label_color, &label_font, texture_creator)?,
        GameStat::Time => create_text_texture("Time", label_color, &label_font, texture_creator)?,
        GameStat::Lines => create_text_texture("Lines", label_color, &label_font, texture_creator)?,
        GameStat::Pieces => create_text_texture("Pieces", label_color, &label_font, texture_creator)?,
        GameStat::Level => create_text_texture("Level", label_color, &label_font, texture_creator)?,
        GameStat::PiecesPerSecond => create_text_texture("Pieces/Second", label_color, &label_font, texture_creator)?,
    };

    Ok(stat_labels)
}

fn load_frame<'a>(texture_creator: &'a TextureCreator<WindowContext>, path: &Path) -> Result<Texture<'a>, String> {
    texture_creator.load_texture(path)
}

fn create_text_texture<'a, T>(text: &str, color: Color, font: &Font, texture_creator: &'a TextureCreator<T>) -> Result<Texture<'a>, String> {
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
