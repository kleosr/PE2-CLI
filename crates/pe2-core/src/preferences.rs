use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::config::preferences_file_path;
use crate::constants;
use crate::errors::CliError;
use crate::json_store::JsonStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferencesData {
    pub theme: String,
    pub compact: bool,
    pub track_usage: bool,
    pub last_used_commands: VecDeque<String>,
}

impl Default for PreferencesData {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            compact: false,
            track_usage: true,
            last_used_commands: VecDeque::with_capacity(constants::MAX_LAST_USED_COMMANDS),
        }
    }
}

#[derive(Debug)]
pub struct UserPreferences {
    store: JsonStore<PreferencesData>,
}

impl UserPreferences {
    pub fn new() -> Self {
        Self {
            store: JsonStore::load(preferences_file_path()),
        }
    }

    pub fn theme(&self) -> &str {
        &self.store.data().theme
    }

    pub fn compact(&self) -> bool {
        self.store.data().compact
    }

    pub fn track_usage(&self) -> bool {
        self.store.data().track_usage
    }

    pub fn set_theme(&mut self, theme: String) {
        self.store.data_mut().theme = theme;
        self.store.persist_best_effort();
    }

    pub fn set_compact(&mut self, compact: bool) {
        self.store.data_mut().compact = compact;
        self.store.persist_best_effort();
    }

    pub fn set_track_usage(&mut self, track: bool) {
        self.store.data_mut().track_usage = track;
        self.store.persist_best_effort();
    }

    pub fn track_command(&mut self, command: &str) {
        let data = self.store.data_mut();
        if data.last_used_commands.len() >= constants::MAX_LAST_USED_COMMANDS {
            data.last_used_commands.pop_front();
        }
        data.last_used_commands.push_back(command.to_string());
        self.store.persist_best_effort();
    }

    pub fn last_used_commands(&self) -> impl Iterator<Item = &str> {
        self.store.data().last_used_commands.iter().map(|s| s.as_str())
    }

    pub fn force_save(&self) -> Result<(), CliError> {
        self.store.persist()
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self::new()
    }
}
