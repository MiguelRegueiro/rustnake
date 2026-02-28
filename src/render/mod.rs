//! UI and rendering module for the Snake game.
//! Handles all terminal-based graphics and user interface elements.

mod gameplay;
mod hud;
mod menu;
mod shared;

pub use gameplay::{clear_for_menu_entry, draw, draw_size_warning, draw_static_frame};
pub use menu::{HighScoresRenderRequest, MenuRenderRequest, draw_high_scores_menu, draw_menu};
