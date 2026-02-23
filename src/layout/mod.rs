//! Terminal layout calculations for responsive rendering.

use crossterm::terminal;

pub const HUD_BOTTOM_PADDING: u16 = 5;
pub const CONTROLS_TEXT: &str = "WASD/Arrows:Move P:Pause M:Mute SPACE:Menu Q:Quit";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub term_width: u16,
    pub term_height: u16,
    pub map_width: u16,
    pub map_height: u16,
    pub origin_x: u16,
    pub origin_y: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct MinSize {
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct SizeCheck {
    pub current_width: u16,
    pub current_height: u16,
    pub minimum: MinSize,
}

impl Layout {
    pub fn map_right(&self) -> u16 {
        self.origin_x + self.map_width - 1
    }

    pub fn map_bottom(&self) -> u16 {
        self.origin_y + self.map_height - 1
    }

    pub fn board_to_screen(&self, x: u16, y: u16) -> (u16, u16) {
        (self.origin_x + x - 1, self.origin_y + y - 1)
    }

    pub fn hud_score_y(&self) -> u16 {
        self.map_bottom() + 2
    }

    pub fn hud_info_y(&self) -> u16 {
        self.map_bottom() + 3
    }

    pub fn hud_controls_y(&self) -> u16 {
        self.map_bottom() + HUD_BOTTOM_PADDING
    }
}

pub fn terminal_size() -> (u16, u16) {
    terminal::size().unwrap_or((80, 24))
}

pub fn min_terminal_size(map_width: u16, map_height: u16) -> MinSize {
    let min_width = map_width.max(CONTROLS_TEXT.len() as u16);
    let min_height = map_height + HUD_BOTTOM_PADDING;
    MinSize {
        width: min_width,
        height: min_height,
    }
}

pub fn compute_layout(
    term_width: u16,
    term_height: u16,
    map_width: u16,
    map_height: u16,
) -> Result<Layout, SizeCheck> {
    let minimum = min_terminal_size(map_width, map_height);
    if term_width < minimum.width || term_height < minimum.height {
        return Err(SizeCheck {
            current_width: term_width,
            current_height: term_height,
            minimum,
        });
    }

    let total_height = map_height + HUD_BOTTOM_PADDING;
    let origin_x = ((term_width - map_width) / 2) + 1;
    let origin_y = ((term_height - total_height) / 2) + 1;

    Ok(Layout {
        term_width,
        term_height,
        map_width,
        map_height,
        origin_x,
        origin_y,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_too_small_terminal() {
        let result = compute_layout(20, 10, 40, 20);
        assert!(result.is_err());
    }

    #[test]
    fn centers_map_on_larger_terminal() {
        let layout = compute_layout(100, 40, 40, 20).unwrap();
        assert_eq!(layout.origin_x, 31);
        assert_eq!(layout.origin_y, 8);
        assert_eq!(layout.map_right(), 70);
        assert_eq!(layout.map_bottom(), 27);
    }
}
