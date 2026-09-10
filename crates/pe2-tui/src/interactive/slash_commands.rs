use crate::display::{print_info, print_separator, print_success};
use colored::Colorize;
use pe2_core::config::{self, Config};
use pe2_core::errors::CliError;
use pe2_core::preferences::UserPreferences;
use pe2_core::session::SessionStore;
use pe2_core::stats::StatsTracker;
use std::io::{self, Write};

fn rl() -> Result<String, CliError> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}
fn ask(l: &str, c: &str) -> Result<Option<String>, CliError> {
    print!("  {} [{}]: ", l.bright_white(), c.dimmed());
    io::stdout().flush()?;
    let v = rl()?;
    Ok(if v.is_empty() { None } else { Some(v) })
}
fn head(ic: colored::ColoredString, t: &str) {
    println!();
    println!("  {} {}", ic, t.bright_white().bold());
    println!();
}
pub fn edit_config(c: &mut Config) -> Result<(), CliError> {
    println!();
    print_info("Configuration (press Enter to keep current value):");
    print_separator();
    if let Some(v) = ask("Provider", &c.provider)? {
        c.provider = v;
    }
    if let Some(v) = ask("Model", &c.model)? {
        c.model = v;
    }
    print_info(
        "API key is kept for this session only (not written to config.json). Prefer env vars.",
    );
    let m = config::mask_api_key(c.api_key.as_deref());
    print!("  {} [{}]: ", "API Key".bright_white(), m.dimmed());
    io::stdout().flush()?;
    let k = rl()?;
    if !k.is_empty() {
        c.api_key = Some(k);
    }
    config::save_config(c)?;
    print_success("Configuration saved!");
    println!();
    Ok(())
}
pub fn show_session(s: &SessionStore) {
    if s.entries.is_empty() {
        println!("  {}", "No sessions recorded yet.".dimmed());
        return;
    }
    head("◆".bright_cyan(), "Session History");
    for (i, e) in s.entries.iter().rev().take(10).enumerate() {
        println!(
            "  {} {}. {} {}",
            " ".dimmed(),
            (i + 1).to_string().bright_blue(),
            e.prompt.chars().take(60).collect::<String>().dimmed(),
            format!("[{}]", e.difficulty).dimmed()
        );
    }
    println!();
}
pub fn show_preferences(p: &UserPreferences) {
    head("◆".bright_yellow(), "Preferences");
    println!(
        "  {} {}",
        "  Track Usage:".dimmed(),
        format!("{}", p.track_usage()).bright_white()
    );
    println!();
}
pub fn show_stats(s: &StatsTracker) {
    let u = s.usage();
    if u.total_prompts == 0 {
        println!("  {}", "No usage statistics yet.".dimmed());
        return;
    }
    head("◆".bright_green(), "Usage Statistics");
    println!(
        "  {} {} {}",
        "  Total prompts:".dimmed(),
        "·".dimmed(),
        u.total_prompts.to_string().bright_white()
    );
    breakdown(
        "  By provider:",
        u.provider_usage
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect(),
        true,
    );
    breakdown(
        "  By date:",
        u.daily_usage.iter().map(|(k, v)| (k.clone(), *v)).collect(),
        false,
    );
    println!();
}
fn breakdown(t: &str, mut v: Vec<(String, u64)>, pv: bool) {
    if v.is_empty() {
        return;
    }
    println!();
    println!("  {}", t.dimmed());
    if pv {
        v.sort_by(|a, b| b.1.cmp(&a.1));
    } else {
        v.sort_by(|a, b| b.0.cmp(&a.0));
    }
    for (k, n) in v.iter().take(10) {
        let kk = if pv {
            k.bright_cyan()
        } else {
            k.bright_white()
        };
        println!(
            "  {} {} {} {}",
            " ".dimmed(),
            kk,
            "·".dimmed(),
            n.to_string().bright_white()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    #[test]
    fn show_stats_handles_empty_tracker() {
        let d = TempDir::new().unwrap();
        show_stats(&StatsTracker::from_path(d.path().join("stats.json")));
    }
}
