//! Utility module for the Snake game.
//! Contains common types, constants, and utilities used throughout the game.

use serde::{Deserialize, Serialize};

// Define the game board dimensions
pub const WIDTH: u16 = 40;
pub const HEIGHT: u16 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Es,
    Ja,
    Pt,
    Zh,
}

impl Language {
    pub const ALL: [Language; 5] = [
        Language::En,
        Language::Es,
        Language::Ja,
        Language::Pt,
        Language::Zh,
    ];

    pub fn to_index(self) -> usize {
        match self {
            Language::En => 0,
            Language::Es => 1,
            Language::Ja => 2,
            Language::Pt => 3,
            Language::Zh => 4,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerUpType {
    SpeedBoost,
    SlowDown,
    ExtraPoints,
    Grow,
    Shrink,
}

#[derive(Clone, Copy, PartialEq)]
pub struct PowerUp {
    pub position: Position,
    pub power_up_type: PowerUpType,
    pub active: bool,
}
