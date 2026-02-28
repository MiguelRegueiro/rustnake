use crate::storage::HighScores;
use crate::utils::Language;
use std::sync::{Mutex, OnceLock};

use super::super::shared::Rect;

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

pub(super) struct MenuStaticView<'a> {
    pub(super) screen_tag: &'a str,
    pub(super) title: &'a str,
    pub(super) subtitle: Option<&'a str>,
    pub(super) options: &'a [String],
    pub(super) danger_option: Option<usize>,
    pub(super) term_width: u16,
    pub(super) term_height: u16,
    pub(super) language: Language,
    pub(super) compact: bool,
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

pub(super) fn rect_union(a: Rect, b: Rect) -> Rect {
    Rect {
        start_x: a.start_x.min(b.start_x),
        end_x: a.end_x.max(b.end_x),
        start_y: a.start_y.min(b.start_y),
        end_y: a.end_y.max(b.end_y),
    }
}

pub(super) fn transition_redraw_region(previous: Option<Rect>, current: Rect) -> Rect {
    previous.map_or(current, |prev| rect_union(prev, current))
}

pub(super) fn claim_redraw_region(current_region: Rect) -> Rect {
    let mut cache = last_menu_region_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let redraw_region = transition_redraw_region(cache.as_ref().copied(), current_region);
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

pub(super) fn menu_redraw_state(
    view: &MenuStaticView<'_>,
    selected_option: usize,
) -> (bool, Option<usize>) {
    let mut cache = menu_render_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let key_changed = !cache
        .key
        .as_ref()
        .is_some_and(|key| menu_static_key_matches_view(key, view));
    let previous_selected = if key_changed {
        None
    } else {
        cache.selected_option
    };
    if key_changed {
        cache.key = Some(menu_static_key_from_view(view));
    }
    cache.selected_option = Some(selected_option);
    (key_changed, previous_selected)
}

pub(super) fn mark_menu_draw() {
    let mut cache = high_scores_render_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    cache.key = None;
}

pub(super) fn begin_high_scores_draw(
    high_scores: &HighScores,
    term_width: u16,
    term_height: u16,
    language: Language,
    compact: bool,
) -> bool {
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
            return true;
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

    false
}

pub(crate) fn invalidate_menu_render_caches() {
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

#[cfg(test)]
pub(super) fn cached_region() -> Option<Rect> {
    let cache = last_menu_region_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *cache
}

#[cfg(test)]
pub(super) fn set_cached_region(region: Option<Rect>) {
    let mut cache = last_menu_region_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *cache = region;
}
