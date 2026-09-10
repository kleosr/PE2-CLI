use crate::config::preferences_file_path;
use crate::write_atomic;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    #[serde(default = "dt")]
    track_usage: bool,
}
fn dt() -> bool {
    true
}
impl Default for UserPreferences {
    fn default() -> Self {
        Self { track_usage: dt() }
    }
}
impl UserPreferences {
    pub fn new() -> Self {
        Self::from_path(preferences_file_path())
    }
    pub fn from_path(p: PathBuf) -> Self {
        write_atomic::read_json_or_default(&p)
    }
    pub fn track_usage(&self) -> bool {
        self.track_usage
    }
}
