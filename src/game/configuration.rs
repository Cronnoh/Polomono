use super::randomizer::{self, RandomizerStyle};

use std::{path::Path, collections::HashSet};

use serde::{Deserialize};

#[derive(Deserialize, Clone, Copy)]
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
pub enum RulesetModifier {
    LoadRuleset(String),
    SetCondition(EndCondition),
    SetScoreMultiplier(ScoreMultiplier),
    SetGravity(u128),
    SetMatrixSize(usize, usize),
    SetPieceList(Vec<String>),
    AddPiece(String),
    RemovePiece(String),
    SetLockDelay(u128),
    SetPreviewCount(usize),
    CanHold(bool),
    ShowGhostPiece(bool),
    ChangeRandomizer(RandomizerStyle),
    ClearMatrix,
    End,
}

#[derive(Hash, PartialEq, Eq)]
pub enum Command {
    RegeneratePieces,
    ResizeMatrix,
    ClearMatrix,
    End,
}

#[derive(Deserialize, Clone, Copy)]
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
    pub level_list: Vec<Vec<RulesetModifier>>,
    pub initial_ruleset: String,
}

impl GameMode {
    pub fn level_up(&self, ruleset: &mut Ruleset, level: usize) -> Result<HashSet<Command>, String> {
        let mut commands = HashSet::new();
        // Minus 2 because level starts at 1, arrays start at 0, and the level 1 ruleset is not in the list
        if level-2 < self.level_list.len() {
            for modifier in self.level_list[level-2].iter() {
                if let Some(new_commands) = ruleset.apply_modifier(modifier)? {
                    commands.extend(new_commands);
                }
            }
        }
        Ok(commands)
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut ruleset: Ruleset = crate::load_data_ron(Path::new(&format!("data/rulesets/{}.ron", &self.initial_ruleset)))?;
        ruleset.validate(&self.initial_ruleset)?;

        // Validate that all rulesets in the gamemode exist, and are well formed
        for i in 2..self.level_list.len() {
            self.level_up(&mut ruleset, i)?;
            ruleset.validate(&format!("Level {}", i))?;
        }
        Ok(())
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

impl Ruleset {
    pub fn apply_modifier(&mut self, modifier: &RulesetModifier) -> Result<Option<Vec<Command>>, String> {
        match modifier {
            RulesetModifier::LoadRuleset(ruleset_name) => {
                *self = crate::load_data_ron(Path::new(&format!("data/rulesets/{}.ron", ruleset_name)))?;
                return Ok(Some(vec![Command::RegeneratePieces, Command::ResizeMatrix]));
            }
            RulesetModifier::SetCondition(x) => {
                self.level_up_condition = *x;
            }
            RulesetModifier::SetScoreMultiplier(x) => {
                self.score_multiplier = *x;
            }
            RulesetModifier::SetGravity(x) => {
                self.gravity = *x;
            }
            RulesetModifier::SetMatrixSize(x, y) => {
                self.matrix_width = *x;
                self.matrix_height = *y;
                return Ok(Some(vec![Command::ResizeMatrix]));
            }
            RulesetModifier::SetPieceList(list) => {
                self.piece_list = list.to_vec();
                return Ok(Some(vec![Command::RegeneratePieces]));
            }
            RulesetModifier::AddPiece(piece_name) => {
                self.piece_list.push(piece_name.to_string());
                return Ok(Some(vec![Command::RegeneratePieces]));
            }
            RulesetModifier::RemovePiece(piece_name) => {
                if let Some(index) = self.piece_list.iter().position(|x| *x == *piece_name) {
                    self.piece_list.swap_remove(index);
                    return Ok(Some(vec![Command::RegeneratePieces]));
                }
            }
            RulesetModifier::SetLockDelay(x) => {
                self.lock_delay = *x;
            }
            RulesetModifier::SetPreviewCount(x) => {
                self.preview_count = *x;
            }
            RulesetModifier::CanHold(x) => {
                self.hold_enabled = *x;
            }
            RulesetModifier::ShowGhostPiece(x) => {
                self.ghost_piece_enabled = *x;
            }
            RulesetModifier::ChangeRandomizer(style) => {
                self.randomizer = *style;
                return Ok(Some(vec![Command::RegeneratePieces]));
            }
            RulesetModifier::ClearMatrix => {
                return Ok(Some(vec![Command::ClearMatrix]));
            }
            RulesetModifier::End => {
                return Ok(Some(vec![Command::End]));
            }
        }
        Ok(None)
    }

    fn validate(&self, ruleset_name: &str) -> Result<(), String> {
        // Check that level_up_conditions will not always be true (which would cause an infinite loop on level up)
        match self.level_up_condition {
            EndCondition::Lines(x) if x < 1 => return Err(format!("Ruleset {} has invalid level_up_condition: Lines({})", ruleset_name, x)),
            EndCondition::Score(x) if x < 1 => return Err(format!("Ruleset {} has invalid level_up_condition: Score({})", ruleset_name, x)),
            EndCondition::Time(min, sec) if min < 1 && sec < 1 => return Err(format!("Ruleset {} has invalid level_up_condition: Time({}, {})", ruleset_name, min, sec)),
            EndCondition::Pieces(x) if x < 1 => return Err(format!("Ruleset {} has invalid level_up_condition: Pieces({})", ruleset_name, x)),
            _ => {}
        }
        Ok(())
    }
}
