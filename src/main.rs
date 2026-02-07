//! Main entry point for the Snake game.
//! Orchestrates the game loop, input handling, and rendering.

use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

mod core;
mod input;
mod render;
mod utils;

use core::Game;
use input::GameInput;
use utils::Difficulty;

fn show_menu(rx: &mpsc::Receiver<GameInput>) -> Difficulty {
    let mut selected_option = 0; // 0 = Easy, 1 = Medium, 2 = Hard

    loop {
        render::draw_menu(selected_option, utils::WIDTH, utils::HEIGHT);

        if let Ok(input_cmd) = rx.recv() {
            match input_cmd {
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
                    return match selected_option {
                        0 => Difficulty::Easy,
                        1 => Difficulty::Medium,
                        2 => Difficulty::Hard,
                        _ => Difficulty::Medium, // fallback
                    };
                }
                GameInput::Quit => {
                    // Exit the game
                    disable_raw_mode().unwrap();
                    execute!(stdout(), LeaveAlternateScreen, Show).unwrap();
                    std::process::exit(0);
                }
                _ => {} // Ignore other inputs
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    // Input handling channel
    let rx = input::setup_input_handler();

    // Main game loop with restart capability
    'game_loop: loop {
        // Show difficulty selection menu
        let difficulty = show_menu(&rx);

        // Create new game instance with selected difficulty
        let mut game = Game::new(difficulty, utils::WIDTH, utils::HEIGHT);
        let mut last_tick = Instant::now();

        // Get tick rates based on difficulty
        let (horizontal_tick_rate, vertical_tick_rate) = game.get_tick_rates();

        // Track last input time to prevent rapid key presses
        let mut last_input_time = Instant::now();
        let input_cooldown = Duration::from_millis(50); // 50ms cooldown between inputs

        loop {
            // Handle inputs during normal gameplay (only when not game over)
            if !game.game_over {
                while let Ok(input_cmd) = rx.try_recv() {
                    // Process MenuConfirm immediately, otherwise respect cooldown
                    match input_cmd {
                        GameInput::MenuConfirm => {
                            // Allow immediate menu return regardless of cooldown
                            break; // Go back to menu (when space is pressed)
                        }
                        _ => {
                            // Only process other inputs if enough time has passed since the last input
                            if last_input_time.elapsed() >= input_cooldown {
                                match input_cmd {
                                    GameInput::Quit => break 'game_loop,
                                    GameInput::Pause => game.toggle_pause(), // Pause/unpause the game
                                    GameInput::ToggleMute => game.toggle_mute(), // Toggle mute
                                    GameInput::Direction(direction) => game.update_snake_direction(direction),
                                    _ => {}
                                }
                                last_input_time = Instant::now();
                            }
                        }
                    }
                }

                // Determine the tick rate based on the current direction and power-ups
                let mut effective_horizontal_rate = horizontal_tick_rate;
                let mut effective_vertical_rate = vertical_tick_rate;

                // Apply power-up effects to speed
                if let Some(_power_up_timer) = game.power_up_timer {
                    if let Some(power_up_type) = game.power_up.as_ref().map(|p| p.power_up_type) {
                        match power_up_type {
                            utils::PowerUpType::SpeedBoost => {
                                // Increase speed (decrease duration)
                                effective_horizontal_rate = Duration::from_millis(
                                    (horizontal_tick_rate.as_millis() as u64 * 70 / 100).max(20),
                                );
                                effective_vertical_rate = Duration::from_millis(
                                    (vertical_tick_rate.as_millis() as u64 * 70 / 100).max(20),
                                );
                            }
                            utils::PowerUpType::SlowDown => {
                                // Decrease speed (increase duration)
                                effective_horizontal_rate = Duration::from_millis(
                                    horizontal_tick_rate.as_millis() as u64 * 150 / 100,
                                );
                                effective_vertical_rate = Duration::from_millis(
                                    vertical_tick_rate.as_millis() as u64 * 150 / 100,
                                );
                            }
                            _ => {} // Other power-ups don't affect speed
                        }
                    }
                }

                let tick_rate = match game.snake.direction {
                    utils::Direction::Up | utils::Direction::Down => effective_vertical_rate,
                    utils::Direction::Left | utils::Direction::Right => effective_horizontal_rate,
                };

                // Update game state
                if !game.game_over && !game.is_paused() && last_tick.elapsed() >= tick_rate {
                    game.tick();
                    last_tick = Instant::now();
                }

                // Draw everything
                render::draw(&game);
            }

            // Check for game over and handle input differently
            if game.game_over {
                // During game over, we handle input from the channel
                if let Ok(input_cmd) = rx.recv_timeout(Duration::from_millis(100)) {
                    match input_cmd {
                        GameInput::MenuConfirm => { // Space bar to restart
                            continue 'game_loop; // Restart the game
                        }
                        GameInput::Quit => { // 'q' key to quit
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

    // Restore terminal
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    Ok(())
}
