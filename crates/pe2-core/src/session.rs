use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::constants;
use crate::config::{ensure_config_dir, sessions_dir_path};
use crate::errors::CliError;
use crate::write_atomic;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntry {
    pub prompt: String,
    pub output: String,
    pub model: String,
    pub provider: String,
    pub difficulty: String,
    pub score: u32,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct SessionStore {
    session_id: String,
    pub entries: Arc<Mutex<Vec<SessionEntry>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub async fn save(&self) -> Result<(), CliError> {
        ensure_config_dir()?;
        let sessions_dir = sessions_dir_path();
        if !sessions_dir.exists() {
            std::fs::create_dir_all(&sessions_dir)?;
        }
        let entries = self.entries.lock().await;
        if entries.is_empty() {
            return Ok(());
        }
        let session_file = sessions_dir.join(format!("session-{}.json", self.session_id));
        write_atomic::write_json_atomic(&session_file, &*entries)?;
        Ok(())
    }

    pub async fn add_entry(&self, entry: SessionEntry) {
        let mut entries = self.entries.lock().await;
        entries.push(entry);
        if entries.len() > constants::MAX_HISTORY_ITEMS {
            entries.remove(0);
        }
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}
