use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::constants;
use crate::errors::CliError;
use crate::write_atomic;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_provider")]
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            model: constants::DEFAULT_MODEL.to_string(),
            provider: constants::DEFAULT_PROVIDER.to_string(),
            api_key: None,
            output_file: None,
        }
    }
}

pub fn config_dir() -> PathBuf {
    let base = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join(".kleosr-pe2")
}

pub fn config_file_path() -> PathBuf {
    config_dir().join("config.json")
}

pub fn preferences_file_path() -> PathBuf {
    config_dir().join("preferences.json")
}

pub fn stats_file_path() -> PathBuf {
    config_dir().join("stats.json")
}

pub fn sessions_dir_path() -> PathBuf {
    config_dir().join("sessions")
}

pub fn ensure_config_dir() -> std::io::Result<()> {
    let dir = config_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(())
}

pub fn load_config() -> Config {
    let path = config_file_path();
    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    } else {
        Config::default()
    }
}

pub fn save_config(config: &Config) -> Result<(), CliError> {
    ensure_config_dir()?;
    let path = config_file_path();
    write_atomic::write_json_atomic(&path, config)?;
    Ok(())
}

pub fn resolve_api_key(provider: &str, config_key: Option<&str>) -> Option<String> {
    if let Some(key) = config_key {
        if !key.trim().is_empty() {
            return Some(key.to_string());
        }
    }
    let env_var = constants::provider_env_var(provider);
    std::env::var(env_var).ok()
}
