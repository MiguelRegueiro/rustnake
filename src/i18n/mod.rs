//! Translation helpers for all user-facing text.

use crate::utils::{Difficulty, Language, PowerUpType};
use unicode_width::UnicodeWidthStr;

fn text_width(text: &str) -> u16 {
    UnicodeWidthStr::width(text) as u16
}

pub fn controls_text(language: Language) -> &'static str {
    match language {
        Language::En => "WASD/Arrows:Move P:Pause M:Mute SPACE:Menu Q:Quit",
        Language::Es => "WASD/Flechas:Mover P:Pausa M:Mutear ESPACIO:Menú Q:Salir",
        Language::Ja => "WASD/矢印:移動 P:一時停止 M:ミュート SPACE:メニュー Q:終了",
        Language::Pt => "WASD/Setas:Mover P:Pausa M:Silenciar ESPAÇO:Menu Q:Sair",
        Language::Zh => "WASD/方向键:移动 P:暂停 M:静音 SPACE:菜单 Q:退出",
    }
}

pub fn menu_title(language: Language) -> &'static str {
    match language {
        Language::En => "SNAKE GAME",
        Language::Es => "SNAKE GAME",
        Language::Ja => "スネークゲーム",
        Language::Pt => "SNAKE GAME",
        Language::Zh => "贪吃蛇",
    }
}

pub fn menu_play(language: Language) -> &'static str {
    match language {
        Language::En => "Play",
        Language::Es => "Jugar",
        Language::Ja => "プレイ",
        Language::Pt => "Jogar",
        Language::Zh => "开始",
    }
}

pub fn menu_difficulty(language: Language) -> &'static str {
    match language {
        Language::En => "Difficulty",
        Language::Es => "Dificultad",
        Language::Ja => "難易度",
        Language::Pt => "Dificuldade",
        Language::Zh => "难度",
    }
}

pub fn menu_settings(language: Language) -> &'static str {
    match language {
        Language::En => "Settings",
        Language::Es => "Ajustes",
        Language::Ja => "設定",
        Language::Pt => "Configuracoes",
        Language::Zh => "设置",
    }
}

pub fn menu_high_scores(language: Language) -> &'static str {
    match language {
        Language::En => "High Scores",
        Language::Es => "Puntuaciones",
        Language::Ja => "ハイスコア",
        Language::Pt => "Pontuacoes",
        Language::Zh => "最高分",
    }
}

pub fn menu_quit(language: Language) -> &'static str {
    match language {
        Language::En => "Quit",
        Language::Es => "Salir",
        Language::Ja => "終了",
        Language::Pt => "Sair",
        Language::Zh => "退出",
    }
}

pub fn high_scores_menu_title(language: Language) -> &'static str {
    match language {
        Language::En => "All High Scores",
        Language::Es => "Todas las puntuaciones",
        Language::Ja => "すべてのハイスコア",
        Language::Pt => "Todas as pontuacoes",
        Language::Zh => "全部最高分",
    }
}

pub fn menu_back(language: Language) -> &'static str {
    match language {
        Language::En => "Back",
        Language::Es => "Atras",
        Language::Ja => "戻る",
        Language::Pt => "Voltar",
        Language::Zh => "返回",
    }
}

pub fn difficulty_menu_title(language: Language) -> &'static str {
    match language {
        Language::En => "Select Difficulty",
        Language::Es => "Selecciona dificultad",
        Language::Ja => "難易度を選択",
        Language::Pt => "Selecionar dificuldade",
        Language::Zh => "选择难度",
    }
}

pub fn settings_pause_on_focus_loss_label(language: Language) -> &'static str {
    match language {
        Language::En => "Pause on Focus Loss",
        Language::Es => "Pausar al perder enfoque",
        Language::Ja => "フォーカス喪失で一時停止",
        Language::Pt => "Pausar ao perder foco",
        Language::Zh => "失去焦点时暂停",
    }
}

pub fn settings_sound_label(language: Language) -> &'static str {
    match language {
        Language::En => "Sound",
        Language::Es => "Sonido",
        Language::Ja => "サウンド",
        Language::Pt => "Som",
        Language::Zh => "声音",
    }
}

pub fn settings_reset_high_scores_label(language: Language) -> &'static str {
    match language {
        Language::En => "Reset High Scores",
        Language::Es => "Reiniciar puntuaciones",
        Language::Ja => "ハイスコアをリセット",
        Language::Pt => "Resetar pontuacoes",
        Language::Zh => "重置最高分",
    }
}

pub fn reset_high_scores_title(language: Language) -> &'static str {
    match language {
        Language::En => "Reset High Scores?",
        Language::Es => "Reiniciar puntuaciones?",
        Language::Ja => "ハイスコアをリセットしますか？",
        Language::Pt => "Resetar pontuacoes?",
        Language::Zh => "重置最高分？",
    }
}

pub fn confirm_yes(language: Language) -> &'static str {
    match language {
        Language::En => "Yes",
        Language::Es => "Si",
        Language::Ja => "はい",
        Language::Pt => "Sim",
        Language::Zh => "是",
    }
}

pub fn confirm_no(language: Language) -> &'static str {
    match language {
        Language::En => "No",
        Language::Es => "No",
        Language::Ja => "いいえ",
        Language::Pt => "Nao",
        Language::Zh => "否",
    }
}

pub fn setting_on(language: Language) -> &'static str {
    match language {
        Language::En => "On",
        Language::Es => "Activado",
        Language::Ja => "オン",
        Language::Pt => "Ligado",
        Language::Zh => "开",
    }
}

pub fn setting_off(language: Language) -> &'static str {
    match language {
        Language::En => "Off",
        Language::Es => "Desactivado",
        Language::Ja => "オフ",
        Language::Pt => "Desligado",
        Language::Zh => "关",
    }
}

pub fn menu_navigation_hint(language: Language) -> &'static str {
    match language {
        Language::En => "Use ↑↓ arrows or WASD to navigate",
        Language::Es => "Usa ↑↓ o WASD para navegar",
        Language::Ja => "↑↓ または WASD で移動",
        Language::Pt => "Use ↑↓ ou WASD para navegar",
        Language::Zh => "使用 ↑↓ 或 WASD 进行选择",
    }
}

pub fn menu_confirm_hint(language: Language) -> &'static str {
    match language {
        Language::En => "Press ENTER/SPACE to select, Q to quit",
        Language::Es => "Pulsa ENTER/ESPACIO para elegir, Q para salir",
        Language::Ja => "ENTER/SPACE で決定、Q で終了",
        Language::Pt => "Pressione ENTER/ESPAÇO para escolher, Q para sair",
        Language::Zh => "按 ENTER/SPACE 确认，Q 退出",
    }
}

pub fn language_name(language: Language) -> &'static str {
    match language {
        Language::En => "English",
        Language::Es => "Español",
        Language::Ja => "日本語",
        Language::Pt => "Português",
        Language::Zh => "简体中文",
    }
}

pub fn language_popup_title(language: Language) -> &'static str {
    match language {
        Language::En => "Select Language",
        Language::Es => "Selecciona idioma",
        Language::Ja => "言語を選択",
        Language::Pt => "Selecionar idioma",
        Language::Zh => "选择语言",
    }
}

pub fn language_label(language: Language) -> &'static str {
    match language {
        Language::En => "Language",
        Language::Es => "Idioma",
        Language::Ja => "言語",
        Language::Pt => "Idioma",
        Language::Zh => "语言",
    }
}

pub fn small_window_title(language: Language) -> &'static str {
    match language {
        Language::En => "WINDOW TOO SMALL",
        Language::Es => "VENTANA MUY PEQUEÑA",
        Language::Ja => "ウィンドウが小さすぎます",
        Language::Pt => "JANELA MUITO PEQUENA",
        Language::Zh => "窗口太小",
    }
}

pub fn small_window_current_label(language: Language) -> &'static str {
    match language {
        Language::En => "Current",
        Language::Es => "Actual",
        Language::Ja => "現在",
        Language::Pt => "Atual",
        Language::Zh => "当前",
    }
}

pub fn small_window_minimum_label(language: Language) -> &'static str {
    match language {
        Language::En => "Minimum",
        Language::Es => "Mínimo",
        Language::Ja => "最小",
        Language::Pt => "Mínimo",
        Language::Zh => "最小",
    }
}

pub fn small_window_hint(language: Language) -> &'static str {
    match language {
        Language::En => "Resize terminal to continue. Press Q to quit.",
        Language::Es => "Ajusta la terminal para continuar. Pulsa Q para salir.",
        Language::Ja => "端末サイズを広げて続行。Qで終了。",
        Language::Pt => "Ajuste o terminal para continuar. Pressione Q para sair.",
        Language::Zh => "请调整终端大小后继续。按 Q 退出。",
    }
}

pub fn status_score_label(language: Language) -> &'static str {
    match language {
        Language::En => "Score",
        Language::Es => "Puntos",
        Language::Ja => "得点",
        Language::Pt => "Pontos",
        Language::Zh => "分数",
    }
}

pub fn status_difficulty_label(language: Language) -> &'static str {
    match language {
        Language::En => "Diff",
        Language::Es => "Nivel",
        Language::Ja => "難易度",
        Language::Pt => "Nível",
        Language::Zh => "难度",
    }
}

pub fn status_paused(language: Language) -> &'static str {
    match language {
        Language::En => "PAUSED",
        Language::Es => "PAUSA",
        Language::Ja => "一時停止",
        Language::Pt => "PAUSADO",
        Language::Zh => "暂停",
    }
}

pub fn status_muted(language: Language) -> &'static str {
    match language {
        Language::En => "MUTED",
        Language::Es => "MUTEADO",
        Language::Ja => "消音",
        Language::Pt => "SEM SOM",
        Language::Zh => "静音",
    }
}

pub fn info_best_label(language: Language) -> &'static str {
    match language {
        Language::En => "Best",
        Language::Es => "Mejor",
        Language::Ja => "最高",
        Language::Pt => "Melhor",
        Language::Zh => "最佳",
    }
}

pub fn info_pace_label(language: Language) -> &'static str {
    match language {
        Language::En => "Pace",
        Language::Es => "Ritmo",
        Language::Ja => "速度",
        Language::Pt => "Ritmo",
        Language::Zh => "速度",
    }
}

pub fn info_effect_label(language: Language) -> &'static str {
    match language {
        Language::En => "Effect",
        Language::Es => "Efecto",
        Language::Ja => "効果",
        Language::Pt => "Efeito",
        Language::Zh => "效果",
    }
}

pub fn difficulty_label(language: Language, difficulty: Difficulty) -> &'static str {
    match (language, difficulty) {
        (Language::En, Difficulty::Easy) => "Easy",
        (Language::En, Difficulty::Medium) => "Medium",
        (Language::En, Difficulty::Hard) => "Hard",
        (Language::En, Difficulty::Extreme) => "Extreme",
        (Language::Es, Difficulty::Easy) => "Fácil",
        (Language::Es, Difficulty::Medium) => "Medio",
        (Language::Es, Difficulty::Hard) => "Difícil",
        (Language::Es, Difficulty::Extreme) => "Extremo",
        (Language::Ja, Difficulty::Easy) => "簡単",
        (Language::Ja, Difficulty::Medium) => "普通",
        (Language::Ja, Difficulty::Hard) => "難しい",
        (Language::Ja, Difficulty::Extreme) => "極限",
        (Language::Pt, Difficulty::Easy) => "Fácil",
        (Language::Pt, Difficulty::Medium) => "Médio",
        (Language::Pt, Difficulty::Hard) => "Difícil",
        (Language::Pt, Difficulty::Extreme) => "Extremo",
        (Language::Zh, Difficulty::Easy) => "简单",
        (Language::Zh, Difficulty::Medium) => "普通",
        (Language::Zh, Difficulty::Hard) => "困难",
        (Language::Zh, Difficulty::Extreme) => "极限",
    }
}

pub fn speed_effect_short(language: Language, power_up_type: PowerUpType) -> &'static str {
    match (language, power_up_type) {
        (Language::En, PowerUpType::SpeedBoost) => "Boost",
        (Language::En, PowerUpType::SlowDown) => "Slow",
        (Language::Es, PowerUpType::SpeedBoost) => "Turbo",
        (Language::Es, PowerUpType::SlowDown) => "Lento",
        (Language::Ja, PowerUpType::SpeedBoost) => "加速",
        (Language::Ja, PowerUpType::SlowDown) => "減速",
        (Language::Pt, PowerUpType::SpeedBoost) => "Turbo",
        (Language::Pt, PowerUpType::SlowDown) => "Lento",
        (Language::Zh, PowerUpType::SpeedBoost) => "加速",
        (Language::Zh, PowerUpType::SlowDown) => "减速",
        (_, _) => "",
    }
}

pub fn game_over_title(language: Language) -> &'static str {
    match language {
        Language::En => "GAME OVER!",
        Language::Es => "FIN DEL JUEGO",
        Language::Ja => "ゲームオーバー",
        Language::Pt => "FIM DE JOGO",
        Language::Zh => "游戏结束",
    }
}

pub fn game_over_menu_hint(language: Language) -> &'static str {
    match language {
        Language::En => "Press SPACE for menu",
        Language::Es => "Pulsa ESPACIO para menú",
        Language::Ja => "SPACEでメニューへ",
        Language::Pt => "Pressione ESPAÇO para o menu",
        Language::Zh => "按 SPACE 返回菜单",
    }
}

pub fn game_over_quit_hint(language: Language) -> &'static str {
    match language {
        Language::En => "or 'q' to quit",
        Language::Es => "o 'q' para salir",
        Language::Ja => "'q'で終了",
        Language::Pt => "ou 'q' para sair",
        Language::Zh => "或按 'q' 退出",
    }
}

pub fn minimum_ui_width(language: Language) -> u16 {
    let option_overhead = 2u16; // selector marker + space
    let max_difficulty = difficulty_label(language, Difficulty::Extreme);
    let difficulty_main_line = format!("{}: {}", menu_difficulty(language), max_difficulty);
    let pause_value = if text_width(setting_on(language)) >= text_width(setting_off(language)) {
        setting_on(language)
    } else {
        setting_off(language)
    };
    let sound_value = if text_width(setting_on(language)) >= text_width(setting_off(language)) {
        setting_on(language)
    } else {
        setting_off(language)
    };

    let main_options = [
        menu_play(language).to_string(),
        difficulty_main_line,
        menu_high_scores(language).to_string(),
        menu_settings(language).to_string(),
        menu_quit(language).to_string(),
    ];
    let difficulty_options = [
        difficulty_label(language, Difficulty::Easy).to_string(),
        difficulty_label(language, Difficulty::Medium).to_string(),
        difficulty_label(language, Difficulty::Hard).to_string(),
        difficulty_label(language, Difficulty::Extreme).to_string(),
        menu_back(language).to_string(),
    ];
    let settings_options = [
        format!("{}: {}", language_label(language), language_name(language)),
        format!(
            "{}: {}",
            settings_pause_on_focus_loss_label(language),
            pause_value
        ),
        format!("{}: {}", settings_sound_label(language), sound_value),
        settings_reset_high_scores_label(language).to_string(),
        menu_back(language).to_string(),
    ];
    let language_options: Vec<String> = Language::ALL
        .iter()
        .map(|lang| language_name(*lang).to_string())
        .chain(std::iter::once(menu_back(language).to_string()))
        .collect();
    let reset_options = [
        confirm_yes(language).to_string(),
        confirm_no(language).to_string(),
    ];
    let max_score = u32::MAX.to_string();
    let high_scores_options = [
        format!(
            "{}: {}",
            difficulty_label(language, Difficulty::Easy),
            max_score
        ),
        format!(
            "{}: {}",
            difficulty_label(language, Difficulty::Medium),
            max_score
        ),
        format!(
            "{}: {}",
            difficulty_label(language, Difficulty::Hard),
            max_score
        ),
        format!(
            "{}: {}",
            difficulty_label(language, Difficulty::Extreme),
            max_score
        ),
        menu_back(language).to_string(),
    ];

    let mut max_width = text_width(controls_text(language))
        .max(text_width(menu_navigation_hint(language)))
        .max(text_width(menu_confirm_hint(language)))
        .max(text_width(small_window_hint(language)))
        .max(text_width(difficulty_menu_title(language)))
        .max(text_width(high_scores_menu_title(language)))
        .max(text_width(language_popup_title(language)))
        .max(text_width(menu_title(language)))
        .max(text_width(reset_high_scores_title(language)))
        .max(text_width(game_over_title(language)))
        .max(text_width(game_over_menu_hint(language)))
        .max(text_width(game_over_quit_hint(language)));

    for option in main_options
        .iter()
        .chain(difficulty_options.iter())
        .chain(settings_options.iter())
        .chain(language_options.iter())
        .chain(reset_options.iter())
        .chain(high_scores_options.iter())
    {
        max_width = max_width.max(text_width(option).saturating_add(option_overhead));
    }

    max_width
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_non_empty_required_keys(language: Language) {
        assert!(!controls_text(language).is_empty());
        assert!(!menu_title(language).is_empty());
        assert!(!menu_play(language).is_empty());
        assert!(!menu_difficulty(language).is_empty());
        assert!(!menu_high_scores(language).is_empty());
        assert!(!menu_settings(language).is_empty());
        assert!(!menu_quit(language).is_empty());
        assert!(!menu_back(language).is_empty());
        assert!(!difficulty_menu_title(language).is_empty());
        assert!(!high_scores_menu_title(language).is_empty());
        assert!(!menu_navigation_hint(language).is_empty());
        assert!(!menu_confirm_hint(language).is_empty());
        assert!(!language_name(language).is_empty());
        assert!(!language_popup_title(language).is_empty());
        assert!(!language_label(language).is_empty());
        assert!(!settings_pause_on_focus_loss_label(language).is_empty());
        assert!(!settings_sound_label(language).is_empty());
        assert!(!settings_reset_high_scores_label(language).is_empty());
        assert!(!reset_high_scores_title(language).is_empty());
        assert!(!setting_on(language).is_empty());
        assert!(!setting_off(language).is_empty());
        assert!(!confirm_yes(language).is_empty());
        assert!(!confirm_no(language).is_empty());
        assert!(!small_window_title(language).is_empty());
        assert!(!small_window_current_label(language).is_empty());
        assert!(!small_window_minimum_label(language).is_empty());
        assert!(!small_window_hint(language).is_empty());
        assert!(!status_score_label(language).is_empty());
        assert!(!status_difficulty_label(language).is_empty());
        assert!(!status_paused(language).is_empty());
        assert!(!status_muted(language).is_empty());
        assert!(!info_best_label(language).is_empty());
        assert!(!info_pace_label(language).is_empty());
        assert!(!info_effect_label(language).is_empty());
        assert!(!difficulty_label(language, Difficulty::Easy).is_empty());
        assert!(!difficulty_label(language, Difficulty::Medium).is_empty());
        assert!(!difficulty_label(language, Difficulty::Hard).is_empty());
        assert!(!difficulty_label(language, Difficulty::Extreme).is_empty());
        assert!(!speed_effect_short(language, PowerUpType::SpeedBoost).is_empty());
        assert!(!speed_effect_short(language, PowerUpType::SlowDown).is_empty());
        assert!(!game_over_title(language).is_empty());
        assert!(!game_over_menu_hint(language).is_empty());
        assert!(!game_over_quit_hint(language).is_empty());
    }

    #[test]
    fn translation_keys_are_present_for_all_languages() {
        for language in Language::ALL {
            assert_non_empty_required_keys(language);
        }
    }
}
