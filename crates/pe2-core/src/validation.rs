use crate::constants;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlashCommand {
    Help,
    Config,
    Session,
    Prefs,
    Stats,
    Clear,
    Exit,
}
const SLASH_TOKENS: &[(&str, SlashCommand)] = &[
    ("/help", SlashCommand::Help),
    ("/h", SlashCommand::Help),
    ("/config", SlashCommand::Config),
    ("/c", SlashCommand::Config),
    ("/session", SlashCommand::Session),
    ("/s", SlashCommand::Session),
    ("/prefs", SlashCommand::Prefs),
    ("/p", SlashCommand::Prefs),
    ("/stats", SlashCommand::Stats),
    ("/clear", SlashCommand::Clear),
    ("/exit", SlashCommand::Exit),
    ("/quit", SlashCommand::Exit),
    ("/q", SlashCommand::Exit),
];
pub fn resolve_slash_command(i: &str) -> Option<SlashCommand> {
    let t = i.split_whitespace().next()?;
    SLASH_TOKENS.iter().find(|(n, _)| *n == t).map(|(_, c)| *c)
}
pub fn validate_prompt(p: &str) -> Option<String> {
    let t = p.trim();
    if t.is_empty() {
        return Some("Prompt cannot be empty.".to_string());
    }
    if t.len() < constants::PROMPT_MIN_LENGTH {
        return Some(format!(
            "Prompt too short ({} chars). Minimum {} characters.",
            t.len(),
            constants::PROMPT_MIN_LENGTH
        ));
    }
    if t.len() > constants::PROMPT_MAX_LENGTH {
        return Some(format!(
            "Prompt too long ({} chars). Maximum {} characters.",
            t.len(),
            constants::PROMPT_MAX_LENGTH
        ));
    }
    None
}
pub fn suggest_slash_command(i: &str) -> Option<&'static str> {
    let t = i.split_whitespace().next()?;
    if !t.starts_with('/') || resolve_slash_command(i).is_some() {
        return None;
    }
    SLASH_TOKENS
        .iter()
        .map(|(n, _)| *n)
        .min_by_key(|k| sim(t, k))
}
pub fn unknown_command_message(i: &str) -> Option<String> {
    let t = i.split_whitespace().next()?;
    let h = suggest_slash_command(i)?;
    Some(format!(
        "Unknown command: {t}. Did you mean {h}? Type /help for available commands."
    ))
}
fn sim(a: &str, b: &str) -> usize {
    if a.len() != b.len() {
        return a.len().abs_diff(b.len()) + 5;
    }
    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_prompt() {
        assert!(validate_prompt("").is_some());
    }
    #[test]
    fn test_short_prompt() {
        assert!(validate_prompt(&"x".repeat(constants::PROMPT_MIN_LENGTH - 1)).is_some());
    }
    #[test]
    fn test_valid_prompt() {
        assert!(validate_prompt(&"x".repeat(constants::PROMPT_MIN_LENGTH)).is_none());
    }
    #[test]
    fn test_long_prompt() {
        assert!(validate_prompt(&"x".repeat(constants::PROMPT_MAX_LENGTH + 1)).is_some());
    }
    #[test]
    fn test_command_validation() {
        assert_eq!(suggest_slash_command("/help"), None);
    }
    #[test]
    fn test_tui_aliases_are_known() {
        for c in ["/h", "/c", "/s", "/p", "/q"] {
            assert_eq!(suggest_slash_command(c), None, "expected {c} to be valid");
        }
    }
    #[test]
    fn unknown_slash_command_suggests_closest_token() {
        assert!(suggest_slash_command("/setings").is_some());
        assert_eq!(suggest_slash_command("plain prompt"), None);
        let m = unknown_command_message("/setings").expect("message for unknown command");
        assert!(m.contains("Did you mean"));
        assert_eq!(unknown_command_message("/help"), None);
    }
    #[test]
    fn resolve_maps_aliases_to_canonical_command() {
        assert_eq!(resolve_slash_command("/c"), Some(SlashCommand::Config));
        assert_eq!(resolve_slash_command("/quit"), Some(SlashCommand::Exit));
        assert_eq!(resolve_slash_command("/nope"), None);
    }
}
