use crate::constants;

#[derive(Debug, Clone)]
pub struct SessionEntry {
    pub prompt: String,
    pub output: String,
    pub model: String,
    pub provider: String,
    pub difficulty: String,
    pub score: u32,
    pub timestamp: String,
}

#[derive(Debug, Default)]
pub struct SessionStore {
    pub entries: Vec<SessionEntry>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, entry: SessionEntry) {
        self.entries.push(entry);
        if self.entries.len() > constants::MAX_HISTORY_ITEMS {
            self.entries.remove(0);
        }
    }
}
