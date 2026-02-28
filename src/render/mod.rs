//! UI and rendering module for the Snake game.
//! Handles all terminal-based graphics and user interface elements.

use crate::core::Game;
use crate::i18n;
use crate::layout::{Layout, SizeCheck};
use crate::storage::HighScores;
use crate::utils::Difficulty;
use crate::utils::Language;
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

const ANSI_RESET: &str = "\x1b[0m";
const STYLE_MENU_BORDER: &str = "\x1b[38;2;89;138;207m";
const STYLE_MENU_LOGO: &str = "\x1b[1;38;2;219;224;232m";
const STYLE_MENU_TITLE: &str = "\x1b[1;97m";
const STYLE_MENU_SUBTITLE: &str = "\x1b[2;37m";
const STYLE_MENU_HINT: &str = "\x1b[2;37m";
const STYLE_MENU_OPTION: &str = "\x1b[97m";
const STYLE_MENU_OPTION_DANGER: &str = "\x1b[91m";
const STYLE_MENU_OPTION_SELECTED_MID: &str = "\x1b[1;38;2;255;255;255;48;2;89;138;207m";
const STYLE_MENU_OPTION_SELECTED_DANGER: &str = "\x1b[1;97;41m";
const STYLE_MENU_TEXTURE: &str = "\x1b[38;2;96;103;117m";

const MENU_LOGO: &str = "Rustnake";

#[derive(Clone, Copy)]
struct Rect {
    start_x: u16,
    end_x: u16,
    start_y: u16,
    end_y: u16,
}

#[derive(Clone, Copy)]
struct TextureContext {
    term_width: u16,
    term_height: u16,
    panel_start_x: u16,
    panel_start_y: u16,
    panel_width: u16,
    panel_height: u16,
}

struct MenuOptionRowContext {
    options_start_x: u16,
    row_width: u16,
    row_label_width: u16,
    selected_option: usize,
    danger_option: Option<usize>,
}

pub struct MenuRenderRequest<'a> {
    pub screen_tag: &'a str,
    pub title: &'a str,
    pub subtitle: Option<&'a str>,
    pub options: &'a [String],
    pub selected_option: usize,
    pub danger_option: Option<usize>,
    pub term_width: u16,
    pub term_height: u16,
    pub language: Language,
    pub compact: bool,
}

pub struct HighScoresRenderRequest<'a> {
    pub high_scores: &'a HighScores,
    pub term_width: u16,
    pub term_height: u16,
    pub language: Language,
    pub compact: bool,
}

#[derive(Clone, PartialEq, Eq)]
struct MenuStaticKey {
    screen_tag: String,
    title: String,
    subtitle: Option<String>,
    options: Vec<String>,
    danger_option: Option<usize>,
    term_width: u16,
    term_height: u16,
    language: Language,
    compact: bool,
}

struct MenuStaticView<'a> {
    screen_tag: &'a str,
    title: &'a str,
    subtitle: Option<&'a str>,
    options: &'a [String],
    danger_option: Option<usize>,
    term_width: u16,
    term_height: u16,
    language: Language,
    compact: bool,
}

#[derive(Default)]
struct MenuRenderCache {
    key: Option<MenuStaticKey>,
    selected_option: Option<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct HighScoresStaticKey {
    high_scores: HighScores,
    term_width: u16,
    term_height: u16,
    language: Language,
    compact: bool,
}

#[derive(Default)]
struct HighScoresRenderCache {
    key: Option<HighScoresStaticKey>,
}

fn menu_render_cache() -> &'static Mutex<MenuRenderCache> {
    static CACHE: OnceLock<Mutex<MenuRenderCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(MenuRenderCache::default()))
}

fn high_scores_render_cache() -> &'static Mutex<HighScoresRenderCache> {
    static CACHE: OnceLock<Mutex<HighScoresRenderCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HighScoresRenderCache::default()))
}

fn last_menu_region_cache() -> &'static Mutex<Option<Rect>> {
    static CACHE: OnceLock<Mutex<Option<Rect>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(None))
}

fn rect_union(a: Rect, b: Rect) -> Rect {
    Rect {
        start_x: a.start_x.min(b.start_x),
        end_x: a.end_x.max(b.end_x),
        start_y: a.start_y.min(b.start_y),
        end_y: a.end_y.max(b.end_y),
    }
}

fn claim_redraw_region(current_region: Rect) -> Rect {
    let mut cache = last_menu_region_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let redraw_region = cache.as_ref().copied().map_or(current_region, |previous| {
        rect_union(previous, current_region)
    });
    *cache = Some(current_region);
    redraw_region
}

fn menu_static_key_matches_view(key: &MenuStaticKey, view: &MenuStaticView<'_>) -> bool {
    key.screen_tag == view.screen_tag
        && key.title == view.title
        && key.subtitle.as_deref() == view.subtitle
        && key.options.as_slice() == view.options
        && key.danger_option == view.danger_option
        && key.term_width == view.term_width
        && key.term_height == view.term_height
        && key.language == view.language
        && key.compact == view.compact
}

fn menu_static_key_from_view(view: &MenuStaticView<'_>) -> MenuStaticKey {
    MenuStaticKey {
        screen_tag: view.screen_tag.to_string(),
        title: view.title.to_string(),
        subtitle: view.subtitle.map(str::to_string),
        options: view.options.to_vec(),
        danger_option: view.danger_option,
        term_width: view.term_width,
        term_height: view.term_height,
        language: view.language,
        compact: view.compact,
    }
}

fn invalidate_menu_render_caches() {
    {
        let mut cache = menu_render_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cache.key = None;
        cache.selected_option = None;
    }
    {
        let mut cache = high_scores_render_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cache.key = None;
    }
    {
        let mut cache = last_menu_region_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *cache = None;
    }
}

fn selected_option_style(is_danger: bool) -> &'static str {
    if is_danger {
        return STYLE_MENU_OPTION_SELECTED_DANGER;
    }
    STYLE_MENU_OPTION_SELECTED_MID
}

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
    draw_centered_line_styled(y, term_width, text, "");
}

fn draw_centered_line_styled(y: u16, term_width: u16, text: &str, style: &str) {
    print!("\x1b[{};1H\x1b[K", y);
    if term_width == 0 {
        return;
    }
    let text_len = display_width(text);
    let draw_len = text_len.min(term_width);
    let start_x = center_start(term_width, draw_len);
    if !style.is_empty() {
        print!("{}", style);
    }
    print_clipped(y, start_x, text, draw_len);
    if !style.is_empty() {
        print!("{}", ANSI_RESET);
    }
}

fn draw_box_line_styled(y: u16, x: u16, inner_width: u16, text: &str, text_style: &str) {
    print!(
        "{}\x1b[{};{}H│{}│{}",
        STYLE_MENU_BORDER,
        y,
        x,
        " ".repeat(inner_width as usize),
        ANSI_RESET
    );
    let clipped = clip_by_display_width(text, inner_width);
    let text_x = x + 1 + (inner_width.saturating_sub(display_width(&clipped)) / 2);
    if !text_style.is_empty() {
        print!("{}", text_style);
    }
    print_clipped(y, text_x, &clipped, inner_width);
    if !text_style.is_empty() {
        print!("{}", ANSI_RESET);
    }
}

fn draw_panel_frame(y: u16, x: u16, inner_width: u16, inner_height: u16, border_style: &str) {
    print!(
        "{}\x1b[{};{}H┌{}┐{}",
        border_style,
        y,
        x,
        "─".repeat(inner_width as usize),
        ANSI_RESET
    );
    for line_y in (y + 1)..=(y + inner_height) {
        print!(
            "{}\x1b[{};{}H│{}│{}",
            border_style,
            line_y,
            x,
            " ".repeat(inner_width as usize),
            ANSI_RESET
        );
    }
    print!(
        "{}\x1b[{};{}H└{}┘{}",
        border_style,
        y + inner_height + 1,
        x,
        "─".repeat(inner_width as usize),
        ANSI_RESET
    );
}

fn draw_panel_separator(y: u16, x: u16, inner_width: u16, border_style: &str) {
    print!(
        "{}\x1b[{};{}H├{}┤{}",
        border_style,
        y,
        x,
        "─".repeat(inner_width as usize),
        ANSI_RESET
    );
}

fn draw_menu_texture_region(texture: TextureContext, region: Rect) {
    let region_start_x = region.start_x.max(1).min(texture.term_width.max(1));
    let region_end_x = region
        .end_x
        .max(region_start_x)
        .min(texture.term_width.max(1));
    let region_start_y = region.start_y.max(1).min(texture.term_height.max(1));
    let region_end_y = region
        .end_y
        .max(region_start_y)
        .min(texture.term_height.max(1));

    for y in region_start_y..=region_end_y {
        let mut row = String::with_capacity((region_end_x - region_start_x + 1) as usize);
        for x in region_start_x..=region_end_x {
            let is_inside_panel = x >= texture.panel_start_x
                && x < texture.panel_start_x.saturating_add(texture.panel_width)
                && y >= texture.panel_start_y
                && y < texture.panel_start_y.saturating_add(texture.panel_height);
            if is_inside_panel {
                row.push(' ');
                continue;
            }
            // Keep menu background fully clean to avoid visual speckles across terminals.
            row.push(' ');
        }
        print!(
            "{}\x1b[{};{}H{}{}",
            STYLE_MENU_TEXTURE, y, region_start_x, row, ANSI_RESET
        );
    }
}

fn clear_rect(rect: Rect) {
    let width = rect.end_x.saturating_sub(rect.start_x).saturating_add(1) as usize;
    let blank = " ".repeat(width);
    for y in rect.start_y..=rect.end_y {
        print!("\x1b[{};{}H{}", y, rect.start_x, blank);
    }
}

fn build_highlight_row_ansi(y: u16, x: u16, row_width: u16, row_style: &str, line: &str) -> String {
    format!(
        "{}\x1b[{};{}H{}{}{}\x1b[{};{}H{}{}",
        row_style,
        y,
        x,
        " ".repeat(row_width as usize),
        ANSI_RESET,
        row_style,
        y,
        x,
        clip_by_display_width(line, row_width),
        ANSI_RESET
    )
}

fn menu_option_line_text(
    option_index: usize,
    option: &str,
    selected_option: usize,
    row_label_width: u16,
) -> String {
    let marker = if selected_option == option_index {
        ">"
    } else {
        " "
    };
    let shortcut = if option_index < 6 {
        format!("[{}]", option_index + 1)
    } else {
        "[ ]".to_string()
    };
    let clipped_label = clip_by_display_width(option, row_label_width);
    let padded_label = pad_to_display_width(&clipped_label, row_label_width);
    format!("{} {} {}", marker, shortcut, padded_label)
}

fn draw_menu_option_row(
    row_y: u16,
    option_index: usize,
    option: &str,
    context: &MenuOptionRowContext,
) {
    let is_selected = context.selected_option == option_index;
    let is_danger = matches!(context.danger_option, Some(index) if index == option_index);
    let line = menu_option_line_text(
        option_index,
        option,
        context.selected_option,
        context.row_label_width,
    );
    let row_style = if is_selected {
        selected_option_style(is_danger)
    } else if is_danger {
        STYLE_MENU_OPTION_DANGER
    } else {
        STYLE_MENU_OPTION
    };

    print!(
        "{}",
        build_highlight_row_ansi(
            row_y,
            context.options_start_x,
            context.row_width,
            row_style,
            &line
        )
    );
}

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
    invalidate_menu_render_caches();
    print!("\x1b[2J\x1b[H");
    draw_border(layout);

    let _ = std::io::stdout().flush();
}

pub fn clear_for_menu_entry() {
    invalidate_menu_render_caches();
    print!("\x1b[2J\x1b[H");
    let _ = std::io::stdout().flush();
}

pub fn draw_size_warning(size_check: SizeCheck, language: Language) {
    invalidate_menu_render_caches();
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
    invalidate_menu_render_caches();
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
    let food_symbol = if game.score % 50 == 0 && game.score != 0 {
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

    // Draw controls reminder - at the bottom, away from other info
    draw_centered_line_styled(
        controls_y,
        layout.term_width,
        i18n::controls_text(language),
        STYLE_MENU_HINT,
    );

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

        draw_panel_frame(
            box_top_y,
            box_start_x,
            box_inner_width,
            box_height.saturating_sub(2),
            STYLE_MENU_BORDER,
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

    let _ = std::io::stdout().flush();
    game.dirty_positions.clear();
}

pub fn draw_menu(request: MenuRenderRequest<'_>) {
    let compact = request.compact;
    let subtitle = request.subtitle.filter(|text| !text.is_empty());
    let nav_hint = i18n::menu_navigation_hint(request.language);
    let confirm_hint = i18n::menu_confirm_hint(request.language);
    let show_logo = !compact;
    let pre_options_blank = if compact { 0u16 } else { 1u16 };
    let pre_footer_blank = if compact { 0u16 } else { 1u16 };

    let max_inner_width = request.term_width.saturating_sub(2).max(1);
    let option_overhead = 6u16; // marker + shortcut token + spacing
    let option_label_width = request
        .options
        .iter()
        .map(|option| display_width(option))
        .max()
        .unwrap_or(0)
        .min(max_inner_width);
    let option_row_width = option_label_width.saturating_add(option_overhead);
    let logo_width = display_width(MENU_LOGO);
    let title_width = display_width(request.title);
    let subtitle_width = subtitle.map(display_width).unwrap_or(0);
    let footer_width = display_width(nav_hint).max(display_width(confirm_hint));

    let desired_inner_width = title_width
        .max(logo_width)
        .max(subtitle_width)
        .max(footer_width)
        .max(option_row_width.saturating_add(2))
        .max(32);
    let panel_inner_width = desired_inner_width.min(max_inner_width);
    let row_width = panel_inner_width.saturating_sub(2).max(1);
    let row_label_width = row_width.saturating_sub(option_overhead).max(1);
    let header_lines = u16::from(show_logo) + 1 + u16::from(subtitle.is_some());
    let panel_inner_height = header_lines
        + 1
        + pre_options_blank
        + request.options.len() as u16
        + pre_footer_blank
        + 1
        + 2;
    let panel_width = panel_inner_width + 2;
    let panel_height = panel_inner_height + 2;
    let panel_start_y = center_start(request.term_height, panel_height);
    let panel_start_x = center_start(request.term_width, panel_width);
    let options_start_x = panel_start_x + 1 + (panel_inner_width.saturating_sub(row_width) / 2);
    let clear_start_x = panel_start_x.saturating_sub(2).max(1);
    let clear_end_x = panel_start_x
        .saturating_add(panel_width)
        .saturating_add(1)
        .min(request.term_width.max(1));
    let clear_start_y = panel_start_y.saturating_sub(1).max(1);
    let clear_end_y = panel_start_y
        .saturating_add(panel_height)
        .saturating_add(1)
        .min(request.term_height.max(1));
    let current_clear_region = Rect {
        start_x: clear_start_x,
        end_x: clear_end_x,
        start_y: clear_start_y,
        end_y: clear_end_y,
    };

    let static_view = MenuStaticView {
        screen_tag: request.screen_tag,
        title: request.title,
        subtitle,
        options: request.options,
        danger_option: request.danger_option,
        term_width: request.term_width,
        term_height: request.term_height,
        language: request.language,
        compact,
    };

    let (full_redraw, previous_selected) = {
        let mut cache = menu_render_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let key_changed = !cache
            .key
            .as_ref()
            .is_some_and(|key| menu_static_key_matches_view(key, &static_view));
        let previous_selected = if key_changed {
            None
        } else {
            cache.selected_option
        };
        if key_changed {
            cache.key = Some(menu_static_key_from_view(&static_view));
        }
        cache.selected_option = Some(request.selected_option);
        (key_changed, previous_selected)
    };

    {
        let mut cache = high_scores_render_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cache.key = None;
    }

    let options_start_y = {
        let mut row_y = panel_start_y + 1;
        if show_logo {
            row_y += 1;
        }
        row_y += 1 + u16::from(subtitle.is_some());
        row_y + 1 + pre_options_blank
    };

    let row_context = MenuOptionRowContext {
        options_start_x,
        row_width,
        row_label_width,
        selected_option: request.selected_option,
        danger_option: request.danger_option,
    };

    if full_redraw {
        let redraw_region = claim_redraw_region(current_clear_region);
        clear_rect(redraw_region);
        draw_menu_texture_region(
            TextureContext {
                term_width: request.term_width,
                term_height: request.term_height,
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
            let logo_x =
                panel_start_x + 1 + (panel_inner_width.saturating_sub(logo_draw_width) / 2);
            print!("{}", STYLE_MENU_LOGO);
            print_clipped(row_y, logo_x, MENU_LOGO, panel_inner_width);
            print!("{}", ANSI_RESET);
            row_y += 1;
        }

        let draw_title_width = title_width.min(panel_inner_width);
        let title_x = panel_start_x + 1 + (panel_inner_width.saturating_sub(draw_title_width) / 2);
        print!("{}", STYLE_MENU_TITLE);
        print_clipped(row_y, title_x, request.title, panel_inner_width);
        print!("{}", ANSI_RESET);
        row_y += 1;

        if let Some(subtitle_text) = subtitle {
            let subtitle_draw_width = display_width(subtitle_text).min(panel_inner_width);
            let subtitle_x =
                panel_start_x + 1 + (panel_inner_width.saturating_sub(subtitle_draw_width) / 2);
            print!("{}", STYLE_MENU_SUBTITLE);
            print_clipped(row_y, subtitle_x, subtitle_text, panel_inner_width);
            print!("{}", ANSI_RESET);
            row_y += 1;
        }

        draw_panel_separator(row_y, panel_start_x, panel_inner_width, STYLE_MENU_BORDER);
        row_y += 1 + pre_options_blank;
        for (i, option) in request.options.iter().enumerate() {
            draw_menu_option_row(row_y, i, option, &row_context);
            row_y += 1;
        }

        row_y += pre_footer_blank;
        draw_panel_separator(row_y, panel_start_x, panel_inner_width, STYLE_MENU_BORDER);
        row_y += 1;

        let nav_hint_width = display_width(nav_hint).min(panel_inner_width);
        let nav_hint_x = panel_start_x + 1 + (panel_inner_width.saturating_sub(nav_hint_width) / 2);
        print!("{}", STYLE_MENU_HINT);
        print_clipped(row_y, nav_hint_x, nav_hint, panel_inner_width);
        print!("{}", ANSI_RESET);
        row_y += 1;

        let confirm_hint_width = display_width(confirm_hint).min(panel_inner_width);
        let confirm_hint_x =
            panel_start_x + 1 + (panel_inner_width.saturating_sub(confirm_hint_width) / 2);
        print!("{}", STYLE_MENU_HINT);
        print_clipped(row_y, confirm_hint_x, confirm_hint, panel_inner_width);
        print!("{}", ANSI_RESET);
    } else {
        if let Some(previous) = previous_selected.filter(|index| *index < request.options.len()) {
            draw_menu_option_row(
                options_start_y + previous as u16,
                previous,
                &request.options[previous],
                &row_context,
            );
        }
        if request.selected_option < request.options.len()
            && previous_selected != Some(request.selected_option)
        {
            draw_menu_option_row(
                options_start_y + request.selected_option as u16,
                request.selected_option,
                &request.options[request.selected_option],
                &row_context,
            );
        }
    }

    let _ = std::io::stdout().flush();
}

pub fn draw_high_scores_menu(request: HighScoresRenderRequest<'_>) {
    let high_scores = request.high_scores;
    let term_width = request.term_width;
    let term_height = request.term_height;
    let language = request.language;
    let compact = request.compact;

    let static_key = HighScoresStaticKey {
        high_scores: *high_scores,
        term_width,
        term_height,
        language,
        compact,
    };
    {
        let mut cache = high_scores_render_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if cache.key == Some(static_key) {
            return;
        }
        cache.key = Some(static_key);
    }
    {
        let mut cache = menu_render_cache()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cache.key = None;
        cache.selected_option = None;
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

    let redraw_region = claim_redraw_region(current_clear_region);
    clear_rect(redraw_region);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_option_line_text_snapshot() {
        let line = menu_option_line_text(0, "Play", 0, 10);
        assert_eq!(line, "> [1] Play      ");
    }

    #[test]
    fn selected_row_ansi_snapshot() {
        let ansi = build_highlight_row_ansi(7, 12, 16, selected_option_style(false), "> [1] Play");
        assert_eq!(
            ansi,
            "\x1b[1;38;2;255;255;255;48;2;89;138;207m\x1b[7;12H                \x1b[0m\x1b[1;38;2;255;255;255;48;2;89;138;207m\x1b[7;12H> [1] Play\x1b[0m"
        );
    }

    #[test]
    fn danger_row_ansi_snapshot() {
        let ansi = build_highlight_row_ansi(5, 3, 14, selected_option_style(true), "> [5] Reset");
        assert_eq!(
            ansi,
            "\x1b[1;97;41m\x1b[5;3H              \x1b[0m\x1b[1;97;41m\x1b[5;3H> [5] Reset\x1b[0m"
        );
    }
}
