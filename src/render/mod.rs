//! UI and rendering module for the Snake game.
//! Handles all terminal-based graphics and user interface elements.

use crate::core::Game;
use crate::i18n;
use crate::layout::{Layout, SizeCheck};
<<<<<<< HEAD
use crate::utils::Language;
=======
use crate::utils::{Difficulty, Language};
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
use std::io::Write;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

fn center_start(total: u16, content: u16) -> u16 {
    total.saturating_sub(content) / 2 + 1
}

fn display_width(text: &str) -> u16 {
    UnicodeWidthStr::width(text) as u16
}

fn clip_by_display_width(text: &str, max_width: u16) -> String {
    if max_width == 0 {
        return String::new();
    }

    let mut clipped = String::new();
    let mut width_used: u16 = 0;

    for ch in text.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0) as u16;
        if ch_width > 0 && width_used.saturating_add(ch_width) > max_width {
            break;
        }
        clipped.push(ch);
        width_used = width_used.saturating_add(ch_width);
    }

    clipped
}

fn print_clipped(y: u16, x: u16, text: &str, max_width: u16) {
    if max_width == 0 {
        return;
    }
    let clipped = clip_by_display_width(text, max_width);
    print!("\x1b[{};{}H{}", y, x, clipped);
}

fn pad_to_display_width(text: &str, target_width: u16) -> String {
    let current = display_width(text);
    if current >= target_width {
        return text.to_string();
    }
    format!("{}{}", text, " ".repeat((target_width - current) as usize))
}

fn draw_centered_line(y: u16, term_width: u16, text: &str) {
    print!("\x1b[{};1H\x1b[K", y);
    if term_width == 0 {
        return;
    }
    let text_len = display_width(text);
    let draw_len = text_len.min(term_width);
    let start_x = center_start(term_width, draw_len);
    print_clipped(y, start_x, text, draw_len);
}

fn draw_box_line(y: u16, x: u16, inner_width: u16, text: &str) {
<<<<<<< HEAD
    print!("\x1b[{};{}H│{}│", y, x, " ".repeat(inner_width as usize));
=======
    print!("\x1b[{};{}H║{}║", y, x, " ".repeat(inner_width as usize));
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
    let clipped = clip_by_display_width(text, inner_width);
    let text_x = x + 1 + (inner_width.saturating_sub(display_width(&clipped)) / 2);
    print_clipped(y, text_x, &clipped, inner_width);
}

<<<<<<< HEAD
=======
fn clear_rect(start_y: u16, start_x: u16, width: u16, height: u16) {
    if width == 0 || height == 0 {
        return;
    }
    let blank = " ".repeat(width as usize);
    for row in 0..height {
        print!("\x1b[{};{}H{}", start_y + row, start_x, blank);
    }
}

fn draw_language_popup(
    term_width: u16,
    term_height: u16,
    ui_language: Language,
    selected: Language,
) {
    let title = i18n::language_popup_title(ui_language);
    let hint = i18n::language_popup_hint(ui_language);
    let options = Language::ALL;
    let option_labels: Vec<String> = options
        .iter()
        .enumerate()
        .map(|(i, option_language)| format!("{}. {}", i + 1, i18n::language_name(*option_language)))
        .collect();
    let option_label_width = option_labels
        .iter()
        .map(|line| display_width(line))
        .max()
        .unwrap_or(0);
    let option_line_width = option_label_width + 2; // marker + space + label

    let mut inner_width = display_width(title).max(display_width(hint));
    for line in &option_labels {
        inner_width = inner_width.max(display_width(line) + 2);
    }
    inner_width = inner_width
        .max(display_width(hint).saturating_add(8))
        .max(44);

    let box_width = inner_width + 2;
    let box_height: u16 = options.len() as u16 + 9;
    let box_start_x = center_start(term_width, box_width);
    let box_start_y = center_start(term_height, box_height);
    let panel_padding_x: u16 = 3;
    let panel_padding_y: u16 = 1;
    let panel_x = box_start_x.saturating_sub(panel_padding_x).max(1);
    let panel_y = box_start_y.saturating_sub(panel_padding_y).max(1);
    let panel_width =
        (box_width + panel_padding_x * 2).min(term_width.saturating_sub(panel_x).saturating_add(1));
    let panel_height = (box_height + panel_padding_y * 2)
        .min(term_height.saturating_sub(panel_y).saturating_add(1));

    clear_rect(panel_y, panel_x, panel_width, panel_height);

    print!(
        "\x1b[{};{}H╔{}╗",
        box_start_y,
        box_start_x,
        "═".repeat(inner_width as usize)
    );
    draw_box_line(box_start_y + 1, box_start_x, inner_width, title);
    draw_box_line(box_start_y + 2, box_start_x, inner_width, "");
    for (i, option_language) in options.iter().enumerate() {
        let marker = if *option_language == selected {
            "▶"
        } else {
            " "
        };
        let padded_label = pad_to_display_width(&option_labels[i], option_label_width);
        let option_line = format!("{} {}", marker, padded_label);
        let centered_option_line = pad_to_display_width(&option_line, option_line_width);
        draw_box_line(
            box_start_y + 3 + i as u16,
            box_start_x,
            inner_width,
            &centered_option_line,
        );
    }
    let post_options_y = box_start_y + 3 + options.len() as u16;
    draw_box_line(post_options_y, box_start_x, inner_width, "");
    draw_box_line(post_options_y + 1, box_start_x, inner_width, "");
    draw_box_line(post_options_y + 2, box_start_x, inner_width, hint);
    draw_box_line(post_options_y + 3, box_start_x, inner_width, "");
    draw_box_line(post_options_y + 4, box_start_x, inner_width, "");
    print!(
        "\x1b[{};{}H╚{}╝",
        post_options_y + 5,
        box_start_x,
        "═".repeat(inner_width as usize)
    );
}

>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
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

pub fn draw_size_warning(size_check: SizeCheck, language: Language) {
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

    let difficulty_short = i18n::difficulty_label(language, game.difficulty);
    let mut status_text = format!(
        "{}:{}  {}:{}",
        i18n::status_score_label(language),
        game.score,
        i18n::status_difficulty_label(language),
        difficulty_short
    );
    if game.is_paused() {
        status_text.push_str(&format!("  {}", i18n::status_paused(language)));
    }
    if game.muted {
        status_text.push_str(&format!("  {}", i18n::status_muted(language)));
    }
    draw_centered_line(score_y, layout.term_width, &status_text);

    // Draw progression/speed telemetry.
    let progression_multiplier = game.difficulty_speed_multiplier_percent();
    let power_up_multiplier = game.speed_multiplier_percent();
    let combined_multiplier = progression_multiplier * power_up_multiplier / 100;
    let mut info_text = format!(
        "{}:{}  {}:{}%",
        i18n::info_best_label(language),
        game.high_score,
        i18n::info_pace_label(language),
        combined_multiplier
    );
    if let (Some(effect_kind), Some(_)) = (game.active_speed_effect, game.power_up_timer) {
        let short_effect = i18n::speed_effect_short(language, effect_kind);
        if !short_effect.is_empty() {
            info_text.push_str(&format!(
                "  {}:{}({})",
                i18n::info_effect_label(language),
                short_effect,
                game.speed_effect_ticks_left()
            ));
        }
    }
    draw_centered_line(info_y, layout.term_width, &info_text);

    // Draw controls reminder - at the bottom, away from other info
    draw_centered_line(controls_y, layout.term_width, i18n::controls_text(language));

    // Draw game over message
    if game.game_over {
        let score_line = format!("{}: {}", i18n::status_score_label(language), game.score);
        let text_lines = [
            i18n::game_over_title(language),
            score_line.as_str(),
            "",
            i18n::game_over_menu_hint(language),
            i18n::game_over_quit_hint(language),
        ];

        let max_line_width = text_lines
            .iter()
            .map(|line| display_width(line))
            .max()
            .unwrap_or(0);
        let interior_width = layout.map_width.saturating_sub(2);
        let interior_height = layout.map_height.saturating_sub(2);

        let desired_box_width = max_line_width.saturating_add(4); // text + side padding + borders
        let box_width = desired_box_width.min(interior_width).max(10);
        let box_inner_width = box_width - 2;
        let box_height: u16 = 7;
        let box_start_x: u16 = layout.origin_x + 1 + (interior_width.saturating_sub(box_width)) / 2;
        let box_top_y: u16 = layout.origin_y + 1 + (interior_height.saturating_sub(box_height)) / 2;

        print!(
            "\x1b[{};{}H┌{}┐",
            box_top_y,
            box_start_x,
<<<<<<< HEAD
            "─".repeat(box_inner_width as usize)
=======
            "═".repeat(box_inner_width as usize)
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
        );
        draw_box_line(
            box_top_y + 1,
            box_start_x,
            box_inner_width,
            i18n::game_over_title(language),
        );
        draw_box_line(box_top_y + 2, box_start_x, box_inner_width, &score_line);
        draw_box_line(box_top_y + 3, box_start_x, box_inner_width, "");
        draw_box_line(
            box_top_y + 4,
            box_start_x,
            box_inner_width,
            i18n::game_over_menu_hint(language),
        );
        draw_box_line(
            box_top_y + 5,
            box_start_x,
            box_inner_width,
            i18n::game_over_quit_hint(language),
        );
        print!(
            "\x1b[{};{}H└{}┘",
            box_top_y + 6,
            box_start_x,
<<<<<<< HEAD
            "─".repeat(box_inner_width as usize)
=======
            "═".repeat(box_inner_width as usize)
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
        );
    }

    let _ = std::io::stdout().flush();
    game.dirty_positions.clear();
}

pub fn draw_menu(
<<<<<<< HEAD
    title: &str,
    options: &[String],
=======
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
    selected_option: usize,
    term_width: u16,
    term_height: u16,
    language: Language,
<<<<<<< HEAD
) {
    print!("\x1b[2J\x1b[H");

    let max_title_inner = term_width.saturating_sub(2).max(1);
    let title_width = display_width(title).min(max_title_inner);
    let mut title_inner_width = title_width.max(22).min(max_title_inner);
    if !(title_inner_width - title_width).is_multiple_of(2) {
        title_inner_width = (title_inner_width + 1).min(max_title_inner);
    }
    let title_box_width = title_inner_width + 2;
    let menu_height = options.len() as u16 + 9;
    let menu_start_y = center_start(term_height, menu_height);
    let title_start_x = center_start(term_width, title_box_width);
=======
    language_popup_selection: Option<Language>,
) {
    let popup_active = language_popup_selection.is_some();

    // Clear screen and draw menu
    print!("\x1b[2J\x1b[H");

    let title = i18n::menu_title(language);
    let title_width = display_width(title);
    let mut title_inner_width = title_width.max(22);
    // Keep same parity so centered text does not drift one cell left/right.
    if !(title_inner_width - title_width).is_multiple_of(2) {
        title_inner_width += 1;
    }
    let title_box_width: u16 = title_inner_width + 2;
    let title_start_x = center_start(term_width, title_box_width);
    let menu_start_y = center_start(term_height, 15);
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704

    print!(
        "\x1b[{};{}H┌{}┐",
        menu_start_y,
        title_start_x,
        "─".repeat((title_box_width - 2) as usize)
    );
    print!(
        "\x1b[{};{}H│{}│",
        menu_start_y + 1,
        title_start_x,
        " ".repeat((title_box_width - 2) as usize)
    );
    let title_x = title_start_x + 1 + (title_inner_width.saturating_sub(title_width) / 2);
    print_clipped(menu_start_y + 1, title_x, title, title_inner_width);
    print!(
        "\x1b[{};{}H└{}┘",
        menu_start_y + 2,
        title_start_x,
        "─".repeat((title_box_width - 2) as usize)
    );

<<<<<<< HEAD
    let option_label_width = options
        .iter()
        .map(|option| display_width(option))
        .max()
        .unwrap_or(0)
        .min(term_width.saturating_sub(2).max(1));
    let option_line_width = option_label_width + 2;
    let options_start_x = center_start(term_width, option_line_width);
    for (i, option) in options.iter().enumerate() {
        let marker = if selected_option == i { ">" } else { " " };
=======
    // Menu options - fixed-width rows inside a centered block.
    let difficulty_options = [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Extreme,
    ];
    let difficulty_labels: Vec<String> = difficulty_options
        .iter()
        .enumerate()
        .map(|(i, difficulty)| {
            format!(
                "{}. {}",
                i + 1,
                i18n::difficulty_label(language, *difficulty)
            )
        })
        .collect();
    let difficulty_label_width = difficulty_labels
        .iter()
        .map(|line| display_width(line))
        .max()
        .unwrap_or(0);
    let difficulty_line_width = difficulty_label_width + 2; // marker + space + label
    let difficulty_start_x = center_start(term_width, difficulty_line_width);

    for (i, _) in difficulty_options.iter().enumerate() {
        let marker = if selected_option == i { "▶" } else { " " };
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
        let color = if selected_option == i {
            "\x1b[92m"
        } else {
            "\x1b[97m"
<<<<<<< HEAD
        };
        let clipped_label = clip_by_display_width(option, option_label_width);
        let padded_label = pad_to_display_width(&clipped_label, option_label_width);
        let line = format!("{} {}", marker, padded_label);
        let row_y = menu_start_y + 4 + i as u16;
        print!("\x1b[{};1H\x1b[K", row_y);
        print!("\x1b[{};{}H{}", row_y, options_start_x, color);
        print_clipped(row_y, options_start_x, &line, option_line_width);
=======
        }; // Green for selected, white for others
        let padded_label = pad_to_display_width(&difficulty_labels[i], difficulty_label_width);
        let full_line = format!("{} {}", marker, padded_label);
        print!(
            "\x1b[{};{}H{}{}",
            menu_start_y + 4 + i as u16,
            difficulty_start_x,
            color,
            full_line
        );
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704
    }
    print!("\x1b[0m");

<<<<<<< HEAD
    let info_y = menu_start_y + 5 + options.len() as u16;
    draw_centered_line(info_y, term_width, i18n::menu_navigation_hint(language));
    draw_centered_line(info_y + 1, term_width, i18n::menu_confirm_hint(language));
=======
    if !popup_active {
        let language_line = format!("{}: {}", i18n::language_label(language), language.code());
        draw_centered_line(menu_start_y + 8, term_width, &language_line);
        draw_centered_line(
            menu_start_y + 10,
            term_width,
            i18n::menu_navigation_hint(language),
        );
        draw_centered_line(
            menu_start_y + 11,
            term_width,
            i18n::menu_confirm_hint(language),
        );
        draw_centered_line(
            menu_start_y + 12,
            term_width,
            i18n::menu_language_hint(language),
        );
    }

    if let Some(selected_language) = language_popup_selection {
        draw_language_popup(term_width, term_height, language, selected_language);
    }
>>>>>>> 2bd0e7008ff5ee461cbaa0237a74463eda54a704

    let _ = std::io::stdout().flush();
}
