use crate::i18n;
use crate::storage::HighScores;
use crate::utils::{Difficulty, Language};
use std::io::Write;

use super::super::shared::{
    ANSI_RESET, MENU_LOGO, Rect, STYLE_MENU_BORDER, STYLE_MENU_HINT, STYLE_MENU_LOGO,
    STYLE_MENU_OPTION, STYLE_MENU_SUBTITLE, STYLE_MENU_TITLE, TextureContext, center_start,
    clear_rect_clipped, clip_by_display_width, display_width, draw_menu_texture_region,
    draw_panel_frame, draw_panel_separator, pad_to_display_width, print_clipped,
};
use super::menu_cache;
use super::menu_main::selected_option_style;

pub struct HighScoresRenderRequest<'a> {
    pub high_scores: &'a HighScores,
    pub term_width: u16,
    pub term_height: u16,
    pub language: Language,
    pub compact: bool,
}

pub fn draw_high_scores_menu(request: HighScoresRenderRequest<'_>) {
    let high_scores = request.high_scores;
    let term_width = request.term_width;
    let term_height = request.term_height;
    let language = request.language;
    let compact = request.compact;

    if menu_cache::begin_high_scores_draw(high_scores, term_width, term_height, language, compact) {
        return;
    }

    let show_logo = !compact;
    let pre_options_blank = if compact { 0u16 } else { 1u16 };
    let pre_footer_blank = if compact { 0u16 } else { 1u16 };

    let entries = [
        (
            Difficulty::Easy,
            high_scores.easy,
            "I",
            "\x1b[38;2;89;138;207m",
        ),
        (Difficulty::Medium, high_scores.medium, "II", "\x1b[32m"),
        (Difficulty::Hard, high_scores.hard, "III", "\x1b[33m"),
        (Difficulty::Extreme, high_scores.extreme, "IV", "\x1b[31m"),
    ];
    let max_score = entries
        .iter()
        .map(|(_, score, _, _)| *score)
        .max()
        .unwrap_or(0);

    let best_label = i18n::info_best_label(language);
    let max_label_width = entries
        .iter()
        .map(|(difficulty, _, _, _)| display_width(i18n::difficulty_label(language, *difficulty)))
        .max()
        .unwrap_or(1);
    let max_score_width = entries
        .iter()
        .map(|(_, score, _, _)| display_width(&score.to_string()))
        .max()
        .unwrap_or(1);
    let max_badge_width = entries
        .iter()
        .map(|(_, _, badge, _)| display_width(badge))
        .max()
        .unwrap_or(1);
    let min_bar_width = 8u16;

    let card_inner_width = (max_label_width
        .max(max_score_width)
        .max(max_badge_width)
        .max(display_width(best_label))
        .max(min_bar_width.saturating_add(2))
        + 2)
    .clamp(12, 20);
    let card_inner_height = 5u16;
    let card_width = card_inner_width + 2;
    let card_height = card_inner_height + 2;
    let gap = 2u16;
    let row_gap = 1u16;

    let total_horizontal_width = 4 * card_width + 3 * gap;
    let use_two_rows = total_horizontal_width > term_width.saturating_sub(2);
    let rows = if use_two_rows { 2u16 } else { 1u16 };
    let columns = if use_two_rows { 2u16 } else { 4u16 };
    let cards_block_height = rows * card_height + (rows - 1) * row_gap;
    let cards_row_width = columns * card_width + (columns - 1) * gap;

    let title = i18n::high_scores_menu_title(language);
    let back_line = format!("> {}", i18n::menu_back(language));
    let back_hint = i18n::high_scores_back_hint(language);
    let logo_width = display_width(MENU_LOGO);
    let max_inner_width = term_width.saturating_sub(2).max(1);
    let desired_inner_width = cards_row_width
        .saturating_add(2)
        .max(logo_width)
        .max(display_width(title))
        .max(display_width(&back_line))
        .max(display_width(back_hint))
        .max(32);
    let panel_inner_width = desired_inner_width.min(max_inner_width);
    let header_lines = u16::from(show_logo) + 1;
    let panel_inner_height =
        header_lines + 1 + pre_options_blank + cards_block_height + pre_footer_blank + 1 + 2;
    let panel_width = panel_inner_width + 2;
    let panel_height = panel_inner_height + 2;
    let panel_start_x = center_start(term_width, panel_width);
    let panel_start_y = center_start(term_height, panel_height);
    let clear_start_x = panel_start_x.saturating_sub(2).max(1);
    let clear_end_x = panel_start_x
        .saturating_add(panel_width)
        .saturating_add(1)
        .min(term_width.max(1));
    let clear_start_y = panel_start_y.saturating_sub(1).max(1);
    let clear_end_y = panel_start_y
        .saturating_add(panel_height)
        .saturating_add(1)
        .min(term_height.max(1));
    let current_clear_region = Rect {
        start_x: clear_start_x,
        end_x: clear_end_x,
        start_y: clear_start_y,
        end_y: clear_end_y,
    };

    let redraw_region = menu_cache::claim_redraw_region(current_clear_region);
    clear_rect_clipped(redraw_region, term_width, term_height);
    draw_menu_texture_region(
        TextureContext {
            term_width,
            term_height,
            panel_start_x,
            panel_start_y,
            panel_width,
            panel_height,
        },
        redraw_region,
    );
    draw_panel_frame(
        panel_start_y,
        panel_start_x,
        panel_inner_width,
        panel_inner_height,
        STYLE_MENU_BORDER,
    );

    let mut row_y = panel_start_y + 1;
    if show_logo {
        let logo_draw_width = logo_width.min(panel_inner_width);
        let logo_x = panel_start_x + 1 + (panel_inner_width.saturating_sub(logo_draw_width) / 2);
        print!("{}", STYLE_MENU_LOGO);
        print_clipped(row_y, logo_x, MENU_LOGO, panel_inner_width);
        print!("{}", ANSI_RESET);
        row_y += 1;
    }

    let title_draw_width = display_width(title).min(panel_inner_width);
    let title_x = panel_start_x + 1 + (panel_inner_width.saturating_sub(title_draw_width) / 2);
    print!("{}", STYLE_MENU_TITLE);
    print_clipped(row_y, title_x, title, panel_inner_width);
    print!("{}", ANSI_RESET);
    row_y += 1;

    draw_panel_separator(row_y, panel_start_x, panel_inner_width, STYLE_MENU_BORDER);
    row_y += 1 + pre_options_blank;
    let cards_y = row_y;

    let draw_card =
        |x: u16, y: u16, difficulty: Difficulty, score: u32, badge: &str, color: &str| {
            let label = i18n::difficulty_label(language, difficulty);
            let score_text = score.to_string();
            let bar_width = card_inner_width.saturating_sub(2).max(4);
            let filled_width = if max_score == 0 {
                0
            } else {
                ((score as u64 * bar_width as u64).div_ceil(max_score as u64) as u16).min(bar_width)
            };
            let empty_width = bar_width.saturating_sub(filled_width);
            let bar_line = format!(
                "{}{}",
                "█".repeat(filled_width as usize),
                "░".repeat(empty_width as usize)
            );

            print!(
                "{}\x1b[{};{}H┌{}┐{}",
                color,
                y,
                x,
                "─".repeat(card_inner_width as usize),
                ANSI_RESET
            );
            for line_y in (y + 1)..=(y + card_inner_height) {
                print!(
                    "{}\x1b[{};{}H│{}│{}",
                    color,
                    line_y,
                    x,
                    " ".repeat(card_inner_width as usize),
                    ANSI_RESET
                );
            }
            print!(
                "{}\x1b[{};{}H└{}┘{}",
                color,
                y + card_inner_height + 1,
                x,
                "─".repeat(card_inner_width as usize),
                ANSI_RESET
            );

            let badge_x = x + 1 + (card_inner_width.saturating_sub(display_width(badge)) / 2);
            print!("\x1b[{};{}H{}", y + 1, badge_x, color);
            print_clipped(y + 1, badge_x, badge, card_inner_width);
            print!("{}", ANSI_RESET);

            let label_x = x + 1 + (card_inner_width.saturating_sub(display_width(label)) / 2);
            print!("{}", STYLE_MENU_OPTION);
            print_clipped(y + 2, label_x, label, card_inner_width);
            print!("{}", ANSI_RESET);

            let best_x = x + 1 + (card_inner_width.saturating_sub(display_width(best_label)) / 2);
            print!("{}", STYLE_MENU_SUBTITLE);
            print_clipped(y + 3, best_x, best_label, card_inner_width);
            print!("{}", ANSI_RESET);

            let score_x = x + 1 + (card_inner_width.saturating_sub(display_width(&score_text)) / 2);
            print!("{}", STYLE_MENU_TITLE);
            print_clipped(y + 4, score_x, &score_text, card_inner_width);
            print!("{}", ANSI_RESET);

            let bar_x = x + 1 + (card_inner_width.saturating_sub(bar_width) / 2);
            print!("{}", color);
            print_clipped(y + 5, bar_x, &bar_line, bar_width);
            print!("{}", ANSI_RESET);
        };

    let row_start_x = panel_start_x + 1 + (panel_inner_width.saturating_sub(cards_row_width) / 2);
    for (index, (difficulty, score, badge, color)) in entries.iter().enumerate() {
        let row = (index as u16) / columns;
        let col = (index as u16) % columns;
        let x = row_start_x + col * (card_width + gap);
        let y = cards_y + row * (card_height + row_gap);
        draw_card(x, y, *difficulty, *score, badge, color);
    }

    row_y = cards_y + cards_block_height;
    row_y += pre_footer_blank;
    draw_panel_separator(row_y, panel_start_x, panel_inner_width, STYLE_MENU_BORDER);
    row_y += 1;

    let back_row_width = panel_inner_width.saturating_sub(2).max(1);
    let back_x = panel_start_x + 1 + (panel_inner_width.saturating_sub(back_row_width) / 2);
    let clipped_back_line = clip_by_display_width(&back_line, back_row_width);
    let padded_back_line = pad_to_display_width(&clipped_back_line, back_row_width);
    let selected_style = selected_option_style(false);
    print!(
        "{}\x1b[{};{}H{}{}",
        selected_style,
        row_y,
        back_x,
        " ".repeat(back_row_width as usize),
        ANSI_RESET
    );
    print!("{}", selected_style);
    print_clipped(row_y, back_x, &padded_back_line, back_row_width);
    print!("{}", ANSI_RESET);
    row_y += 1;

    let back_hint_width = display_width(back_hint).min(panel_inner_width);
    let back_hint_x = panel_start_x + 1 + (panel_inner_width.saturating_sub(back_hint_width) / 2);
    print!("{}", STYLE_MENU_HINT);
    print_clipped(row_y, back_hint_x, back_hint, panel_inner_width);
    print!("{}", ANSI_RESET);

    let _ = std::io::stdout().flush();
}
