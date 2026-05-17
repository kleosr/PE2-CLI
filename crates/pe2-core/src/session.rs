use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::constants;
use crate::config::{ensure_config_dir, sessions_dir_path};
use crate::errors::CliError;
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionData {
    pub prompts: Vec<String>,
}

#[derive(Debug)]
pub struct SessionManager {
    current: SessionData,
    session_id: String,
    pub entries: Arc<Mutex<Vec<SessionEntry>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            current: SessionData::default(),
            session_id: Uuid::new_v4().to_string(),
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn add_prompt(&mut self, prompt: String) {
        self.current.prompts.push(prompt);
        if self.current.prompts.len() > constants::MAX_HISTORY_ITEMS {
            self.current.prompts.remove(0);
        }
    }

    pub fn prompts(&self) -> &[String] {
        &self.current.prompts
    }

    pub fn prompt_count(&self) -> usize {
        self.current.prompts.len()
    }

    pub async fn save_session(&self, output_file: &str, prompt: &str, result: &str) -> Result<(), CliError> {
        ensure_config_dir()?;
        let sessions_dir = sessions_dir_path();
        if !sessions_dir.exists() {
            std::fs::create_dir_all(&sessions_dir)?;
        }
        let session_file = sessions_dir.join(format!("session-{}.json", self.session_id));
        let entry = serde_json::json!({
            "session_id": self.session_id,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "input_prompt": prompt,
            "output_file": output_file,
            "result_snippet": result.chars().take(500).collect::<String>(),
        });
        let content = serde_json::to_string_pretty(&entry)?;
        std::fs::write(&session_file, content)?;
        Ok(())
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
        let content = serde_json::to_string_pretty(&*entries)?;
        std::fs::write(&session_file, content)?;
        Ok(())
    }

    pub async fn add_entry(&self, entry: SessionEntry) {
        let mut entries = self.entries.lock().await;
        entries.push(entry);
        if entries.len() > constants::MAX_HISTORY_ITEMS {
            entries.remove(0);
        }
    }

    pub async fn load_history(limit: Option<usize>) -> Vec<String> {
        let limit = limit.unwrap_or(50);
        let sessions_dir = sessions_dir_path();
        if !sessions_dir.exists() {
            return Vec::new();
        }
        let mut entries = Vec::new();
        let mut read_dir = match std::fs::read_dir(&sessions_dir) {
            Ok(d) => d,
            Err(_) => return Vec::new(),
        };
        while let Ok(Some(entry)) = read_dir.next() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    entries.push(content);
                }
            }
        }
        entries.sort();
        entries.truncate(limit);
        entries
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
