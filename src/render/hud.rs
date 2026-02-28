use crate::core::Game;
use crate::i18n;
use crate::layout::Layout;
use crate::utils::Language;

use super::shared::{
    STYLE_MENU_HINT, STYLE_MENU_OPTION, STYLE_MENU_SUBTITLE, STYLE_MENU_TITLE, display_width,
    draw_box_line_styled, draw_centered_line_styled, draw_panel_frame,
};

pub(crate) fn draw_gameplay_hud(game: &Game, layout: &Layout, language: Language) {
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
    draw_centered_line_styled(score_y, layout.term_width, &status_text, STYLE_MENU_TITLE);

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
    draw_centered_line_styled(info_y, layout.term_width, &info_text, STYLE_MENU_SUBTITLE);

    draw_centered_line_styled(
        controls_y,
        layout.term_width,
        i18n::controls_text(language),
        STYLE_MENU_HINT,
    );

    if game.game_over {
        draw_game_over_panel(game, layout, language);
    }
}

fn draw_game_over_panel(game: &Game, layout: &Layout, language: Language) {
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

    draw_panel_frame(
        box_top_y,
        box_start_x,
        box_inner_width,
        box_height.saturating_sub(2),
        super::shared::STYLE_MENU_BORDER,
    );
    draw_box_line_styled(
        box_top_y + 1,
        box_start_x,
        box_inner_width,
        i18n::game_over_title(language),
        STYLE_MENU_TITLE,
    );
    draw_box_line_styled(
        box_top_y + 2,
        box_start_x,
        box_inner_width,
        &score_line,
        STYLE_MENU_OPTION,
    );
    draw_box_line_styled(box_top_y + 3, box_start_x, box_inner_width, "", "");
    draw_box_line_styled(
        box_top_y + 4,
        box_start_x,
        box_inner_width,
        i18n::game_over_menu_hint(language),
        STYLE_MENU_HINT,
    );
    draw_box_line_styled(
        box_top_y + 5,
        box_start_x,
        box_inner_width,
        i18n::game_over_quit_hint(language),
        STYLE_MENU_HINT,
    );
}
