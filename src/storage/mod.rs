//! Persistence helpers for local game data.

use crate::utils::Difficulty;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HighScores {
    pub easy: u32,
    pub medium: u32,
    pub hard: u32,
}

impl HighScores {
    pub fn get(&self, difficulty: Difficulty) -> u32 {
        match difficulty {
            Difficulty::Easy => self.easy,
            Difficulty::Medium => self.medium,
            Difficulty::Hard => self.hard,
        }
    }

    pub fn set(&mut self, difficulty: Difficulty, score: u32) {
        match difficulty {
            Difficulty::Easy => self.easy = score,
            Difficulty::Medium => self.medium = score,
            Difficulty::Hard => self.hard = score,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HighScoresFile {
    high_scores: HighScores,
}

#[derive(Debug, Serialize, Deserialize)]
struct LegacyHighScoreFile {
    high_score: u32,
}

impl From<LegacyHighScoreFile> for HighScores {
    fn from(value: LegacyHighScoreFile) -> Self {
        Self {
            easy: value.high_score,
            medium: value.high_score,
            hard: value.high_score,
        }
    }
}

fn high_score_path() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".rustnake.toml");
    }

    PathBuf::from(".rustnake.toml")
}

pub fn load_high_scores() -> HighScores {
    let path = high_score_path();
    let Ok(contents) = fs::read_to_string(path) else {
        return HighScores::default();
    };

    if let Ok(file) = toml::from_str::<HighScoresFile>(&contents) {
        return file.high_scores;
    }

    toml::from_str::<LegacyHighScoreFile>(&contents)
        .map(HighScores::from)
        .unwrap_or_default()
}

pub fn save_high_scores(high_scores: &HighScores) -> Result<(), String> {
    let path = high_score_path();
    let data = HighScoresFile {
        high_scores: HighScores {
            easy: high_scores.easy,
            medium: high_scores.medium,
            hard: high_scores.hard,
        },
    };
    let serialized = toml::to_string(&data).map_err(|err| err.to_string())?;
    fs::write(path, serialized).map_err(|err| err.to_string())
}
