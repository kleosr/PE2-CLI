use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::config::preferences_file_path;
use crate::json_store::JsonStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrefs {
    pub theme: String,
    pub compact: bool,
    pub track_usage: bool,
}

impl Default for UserPrefs {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            compact: false,
            track_usage: true,
        }
    }
}

#[derive(Debug)]
pub struct UserPreferences {
    store: JsonStore<UserPrefs>,
}

impl UserPreferences {
    pub fn new() -> Self {
        Self::from_path(preferences_file_path())
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self {
            store: JsonStore::load_or_default(path),
        }
    }

    pub fn theme(&self) -> &str {
        &self.store.snapshot().theme
    }

    pub fn compact(&self) -> bool {
        self.store.snapshot().compact
    }

    pub fn track_usage(&self) -> bool {
        self.store.snapshot().track_usage
    }

    pub fn set_theme(&mut self, theme: String) {
        self.store.snapshot_mut().theme = theme;
        self.store.persist_best_effort();
    }

    pub fn set_compact(&mut self, compact: bool) {
        self.store.snapshot_mut().compact = compact;
        self.store.persist_best_effort();
    }

    pub fn set_track_usage(&mut self, track: bool) {
        self.store.snapshot_mut().track_usage = track;
        self.store.persist_best_effort();
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self::new()
    }
}
