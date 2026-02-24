//! Translation helpers for all user-facing text.

use crate::utils::{Difficulty, Language, PowerUpType};

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

pub fn menu_language_hint(language: Language) -> &'static str {
    match language {
        Language::En => "Press L to change language",
        Language::Es => "Pulsa L para cambiar idioma",
        Language::Ja => "Lキーで言語を変更",
        Language::Pt => "Pressione L para mudar idioma",
        Language::Zh => "按 L 切换语言",
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

pub fn language_popup_hint(language: Language) -> &'static str {
    match language {
        Language::En => "↑↓ or 1-5, ENTER apply, L cancel",
        Language::Es => "↑↓ o 1-5, ENTER aplica, L cancela",
        Language::Ja => "↑↓ または 1-5、ENTER 決定、L 戻る",
        Language::Pt => "↑↓ ou 1-5, ENTER aplica, L cancela",
        Language::Zh => "↑↓ 或 1-5，ENTER 确认，L 返回",
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_non_empty_required_keys(language: Language) {
        assert!(!controls_text(language).is_empty());
        assert!(!menu_title(language).is_empty());
        assert!(!menu_navigation_hint(language).is_empty());
        assert!(!menu_confirm_hint(language).is_empty());
        assert!(!menu_language_hint(language).is_empty());
        assert!(!language_name(language).is_empty());
        assert!(!language_popup_title(language).is_empty());
        assert!(!language_popup_hint(language).is_empty());
        assert!(!language_label(language).is_empty());
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
