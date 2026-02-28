mod menu_cache;
mod menu_high_scores;
mod menu_main;

pub use menu_high_scores::{HighScoresRenderRequest, draw_high_scores_menu};
pub use menu_main::{MenuRenderRequest, draw_menu};

pub(crate) use menu_cache::invalidate_menu_render_caches;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::storage::HighScores;
    use crate::utils::Language;

    #[test]
    fn menu_option_line_text_snapshot() {
        let _guard = super::super::render_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let line = menu_main::menu_option_line_text(0, "Play", 0, 10);
        assert_eq!(line, "> [1] Play      ");
    }

    #[test]
    fn selected_row_ansi_snapshot() {
        let _guard = super::super::render_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let ansi = menu_main::build_highlight_row_ansi(
            7,
            12,
            16,
            menu_main::selected_option_style(false),
            "> [1] Play",
        );
        assert_eq!(
            ansi,
            "\x1b[1;38;2;255;255;255;48;2;89;138;207m\x1b[7;12H                \x1b[0m\x1b[1;38;2;255;255;255;48;2;89;138;207m\x1b[7;12H> [1] Play\x1b[0m"
        );
    }

    #[test]
    fn danger_row_ansi_snapshot() {
        let _guard = super::super::render_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let ansi = menu_main::build_highlight_row_ansi(
            5,
            3,
            14,
            menu_main::selected_option_style(true),
            "> [5] Reset",
        );
        assert_eq!(
            ansi,
            "\x1b[1;97;41m\x1b[5;3H              \x1b[0m\x1b[1;97;41m\x1b[5;3H> [5] Reset\x1b[0m"
        );
    }

    #[test]
    fn transition_sequence_game_menu_high_scores_menu_uses_union() {
        let _guard = super::super::render_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        invalidate_menu_render_caches();

        let main_region = super::super::shared::Rect {
            start_x: 40,
            end_x: 90,
            start_y: 6,
            end_y: 24,
        };
        let high_scores_region = super::super::shared::Rect {
            start_x: 20,
            end_x: 110,
            start_y: 5,
            end_y: 25,
        };

        let first = menu_cache::claim_redraw_region(main_region);
        assert_eq!(first, main_region);

        let second = menu_cache::claim_redraw_region(high_scores_region);
        assert_eq!(
            second,
            menu_cache::rect_union(main_region, high_scores_region)
        );

        let third = menu_cache::claim_redraw_region(main_region);
        assert_eq!(
            third,
            menu_cache::rect_union(high_scores_region, main_region)
        );

        invalidate_menu_render_caches();
    }

    #[test]
    fn renderer_sequence_menu_high_scores_menu_tracks_regions() {
        let _guard = super::super::render_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        invalidate_menu_render_caches();

        let options = vec![
            "Play".to_string(),
            "Difficulty: Extreme".to_string(),
            "High Scores".to_string(),
            "Settings".to_string(),
            "Quit".to_string(),
        ];
        let make_menu_request = || MenuRenderRequest {
            screen_tag: "MENU",
            title: "SNAKE GAME",
            subtitle: Some("Difficulty: Extreme"),
            options: &options,
            selected_option: 0,
            danger_option: None,
            term_width: 120,
            term_height: 40,
            language: Language::En,
            compact: false,
        };

        draw_menu(make_menu_request());
        let menu_region =
            menu_cache::cached_region().expect("menu should populate a redraw region");

        let high_scores = HighScores {
            easy: 50,
            medium: 80,
            hard: 120,
            extreme: 460,
        };
        draw_high_scores_menu(HighScoresRenderRequest {
            high_scores: &high_scores,
            term_width: 120,
            term_height: 40,
            language: Language::En,
            compact: false,
        });
        let high_scores_region =
            menu_cache::cached_region().expect("high-scores should populate a redraw region");

        assert_ne!(menu_region, high_scores_region);
        assert!(high_scores_region.start_x <= menu_region.start_x);
        assert!(high_scores_region.end_x >= menu_region.end_x);

        draw_menu(make_menu_request());
        let menu_region_after_return =
            menu_cache::cached_region().expect("menu should restore its own cached redraw region");
        assert_eq!(menu_region_after_return, menu_region);

        invalidate_menu_render_caches();
    }

    #[test]
    fn clear_for_menu_entry_resets_menu_region_cache() {
        let _guard = super::super::render_test_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        invalidate_menu_render_caches();

        let previous = super::super::shared::Rect {
            start_x: 10,
            end_x: 20,
            start_y: 10,
            end_y: 20,
        };
        menu_cache::set_cached_region(Some(previous));

        super::super::clear_for_menu_entry();

        let cached_region = menu_cache::cached_region();
        assert_eq!(cached_region, None);
    }
}
