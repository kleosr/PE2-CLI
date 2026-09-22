use crate::constants;
use crate::errors::CliError;
use crate::write_atomic;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_provider")]
    pub provider: String,
    #[serde(default, skip_serializing)]
    pub api_key: Option<String>,
    #[serde(skip)]
    pub output_file: Option<String>,
}

fn default_model() -> String {
    constants::DEFAULT_MODEL.to_string()
}

fn default_provider() -> String {
    constants::DEFAULT_PROVIDER.to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: default_model(),
            provider: default_provider(),
            api_key: None,
            output_file: None,
        }
    }
}

pub fn config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".kleosr-pe2")
}

fn named_path(name: &str) -> PathBuf {
    config_dir().join(name)
}

pub fn config_file_path() -> PathBuf {
    named_path("config.json")
}

pub fn preferences_file_path() -> PathBuf {
    named_path("preferences.json")
}

pub fn stats_file_path() -> PathBuf {
    named_path("stats.json")
}

pub fn ensure_config_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(config_dir())
}

pub fn load_config() -> Result<Config, CliError> {
    let path = config_file_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let text = std::fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn load_config_or_default() -> Config {
    load_config().unwrap_or_else(|error| {
        tracing::warn!("failed to load config: {error}; using defaults");
        Config::default()
    })
}

pub fn save_config(config: &Config) -> Result<(), CliError> {
    ensure_config_dir()?;
    write_atomic::write_json_atomic(&config_file_path(), config)
}

pub fn resolve_api_key(provider: &str, configured: Option<&str>) -> Option<String> {
    if provider == "ollama" {
        return None;
    }
    if let Some(key) = configured.filter(|key| !key.trim().is_empty()) {
        return Some(key.to_string());
    }
    std::env::var(constants::provider_env_var(provider)).ok()
}

pub fn mask_api_key(key: Option<&str>) -> String {
    match key {
        Some(key) if key.len() > constants::SHORT_API_KEY_THRESHOLD => {
            let prefix = &key[..constants::SHORT_API_KEY_PREFIX];
            let suffix = &key[key.len() - constants::SHORT_API_KEY_SUFFIX..];
            format!("{prefix}...{suffix}")
        }
        Some(key) if !key.is_empty() => "**** (short key)".to_string(),
        _ => "not set".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_api_key_none() {
        assert_eq!(mask_api_key(None), "not set");
    }

    #[test]
    fn mask_api_key_empty() {
        assert_eq!(mask_api_key(Some("")), "not set");
    }

    #[test]
    fn mask_api_key_short() {
        assert_eq!(mask_api_key(Some("abc")), "**** (short key)");
    }

    #[test]
    fn mask_api_key_long_shows_prefix_and_suffix() {
        let masked = mask_api_key(Some("sk-abcdefghijklmnop"));
        assert!(masked.starts_with("sk-a"));
        assert!(masked.contains("..."));
        assert!(masked.ends_with("mnop"));
    }

    #[test]
    fn mask_api_key_threshold_boundary() {
        assert_eq!(mask_api_key(Some(&"a".repeat(12))), "**** (short key)");
        let masked = mask_api_key(Some(&"a".repeat(13)));
        assert!(masked.contains("..."));
        assert!(!masked.contains("short key"));
    }
}
