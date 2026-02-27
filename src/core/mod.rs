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
    pub fn new(width: u16, height: u16) -> Self {
        let center_x = (width / 2).max(3);
        let center_y = (height / 2).max(2);
        Snake {
            body: vec![
                Position {
                    x: center_x,
                    y: center_y,
                }, // Head
                Position {
                    x: center_x + 1,
                    y: center_y,
                },
                Position {
                    x: center_x + 2,
                    y: center_y,
                }, // Tail
            ],
            direction: Direction::Left,
        }
    }

    pub fn next_head(&self, width: u16, height: u16) -> Position {
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

        // Wrap around the screen edges (Nokia style) while keeping movement inside borders.
        if new_head.x <= 1 {
            new_head.x = width - 1;
        } else if new_head.x >= width {
            new_head.x = 2;
        }

        if new_head.y <= 1 {
            new_head.y = height - 1;
        } else if new_head.y >= height {
            new_head.y = 2;
        }

        new_head
    }

    pub fn move_forward(&mut self, grow: bool, width: u16, height: u16) {
        let new_head = self.next_head(width, height);
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
    pub high_score: u32,
    pub game_over: bool,
    pub difficulty: Difficulty,
    pub paused: bool,
    pub power_up: Option<PowerUp>,
    pub power_up_timer: Option<u32>, // Counter for how long power-up effect lasts
    pub active_speed_effect: Option<PowerUpType>,
    // Positions that need to be redrawn
    pub dirty_positions: HashSet<Position>,
    pub width: u16,
    pub height: u16,
    pub muted: bool,
}

impl Game {
    pub fn new(difficulty: Difficulty, width: u16, height: u16, high_score: u32) -> Self {
        let mut game = Game {
            snake: Snake::new(width, height),
            food: Position { x: 0, y: 0 },
            score: 0,
            high_score,
            game_over: false,
            difficulty,
            paused: false,
            power_up: None,
            power_up_timer: None,
            active_speed_effect: None,
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
            Difficulty::Extreme => (
                std::time::Duration::from_millis(35),
                std::time::Duration::from_millis(70),
            ), // Fastest
        }
    }

    fn speed_effect_duration_ticks(&self) -> u32 {
        match self.difficulty {
            Difficulty::Easy => 120,
            Difficulty::Medium => 100,
            Difficulty::Hard => 85,
            Difficulty::Extreme => 70,
        }
    }

    fn power_up_refresh_spawn_chance(&self) -> f32 {
        match self.difficulty {
            Difficulty::Easy => 0.35,
            Difficulty::Medium => 0.30,
            Difficulty::Hard => 0.24,
            Difficulty::Extreme => 0.16,
        }
    }

    fn power_up_tick_spawn_chance(&self) -> f32 {
        match self.difficulty {
            Difficulty::Easy => 0.025,
            Difficulty::Medium => 0.020,
            Difficulty::Hard => 0.015,
            Difficulty::Extreme => 0.010,
        }
    }

    fn progression_step_percent(&self) -> u64 {
        match self.difficulty {
            Difficulty::Easy => 2,
            Difficulty::Medium => 3,
            Difficulty::Hard => 4,
            Difficulty::Extreme => 5,
        }
    }

    fn progression_max_steps(&self) -> u64 {
        match self.difficulty {
            Difficulty::Easy => 12,
            Difficulty::Medium => 15,
            Difficulty::Hard => 12,
            Difficulty::Extreme => 13,
        }
    }

    pub fn check_power_up_collision(&mut self) {
        if let Some(power_up) = self.power_up {
            if self.snake.head_position() == power_up.position && power_up.active {
                self.mark_position_dirty(power_up.position);
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
                self.power_up_timer = Some(self.speed_effect_duration_ticks());
                self.active_speed_effect = Some(PowerUpType::SpeedBoost);
                self.play_sound(); // Play sound when collecting power-up
            }
            PowerUpType::SlowDown => {
                // Temporarily decrease snake speed
                self.power_up_timer = Some(self.speed_effect_duration_ticks());
                self.active_speed_effect = Some(PowerUpType::SlowDown);
                self.play_sound(); // Play sound when collecting power-up
            }
            PowerUpType::ExtraPoints => {
                self.score += 50; // Add extra points
                self.update_high_score();
                self.play_sound(); // Play sound when collecting power-up
            }
            PowerUpType::Grow => {
                // Grow the snake by 2 segments
                for _ in 0..2 {
                    if let Some(last_segment) = self.snake.body.last().copied() {
                        self.snake.body.push(last_segment);
                        self.mark_position_dirty(last_segment);
                    }
                }
                self.play_sound(); // Play sound when collecting power-up
            }
            PowerUpType::Shrink => {
                // Shrink the snake by removing 2 segments (but keep at least 3)
                for _ in 0..2 {
                    if self.snake.body.len() > 3 {
                        if let Some(removed) = self.snake.body.pop() {
                            self.mark_position_dirty(removed);
                        }
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
                self.active_speed_effect = None;
            }
        }
    }

    pub fn speed_multiplier_percent(&self) -> u64 {
        match (self.power_up_timer, self.active_speed_effect) {
            (Some(_), Some(PowerUpType::SpeedBoost)) => 70,
            (Some(_), Some(PowerUpType::SlowDown)) => 150,
            _ => 100,
        }
    }

    pub fn difficulty_speed_multiplier_percent(&self) -> u64 {
        // Difficulty-specific pace scaling: harder modes accelerate faster and cap lower.
        let steps = (self.score / 50).min(self.progression_max_steps() as u32) as u64;
        let reduction = steps * self.progression_step_percent();
        100u64.saturating_sub(reduction)
    }

    pub fn speed_effect_ticks_left(&self) -> u32 {
        self.power_up_timer.unwrap_or(0)
    }

    pub fn update_high_score(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn mark_position_dirty(&mut self, pos: Position) {
        self.dirty_positions.insert(pos);
    }

    fn interior_cells(&self) -> usize {
        self.width.saturating_sub(2) as usize * self.height.saturating_sub(2) as usize
    }

    fn find_food_spawn_position(&self, rng: &mut rand::rngs::ThreadRng) -> Option<Position> {
        let total_cells = self.interior_cells();
        if total_cells == 0 {
            return None;
        }

        let blocked_cells = self.snake.body.len() + usize::from(self.power_up.is_some());
        if blocked_cells >= total_cells {
            return None;
        }

        let max_attempts = total_cells.saturating_mul(2).max(16);
        for _ in 0..max_attempts {
            let candidate = Position {
                x: rng.gen_range(2..self.width),
                y: rng.gen_range(2..self.height),
            };
            let overlaps_power_up = self
                .power_up
                .map(|power_up| power_up.position == candidate)
                .unwrap_or(false);
            if !self.snake.overlaps_with(candidate) && !overlaps_power_up {
                return Some(candidate);
            }
        }

        for y in 2..self.height {
            for x in 2..self.width {
                let candidate = Position { x, y };
                let overlaps_power_up = self
                    .power_up
                    .map(|power_up| power_up.position == candidate)
                    .unwrap_or(false);
                if !self.snake.overlaps_with(candidate) && !overlaps_power_up {
                    return Some(candidate);
                }
            }
        }

        None
    }

    fn find_power_up_spawn_position(&self, rng: &mut rand::rngs::ThreadRng) -> Option<Position> {
        let total_cells = self.interior_cells();
        if total_cells == 0 {
            return None;
        }

        // Power-ups cannot overlap snake or food.
        let blocked_cells = self.snake.body.len().saturating_add(1);
        if blocked_cells >= total_cells {
            return None;
        }

        let max_attempts = total_cells.saturating_mul(2).max(16);
        for _ in 0..max_attempts {
            let candidate = Position {
                x: rng.gen_range(2..self.width),
                y: rng.gen_range(2..self.height),
            };
            if !self.snake.overlaps_with(candidate) && candidate != self.food {
                return Some(candidate);
            }
        }

        for y in 2..self.height {
            for x in 2..self.width {
                let candidate = Position { x, y };
                if !self.snake.overlaps_with(candidate) && candidate != self.food {
                    return Some(candidate);
                }
            }
        }

        None
    }

    pub fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        let Some(new_food) = self.find_food_spawn_position(&mut rng) else {
            return;
        };

        // Mark old food position as dirty
        self.mark_position_dirty(self.food);
        self.food = new_food;
        // Mark new food position as dirty
        self.mark_position_dirty(self.food);
    }

    pub fn generate_power_up(&mut self) {
        if self.power_up.is_some() {
            return; // Only one power-up at a time
        }

        let mut rng = rand::thread_rng();

        // Difficulty-specific chance to spawn a replacement/initial power-up.
        if rng.r#gen::<f32>() < self.power_up_refresh_spawn_chance() {
            let Some(new_power_up_pos) = self.find_power_up_spawn_position(&mut rng) else {
                return;
            };

            let power_up_types = [
                PowerUpType::SpeedBoost,
                PowerUpType::SlowDown,
                PowerUpType::ExtraPoints,
                PowerUpType::Grow,
                PowerUpType::Shrink,
            ];
            let power_up_type = power_up_types[rng.gen_range(0..power_up_types.len())];

            self.power_up = Some(PowerUp {
                position: new_power_up_pos,
                power_up_type,
                active: true,
            });

            // Mark new power-up position as dirty
            self.mark_position_dirty(new_power_up_pos);
        }
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        let old_body_positions = self.snake.body.clone();
        let next_head = self.snake.next_head(self.width, self.height);
        let grow = next_head == self.food;
        self.snake.move_forward(grow, self.width, self.height);
        let head_pos = self.snake.head_position();

        // Check collision after movement so collision/eat behavior happens on the correct tick.
        if self.snake.body[1..].contains(&head_pos) {
            self.game_over = true;
            self.play_sound(); // Play sound when game over
        }

        // Check if snake ate the food
        if grow {
            self.score += 10;
            self.update_high_score();
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
        if self.power_up.is_none() && rng.r#gen::<f32>() < self.power_up_tick_spawn_chance() {
            self.generate_power_up();
        }

        // Mark old and new body positions as dirty to support incremental redraw.
        for pos in old_body_positions {
            self.mark_position_dirty(pos);
        }
        let new_body_positions = self.snake.body.clone();
        for pos in new_body_positions {
            self.mark_position_dirty(pos);
        }
    }

    pub fn update_snake_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }

    pub fn play_sound(&self) {
        // Use terminal bell character to simulate sound
        if !self.muted {
            print!("\x07"); // Terminal bell
            let _ = std::io::stdout().flush();
        }
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_game() -> Game {
        let mut game = Game::new(Difficulty::Medium, 20, 12, 0);
        game.power_up = None;
        game.power_up_timer = None;
        game.active_speed_effect = None;
        game
    }

    #[test]
    fn snake_wraps_left_across_border() {
        let mut snake = Snake {
            body: vec![
                Position { x: 2, y: 5 },
                Position { x: 3, y: 5 },
                Position { x: 4, y: 5 },
            ],
            direction: Direction::Left,
        };

        snake.move_forward(false, 20, 12);
        assert_eq!(snake.head_position(), Position { x: 19, y: 5 });
    }

    #[test]
    fn snake_wraps_up_across_border() {
        let mut snake = Snake {
            body: vec![
                Position { x: 8, y: 2 },
                Position { x: 8, y: 3 },
                Position { x: 8, y: 4 },
            ],
            direction: Direction::Up,
        };

        snake.move_forward(false, 20, 12);
        assert_eq!(snake.head_position(), Position { x: 8, y: 11 });
    }

    #[test]
    fn snake_cannot_reverse_direction() {
        let mut snake = Snake {
            body: vec![
                Position { x: 5, y: 5 },
                Position { x: 6, y: 5 },
                Position { x: 7, y: 5 },
            ],
            direction: Direction::Left,
        };

        snake.change_direction(Direction::Right);
        assert_eq!(snake.direction, Direction::Left);
    }

    #[test]
    fn snake_can_turn_perpendicular() {
        let mut snake = Snake {
            body: vec![
                Position { x: 5, y: 5 },
                Position { x: 6, y: 5 },
                Position { x: 7, y: 5 },
            ],
            direction: Direction::Left,
        };

        snake.change_direction(Direction::Up);
        assert_eq!(snake.direction, Direction::Up);
    }

    #[test]
    fn tick_applies_food_collision_immediately() {
        let mut game = make_game();
        game.snake.body = vec![
            Position { x: 6, y: 5 },
            Position { x: 7, y: 5 },
            Position { x: 8, y: 5 },
        ];
        game.snake.direction = Direction::Left;
        game.food = Position { x: 5, y: 5 };

        game.tick();

        assert_eq!(game.score, 10);
        assert_eq!(game.snake.body.len(), 4);
        assert_eq!(game.snake.head_position(), Position { x: 5, y: 5 });
    }

    #[test]
    fn tick_detects_self_collision_after_move() {
        let mut game = make_game();
        game.snake.body = vec![
            Position { x: 5, y: 5 },
            Position { x: 5, y: 6 },
            Position { x: 6, y: 6 },
            Position { x: 6, y: 5 },
            Position { x: 6, y: 4 },
            Position { x: 5, y: 4 },
        ];
        game.snake.direction = Direction::Right;
        game.food = Position { x: 2, y: 2 };

        game.tick();

        assert!(game.game_over);
    }

    #[test]
    fn speed_effect_uses_collected_power_up_type() {
        let mut game = make_game();
        game.apply_power_up_effect(PowerUpType::SpeedBoost);
        game.power_up = Some(PowerUp {
            position: Position { x: 2, y: 2 },
            power_up_type: PowerUpType::SlowDown,
            active: true,
        });

        assert_eq!(game.speed_multiplier_percent(), 70);
    }

    #[test]
    fn speed_effect_expires_after_timer_runs_out() {
        let mut game = make_game();
        game.apply_power_up_effect(PowerUpType::SlowDown);

        for _ in 0..100 {
            game.update_power_up_effects();
        }

        assert_eq!(game.power_up_timer, None);
        assert!(game.active_speed_effect.is_none());
        assert_eq!(game.speed_multiplier_percent(), 100);
    }

    #[test]
    fn high_score_updates_when_score_increases() {
        let mut game = Game::new(Difficulty::Easy, 20, 12, 120);
        game.score = 130;
        game.update_high_score();
        assert_eq!(game.high_score, 130);
    }

    #[test]
    fn difficulty_speed_multiplier_scales_and_caps() {
        let mut game = make_game();
        game.score = 50;
        assert_eq!(game.difficulty_speed_multiplier_percent(), 97);

        game.score = 1_000;
        assert_eq!(game.difficulty_speed_multiplier_percent(), 55);
    }

    #[test]
    fn difficulty_tick_rates_get_faster_by_level() {
        let easy = Game::new(Difficulty::Easy, 20, 12, 0);
        let medium = Game::new(Difficulty::Medium, 20, 12, 0);
        let hard = Game::new(Difficulty::Hard, 20, 12, 0);
        let extreme = Game::new(Difficulty::Extreme, 20, 12, 0);

        let (easy_h, easy_v) = easy.get_tick_rates();
        let (med_h, med_v) = medium.get_tick_rates();
        let (hard_h, hard_v) = hard.get_tick_rates();
        let (ext_h, ext_v) = extreme.get_tick_rates();

        assert!(easy_h > med_h && med_h > hard_h && hard_h > ext_h);
        assert!(easy_v > med_v && med_v > hard_v && hard_v > ext_v);
    }

    #[test]
    fn power_up_spawn_chances_reduce_with_harder_difficulties() {
        let easy = Game::new(Difficulty::Easy, 20, 12, 0);
        let medium = Game::new(Difficulty::Medium, 20, 12, 0);
        let hard = Game::new(Difficulty::Hard, 20, 12, 0);
        let extreme = Game::new(Difficulty::Extreme, 20, 12, 0);

        assert!(
            easy.power_up_refresh_spawn_chance() > medium.power_up_refresh_spawn_chance()
                && medium.power_up_refresh_spawn_chance() > hard.power_up_refresh_spawn_chance()
                && hard.power_up_refresh_spawn_chance() > extreme.power_up_refresh_spawn_chance()
        );
        assert!(
            easy.power_up_tick_spawn_chance() > medium.power_up_tick_spawn_chance()
                && medium.power_up_tick_spawn_chance() > hard.power_up_tick_spawn_chance()
                && hard.power_up_tick_spawn_chance() > extreme.power_up_tick_spawn_chance()
        );
    }

    #[test]
    fn speed_effect_duration_shortens_with_harder_difficulties() {
        let easy = Game::new(Difficulty::Easy, 20, 12, 0);
        let medium = Game::new(Difficulty::Medium, 20, 12, 0);
        let hard = Game::new(Difficulty::Hard, 20, 12, 0);
        let extreme = Game::new(Difficulty::Extreme, 20, 12, 0);

        assert!(
            easy.speed_effect_duration_ticks() > medium.speed_effect_duration_ticks()
                && medium.speed_effect_duration_ticks() > hard.speed_effect_duration_ticks()
                && hard.speed_effect_duration_ticks() > extreme.speed_effect_duration_ticks()
        );
    }

    #[test]
    fn progression_scaling_is_stricter_for_harder_difficulties() {
        let mut easy = Game::new(Difficulty::Easy, 20, 12, 0);
        let mut medium = Game::new(Difficulty::Medium, 20, 12, 0);
        let mut hard = Game::new(Difficulty::Hard, 20, 12, 0);
        let mut extreme = Game::new(Difficulty::Extreme, 20, 12, 0);

        easy.score = 500;
        medium.score = 500;
        hard.score = 500;
        extreme.score = 500;

        assert_eq!(easy.difficulty_speed_multiplier_percent(), 80);
        assert_eq!(medium.difficulty_speed_multiplier_percent(), 70);
        assert_eq!(hard.difficulty_speed_multiplier_percent(), 60);
        assert_eq!(extreme.difficulty_speed_multiplier_percent(), 50);

        easy.score = 10_000;
        medium.score = 10_000;
        hard.score = 10_000;
        extreme.score = 10_000;

        assert_eq!(easy.difficulty_speed_multiplier_percent(), 76);
        assert_eq!(medium.difficulty_speed_multiplier_percent(), 55);
        assert_eq!(hard.difficulty_speed_multiplier_percent(), 52);
        assert_eq!(extreme.difficulty_speed_multiplier_percent(), 35);
    }

    #[test]
    fn find_food_spawn_position_returns_none_when_board_is_full() {
        let mut game = Game::new(Difficulty::Medium, 6, 6, 0);
        game.power_up = None;
        game.snake.body = (2..6)
            .flat_map(|y| (2..6).map(move |x| Position { x, y }))
            .collect();

        let mut rng = rand::thread_rng();
        assert!(game.find_food_spawn_position(&mut rng).is_none());
    }

    #[test]
    fn find_power_up_spawn_position_returns_none_when_only_food_cell_is_free() {
        let mut game = Game::new(Difficulty::Medium, 6, 6, 0);
        game.food = Position { x: 2, y: 2 };
        game.power_up = None;
        let food = game.food;
        game.snake.body = (2..6)
            .flat_map(|y| (2..6).map(move |x| Position { x, y }))
            .filter(|pos| *pos != food)
            .collect();

        let mut rng = rand::thread_rng();
        assert!(game.find_power_up_spawn_position(&mut rng).is_none());
    }
}
