use crate::constants;
use crate::errors::CliError;
use crate::write_atomic;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "dm")]
    pub model: String,
    #[serde(default = "dp")]
    pub provider: String,
    #[serde(default, skip_serializing)]
    pub api_key: Option<String>,
    #[serde(skip)]
    pub output_file: Option<String>,
}
fn dm() -> String {
    constants::DEFAULT_MODEL.to_string()
}
fn dp() -> String {
    constants::DEFAULT_PROVIDER.to_string()
}
impl Default for Config {
    fn default() -> Self {
        Self {
            model: dm(),
            provider: dp(),
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
fn p(n: &str) -> PathBuf {
    config_dir().join(n)
}
pub fn config_file_path() -> PathBuf {
    p("config.json")
}
pub fn preferences_file_path() -> PathBuf {
    p("preferences.json")
}
pub fn stats_file_path() -> PathBuf {
    p("stats.json")
}
pub fn ensure_config_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(config_dir())
}
pub fn load_config() -> Result<Config, CliError> {
    let f = config_file_path();
    if !f.exists() {
        return Ok(Config::default());
    }
    serde_json::from_str(&std::fs::read_to_string(&f).map_err(CliError::Io)?)
        .map_err(CliError::Json)
}
pub fn load_config_or_default() -> Config {
    load_config().unwrap_or_else(|e| {
        tracing::warn!("failed to load config: {e}; using defaults");
        Config::default()
    })
}
pub fn save_config(c: &Config) -> Result<(), CliError> {
    ensure_config_dir()?;
    write_atomic::write_json_atomic(&config_file_path(), c)
}
pub fn resolve_api_key(pv: &str, k: Option<&str>) -> Option<String> {
    if pv == "ollama" {
        return None;
    }
    if let Some(k) = k.filter(|k| !k.trim().is_empty()) {
        return Some(k.to_string());
    }
    std::env::var(constants::provider_env_var(pv)).ok()
}
pub fn mask_api_key(k: Option<&str>) -> String {
    match k {
        Some(k) if k.len() > constants::SHORT_API_KEY_THRESHOLD => {
            format!(
                "{}...{}",
                &k[..constants::SHORT_API_KEY_PREFIX],
                &k[k.len() - constants::SHORT_API_KEY_SUFFIX..]
            )
        }
        Some(k) if !k.is_empty() => "**** (short key)".to_string(),
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
        let m = mask_api_key(Some("sk-abcdefghijklmnop"));
        assert!(m.starts_with("sk-a"));
        assert!(m.contains("..."));
        assert!(m.ends_with("mnop"));
    }
    #[test]
    fn mask_api_key_threshold_boundary() {
        assert_eq!(mask_api_key(Some(&"a".repeat(12))), "**** (short key)");
        let m = mask_api_key(Some(&"a".repeat(13)));
        assert!(m.contains("..."));
        assert!(!m.contains("short key"));
    }
}
