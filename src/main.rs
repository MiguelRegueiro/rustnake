//! Main entry point for the Snake game.
//! Orchestrates the game loop, input handling, and rendering.

use crossterm::{
    cursor::{Hide, Show},
    event::{DisableFocusChange, EnableFocusChange},
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
mod i18n;
mod input;
mod layout;
mod render;
mod storage;
mod utils;

use core::Game;
use input::GameInput;
<<<<<<< HEAD
use storage::{HighScores, Settings};
=======
use storage::HighScores;
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
use utils::{Difficulty, Language};

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = stdout();
        let _ = execute!(stdout, DisableFocusChange, LeaveAlternateScreen, Show);
    }
}

<<<<<<< HEAD
fn persist_config(high_scores: &HighScores, settings: Settings) {
    let config = storage::AppConfig {
        high_scores: *high_scores,
        settings,
=======
fn persist_config(high_scores: &HighScores, language: Language) {
    let config = storage::AppConfig {
        high_scores: *high_scores,
        settings: storage::Settings { language },
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
    };
    let _ = storage::save_config(&config);
}

<<<<<<< HEAD
#[derive(Clone, Copy)]
enum MenuScreen {
    Main,
    Difficulty,
    Settings,
    Language,
    ResetScoresConfirm,
}

fn difficulty_to_index(difficulty: Difficulty) -> usize {
    match difficulty {
        Difficulty::Easy => 0,
        Difficulty::Medium => 1,
        Difficulty::Hard => 2,
        Difficulty::Extreme => 3,
    }
}

fn difficulty_from_index(index: usize) -> Difficulty {
    match index {
        0 => Difficulty::Easy,
        1 => Difficulty::Medium,
        2 => Difficulty::Hard,
        3 => Difficulty::Extreme,
        _ => Difficulty::Medium,
    }
}

fn show_menu(
    rx: &mpsc::Receiver<GameInput>,
    term_size: &mut (u16, u16),
    settings: &mut Settings,
    selected_difficulty: &mut Difficulty,
    high_scores: &mut HighScores,
) -> Option<Difficulty> {
    let mut screen = MenuScreen::Main;
    let mut main_selected = 0usize;
    let mut difficulty_selected = difficulty_to_index(*selected_difficulty);
    let mut settings_selected = 0usize;
    let mut language_selected = settings.language.to_index();
    let mut reset_selected = 1usize; // Default to "No"

    loop {
        let ui_language = settings.language;
=======
fn show_menu(
    rx: &mpsc::Receiver<GameInput>,
    term_size: &mut (u16, u16),
    language: &mut Language,
    high_scores: &HighScores,
) -> Option<Difficulty> {
    let mut selected_option = 0; // 0 = Easy, 1 = Medium, 2 = Hard, 3 = Extreme
    let mut showing_language_popup = false;
    let mut popup_language_index = language.to_index();
    let max_language_index = Language::ALL.len().saturating_sub(1);

    loop {
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
        let layout_check = layout::compute_layout(
            term_size.0,
            term_size.1,
            utils::WIDTH,
            utils::HEIGHT,
<<<<<<< HEAD
            ui_language,
        );
        match layout_check {
            Ok(_) => {
                let (title, options, selected) = match screen {
                    MenuScreen::Main => (
                        i18n::menu_title(ui_language),
                        vec![
                            i18n::menu_play(ui_language).to_string(),
                            format!(
                                "{}: {}",
                                i18n::menu_difficulty(ui_language),
                                i18n::difficulty_label(ui_language, *selected_difficulty)
                            ),
                            i18n::menu_settings(ui_language).to_string(),
                            i18n::menu_quit(ui_language).to_string(),
                        ],
                        main_selected,
                    ),
                    MenuScreen::Difficulty => (
                        i18n::difficulty_menu_title(ui_language),
                        vec![
                            i18n::difficulty_label(ui_language, Difficulty::Easy).to_string(),
                            i18n::difficulty_label(ui_language, Difficulty::Medium).to_string(),
                            i18n::difficulty_label(ui_language, Difficulty::Hard).to_string(),
                            i18n::difficulty_label(ui_language, Difficulty::Extreme).to_string(),
                            i18n::menu_back(ui_language).to_string(),
                        ],
                        difficulty_selected,
                    ),
                    MenuScreen::Settings => (
                        i18n::menu_settings(ui_language),
                        vec![
                            format!(
                                "{}: {}",
                                i18n::language_label(ui_language),
                                i18n::language_name(settings.language)
                            ),
                            format!(
                                "{}: {}",
                                i18n::settings_pause_on_focus_loss_label(ui_language),
                                if settings.pause_on_focus_loss {
                                    i18n::setting_on(ui_language)
                                } else {
                                    i18n::setting_off(ui_language)
                                }
                            ),
                            format!(
                                "{}: {}",
                                i18n::settings_sound_label(ui_language),
                                if settings.sound_on {
                                    i18n::setting_on(ui_language)
                                } else {
                                    i18n::setting_off(ui_language)
                                }
                            ),
                            i18n::settings_reset_high_scores_label(ui_language).to_string(),
                            i18n::menu_back(ui_language).to_string(),
                        ],
                        settings_selected,
                    ),
                    MenuScreen::Language => {
                        let mut options: Vec<String> = Language::ALL
                            .iter()
                            .map(|language| i18n::language_name(*language).to_string())
                            .collect();
                        options.push(i18n::menu_back(ui_language).to_string());
                        (
                            i18n::language_popup_title(ui_language),
                            options,
                            language_selected,
                        )
                    }
                    MenuScreen::ResetScoresConfirm => (
                        i18n::reset_high_scores_title(ui_language),
                        vec![
                            i18n::confirm_yes(ui_language).to_string(),
                            i18n::confirm_no(ui_language).to_string(),
                        ],
                        reset_selected,
                    ),
                };
                render::draw_menu(
                    title,
                    &options,
                    selected,
                    term_size.0,
                    term_size.1,
                    ui_language,
                );
            }
            Err(size_check) => render::draw_size_warning(size_check, ui_language),
        }

        if let Ok(input_cmd) = rx.recv() {
            let max_index = match screen {
                MenuScreen::Main => 3,
                MenuScreen::Difficulty => 4,
                MenuScreen::Settings => 4,
                MenuScreen::Language => Language::ALL.len(),
                MenuScreen::ResetScoresConfirm => 1,
            };
=======
            *language,
        );
        match layout_check {
            Ok(_) => {
                let popup_selection = if showing_language_popup {
                    Some(Language::from_index(popup_language_index))
                } else {
                    None
                };
                render::draw_menu(
                    selected_option,
                    term_size.0,
                    term_size.1,
                    *language,
                    popup_selection,
                );
            }
            Err(size_check) => render::draw_size_warning(size_check, *language),
        }

        if let Ok(input_cmd) = rx.recv() {
            if showing_language_popup {
                match input_cmd {
                    GameInput::Resize(width, height) => {
                        *term_size = (width, height);
                    }
                    GameInput::Direction(utils::Direction::Up) => {
                        popup_language_index = popup_language_index.saturating_sub(1);
                    }
                    GameInput::Direction(utils::Direction::Down) => {
                        popup_language_index = (popup_language_index + 1).min(max_language_index);
                    }
                    GameInput::MenuSelect(option) => {
                        popup_language_index = option.min(max_language_index);
                    }
                    GameInput::MenuConfirm => {
                        *language = Language::from_index(popup_language_index);
                        persist_config(high_scores, *language);
                        showing_language_popup = false;
                    }
                    GameInput::CycleLanguage => {
                        showing_language_popup = false;
                    }
                    GameInput::Quit => {
                        return None;
                    }
                    _ => {}
                }
                continue;
            }

>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
            match input_cmd {
                GameInput::Resize(width, height) => {
                    *term_size = (width, height);
                }
                GameInput::CycleLanguage => {
                    if layout_check.is_ok() {
                        showing_language_popup = true;
                        popup_language_index = language.to_index();
                    }
                }
                GameInput::MenuSelect(option) => {
<<<<<<< HEAD
                    let selection = option.min(max_index);
                    match screen {
                        MenuScreen::Main => main_selected = selection,
                        MenuScreen::Difficulty => difficulty_selected = selection,
                        MenuScreen::Settings => settings_selected = selection,
                        MenuScreen::Language => language_selected = selection,
                        MenuScreen::ResetScoresConfirm => reset_selected = selection,
                    }
                }
                GameInput::Direction(utils::Direction::Up) => match screen {
                    MenuScreen::Main => main_selected = main_selected.saturating_sub(1),
                    MenuScreen::Difficulty => {
                        difficulty_selected = difficulty_selected.saturating_sub(1)
=======
                    if option <= 3 {
                        selected_option = option;
                    }
                }
                GameInput::Direction(utils::Direction::Up) => {
                    selected_option = selected_option.saturating_sub(1);
                }
                GameInput::Direction(utils::Direction::Down) => {
                    if selected_option < 3 {
                        selected_option += 1;
                    }
                }
                GameInput::MenuConfirm => {
                    if layout_check.is_ok() {
                        return Some(match selected_option {
                            0 => Difficulty::Easy,
                            1 => Difficulty::Medium,
                            2 => Difficulty::Hard,
                            3 => Difficulty::Extreme,
                            _ => Difficulty::Medium, // fallback
                        });
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
                    }
                    MenuScreen::Settings => settings_selected = settings_selected.saturating_sub(1),
                    MenuScreen::Language => language_selected = language_selected.saturating_sub(1),
                    MenuScreen::ResetScoresConfirm => {
                        reset_selected = reset_selected.saturating_sub(1)
                    }
                },
                GameInput::Direction(utils::Direction::Down) => match screen {
                    MenuScreen::Main => main_selected = (main_selected + 1).min(3),
                    MenuScreen::Difficulty => {
                        difficulty_selected = (difficulty_selected + 1).min(4)
                    }
                    MenuScreen::Settings => settings_selected = (settings_selected + 1).min(4),
                    MenuScreen::Language => {
                        language_selected = (language_selected + 1).min(Language::ALL.len())
                    }
                    MenuScreen::ResetScoresConfirm => reset_selected = (reset_selected + 1).min(1),
                },
                GameInput::MenuConfirm => match screen {
                    MenuScreen::Main => match main_selected {
                        0 => {
                            if layout_check.is_ok() {
                                return Some(*selected_difficulty);
                            }
                        }
                        1 => {
                            difficulty_selected = difficulty_to_index(*selected_difficulty);
                            screen = MenuScreen::Difficulty;
                        }
                        2 => screen = MenuScreen::Settings,
                        3 => return None,
                        _ => {}
                    },
                    MenuScreen::Difficulty => {
                        if difficulty_selected <= 3 {
                            *selected_difficulty = difficulty_from_index(difficulty_selected);
                            settings.default_difficulty = *selected_difficulty;
                            persist_config(high_scores, *settings);
                        }
                        screen = MenuScreen::Main;
                    }
                    MenuScreen::Settings => match settings_selected {
                        0 => {
                            language_selected = settings.language.to_index();
                            screen = MenuScreen::Language;
                        }
                        1 => {
                            settings.pause_on_focus_loss = !settings.pause_on_focus_loss;
                            persist_config(high_scores, *settings);
                        }
                        2 => {
                            settings.sound_on = !settings.sound_on;
                            persist_config(high_scores, *settings);
                        }
                        3 => {
                            reset_selected = 1;
                            screen = MenuScreen::ResetScoresConfirm;
                        }
                        4 => screen = MenuScreen::Main,
                        _ => {}
                    },
                    MenuScreen::Language => {
                        if language_selected < Language::ALL.len() {
                            settings.language = Language::ALL[language_selected];
                            persist_config(high_scores, *settings);
                        }
                        screen = MenuScreen::Settings;
                    }
                    MenuScreen::ResetScoresConfirm => {
                        if reset_selected == 0 {
                            *high_scores = HighScores::default();
                            persist_config(high_scores, *settings);
                        }
                        screen = MenuScreen::Settings;
                    }
                },
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
    execute!(stdout, EnterAlternateScreen, Hide, EnableFocusChange)?;
    enable_raw_mode()?;
    let _terminal_guard = TerminalGuard;

    // Input handling channel
    let rx = input::setup_input_handler();
    let config = storage::load_config();
    let mut high_scores: HighScores = config.high_scores;
<<<<<<< HEAD
    let mut settings: Settings = config.settings;
    let mut selected_difficulty = settings.default_difficulty;
=======
    let mut language: Language = config.settings.language;
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
    let mut term_size = layout::terminal_size();

    // Main game loop with restart capability
    'game_loop: loop {
        // Show difficulty selection menu
<<<<<<< HEAD
        let Some(difficulty) = show_menu(
            &rx,
            &mut term_size,
            &mut settings,
            &mut selected_difficulty,
            &mut high_scores,
        ) else {
=======
        let Some(difficulty) = show_menu(&rx, &mut term_size, &mut language, &high_scores) else {
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
            break;
        };

        // Create new game instance with selected difficulty
        let mut game = Game::new(
            difficulty,
            utils::WIDTH,
            utils::HEIGHT,
            high_scores.get(difficulty),
        );
        game.muted = !settings.sound_on;
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
                        GameInput::FocusLost => {
                            if settings.pause_on_focus_loss && !game.is_paused() {
                                game.toggle_pause();
                            }
                        }
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

                let layout = match layout::compute_layout(
                    term_size.0,
                    term_size.1,
                    game.width,
                    game.height,
<<<<<<< HEAD
                    settings.language,
                ) {
                    Ok(layout) => layout,
                    Err(size_check) => {
                        render::draw_size_warning(size_check, settings.language);
=======
                    language,
                ) {
                    Ok(layout) => layout,
                    Err(size_check) => {
                        render::draw_size_warning(size_check, language);
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
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
<<<<<<< HEAD
                        persist_config(&high_scores, settings);
=======
                        persist_config(&high_scores, language);
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
                    }
                    last_tick = Instant::now();
                }

                // Draw everything
<<<<<<< HEAD
                render::draw(&mut game, &layout, settings.language);
=======
                render::draw(&mut game, &layout, language);
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
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

                let layout = match layout::compute_layout(
                    term_size.0,
                    term_size.1,
                    game.width,
                    game.height,
<<<<<<< HEAD
                    settings.language,
                ) {
                    Ok(layout) => layout,
                    Err(size_check) => {
                        render::draw_size_warning(size_check, settings.language);
=======
                    language,
                ) {
                    Ok(layout) => layout,
                    Err(size_check) => {
                        render::draw_size_warning(size_check, language);
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
                        active_layout = None;
                        thread::sleep(Duration::from_millis(25));
                        continue;
                    }
                };
                if active_layout != Some(layout) {
                    render::draw_static_frame(&layout);
                    active_layout = Some(layout);
                }
<<<<<<< HEAD
                render::draw(&mut game, &layout, settings.language);
=======
                render::draw(&mut game, &layout, language);
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
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
