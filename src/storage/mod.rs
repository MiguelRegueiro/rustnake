//! Persistence helpers for local game data.

use crate::utils::{Difficulty, Language};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

const CURRENT_CONFIG_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct HighScores {
    pub easy: u32,
    pub medium: u32,
    pub hard: u32,
    pub extreme: u32,
}

impl HighScores {
    pub fn get(&self, difficulty: Difficulty) -> u32 {
        match difficulty {
            Difficulty::Easy => self.easy,
            Difficulty::Medium => self.medium,
            Difficulty::Hard => self.hard,
            Difficulty::Extreme => self.extreme,
        }
    }

    pub fn set(&mut self, difficulty: Difficulty, score: u32) {
        match difficulty {
            Difficulty::Easy => self.easy = score,
            Difficulty::Medium => self.medium = score,
            Difficulty::Hard => self.hard = score,
            Difficulty::Extreme => self.extreme = score,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RawConfigFile {
    config_version: Option<u32>,
    #[serde(default)]
    high_scores: HighScores,
    #[serde(default)]
    settings: Settings,
    high_score: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFileV1 {
    config_version: u32,
    #[serde(default)]
    high_scores: HighScores,
    #[serde(default)]
    settings: Settings,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub language: Language,
    pub pause_on_focus_loss: bool,
    pub sound_on: bool,
    pub default_difficulty: Difficulty,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: Language::En,
            pause_on_focus_loss: true,
            sound_on: true,
            default_difficulty: Difficulty::Medium,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub high_scores: HighScores,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
struct LegacyHighScoreFile {
    high_score: u32,
}

impl From<LegacyHighScoreFile> for HighScores {
    fn from(value: LegacyHighScoreFile) -> Self {
        Self {
            easy: value.high_score,
            medium: value.high_score,
            hard: value.high_score,
            extreme: value.high_score,
        }
    }
}

fn config_path() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".rustnake.toml");
    }

    PathBuf::from(".rustnake.toml")
}

fn migrate_config(raw: RawConfigFile) -> (AppConfig, bool) {
    let version = raw.config_version.unwrap_or(0);
    let migrated = if version == 0 {
        let high_scores = if raw.high_scores == HighScores::default() {
            raw.high_score
                .map(|high_score| HighScores::from(LegacyHighScoreFile { high_score }))
                .unwrap_or_default()
        } else {
            raw.high_scores
        };
        AppConfig {
            high_scores,
            settings: raw.settings,
        }
    } else {
        AppConfig {
            high_scores: raw.high_scores,
            settings: raw.settings,
        }
    };

    let should_persist_migration = version < CURRENT_CONFIG_VERSION;
    (migrated, should_persist_migration)
}

fn load_config_from_path(path: &Path) -> AppConfig {
    let Ok(contents) = fs::read_to_string(path) else {
        return AppConfig::default();
    };

    if let Ok(raw) = toml::from_str::<RawConfigFile>(&contents) {
        let (config, migrated) = migrate_config(raw);
        if migrated {
            let _ = save_config_to_path(path, &config);
        }
        return config;
    }

    AppConfig::default()
}

fn save_config_to_path(path: &Path, config: &AppConfig) -> Result<(), String> {
    let data = ConfigFileV1 {
        config_version: CURRENT_CONFIG_VERSION,
        high_scores: config.high_scores,
        settings: config.settings,
    };
    let serialized = toml::to_string(&data).map_err(|err| err.to_string())?;
    fs::write(path, serialized).map_err(|err| err.to_string())
}

pub fn load_config() -> AppConfig {
    let path = config_path();
    load_config_from_path(&path)
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = config_path();
    save_config_to_path(&path, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_config_path(test_name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rustnake-storage-{}-{}-{}.toml",
            test_name,
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn migrates_old_high_scores_without_version_and_without_extreme_field() {
        let data = r#"
[high_scores]
easy = 10
medium = 20
hard = 30

[settings]
language = "en"
"#;
        let raw: RawConfigFile = toml::from_str(data).unwrap();
        let (config, migrated) = migrate_config(raw);

        assert_eq!(config.high_scores.easy, 10);
        assert_eq!(config.high_scores.medium, 20);
        assert_eq!(config.high_scores.hard, 30);
        assert_eq!(config.high_scores.extreme, 0);
        assert_eq!(config.settings.language, Language::En);
        assert!(config.settings.pause_on_focus_loss);
        assert!(config.settings.sound_on);
        assert_eq!(config.settings.default_difficulty, Difficulty::Medium);
        assert!(migrated);
    }

    #[test]
    fn migrates_legacy_single_score_populates_all_difficulties() {
        let data = r#"
high_score = 42
"#;
        let raw: RawConfigFile = toml::from_str(data).unwrap();
        let (config, migrated) = migrate_config(raw);

        assert_eq!(config.high_scores.easy, 42);
        assert_eq!(config.high_scores.medium, 42);
        assert_eq!(config.high_scores.hard, 42);
        assert_eq!(config.high_scores.extreme, 42);
        assert!(migrated);
    }

    #[test]
    fn keeps_current_version_without_migration() {
        let data = r#"
config_version = 1

[high_scores]
easy = 7
medium = 8
hard = 9
extreme = 10

[settings]
language = "pt"
"#;
        let raw: RawConfigFile = toml::from_str(data).unwrap();
        let (config, migrated) = migrate_config(raw);

        assert_eq!(config.high_scores.easy, 7);
        assert_eq!(config.high_scores.medium, 8);
        assert_eq!(config.high_scores.hard, 9);
        assert_eq!(config.high_scores.extreme, 10);
        assert_eq!(config.settings.language, Language::Pt);
        assert!(config.settings.pause_on_focus_loss);
        assert!(config.settings.sound_on);
        assert_eq!(config.settings.default_difficulty, Difficulty::Medium);
        assert!(!migrated);
    }

    #[test]
    fn save_format_includes_config_version() {
        let config = AppConfig {
            high_scores: HighScores {
                easy: 1,
                medium: 2,
                hard: 3,
                extreme: 4,
            },
            settings: Settings {
                language: Language::Ja,
                pause_on_focus_loss: false,
                sound_on: true,
                default_difficulty: Difficulty::Extreme,
            },
        };
        let serialized = toml::to_string(&ConfigFileV1 {
            config_version: CURRENT_CONFIG_VERSION,
            high_scores: config.high_scores,
            settings: config.settings,
        })
        .unwrap();

        assert!(serialized.contains("config_version = 1"));
        assert!(serialized.contains("extreme = 4"));
        assert!(serialized.contains("language = \"ja\""));
        assert!(serialized.contains("pause_on_focus_loss = false"));
        assert!(serialized.contains("sound_on = true"));
        assert!(serialized.contains("default_difficulty = \"extreme\""));
    }

    #[test]
    fn load_migrates_unversioned_file_and_persists_v1_format() {
        let path = temp_config_path("migration");
        let legacy_data = r#"
[high_scores]
easy = 11
medium = 22
hard = 33

[settings]
language = "es"
"#;
        fs::write(&path, legacy_data).unwrap();

        let loaded = load_config_from_path(&path);
        assert_eq!(loaded.high_scores.easy, 11);
        assert_eq!(loaded.high_scores.medium, 22);
        assert_eq!(loaded.high_scores.hard, 33);
        assert_eq!(loaded.high_scores.extreme, 0);
        assert_eq!(loaded.settings.language, Language::Es);
        assert!(loaded.settings.pause_on_focus_loss);
        assert!(loaded.settings.sound_on);
        assert_eq!(loaded.settings.default_difficulty, Difficulty::Medium);

        let rewritten = fs::read_to_string(&path).unwrap();
        assert!(rewritten.contains("config_version = 1"));
        assert!(rewritten.contains("extreme = 0"));

        let _ = fs::remove_file(path);
    }
}
