use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;
use crate::config::ensure_config_dir;
use crate::errors::CliError;
use crate::write_atomic;

#[derive(Debug)]
pub struct JsonStore<T> {
    data: T,
    path: PathBuf,
}

impl<T> JsonStore<T>
where
    T: Default + Clone + Serialize + DeserializeOwned,
{
    pub fn try_load(path: PathBuf) -> Result<Self, CliError> {
        if !path.exists() {
            return Ok(Self {
                data: T::default(),
                path,
            });
        }
        let content = std::fs::read_to_string(&path).map_err(CliError::Io)?;
        let data = serde_json::from_str(&content).map_err(CliError::Json)?;
        Ok(Self { data, path })
    }

    pub fn load_or_default(path: PathBuf) -> Self {
        match Self::try_load(path.clone()) {
            Ok(store) => store,
            Err(e) => {
                tracing::warn!("failed to load {}: {e}; using defaults", path.display());
                Self {
                    data: T::default(),
                    path,
                }
            }
        }
    }

    pub fn snapshot(&self) -> &T {
        &self.data
    }

    pub fn snapshot_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn persist_best_effort(&self) {
        if let Err(e) = self.persist() {
            tracing::warn!("failed to persist {}: {e}", self.path.display());
        }
    }

    pub fn persist(&self) -> Result<(), CliError> {
        ensure_config_dir()?;
        write_atomic::write_json_atomic(&self.path, &self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    #[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
    struct Sample {
        count: u32,
    }

    #[test]
    fn round_trip_persist_and_reload() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("sample.json");
        let mut store = JsonStore::<Sample>::try_load(path.clone()).unwrap();
        store.snapshot_mut().count = 3;
        store.persist().unwrap();

        let reloaded = JsonStore::<Sample>::try_load(path).unwrap();
        assert_eq!(reloaded.snapshot().count, 3);
    }

    #[test]
    fn load_missing_file_uses_default() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("missing.json");
        let store = JsonStore::<Sample>::try_load(path).unwrap();
        assert_eq!(store.snapshot().count, 0);
    }

    #[test]
    fn load_corrupt_file_returns_error() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "not-json").unwrap();
        assert!(JsonStore::<Sample>::try_load(path).is_err());
    }

    #[test]
    fn load_or_default_recovers_from_corrupt_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "not-json").unwrap();
        let store = JsonStore::<Sample>::load_or_default(path);
        assert_eq!(store.snapshot().count, 0);
    }
}
