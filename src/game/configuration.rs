use super::randomizer;

use std::path::Path;

use serde::{Deserialize};

#[derive(Deserialize)]
pub enum EndCondition {
    Time(u32, u32),
    Score(u32),
    Lines(u32),
    Pieces(u32),
    Endless,
}

impl EndCondition {
    pub fn check(&self, stats: &super::Stats) -> bool {
        match self {
            EndCondition::Time(min, sec) => {
                let microseconds = (min * 60 + sec) as u128 * 1_000_000;
                stats.time >= microseconds
            },
            EndCondition::Score(points) => {
                stats.score >= *points
            },
            EndCondition::Lines(lines) => {
                stats.lines_cleared >= *lines
            },
            EndCondition::Pieces(pieces) => {
                stats.pieces_placed >= *pieces
            }
            EndCondition::Endless => {
                false
            },
        }
    }
}

#[derive(Deserialize)]
pub enum Goal {
    Time,
    Score,
    Lines,
}

#[derive(Deserialize, enum_map::Enum, Clone, Copy)]
pub enum GameStat {
    Score,
    Time,
    Lines,
    Pieces,
    Level,
    PiecesPerSecond,
    // Streak,
    // Combo,
}

#[derive(Deserialize)]
pub enum LevelUp {
    RuleChange(Vec<String>),
    GravityIncrease(u128),
}

#[derive(Deserialize)]
pub enum ScoreMultiplier {
    Level,
    Special(u32),
}

impl ScoreMultiplier {
    pub fn apply(&self, points: u32, level: usize) -> u32 {
        match self {
            Self::Level => points * level as u32,
            Self::Special(x) => points * x,
        }
    }
}

#[derive(Deserialize)]
pub struct GameMode {
    pub end_condition: EndCondition,
    pub goal: Goal,
    pub displayed_stats: Vec<GameStat>,
    pub level_up_style: LevelUp,
    pub initial_ruleset: String,
}

impl GameMode {
    pub fn level_up(&self, ruleset: &mut Ruleset, level: usize) {
        match &self.level_up_style {
            LevelUp::RuleChange(level_list) => {
                if level-1 < level_list.len() {
                    *ruleset = crate::load_data_ron(Path::new(&format!("data/rulesets/{}.ron", level_list[level-1]))).unwrap();
                }
            },
            LevelUp::GravityIncrease(grav_increase) => {
                ruleset.gravity = std::cmp::max(0, ruleset.gravity as i128 - *grav_increase as i128) as u128;
            },
        }
    }
}

#[derive(Deserialize)]
pub struct Ruleset {
    pub level_up_condition: EndCondition,
    pub score_multiplier: ScoreMultiplier,
 
    pub matrix_height: usize,
    pub matrix_width: usize,
 
    pub gravity: u128,
    pub lock_delay: u128,
    pub preview_count: usize,
    pub hold_enabled: bool,
    pub ghost_piece_enabled: bool,

    pub piece_list: Vec<String>,
    pub cannot_start_with: Option<Vec<String>>,
    pub starting_randomizer: Option<randomizer::RandomizerStyle>,
    pub randomizer: randomizer::RandomizerStyle,
}
