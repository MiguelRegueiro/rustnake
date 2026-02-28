use crate::i18n;
use crate::utils::Language;
use std::io::Write;

use super::super::shared::{
    ANSI_RESET, MENU_LOGO, Rect, STYLE_MENU_BORDER, STYLE_MENU_HINT, STYLE_MENU_LOGO,
    STYLE_MENU_OPTION, STYLE_MENU_OPTION_DANGER, STYLE_MENU_OPTION_SELECTED_DANGER,
    STYLE_MENU_OPTION_SELECTED_MID, STYLE_MENU_SUBTITLE, STYLE_MENU_TITLE, TextureContext,
    center_start, clear_rect, clip_by_display_width, display_width, draw_menu_texture_region,
    draw_panel_frame, draw_panel_separator, pad_to_display_width, print_clipped,
};
use super::menu_cache::{self, MenuStaticView};

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

pub(super) fn selected_option_style(is_danger: bool) -> &'static str {
    if is_danger {
        return STYLE_MENU_OPTION_SELECTED_DANGER;
    }
    STYLE_MENU_OPTION_SELECTED_MID
}

pub(super) fn build_highlight_row_ansi(
    y: u16,
    x: u16,
    row_width: u16,
    row_style: &str,
    line: &str,
) -> String {
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

pub(super) fn menu_option_line_text(
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

    let (full_redraw, previous_selected) =
        menu_cache::menu_redraw_state(&static_view, request.selected_option);
    menu_cache::mark_menu_draw();

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
        let redraw_region = menu_cache::claim_redraw_region(current_clear_region);
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
