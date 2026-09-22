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

    pub fn record_usage(&mut self, provider: &str, complexity: Option<u32>) {
        self.stats.total_prompts += 1;
        if let Some(score) = complexity {
            let count = self.stats.total_prompts as f64;
            self.stats.running_avg_complexity = if count > 1.0 {
                ((count - 1.0) / count) * self.stats.running_avg_complexity + score as f64 / count
            } else {
                score as f64
            };
        }
        let today = Local::now().format("%Y-%m-%d").to_string();
        *self.stats.daily_usage.entry(today).or_insert(0) += 1;
        *self
            .stats
            .provider_usage
            .entry(provider.to_string())
            .or_insert(0) += 1;
        self.stats.last_updated = Utc::now().to_rfc3339();
        prune_daily(&mut self.stats);
        if let Err(error) = self.save() {
            tracing::warn!("failed to persist {}: {error}", self.path.display());
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

fn prune_daily(stats: &mut UsageStats) {
    if stats.daily_usage.len() <= 120 {
        return;
    }
    let mut days: Vec<String> = stats.daily_usage.keys().cloned().collect();
    days.sort();
    let drop_count = days.len().saturating_sub(90);
    for day in days.into_iter().take(drop_count) {
        stats.daily_usage.remove(&day);
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
        let mut tracker = StatsTracker::from_path(dir.path().join("stats.json"));
        tracker.record_usage("openrouter", None);
        assert_eq!(tracker.usage().total_prompts, 1);
        assert_eq!(tracker.usage().provider_usage.get("openrouter"), Some(&1));
        assert_eq!(tracker.usage().daily_usage.len(), 1);
    }

    #[test]
    fn record_usage_updates_complexity_average() {
        let dir = tempfile::TempDir::new().unwrap();
        let mut tracker = StatsTracker::from_path(dir.path().join("stats.json"));
        tracker.record_usage("openai", Some(10));
        tracker.record_usage("openai", Some(20));
        assert_eq!(tracker.usage().total_prompts, 2);
        assert!((tracker.usage().running_avg_complexity - 15.0).abs() < f64::EPSILON);
    }
}
