use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub(crate) const ANSI_RESET: &str = "\x1b[0m";
pub(crate) const STYLE_MENU_BORDER: &str = "\x1b[38;2;89;138;207m";
pub(crate) const STYLE_MENU_LOGO: &str = "\x1b[1;38;2;219;224;232m";
pub(crate) const STYLE_MENU_TITLE: &str = "\x1b[1;97m";
pub(crate) const STYLE_MENU_SUBTITLE: &str = "\x1b[2;37m";
pub(crate) const STYLE_MENU_HINT: &str = "\x1b[2;37m";
pub(crate) const STYLE_MENU_OPTION: &str = "\x1b[97m";
pub(crate) const STYLE_MENU_OPTION_DANGER: &str = "\x1b[91m";
pub(crate) const STYLE_MENU_OPTION_SELECTED_MID: &str = "\x1b[1;38;2;255;255;255;48;2;89;138;207m";
pub(crate) const STYLE_MENU_OPTION_SELECTED_DANGER: &str = "\x1b[1;97;41m";
pub(crate) const STYLE_MENU_TEXTURE: &str = "\x1b[38;2;96;103;117m";

pub(crate) const MENU_LOGO: &str = "Rustnake";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Rect {
    pub(crate) start_x: u16,
    pub(crate) end_x: u16,
    pub(crate) start_y: u16,
    pub(crate) end_y: u16,
}

#[derive(Clone, Copy)]
pub(crate) struct TextureContext {
    pub(crate) term_width: u16,
    pub(crate) term_height: u16,
    pub(crate) panel_start_x: u16,
    pub(crate) panel_start_y: u16,
    pub(crate) panel_width: u16,
    pub(crate) panel_height: u16,
}

pub(crate) fn center_start(total: u16, content: u16) -> u16 {
    total.saturating_sub(content) / 2 + 1
}

pub(crate) fn display_width(text: &str) -> u16 {
    UnicodeWidthStr::width(text) as u16
}

pub(crate) fn clip_by_display_width(text: &str, max_width: u16) -> String {
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

pub(crate) fn print_clipped(y: u16, x: u16, text: &str, max_width: u16) {
    if max_width == 0 {
        return;
    }
    let clipped = clip_by_display_width(text, max_width);
    print!("\x1b[{};{}H{}", y, x, clipped);
}

pub(crate) fn pad_to_display_width(text: &str, target_width: u16) -> String {
    let current = display_width(text);
    if current >= target_width {
        return text.to_string();
    }
    format!("{}{}", text, " ".repeat((target_width - current) as usize))
}

pub(crate) fn draw_centered_line(y: u16, term_width: u16, text: &str) {
    draw_centered_line_styled(y, term_width, text, "");
}

pub(crate) fn draw_centered_line_styled(y: u16, term_width: u16, text: &str, style: &str) {
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

pub(crate) fn draw_box_line_styled(y: u16, x: u16, inner_width: u16, text: &str, text_style: &str) {
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

pub(crate) fn draw_panel_frame(
    y: u16,
    x: u16,
    inner_width: u16,
    inner_height: u16,
    border_style: &str,
) {
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

pub(crate) fn draw_panel_separator(y: u16, x: u16, inner_width: u16, border_style: &str) {
    print!(
        "{}\x1b[{};{}H├{}┤{}",
        border_style,
        y,
        x,
        "─".repeat(inner_width as usize),
        ANSI_RESET
    );
}

pub(crate) fn draw_menu_texture_region(texture: TextureContext, region: Rect) {
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

pub(crate) fn clear_rect(rect: Rect) {
    let width = rect.end_x.saturating_sub(rect.start_x).saturating_add(1) as usize;
    let blank = " ".repeat(width);
    for y in rect.start_y..=rect.end_y {
        print!("\x1b[{};{}H{}", y, rect.start_x, blank);
    }
}
