//! Utility module for the Snake game.
//! Contains common types, constants, and utilities used throughout the game.

// Define the game board dimensions
pub const WIDTH: u16 = 40;
pub const HEIGHT: u16 = 20;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
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
