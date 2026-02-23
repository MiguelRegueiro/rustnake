//! Main entry point for the Snake game.
//! Orchestrates the game loop, input handling, and rendering.

use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    collections::VecDeque,
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

mod core;
mod input;
mod layout;
mod render;
mod storage;
mod utils;

use core::Game;
use input::GameInput;
use storage::HighScores;
use utils::Difficulty;

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = stdout();
        let _ = execute!(stdout, LeaveAlternateScreen, Show);
    }
}

fn show_menu(rx: &mpsc::Receiver<GameInput>, term_size: &mut (u16, u16)) -> Option<Difficulty> {
    let mut selected_option = 0; // 0 = Easy, 1 = Medium, 2 = Hard

    loop {
        let layout_check =
            layout::compute_layout(term_size.0, term_size.1, utils::WIDTH, utils::HEIGHT);
        match layout_check {
            Ok(_) => render::draw_menu(selected_option, term_size.0, term_size.1),
            Err(size_check) => render::draw_size_warning(size_check),
        }

        if let Ok(input_cmd) = rx.recv() {
            match input_cmd {
                GameInput::Resize(width, height) => {
                    *term_size = (width, height);
                }
                GameInput::MenuSelect(option) => {
                    selected_option = option.min(2);
                }
                GameInput::Direction(utils::Direction::Up) => {
                    selected_option = selected_option.saturating_sub(1);
                }
                GameInput::Direction(utils::Direction::Down) => {
                    if selected_option < 2 {
                        selected_option += 1;
                    }
                }
                GameInput::MenuConfirm => {
                    if layout_check.is_ok() {
                        return Some(match selected_option {
                            0 => Difficulty::Easy,
                            1 => Difficulty::Medium,
                            2 => Difficulty::Hard,
                            _ => Difficulty::Medium, // fallback
                        });
                    }
                }
                GameInput::Quit => {
                    return None;
                }
                _ => {} // Ignore other inputs
            }
        }
    }
}

fn is_reverse_direction(current: utils::Direction, next: utils::Direction) -> bool {
    matches!(
        (current, next),
        (utils::Direction::Up, utils::Direction::Down)
            | (utils::Direction::Down, utils::Direction::Up)
            | (utils::Direction::Left, utils::Direction::Right)
            | (utils::Direction::Right, utils::Direction::Left)
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;
    let _terminal_guard = TerminalGuard;

    // Input handling channel
    let rx = input::setup_input_handler();
    let mut high_scores: HighScores = storage::load_high_scores();
    let mut term_size = layout::terminal_size();

    // Main game loop with restart capability
    'game_loop: loop {
        // Show difficulty selection menu
        let Some(difficulty) = show_menu(&rx, &mut term_size) else {
            break;
        };

        // Create new game instance with selected difficulty
        let mut game = Game::new(
            difficulty,
            utils::WIDTH,
            utils::HEIGHT,
            high_scores.get(difficulty),
        );
        let mut active_layout: Option<layout::Layout> = None;
        let mut last_tick = Instant::now();
        let mut direction_queue: VecDeque<utils::Direction> = VecDeque::with_capacity(2);

        // Get tick rates based on difficulty
        let (horizontal_tick_rate, vertical_tick_rate) = game.get_tick_rates();

        loop {
            let mut return_to_menu = false;

            // Handle inputs during normal gameplay (only when not game over)
            if !game.game_over {
                while let Ok(input_cmd) = rx.try_recv() {
                    // Process MenuConfirm immediately, otherwise respect cooldown
                    match input_cmd {
                        GameInput::Resize(width, height) => {
                            term_size = (width, height);
                        }
                        GameInput::MenuConfirm => {
                            return_to_menu = true;
                            break;
                        }
                        GameInput::Quit => break 'game_loop,
                        GameInput::Pause => game.toggle_pause(), // Pause/unpause the game
                        GameInput::ToggleMute => game.toggle_mute(), // Toggle mute
                        GameInput::Direction(direction) => {
                            let reference_direction = direction_queue
                                .back()
                                .copied()
                                .unwrap_or(game.snake.direction);
                            let is_same_direction = direction == reference_direction;
                            if !is_same_direction
                                && !is_reverse_direction(reference_direction, direction)
                            {
                                if direction_queue.len() >= 2 {
                                    direction_queue.pop_back();
                                }
                                direction_queue.push_back(direction);
                            }
                        }
                        _ => {}
                    }
                }

                if return_to_menu {
                    continue 'game_loop;
                }

                let layout =
                    match layout::compute_layout(term_size.0, term_size.1, game.width, game.height)
                    {
                        Ok(layout) => layout,
                        Err(size_check) => {
                            render::draw_size_warning(size_check);
                            active_layout = None;
                            thread::sleep(Duration::from_millis(25));
                            continue;
                        }
                    };
                if active_layout != Some(layout) {
                    render::draw_static_frame(&layout);
                    active_layout = Some(layout);
                }

                // Determine the tick rate based on the current direction and power-ups
                let progression_multiplier = game.difficulty_speed_multiplier_percent();
                let power_up_multiplier = game.speed_multiplier_percent();
                let speed_multiplier = progression_multiplier * power_up_multiplier / 100;
                let effective_horizontal_rate = Duration::from_millis(
                    (horizontal_tick_rate.as_millis() as u64 * speed_multiplier / 100).max(20),
                );
                let effective_vertical_rate = Duration::from_millis(
                    (vertical_tick_rate.as_millis() as u64 * speed_multiplier / 100).max(20),
                );

                let direction_for_tick_rate = direction_queue
                    .front()
                    .copied()
                    .unwrap_or(game.snake.direction);
                let tick_rate = match direction_for_tick_rate {
                    utils::Direction::Up | utils::Direction::Down => effective_vertical_rate,
                    utils::Direction::Left | utils::Direction::Right => effective_horizontal_rate,
                };

                // Update game state
                if !game.game_over && !game.is_paused() && last_tick.elapsed() >= tick_rate {
                    if let Some(direction) = direction_queue.pop_front() {
                        game.update_snake_direction(direction);
                    }
                    game.tick();
                    if game.high_score > high_scores.get(difficulty) {
                        high_scores.set(difficulty, game.high_score);
                        let _ = storage::save_high_scores(&high_scores);
                    }
                    last_tick = Instant::now();
                }

                // Draw everything
                render::draw(&mut game, &layout);
            } else {
                while let Ok(input_cmd) = rx.try_recv() {
                    match input_cmd {
                        GameInput::Resize(width, height) => {
                            term_size = (width, height);
                        }
                        GameInput::MenuConfirm => {
                            // Space bar to go back to menu
                            continue 'game_loop;
                        }
                        GameInput::Quit => {
                            break 'game_loop; // Quit the game
                        }
                        _ => {}
                    }
                }

                let layout =
                    match layout::compute_layout(term_size.0, term_size.1, game.width, game.height)
                    {
                        Ok(layout) => layout,
                        Err(size_check) => {
                            render::draw_size_warning(size_check);
                            active_layout = None;
                            thread::sleep(Duration::from_millis(25));
                            continue;
                        }
                    };
                if active_layout != Some(layout) {
                    render::draw_static_frame(&layout);
                    active_layout = Some(layout);
                }
                render::draw(&mut game, &layout);
            }

            // Check for game over and handle input differently
            if game.game_over {
                // During game over, we handle input from the channel
                if let Ok(input_cmd) = rx.recv_timeout(Duration::from_millis(100)) {
                    match input_cmd {
                        GameInput::Resize(width, height) => {
                            term_size = (width, height);
                        }
                        GameInput::MenuConfirm => {
                            // Space bar to go back to menu
                            continue 'game_loop;
                        }
                        GameInput::Quit => {
                            // 'q' key to quit
                            break 'game_loop; // Quit the game
                        }
                        _ => {} // Ignore other inputs during game over
                    }
                }
            } else {
                // Small delay to prevent excessive CPU usage (only when not game over)
                thread::sleep(Duration::from_millis(10));
            }
        }
        // When we break from the inner loop (either game over or back to menu),
        // we continue to the next iteration of the outer loop which shows the menu again
    }

    Ok(())
}
