//! UI and rendering module for the Snake game.
//! Handles all terminal-based graphics and user interface elements.

use std::cell::RefCell;
use std::fmt::{self, Write as _};
#[cfg(not(test))]
use std::io::Write as _;
#[cfg(test)]
use std::sync::{Mutex, OnceLock};

thread_local! {
    static RENDER_CAPTURE: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub(crate) fn emit(args: fmt::Arguments<'_>) {
    RENDER_CAPTURE.with(|slot| {
        let mut slot = slot.borrow_mut();
        if let Some(buffer) = slot.as_mut() {
            let _ = buffer.write_fmt(args);
            return;
        }

        #[cfg(test)]
        {
            let _ = args;
        }

        #[cfg(not(test))]
        {
            let mut stdout = std::io::stdout();
            let _ = stdout.write_fmt(args);
        }
    });
}

#[cfg(test)]
pub(crate) fn begin_capture() {
    RENDER_CAPTURE.with(|slot| {
        *slot.borrow_mut() = Some(String::new());
    });
}

#[cfg(test)]
pub(crate) fn end_capture() -> String {
    RENDER_CAPTURE.with(|slot| slot.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
pub(crate) fn render_test_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::render::emit(format_args!($($arg)*));
    }};
}

mod gameplay;
mod hud;
mod menu;
mod shared;

pub use gameplay::{clear_for_menu_entry, draw, draw_size_warning, draw_static_frame};
pub use menu::{HighScoresRenderRequest, MenuRenderRequest, draw_high_scores_menu, draw_menu};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Game;
    use crate::layout;
    use crate::storage::HighScores;
    use crate::utils::{Difficulty, Direction, Language, Position, PowerUp, PowerUpType};
    use std::fs;
    use std::path::PathBuf;

    fn capture_render_output<F: FnOnce()>(render_fn: F) -> String {
        begin_capture();
        render_fn();
        end_capture()
    }

    fn snapshot_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("render")
            .join("snapshots")
            .join(name)
    }

    fn assert_snapshot(name: &str, actual: &str) {
        let path = snapshot_path(name);

        if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
            fs::write(&path, actual).expect("failed to write snapshot");
            return;
        }

        let expected = fs::read_to_string(&path).unwrap_or_else(|_| {
            panic!(
                "missing snapshot file: {} (run with UPDATE_SNAPSHOTS=1 to generate)",
                path.display()
            )
        });
        assert_eq!(actual, expected, "snapshot mismatch for {}", path.display());
    }

    #[test]
    fn ansi_snapshot_main_menu_screen() {
        let _guard = render_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let options = vec![
            "Play".to_string(),
            "Difficulty: Extreme".to_string(),
            "High Scores".to_string(),
            "Settings".to_string(),
            "Quit".to_string(),
        ];

        let ansi = capture_render_output(|| {
            clear_for_menu_entry();
            draw_menu(MenuRenderRequest {
                screen_tag: "MENU",
                title: "SNAKE GAME",
                subtitle: Some("Difficulty: Extreme"),
                options: &options,
                selected_option: 0,
                danger_option: None,
                term_width: 120,
                term_height: 40,
                language: Language::En,
                compact: false,
            });
        });

        assert_snapshot("main_menu.ansi", &ansi);
    }

    #[test]
    fn ansi_snapshot_high_scores_screen() {
        let _guard = render_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let high_scores = HighScores {
            easy: 50,
            medium: 80,
            hard: 120,
            extreme: 460,
        };

        let ansi = capture_render_output(|| {
            clear_for_menu_entry();
            draw_high_scores_menu(HighScoresRenderRequest {
                high_scores: &high_scores,
                term_width: 120,
                term_height: 40,
                language: Language::En,
                compact: false,
            });
        });

        assert_snapshot("high_scores.ansi", &ansi);
    }

    #[test]
    fn ansi_snapshot_game_over_panel() {
        let _guard = render_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let mut game = Game::new(
            Difficulty::Extreme,
            crate::utils::WIDTH,
            crate::utils::HEIGHT,
            460,
        );
        game.snake.body = vec![
            Position { x: 8, y: 8 },
            Position { x: 7, y: 8 },
            Position { x: 6, y: 8 },
        ];
        game.snake.direction = Direction::Right;
        game.food = Position { x: 20, y: 10 };
        game.power_up = Some(PowerUp {
            position: Position { x: 15, y: 6 },
            power_up_type: PowerUpType::SpeedBoost,
            active: true,
        });
        game.score = 123;
        game.high_score = 460;
        game.game_over = true;
        game.paused = false;
        game.muted = false;
        game.dirty_positions.clear();

        let layout = layout::compute_layout(120, 40, game.width, game.height, Language::En)
            .expect("layout should fit snapshot terminal");

        let ansi = capture_render_output(|| {
            draw_static_frame(&layout);
            draw(&mut game, &layout, Language::En);
        });

        assert_snapshot("game_over_panel.ansi", &ansi);
    }
}
