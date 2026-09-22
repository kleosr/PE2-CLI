use crate::config::preferences_file_path;
use crate::write_atomic;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    #[serde(default = "default_track_usage")]
    track_usage: bool,
}

fn default_track_usage() -> bool {
    true
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            track_usage: default_track_usage(),
        }
    }
}

impl UserPreferences {
    pub fn new() -> Self {
        Self::from_path(preferences_file_path())
    }

    pub fn from_path(path: PathBuf) -> Self {
        write_atomic::read_json_or_default(&path)
    }

    pub fn track_usage(&self) -> bool {
        self.track_usage
    }
}
