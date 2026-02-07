//! Game logic module for the Snake game.
//! Contains the core game entities and mechanics.

use crate::utils::{Difficulty, Direction, Position, PowerUp, PowerUpType};
use rand::Rng;
use std::collections::HashSet;
use std::io::Write;

pub struct Snake {
    pub body: Vec<Position>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            body: vec![
                Position { x: 10, y: 10 }, // Head
                Position { x: 11, y: 10 },
                Position { x: 12, y: 10 }, // Tail
            ],
            direction: Direction::Left,
        }
    }

    pub fn move_forward(&mut self, grow: bool, width: u16, height: u16) {
        let head = self.body[0];
        let mut new_head = match self.direction {
            Direction::Up => Position {
                x: head.x,
                y: head.y.wrapping_sub(1),
            },
            Direction::Down => Position {
                x: head.x,
                y: head.y.wrapping_add(1),
            },
            Direction::Left => Position {
                x: head.x.wrapping_sub(1),
                y: head.y,
            },
            Direction::Right => Position {
                x: head.x.wrapping_add(1),
                y: head.y,
            },
        };

        // Wrap around the screen edges (Nokia style)
        if new_head.x == 0 {
            new_head.x = width - 2; // -2 to account for border
        } else if new_head.x == width - 1 {
            new_head.x = 1; // +1 to account for border
        }

        if new_head.y == 0 {
            new_head.y = height - 2; // -2 to account for border
        } else if new_head.y == height - 1 {
            new_head.y = 1; // +1 to account for border
        }

        self.body.insert(0, new_head);

        if !grow {
            self.body.pop();
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        // Prevent 180-degree turns
        match (self.direction, new_direction) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => (),
            _ => self.direction = new_direction,
        }
    }

    pub fn head_position(&self) -> Position {
        self.body[0]
    }

    pub fn overlaps_with(&self, pos: Position) -> bool {
        self.body.contains(&pos)
    }
}

pub struct Game {
    pub snake: Snake,
    pub food: Position,
    pub score: u32,
    pub game_over: bool,
    pub difficulty: Difficulty,
    pub paused: bool,
    pub power_up: Option<PowerUp>,
    pub power_up_timer: Option<u32>, // Counter for how long power-up effect lasts
    // Positions that need to be redrawn
    pub dirty_positions: HashSet<Position>,
    pub width: u16,
    pub height: u16,
    pub muted: bool,
}

impl Game {
    pub fn new(difficulty: Difficulty, width: u16, height: u16) -> Self {
        let mut game = Game {
            snake: Snake::new(),
            food: Position { x: 0, y: 0 },
            score: 0,
            game_over: false,
            difficulty,
            paused: false,
            power_up: None,
            power_up_timer: None,
            dirty_positions: HashSet::new(),
            width,
            height,
            muted: false,
        };
        game.generate_food();
        game.generate_power_up(); // Generate initial power-up
                                  // Initially mark all snake positions as dirty
        for pos in &game.snake.body {
            game.dirty_positions.insert(*pos);
        }
        game.dirty_positions.insert(game.food);
        if let Some(power_up) = game.power_up {
            game.dirty_positions.insert(power_up.position);
        }
        game
    }

    pub fn toggle_pause(&mut self) {
        if !self.game_over {
            self.paused = !self.paused;
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn get_tick_rates(&self) -> (std::time::Duration, std::time::Duration) {
        match self.difficulty {
            Difficulty::Easy => (
                std::time::Duration::from_millis(150),
                std::time::Duration::from_millis(300),
            ), // Slower
            Difficulty::Medium => (
                std::time::Duration::from_millis(100),
                std::time::Duration::from_millis(200),
            ), // Default
            Difficulty::Hard => (
                std::time::Duration::from_millis(60),
                std::time::Duration::from_millis(120),
            ), // Faster
        }
    }

    pub fn check_power_up_collision(&mut self) {
        if let Some(power_up) = self.power_up {
            if self.snake.head_position() == power_up.position && power_up.active {
                self.apply_power_up_effect(power_up.power_up_type);
                self.power_up = None; // Remove the power-up after collecting it
                self.generate_power_up(); // Generate a new one
            }
        }
    }

    pub fn apply_power_up_effect(&mut self, power_up_type: PowerUpType) {
        match power_up_type {
            PowerUpType::SpeedBoost => {
                // Temporarily increase snake speed (handled in main loop)
                self.power_up_timer = Some(100); // Effect lasts for 100 ticks
                self.play_sound(); // Play sound when collecting power-up
            }
            PowerUpType::SlowDown => {
                // Temporarily decrease snake speed
                self.power_up_timer = Some(100); // Effect lasts for 100 ticks
                self.play_sound(); // Play sound when collecting power-up
            }
            PowerUpType::ExtraPoints => {
                self.score += 50; // Add extra points
                self.play_sound(); // Play sound when collecting power-up
            }
            PowerUpType::Grow => {
                // Grow the snake by 2 segments
                for _ in 0..2 {
                    if let Some(last_segment) = self.snake.body.last().copied() {
                        self.snake.body.push(last_segment);
                    }
                }
                self.play_sound(); // Play sound when collecting power-up
            }
            PowerUpType::Shrink => {
                // Shrink the snake by removing 2 segments (but keep at least 3)
                if self.snake.body.len() > 3 {
                    self.snake.body.pop();
                    if self.snake.body.len() > 3 {
                        self.snake.body.pop();
                    }
                }
                self.play_sound(); // Play sound when collecting power-up
            }
        }
    }

    pub fn update_power_up_effects(&mut self) {
        if let Some(timer) = &mut self.power_up_timer {
            *timer -= 1;
            if *timer == 0 {
                self.power_up_timer = None; // Remove the effect when timer reaches 0
            }
        }
    }

    pub fn mark_position_dirty(&mut self, pos: Position) {
        self.dirty_positions.insert(pos);
    }

    pub fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            let new_food = Position {
                x: rng.gen_range(1..self.width - 1),
                y: rng.gen_range(1..self.height - 1),
            };

            // Make sure food doesn't appear on snake
            if !self.snake.overlaps_with(new_food) {
                // Mark old food position as dirty
                self.mark_position_dirty(self.food);
                self.food = new_food;
                // Mark new food position as dirty
                self.mark_position_dirty(self.food);
                break;
            }
        }
    }

    pub fn generate_power_up(&mut self) {
        if self.power_up.is_some() {
            return; // Only one power-up at a time
        }

        let mut rng = rand::thread_rng();

        // Random chance to spawn a power-up (lower probability than food)
        if rng.gen::<f32>() < 0.3 {
            // 30% chance to spawn a power-up
            loop {
                let new_power_up_pos = Position {
                    x: rng.gen_range(1..self.width - 1),
                    y: rng.gen_range(1..self.height - 1),
                };

                // Make sure power-up doesn't appear on snake or food
                if !self.snake.overlaps_with(new_power_up_pos) && new_power_up_pos != self.food {
                    let power_up_types = [
                        PowerUpType::SpeedBoost,
                        PowerUpType::SlowDown,
                        PowerUpType::ExtraPoints,
                        PowerUpType::Grow,
                        PowerUpType::Shrink,
                    ];
                    let power_up_type = power_up_types[rng.gen_range(0..power_up_types.len())];

                    // Mark old power-up position as dirty if it existed
                    if let Some(old_power_up) = self.power_up {
                        self.mark_position_dirty(old_power_up.position);
                    }

                    self.power_up = Some(PowerUp {
                        position: new_power_up_pos,
                        power_up_type,
                        active: true,
                    });

                    // Mark new power-up position as dirty
                    self.mark_position_dirty(new_power_up_pos);
                    break;
                }
            }
        }
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        // Remember the tail position before moving (for clearing later)
        let old_tail_positions: Vec<Position> = self.snake.body.to_vec();

        let head_pos = self.snake.head_position();

        // Check for collisions with self only (walls wrap around now)
        if self.snake.body[1..].contains(&head_pos) {
            self.game_over = true;
            self.play_sound(); // Play sound when game over
            return;
        }

        // Check if snake ate the food
        let grow = head_pos == self.food;
        if grow {
            self.score += 10;
            // Mark old food position as dirty
            self.mark_position_dirty(self.food);
            self.generate_food();
            // Mark new food position as dirty
            self.mark_position_dirty(self.food);
            self.play_sound(); // Play sound when food is eaten
        }

        // Check for power-up collision
        self.check_power_up_collision();

        // Update power-up effects
        if self.power_up_timer.is_some() {
            self.update_power_up_effects();
        }

        // Random chance to generate a new power-up occasionally
        let mut rng = rand::thread_rng();
        if self.power_up.is_none() && rng.gen::<f32>() < 0.02 {
            // 2% chance each tick
            self.generate_power_up();
        }

        self.snake.move_forward(grow, self.width, self.height);

        // Mark positions as dirty
        // Mark the new head position
        self.mark_position_dirty(self.snake.head_position());

        // Mark the old head position (which is now the second segment)
        if self.snake.body.len() > 1 {
            self.mark_position_dirty(self.snake.body[1]);
        }

        // If not growing, mark the old tail position as dirty (to clear it)
        if !grow && old_tail_positions.len() > self.snake.body.len() {
            if let Some(old_tail) = old_tail_positions.last() {
                self.mark_position_dirty(*old_tail);
            }
        }
    }

    pub fn update_snake_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    pub fn play_sound(&self) {
        // Use terminal bell character to simulate sound
        if !self.muted {
            print!("\x07"); // Terminal bell
            std::io::stdout().flush().unwrap();
        }
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }
}
