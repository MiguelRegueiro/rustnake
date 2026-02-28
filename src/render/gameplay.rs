use crate::core::Game;
use crate::i18n;
use crate::layout::{Layout, SizeCheck};
use crate::utils::Language;
use std::io::Write;

use super::hud;
use super::menu;
use super::shared::{ANSI_RESET, STYLE_MENU_BORDER, center_start, draw_centered_line};

fn draw_border(layout: &Layout) {
    let inner_width = layout.map_width.saturating_sub(2) as usize;
    let top = format!("┌{}┐", "─".repeat(inner_width));
    let bottom = format!("└{}┘", "─".repeat(inner_width));

    print!(
        "{}\x1b[{};{}H{}{}",
        STYLE_MENU_BORDER, layout.origin_y, layout.origin_x, top, ANSI_RESET
    );
    print!(
        "{}\x1b[{};{}H{}{}",
        STYLE_MENU_BORDER,
        layout.map_bottom(),
        layout.origin_x,
        bottom,
        ANSI_RESET
    );

    for y in (layout.origin_y + 1)..layout.map_bottom() {
        print!(
            "{}\x1b[{};{}H│{}",
            STYLE_MENU_BORDER, y, layout.origin_x, ANSI_RESET
        );
        print!(
            "{}\x1b[{};{}H│{}",
            STYLE_MENU_BORDER,
            y,
            layout.map_right(),
            ANSI_RESET
        );
    }
}

pub fn draw_static_frame(layout: &Layout) {
    menu::invalidate_menu_render_caches();
    print!("\x1b[2J\x1b[H");
    draw_border(layout);

    let _ = std::io::stdout().flush();
}

pub fn clear_for_menu_entry() {
    menu::invalidate_menu_render_caches();
    print!("\x1b[2J\x1b[H");
    let _ = std::io::stdout().flush();
}

pub fn draw_size_warning(size_check: SizeCheck, language: Language) {
    menu::invalidate_menu_render_caches();
    print!("\x1b[2J\x1b[H");
    let start_y = center_start(size_check.current_height, 5);
    draw_centered_line(
        start_y,
        size_check.current_width,
        i18n::small_window_title(language),
    );
    draw_centered_line(
        start_y + 1,
        size_check.current_width,
        &format!(
            "{}: {}x{}  {}: {}x{}",
            i18n::small_window_current_label(language),
            size_check.current_width,
            size_check.current_height,
            i18n::small_window_minimum_label(language),
            size_check.minimum.width,
            size_check.minimum.height
        ),
    );
    draw_centered_line(
        start_y + 3,
        size_check.current_width,
        i18n::small_window_hint(language),
    );

    let _ = std::io::stdout().flush();
}

pub fn draw(game: &mut Game, layout: &Layout, language: Language) {
    menu::invalidate_menu_render_caches();
    for pos in &game.dirty_positions {
        let (x, y) = layout.board_to_screen(pos.x, pos.y);
        print!("\x1b[{};{}H ", y, x);
    }

    draw_border(layout);

    for (i, pos) in game.snake.body.iter().enumerate() {
        // Head is bright green, body segments get darker toward the tail.
        let color = if i == 0 {
            "\x1b[92m"
        } else if i < game.snake.body.len() / 3 {
            "\x1b[32m"
        } else if i < game.snake.body.len() * 2 / 3 {
            "\x1b[33m"
        } else {
            "\x1b[90m"
        };

        let (x, y) = layout.board_to_screen(pos.x, pos.y);
        print!("\x1b[{};{}H{}", y, x, color);

        if i == 0 {
            let head_symbol = match game.snake.direction {
                crate::utils::Direction::Up | crate::utils::Direction::Down => "█",
                crate::utils::Direction::Left | crate::utils::Direction::Right => "█",
            };
            print!("{}", head_symbol);
        } else {
            print!("■");
        }
    }

    let food_symbol = if game.score % 50 == 0 && game.score != 0 {
        "★"
    } else {
        "●"
    };
    let (food_x, food_y) = layout.board_to_screen(game.food.x, game.food.y);
    print!("\x1b[{};{}H\x1b[91m{}", food_y, food_x, food_symbol);

    if let Some(power_up) = game.power_up {
        let (symbol, color) = match power_up.power_up_type {
            crate::utils::PowerUpType::SpeedBoost => (">", "\x1b[94m"),
            crate::utils::PowerUpType::SlowDown => ("<", "\x1b[96m"),
            crate::utils::PowerUpType::ExtraPoints => ("$", "\x1b[93m"),
            crate::utils::PowerUpType::Grow => ("+", "\x1b[92m"),
            crate::utils::PowerUpType::Shrink => ("-", "\x1b[95m"),
        };
        let (power_up_x, power_up_y) =
            layout.board_to_screen(power_up.position.x, power_up.position.y);
        print!("\x1b[{};{}H{}{}", power_up_y, power_up_x, color, symbol);
    }

    print!("\x1b[0m");

    hud::draw_gameplay_hud(game, layout, language);

    let _ = std::io::stdout().flush();
    game.dirty_positions.clear();
}
