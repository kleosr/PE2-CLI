use crate::config::stats_file_path;
use crate::errors::CliError;
use crate::json_store::JsonStore;
use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageStats {
    pub total_prompts: u64,
    pub running_avg_complexity: f64,
    pub daily_usage: HashMap<String, u64>,
    #[serde(default)]
    pub provider_usage: HashMap<String, u64>,
    pub last_updated: String,
}

#[derive(Debug)]
pub struct StatsTracker {
    store: JsonStore<UsageStats>,
}

impl StatsTracker {
    pub fn new() -> Self {
        Self::from_path(stats_file_path())
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self {
            store: JsonStore::load_or_default(path),
        }
    }

    pub fn record_usage(&mut self, provider: &str, complexity_score: Option<u32>) {
        let stats = self.store.snapshot_mut();
        stats.total_prompts += 1;
        if let Some(score) = complexity_score {
            let n = stats.total_prompts as f64;
            stats.running_avg_complexity = if n > 1.0 {
                ((n - 1.0) / n) * stats.running_avg_complexity + (1.0 / n) * score as f64
            } else {
                score as f64
            };
        }
        let today = Local::now().format("%Y-%m-%d").to_string();
        *stats.daily_usage.entry(today).or_insert(0) += 1;
        *stats
            .provider_usage
            .entry(provider.to_string())
            .or_insert(0) += 1;
        stats.last_updated = Utc::now().to_rfc3339();
        prune_daily_usage(stats);
        self.store.persist_best_effort();
    }

    pub fn save(&self) -> Result<(), CliError> {
        self.store.persist()
    }

    pub fn usage(&self) -> &UsageStats {
        self.store.snapshot()
    }
}

fn prune_daily_usage(stats: &mut UsageStats) {
    if stats.daily_usage.len() > 120 {
        let mut keys: Vec<String> = stats.daily_usage.keys().cloned().collect();
        keys.sort();
        let cutoff = keys.len().saturating_sub(90);
        for key in keys.iter().take(cutoff) {
            stats.daily_usage.remove(key);
        }
    }
}

impl Default for StatsTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_usage_tracks_provider_and_daily() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut st = StatsTracker::from_path(dir.path().join("stats.json"));
        st.record_usage("openrouter", None);
        assert_eq!(st.usage().total_prompts, 1);
        assert_eq!(st.usage().provider_usage.get("openrouter"), Some(&1));
        assert_eq!(st.usage().daily_usage.len(), 1);
    }

    #[test]
    fn record_usage_updates_complexity_average() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut st = StatsTracker::from_path(dir.path().join("stats.json"));
        st.record_usage("openai", Some(10));
        st.record_usage("openai", Some(20));
        assert_eq!(st.usage().total_prompts, 2);
        assert!((st.usage().running_avg_complexity - 15.0).abs() < f64::EPSILON);
    }
}
