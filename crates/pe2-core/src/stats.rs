use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::config::stats_file_path;
use crate::errors::CliError;
use crate::write_atomic;
use chrono;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatsData {
    pub total_prompts: u64,
    pub running_avg_complexity: f64,
    pub daily_usage: HashMap<String, u64>,
    pub last_updated: String,
}

#[derive(Debug)]
pub struct StatsTracker {
    data: StatsData,
    save_pending: bool,
    pub daily_usage: Arc<Mutex<HashMap<String, u64>>>,
}

impl StatsTracker {
    pub fn new() -> Self {
        let data = Self::load();
        let daily_usage = Arc::new(Mutex::new(data.daily_usage.clone()));
        Self {
            data,
            save_pending: false,
            daily_usage,
        }
    }

    fn load() -> StatsData {
        let path = stats_file_path();
        if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            StatsData::default()
        }
    }

    pub fn record_usage(&mut self, provider: &str) {
        self.data.total_prompts += 1;
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        *self.data.daily_usage.entry(today.clone()).or_insert(0) += 1;
        *self.data.last_updated = chrono::Utc::now().to_rfc3339();

        // Also update the shared mutex
        if let Ok(mut usage) = self.daily_usage.try_lock() {
            *usage.entry(today).or_insert(0) += 1;
        }

        self.prune_daily_usage();
        self.schedule_save();
    }

    pub fn track(&mut self, model: &str, complexity_score: u32) {
        self.data.total_prompts += 1;
        let n = self.data.total_prompts as f64;
        self.data.running_avg_complexity = if n > 1.0 {
            ((n - 1.0) / n) * self.data.running_avg_complexity + (1.0 / n) * complexity_score as f64
        } else {
            complexity_score as f64
        };

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        *self.data.daily_usage.entry(today.clone()).or_insert(0) += 1;
        self.data.last_updated = chrono::Utc::now().to_rfc3339();

        if let Ok(mut usage) = self.daily_usage.try_lock() {
            *usage.entry(today).or_insert(0) += 1;
        }

        self.prune_daily_usage();
        self.schedule_save();
    }

    fn prune_daily_usage(&mut self) {
        if self.data.daily_usage.len() > 120 {
            let mut keys: Vec<String> = self.data.daily_usage.keys().cloned().collect();
            keys.sort();
            let cutoff = keys.len().saturating_sub(90);
            for key in keys.iter().take(cutoff) {
                self.data.daily_usage.remove(key);
            }
        }
    }

    fn schedule_save(&mut self) {
        if !self.save_pending {
            self.save_pending = true;
            let data = self.data.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                let path = stats_file_path();
                write_atomic::write_json_atomic(&path, &data).ok();
            });
        }
    }

    pub async fn save(&self) -> Result<(), CliError> {
        write_atomic::write_json_atomic(&stats_file_path(), &self.data)?;
        Ok(())
    }

    pub fn force_save(&self) -> Result<(), CliError> {
        write_atomic::write_json_atomic(&stats_file_path(), &self.data)?;
        Ok(())
    }

    pub fn stats(&self) -> &StatsData {
        &self.data
    }
}

impl Default for StatsTracker {
    fn default() -> Self {
        Self::new()
    }
}
