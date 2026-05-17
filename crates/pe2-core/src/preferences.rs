use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::config::preferences_file_path;
use crate::constants;
use crate::errors::CliError;
use crate::write_atomic;

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
    data: PreferencesData,
    save_pending: bool,
}

impl UserPreferences {
    pub fn new() -> Self {
        let data = Self::load();
        Self {
            data,
            save_pending: false,
        }
    }

    fn load() -> PreferencesData {
        let path = preferences_file_path();
        if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            PreferencesData::default()
        }
    }

    pub fn theme(&self) -> &str {
        &self.data.theme
    }

    pub fn compact(&self) -> bool {
        self.data.compact
    }

    pub fn track_usage(&self) -> bool {
        self.data.track_usage
    }

    pub fn set_theme(&mut self, theme: String) {
        self.data.theme = theme;
        self.schedule_save();
    }

    pub fn set_compact(&mut self, compact: bool) {
        self.data.compact = compact;
        self.schedule_save();
    }

    pub fn set_track_usage(&mut self, track: bool) {
        self.data.track_usage = track;
        self.schedule_save();
    }

    pub fn track_command(&mut self, command: &str) {
        if self.data.last_used_commands.len() >= constants::MAX_LAST_USED_COMMANDS {
            self.data.last_used_commands.pop_front();
        }
        self.data.last_used_commands.push_back(command.to_string());
        self.schedule_save();
    }

    pub fn last_used_commands(&self) -> impl Iterator<Item = &str> {
        self.data.last_used_commands.iter().map(|s| s.as_str())
    }

    fn schedule_save(&mut self) {
        if !self.save_pending {
            self.save_pending = true;
            let data = self.data.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                let path = preferences_file_path();
                write_atomic::write_json_atomic(&path, &data).ok();
            });
        }
    }

    pub fn force_save(&self) -> Result<(), CliError> {
        write_atomic::write_json_atomic(&preferences_file_path(), &self.data)?;
        Ok(())
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self::new()
    }
}
