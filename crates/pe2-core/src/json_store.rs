use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;
use crate::config::ensure_config_dir;
use crate::errors::CliError;
use crate::write_atomic;

pub struct JsonStore<T> {
    data: T,
    path: PathBuf,
}

impl<T> JsonStore<T>
where
    T: Default + Clone + Serialize + DeserializeOwned,
{
    pub fn load(path: PathBuf) -> Self {
        let data = if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            T::default()
        };
        Self { data, path }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
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
        let mut store = JsonStore::load(path.clone());
        store.data_mut().count = 3;
        store.persist().unwrap();

        let reloaded = JsonStore::<Sample>::load(path);
        assert_eq!(reloaded.data().count, 3);
    }

    #[test]
    fn load_missing_file_uses_default() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("missing.json");
        let store = JsonStore::<Sample>::load(path);
        assert_eq!(store.data().count, 0);
    }

    #[test]
    fn load_corrupt_file_uses_default() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "not-json").unwrap();
        let store = JsonStore::<Sample>::load(path);
        assert_eq!(store.data().count, 0);
    }
}
