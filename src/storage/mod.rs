//! Persistence helpers for local game data.

use crate::utils::{Difficulty, Language};
use serde::{Deserialize, Serialize};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

const CURRENT_CONFIG_VERSION: u32 = 1;
const MAX_CONFIG_BYTES: u64 = 64 * 1024;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

fn legacy_local_config_path() -> PathBuf {
    PathBuf::from(".rustnake.toml")
}

fn legacy_home_config_path() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        return Some(PathBuf::from(home).join(".rustnake.toml"));
    }
    #[cfg(target_os = "windows")]
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        return Some(PathBuf::from(user_profile).join(".rustnake.toml"));
    }

    None
}

#[cfg(target_os = "windows")]
fn config_path() -> PathBuf {
    if let Ok(app_data) = std::env::var("APPDATA") {
        return PathBuf::from(app_data).join("Rustnake").join("config.toml");
    }
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        return PathBuf::from(local_app_data)
            .join("Rustnake")
            .join("config.toml");
    }
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        return PathBuf::from(user_profile)
            .join("AppData")
            .join("Roaming")
            .join("Rustnake")
            .join("config.toml");
    }

    legacy_local_config_path()
}

#[cfg(target_os = "macos")]
fn config_path() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("Rustnake")
            .join("config.toml");
    }

    legacy_local_config_path()
}

#[cfg(all(unix, not(target_os = "macos")))]
fn config_path() -> PathBuf {
    legacy_home_config_path().unwrap_or_else(legacy_local_config_path)
}

#[cfg(not(any(unix, target_os = "windows")))]
fn config_path() -> PathBuf {
    legacy_local_config_path()
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

fn load_raw_config(path: &Path) -> Option<RawConfigFile> {
    let metadata = fs::metadata(path).ok()?;
    if metadata.len() > MAX_CONFIG_BYTES {
        return None;
    }
    let contents = fs::read_to_string(path).ok()?;
    toml::from_str::<RawConfigFile>(&contents).ok()
}

fn load_config_from_path(path: &Path) -> AppConfig {
    if let Some(raw) = load_raw_config(path) {
        let (config, migrated) = migrate_config(raw);
        if migrated {
            let _ = save_config_to_path(path, &config);
        }
        return config;
    }

    AppConfig::default()
}

fn migrate_legacy_config_if_needed(target_path: &Path) {
    if fs::metadata(target_path).is_ok() {
        return;
    }

    let mut legacy_paths = Vec::with_capacity(2);
    if let Some(path) = legacy_home_config_path() {
        legacy_paths.push(path);
    }
    legacy_paths.push(legacy_local_config_path());

    for legacy_path in legacy_paths {
        if legacy_path == target_path {
            continue;
        }
        let Some(raw) = load_raw_config(&legacy_path) else {
            continue;
        };
        let (config, _) = migrate_config(raw);
        if save_config_to_path(target_path, &config).is_ok() {
            break;
        }
    }
}

fn save_atomic(path: &Path, contents: &str) -> Result<(), String> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    let file_name = path
        .file_name()
        .ok_or_else(|| "invalid config path".to_string())?
        .to_string_lossy();

    for attempt in 0..16u32 {
        let tmp_path = parent.join(format!(
            ".{}.tmp-{}-{}",
            file_name,
            std::process::id(),
            attempt
        ));

        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        options.mode(0o600);

        let mut temp_file = match options.open(&tmp_path) {
            Ok(file) => file,
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(err.to_string()),
        };

        if let Err(err) = temp_file.write_all(contents.as_bytes()) {
            let _ = fs::remove_file(&tmp_path);
            return Err(err.to_string());
        }

        if let Err(err) = temp_file.sync_all() {
            let _ = fs::remove_file(&tmp_path);
            return Err(err.to_string());
        }

        drop(temp_file);
        if let Err(err) = fs::rename(&tmp_path, path) {
            let _ = fs::remove_file(&tmp_path);
            return Err(err.to_string());
        }

        return Ok(());
    }

    Err("failed to create temporary config file".to_string())
}

fn save_config_to_path(path: &Path, config: &AppConfig) -> Result<(), String> {
    let data = ConfigFileV1 {
        config_version: CURRENT_CONFIG_VERSION,
        high_scores: config.high_scores,
        settings: config.settings,
    };
    let serialized = toml::to_string(&data).map_err(|err| err.to_string())?;
    save_atomic(path, &serialized)
}

pub fn load_config() -> AppConfig {
    let path = config_path();
    migrate_legacy_config_if_needed(&path);
    load_config_from_path(&path)
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = config_path();
    save_config_to_path(&path, config)
}

pub fn config_path_for_current_user() -> PathBuf {
    config_path()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
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

    #[test]
    fn oversized_config_file_is_ignored() {
        let path = temp_config_path("oversized");
        let oversized_data = "x".repeat((MAX_CONFIG_BYTES as usize) + 1);
        fs::write(&path, oversized_data).unwrap();

        let loaded = load_config_from_path(&path);
        assert_eq!(loaded.high_scores, HighScores::default());
        assert_eq!(loaded.settings, Settings::default());

        let _ = fs::remove_file(path);
    }

    #[cfg(unix)]
    #[test]
    fn save_config_uses_private_file_permissions() {
        let path = temp_config_path("permissions");
        let config = AppConfig::default();
        save_config_to_path(&path, &config).unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let mode = metadata.permissions().mode();
        assert_eq!(
            mode & 0o077,
            0,
            "config file must not be group/world readable"
        );

        let _ = fs::remove_file(path);
    }
}
