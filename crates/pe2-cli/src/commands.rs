use colored::Colorize;
use pe2_core::config::{self, Config};
use pe2_core::constants;
use pe2_core::errors::CliError;
use pe2_core::preferences::UserPreferences;

pub fn cmd_config() -> Result<(), CliError> {
    let mut cfg = config::load_config();
    println!();
    println!("  {} {}", "◆".bright_cyan(), "Configuration".bright_white().bold());
    println!("  {} {}", "  Provider:", cfg.provider.bright_cyan());
    println!("  {} {}", "  Model:", cfg.model.bright_white());
    println!("  {} {}", "  API Key:", mask_key(cfg.api_key.as_deref()));
    println!();
    Ok(())
}

pub fn cmd_show_session() -> Result<(), CliError> {
    println!();
    println!("  {} {}", "◆".bright_cyan(), "Session Info".bright_white().bold());
    println!("  {} {}", "  Sessions Dir:", config::sessions_dir_path().display());
    println!("  {} {}", "  Config File:", config::config_file_path().display());
    println!();
    Ok(())
}

pub fn cmd_show_preferences() -> Result<(), CliError> {
    let prefs = UserPreferences::load();
    println!();
    println!("  {} {}", "◆".bright_yellow(), "Preferences".bright_white().bold());
    println!("  {} {}", "  Theme:", prefs.theme.bright_white());
    println!("  {} {}", "  Compact:", format!("{}", prefs.compact).bright_white());
    println!("  {} {}", "  Track Usage:", format!("{}", prefs.track_usage).bright_white());
    println!();
    Ok(())
}

pub fn cmd_help() -> Result<(), CliError> {
    println!(r#"
  Usage: pe2 [OPTIONS] [PROMPT]
         pe2 --config

  Arguments:
    [PROMPT]  Raw prompt text or path to prompt file

  Options:
    -p, --provider <PROVIDER>    LLM provider [openai, anthropic, google, openrouter, ollama]
    -m, --model <MODEL>          Model identifier
    --api-key <KEY>              API key
    -o, --output-file <FILE>     Output file path
    -i, --iterations <N>         Override refinement iterations
    --auto-difficulty            Enable auto-difficulty detection
    --max-tokens <N>             Max tokens for LLM [default: 1024]
    --temperature <F>            Temperature [default: 0.3]
    --config                     Show interactive configuration
    -h, --help                   Print help
    -V, --version                Print version
"#);
    Ok(())
}

fn mask_key(key: Option<&str>) -> String {
    match key {
        Some(k) if k.len() > 12 => {
            let visible = &k[..4];
            format!("{}...{}", visible, "****".dimmed())
        }
        Some(k) if !k.is_empty() => {
            format!("{} (short key)", "****".dimmed())
        }
        _ => "not set".dimmed().to_string(),
    }
}
