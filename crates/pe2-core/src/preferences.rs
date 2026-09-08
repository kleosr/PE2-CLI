use crate::config::preferences_file_path;
use crate::write_atomic;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPrefs {
    #[serde(default = "default_track_usage")]
    pub track_usage: bool,
}

fn default_track_usage() -> bool {
    true
}

impl Default for UserPrefs {
    fn default() -> Self {
        Self {
            track_usage: default_track_usage(),
        }
    }
}

#[derive(Debug)]
pub struct UserPreferences {
    prefs: UserPrefs,
}

impl UserPreferences {
    pub fn new() -> Self {
        Self::from_path(preferences_file_path())
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self {
            prefs: write_atomic::read_json_or_default(&path),
        }
    }

    pub fn track_usage(&self) -> bool {
        self.prefs.track_usage
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self::new()
    }
}
