use crate::display::{print_info, print_separator, print_success};
use colored::Colorize;
use pe2_core::config::{self, Config};
use pe2_core::errors::CliError;
use pe2_core::preferences::UserPreferences;
use pe2_core::session::SessionStore;
use pe2_core::stats::StatsTracker;
use std::io::{self, Write};

fn read_line() -> Result<String, CliError> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn ask(label: &str, current: &str) -> Result<Option<String>, CliError> {
    print!("  {} [{}]: ", label.bright_white(), current.dimmed());
    io::stdout().flush()?;
    let value = read_line()?;
    Ok(if value.is_empty() { None } else { Some(value) })
}

fn heading(icon: colored::ColoredString, title: &str) {
    println!();
    println!("  {} {}", icon, title.bright_white().bold());
    println!();
}

pub fn edit_config(config: &mut Config) -> Result<(), CliError> {
    println!();
    print_info("Configuration (press Enter to keep current value):");
    print_separator();
    if let Some(provider) = ask("Provider", &config.provider)? {
        config.provider = provider;
    }
    if let Some(model) = ask("Model", &config.model)? {
        config.model = model;
    }
    print_info(
        "API key is kept for this session only (not written to config.json). Prefer env vars.",
    );
    let masked = config::mask_api_key(config.api_key.as_deref());
    print!("  {} [{}]: ", "API Key".bright_white(), masked.dimmed());
    io::stdout().flush()?;
    let key = read_line()?;
    if !key.is_empty() {
        config.api_key = Some(key);
    }
    config::save_config(config)?;
    print_success("Configuration saved!");
    println!();
    Ok(())
}

pub fn show_session(store: &SessionStore) {
    if store.entries.is_empty() {
        println!("  {}", "No sessions recorded yet.".dimmed());
        return;
    }
    heading("◆".bright_cyan(), "Session History");
    for (index, entry) in store.entries.iter().rev().take(10).enumerate() {
        let preview: String = entry.prompt.chars().take(60).collect();
        let index_label = (index + 1).to_string().bright_blue();
        let difficulty = format!("[{}]", entry.difficulty).dimmed();
        println!(
            "  {} {}. {} {}",
            " ".dimmed(),
            index_label,
            preview.dimmed(),
            difficulty
        );
    }
    println!();
}

pub fn show_preferences(preferences: &UserPreferences) {
    heading("◆".bright_yellow(), "Preferences");
    println!(
        "  {} {}",
        "  Track Usage:".dimmed(),
        preferences.track_usage().to_string().bright_white()
    );
    println!();
}

pub fn show_stats(tracker: &StatsTracker) {
    let usage = tracker.usage();
    if usage.total_prompts == 0 {
        println!("  {}", "No usage statistics yet.".dimmed());
        return;
    }
    heading("◆".bright_green(), "Usage Statistics");
    println!(
        "  {} {} {}",
        "  Total prompts:".dimmed(),
        "·".dimmed(),
        usage.total_prompts.to_string().bright_white()
    );
    let mut providers: Vec<(String, u64)> = usage
        .provider_usage
        .iter()
        .map(|(name, count)| (name.clone(), *count))
        .collect();
    let mut days: Vec<(String, u64)> = usage
        .daily_usage
        .iter()
        .map(|(day, count)| (day.clone(), *count))
        .collect();
    breakdown("  By provider:", &mut providers, true);
    breakdown("  By date:", &mut days, false);
    println!();
}

fn breakdown(title: &str, rows: &mut [(String, u64)], by_count: bool) {
    if rows.is_empty() {
        return;
    }
    println!();
    println!("  {}", title.dimmed());
    if by_count {
        rows.sort_by(|left, right| right.1.cmp(&left.1));
    } else {
        rows.sort_by(|left, right| right.0.cmp(&left.0));
    }
    for (label, count) in rows.iter().take(10) {
        let painted = if by_count {
            label.bright_cyan()
        } else {
            label.bright_white()
        };
        println!(
            "  {} {} {} {}",
            " ".dimmed(),
            painted,
            "·".dimmed(),
            count.to_string().bright_white()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn show_stats_handles_empty_tracker() {
        let dir = TempDir::new().unwrap();
        show_stats(&StatsTracker::from_path(dir.path().join("stats.json")));
    }
}
