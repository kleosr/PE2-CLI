use crate::display::{print_info, print_separator, print_success};
use colored::Colorize;
use pe2_core::config::{self, Config};
use pe2_core::errors::CliError;
use pe2_core::preferences::UserPreferences;
use pe2_core::session::SessionStore;
use pe2_core::stats::StatsTracker;
use std::io::{self, Write};

pub async fn edit_config(config: &mut Config) -> Result<(), CliError> {
    print_config_header();
    edit_provider_field(config)?;
    edit_model_field(config)?;
    edit_api_key_field(config)?;
    save_edited_config(config)?;
    Ok(())
}

fn print_config_header() {
    println!();
    print_info("Configuration (press Enter to keep current value):");
    print_separator();
}

fn edit_provider_field(config: &mut Config) -> Result<(), CliError> {
    prompt_field(
        "Provider",
        |cfg| &cfg.provider,
        |cfg, value| cfg.provider = value,
        config,
    )
}

fn edit_model_field(config: &mut Config) -> Result<(), CliError> {
    prompt_field(
        "Model",
        |cfg| &cfg.model,
        |cfg, value| cfg.model = value,
        config,
    )
}

fn edit_api_key_field(config: &mut Config) -> Result<(), CliError> {
    print_info(
        "API key is kept for this session only (not written to config.json). Prefer env vars.",
    );
    let masked = config::mask_api_key(config.api_key.as_deref());
    print!("  {} [{}]: ", "API Key".bright_white(), masked.dimmed());
    io::stdout().flush()?;
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    if !key.trim().is_empty() {
        config.api_key = Some(key.trim().to_string());
    }
    Ok(())
}

fn save_edited_config(config: &mut Config) -> Result<(), CliError> {
    config::save_config(config)?;
    print_success("Configuration saved!");
    println!();
    Ok(())
}

fn prompt_field(
    label: &str,
    get: fn(&Config) -> &String,
    set: fn(&mut Config, String),
    config: &mut Config,
) -> Result<(), CliError> {
    print!("  {} [{}]: ", label.bright_white(), get(config).dimmed());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let value = input.trim();
    if !value.is_empty() {
        set(config, value.to_string());
    }
    Ok(())
}

pub async fn show_session(session_store: &SessionStore) {
    let entries = session_store.entries.lock().await;
    if entries.is_empty() {
        println!("  {}", "No sessions recorded yet.".dimmed());
        return;
    }
    println!();
    println!(
        "  {} {}",
        "◆".bright_cyan(),
        "Session History".bright_white().bold()
    );
    println!();
    for (i, entry) in entries.iter().rev().take(10).enumerate() {
        let preview: String = entry.prompt.chars().take(60).collect();
        println!(
            "  {} {}. {} {}",
            " ".dimmed(),
            (i + 1).to_string().bright_blue(),
            preview.dimmed(),
            format!("[{}]", entry.difficulty).dimmed(),
        );
    }
    println!();
}

pub fn show_preferences(preferences: &UserPreferences) {
    println!();
    println!(
        "  {} {}",
        "◆".bright_yellow(),
        "Preferences".bright_white().bold()
    );
    println!();
    println!(
        "  {} {}",
        "  Theme:".dimmed(),
        preferences.theme().bright_white()
    );
    println!(
        "  {} {}",
        "  Compact:".dimmed(),
        format!("{}", preferences.compact()).bright_white()
    );
    println!(
        "  {} {}",
        "  Track Usage:".dimmed(),
        format!("{}", preferences.track_usage()).bright_white()
    );
    println!();
}

pub fn show_stats(stats: &StatsTracker) {
    let usage = stats.usage();
    if usage.total_prompts == 0 {
        println!("  {}", "No usage statistics yet.".dimmed());
        return;
    }
    println!();
    println!(
        "  {} {}",
        "◆".bright_green(),
        "Usage Statistics".bright_white().bold()
    );
    println!();
    println!(
        "  {} {} {}",
        "  Total prompts:".dimmed(),
        "·".dimmed(),
        usage.total_prompts.to_string().bright_white()
    );
    show_provider_breakdown(usage);
    show_daily_breakdown(usage);
    println!();
}

fn show_provider_breakdown(usage: &pe2_core::stats::UsageStats) {
    if usage.provider_usage.is_empty() {
        return;
    }
    println!();
    println!("  {}", "  By provider:".dimmed());
    let mut providers: Vec<_> = usage.provider_usage.iter().collect();
    providers.sort_by(|a, b| b.1.cmp(a.1));
    for (provider, count) in providers.iter().take(10) {
        println!(
            "  {} {} {} {}",
            " ".dimmed(),
            provider.bright_cyan(),
            "·".dimmed(),
            count.to_string().bright_white()
        );
    }
}

fn show_daily_breakdown(usage: &pe2_core::stats::UsageStats) {
    if usage.daily_usage.is_empty() {
        return;
    }
    println!();
    println!("  {}", "  By date:".dimmed());
    let mut dates: Vec<_> = usage.daily_usage.iter().collect();
    dates.sort_by(|a, b| b.0.cmp(a.0));
    for (date, count) in dates.iter().take(10) {
        println!(
            "  {} {} {} {}",
            " ".dimmed(),
            date.bright_white(),
            "·".dimmed(),
            count.to_string().bright_white()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pe2_core::stats::StatsTracker;
    use tempfile::TempDir;

    #[test]
    fn show_stats_handles_empty_tracker() {
        let dir = TempDir::new().unwrap();
        let stats = StatsTracker::from_path(dir.path().join("stats.json"));
        show_stats(&stats);
    }
}
