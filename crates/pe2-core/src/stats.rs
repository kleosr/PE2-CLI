use crate::config::stats_file_path;
use crate::errors::CliError;
use crate::write_atomic;
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
    stats: UsageStats,
    path: PathBuf,
}
impl StatsTracker {
    pub fn new() -> Self {
        Self::from_path(stats_file_path())
    }
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            stats: write_atomic::read_json_or_default(&path),
            path,
        }
    }
    pub fn record_usage(&mut self, pv: &str, c: Option<u32>) {
        self.stats.total_prompts += 1;
        if let Some(s) = c {
            let n = self.stats.total_prompts as f64;
            self.stats.running_avg_complexity = if n > 1.0 {
                ((n - 1.0) / n) * self.stats.running_avg_complexity + s as f64 / n
            } else {
                s as f64
            };
        }
        *self
            .stats
            .daily_usage
            .entry(Local::now().format("%Y-%m-%d").to_string())
            .or_insert(0) += 1;
        *self.stats.provider_usage.entry(pv.to_string()).or_insert(0) += 1;
        self.stats.last_updated = Utc::now().to_rfc3339();
        prune(&mut self.stats);
        if let Err(e) = self.save() {
            tracing::warn!("failed to persist {}: {e}", self.path.display());
        }
    }
    pub fn save(&self) -> Result<(), CliError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        write_atomic::write_json_atomic(&self.path, &self.stats)
    }
    pub fn usage(&self) -> &UsageStats {
        &self.stats
    }
}
fn prune(s: &mut UsageStats) {
    if s.daily_usage.len() > 120 {
        let mut k: Vec<String> = s.daily_usage.keys().cloned().collect();
        k.sort();
        for x in k.iter().take(k.len().saturating_sub(90)) {
            s.daily_usage.remove(x);
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
        let d = tempfile::TempDir::new().unwrap();
        let mut st = StatsTracker::from_path(d.path().join("stats.json"));
        st.record_usage("openrouter", None);
        assert_eq!(st.usage().total_prompts, 1);
        assert_eq!(st.usage().provider_usage.get("openrouter"), Some(&1));
        assert_eq!(st.usage().daily_usage.len(), 1);
    }
    #[test]
    fn record_usage_updates_complexity_average() {
        let d = tempfile::TempDir::new().unwrap();
        let mut st = StatsTracker::from_path(d.path().join("stats.json"));
        st.record_usage("openai", Some(10));
        st.record_usage("openai", Some(20));
        assert_eq!(st.usage().total_prompts, 2);
        assert!((st.usage().running_avg_complexity - 15.0).abs() < f64::EPSILON);
    }
}
