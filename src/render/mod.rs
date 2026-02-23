//! UI and rendering module for the Snake game.
//! Handles all terminal-based graphics and user interface elements.

use crate::core::Game;
use crate::layout::{Layout, SizeCheck, CONTROLS_TEXT};
use std::io::Write;

fn center_start(total: u16, content: u16) -> u16 {
    total.saturating_sub(content) / 2 + 1
}

fn print_clipped(y: u16, x: u16, text: &str, max_width: u16) {
    if max_width == 0 {
        return;
    }
    let clipped: String = text.chars().take(max_width as usize).collect();
    print!("\x1b[{};{}H{}", y, x, clipped);
}

fn draw_centered_line(y: u16, term_width: u16, text: &str) {
    print!("\x1b[{};1H\x1b[K", y);
    if term_width == 0 {
        return;
    }
    let text_len = text.chars().count() as u16;
    let draw_len = text_len.min(term_width);
    let start_x = center_start(term_width, draw_len);
    print_clipped(y, start_x, text, draw_len);
}

fn draw_border(layout: &Layout) {
    let inner_width = layout.map_width.saturating_sub(2) as usize;
    let top = format!("┌{}┐", "─".repeat(inner_width));
    let bottom = format!("└{}┘", "─".repeat(inner_width));

    print!("\x1b[{};{}H{}", layout.origin_y, layout.origin_x, top);
    print!(
        "\x1b[{};{}H{}",
        layout.map_bottom(),
        layout.origin_x,
        bottom
    );

    for y in (layout.origin_y + 1)..layout.map_bottom() {
        print!("\x1b[{};{}H│", y, layout.origin_x);
        print!("\x1b[{};{}H│", y, layout.map_right());
    }
}

pub fn draw_static_frame(layout: &Layout) {
    print!("\x1b[2J\x1b[H");
    draw_border(layout);

    let _ = std::io::stdout().flush();
}

pub fn draw_size_warning(size_check: SizeCheck) {
    print!("\x1b[2J\x1b[H");
    let start_y = center_start(size_check.current_height, 5);
    draw_centered_line(start_y, size_check.current_width, "WINDOW TOO SMALL");
    draw_centered_line(
        start_y + 1,
        size_check.current_width,
        &format!(
            "Current: {}x{}  Minimum: {}x{}",
            size_check.current_width,
            size_check.current_height,
            size_check.minimum.width,
            size_check.minimum.height
        ),
    );
    draw_centered_line(
        start_y + 3,
        size_check.current_width,
        "Resize terminal to continue. Press Q to quit.",
    );

    let _ = std::io::stdout().flush();
}

pub fn draw(game: &mut Game, layout: &Layout) {
    for pos in &game.dirty_positions {
        let (x, y) = layout.board_to_screen(pos.x, pos.y);
        print!("\x1b[{};{}H ", y, x);
    }

    // Re-draw border every frame so the playfield frame is always continuous.
    draw_border(layout);

    // Draw snake
    for (i, pos) in game.snake.body.iter().enumerate() {
        // Head is bright green, body segments get darker toward the tail
        let color = if i == 0 {
            "\x1b[92m" // Bright green for head
        } else if i < game.snake.body.len() / 3 {
            "\x1b[32m" // Regular green for front segments
        } else if i < game.snake.body.len() * 2 / 3 {
            "\x1b[33m" // Yellow for middle segments
        } else {
            "\x1b[90m" // Dark gray for tail segments
        };

        let (x, y) = layout.board_to_screen(pos.x, pos.y);
        print!("\x1b[{};{}H{}", y, x, color);

        // Different symbols for head and body, with head indicating direction
        if i == 0 {
            // Head symbol depends on direction for rotation effect
            let head_symbol = match game.snake.direction {
                crate::utils::Direction::Up | crate::utils::Direction::Down => "█", // Vertical orientation
                crate::utils::Direction::Left | crate::utils::Direction::Right => "█", // Same symbol but conceptually rotated
            };
            print!("{}", head_symbol);
        } else {
            print!("■"); // Smaller block for body
        }
    }

    // Draw food with different symbols based on score
    let food_symbol = if game.score.is_multiple_of(50) && game.score != 0 {
        "★"
    } else {
        "●"
    };
    let (food_x, food_y) = layout.board_to_screen(game.food.x, game.food.y);
    print!("\x1b[{};{}H\x1b[91m{}", food_y, food_x, food_symbol); // Bright red for food

    // Draw power-up if it exists
    if let Some(power_up) = game.power_up {
        let (symbol, color) = match power_up.power_up_type {
            crate::utils::PowerUpType::SpeedBoost => (">", "\x1b[94m"), // Blue for speed boost
            crate::utils::PowerUpType::SlowDown => ("<", "\x1b[96m"),   // Cyan for slow down
            crate::utils::PowerUpType::ExtraPoints => ("$", "\x1b[93m"), // Yellow for extra points
            crate::utils::PowerUpType::Grow => ("+", "\x1b[92m"),       // Green for grow
            crate::utils::PowerUpType::Shrink => ("-", "\x1b[95m"),     // Magenta for shrink
        };
        let (power_up_x, power_up_y) =
            layout.board_to_screen(power_up.position.x, power_up.position.y);
        print!("\x1b[{};{}H{}{}", power_up_y, power_up_x, color, symbol);
    }

    // Reset color
    print!("\x1b[0m");

    let score_y = layout.hud_score_y();
    let info_y = layout.hud_info_y();
    let controls_y = layout.hud_controls_y();

    let difficulty_short = match game.difficulty {
        crate::utils::Difficulty::Easy => "Easy",
        crate::utils::Difficulty::Medium => "Medium",
        crate::utils::Difficulty::Hard => "Hard",
    };
    let mut status_text = format!("Score:{}  Diff:{}", game.score, difficulty_short);
    if game.is_paused() {
        status_text.push_str("  PAUSED");
    }
    if game.muted {
        status_text.push_str("  MUTED");
    }
    draw_centered_line(score_y, layout.term_width, &status_text);

    // Draw progression/speed telemetry.
    let progression_multiplier = game.difficulty_speed_multiplier_percent();
    let power_up_multiplier = game.speed_multiplier_percent();
    let combined_multiplier = progression_multiplier * power_up_multiplier / 100;
    let mut info_text = format!("Best:{}  Pace:{}%", game.high_score, combined_multiplier);
    if let Some(effect_label) = game.active_speed_effect_label() {
        let short_effect = match effect_label {
            "Speed Boost" => "Boost",
            "Slow Down" => "Slow",
            other => other,
        };
        info_text.push_str(&format!(
            "  Effect:{}({})",
            short_effect,
            game.speed_effect_ticks_left()
        ));
    }
    draw_centered_line(info_y, layout.term_width, &info_text);

    // Draw controls reminder - at the bottom, away from other info
    draw_centered_line(controls_y, layout.term_width, CONTROLS_TEXT);

    // Draw game over message
    if game.game_over {
        let box_width: u16 = 36;
        let box_start_x: u16 = layout.origin_x + (game.width.saturating_sub(box_width)) / 2;
        let box_top_y: u16 = layout.origin_y + (game.height / 2).saturating_sub(2);

        print!(
            "\x1b[{};{}H╔{}╗",
            box_top_y,
            box_start_x,
            "═".repeat((box_width - 2) as usize)
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            box_top_y + 1,
            box_start_x,
            "GAME OVER!",
            width = (box_width - 2) as usize
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            box_top_y + 2,
            box_start_x,
            format!("Score: {}", game.score),
            width = (box_width - 2) as usize
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            box_top_y + 3,
            box_start_x,
            "",
            width = (box_width - 2) as usize
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            box_top_y + 4,
            box_start_x,
            "Press SPACE to menu",
            width = (box_width - 2) as usize
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            box_top_y + 5,
            box_start_x,
            "or 'q' to quit",
            width = (box_width - 2) as usize
        );
        print!(
            "\x1b[{};{}H╚{}╝",
            box_top_y + 6,
            box_start_x,
            "═".repeat((box_width - 2) as usize)
        );
    }

    let _ = std::io::stdout().flush();
    game.dirty_positions.clear();
}

pub fn draw_menu(selected_option: usize, term_width: u16, term_height: u16) {
    let options = ["Easy", "Medium", "Hard"];

    // Clear screen and draw menu
    print!("\x1b[2J\x1b[H");

    let title_box_width: u16 = 25;
    let title_start_x = center_start(term_width, title_box_width);
    let menu_start_y = center_start(term_height, 12);

    // Title box - centered
    print!(
        "\x1b[{};{}H┌{}┐",
        menu_start_y,
        title_start_x,
        "─".repeat((title_box_width - 2) as usize)
    );
    print!(
        "\x1b[{};{}H│{: ^width$}│",
        menu_start_y + 1,
        title_start_x,
        "SNAKE GAME",
        width = (title_box_width - 2) as usize
    );
    print!(
        "\x1b[{};{}H└{}┘",
        menu_start_y + 2,
        title_start_x,
        "─".repeat((title_box_width - 2) as usize)
    );

    // Menu options - centered under the title with color highlighting
    for (i, option) in options.iter().enumerate() {
        let option_center_x = center_start(term_width, 12);
        let marker = if selected_option == i { "▶" } else { " " };
        let color = if selected_option == i {
            "\x1b[92m"
        } else {
            "\x1b[97m"
        }; // Green for selected, white for others
        print!(
            "\x1b[{};{}H{}{}{}. {}",
            menu_start_y + 4 + i as u16,
            option_center_x,
            color,
            marker,
            i + 1,
            option
        );
    }

    // Reset color
    print!("\x1b[0m");

    // Instructions - centered
    let instruction_width: u16 = 35;
    let instruction_start_x = center_start(term_width, instruction_width);
    print!(
        "\x1b[{};{}HUse ↑↓ arrows or WASD to navigate",
        menu_start_y + 8,
        instruction_start_x
    );
    print!(
        "\x1b[{};{}HPress ENTER to select, Q to quit",
        menu_start_y + 9,
        instruction_start_x
    );

    let _ = std::io::stdout().flush();
}
