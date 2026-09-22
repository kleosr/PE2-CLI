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

pub fn resolve_slash_command(input: &str) -> Option<SlashCommand> {
    let token = input.split_whitespace().next()?;
    SLASH_TOKENS
        .iter()
        .find(|(name, _)| *name == token)
        .map(|(_, command)| *command)
}

pub fn validate_prompt(prompt: &str) -> Option<String> {
    let trimmed = prompt.trim();
    if trimmed.is_empty() {
        return Some("Prompt cannot be empty.".to_string());
    }
    if trimmed.len() < constants::PROMPT_MIN_LENGTH {
        return Some(format!(
            "Prompt too short ({} chars). Minimum {} characters.",
            trimmed.len(),
            constants::PROMPT_MIN_LENGTH
        ));
    }
    if trimmed.len() > constants::PROMPT_MAX_LENGTH {
        return Some(format!(
            "Prompt too long ({} chars). Maximum {} characters.",
            trimmed.len(),
            constants::PROMPT_MAX_LENGTH
        ));
    }
    None
}

pub fn suggest_slash_command(input: &str) -> Option<&'static str> {
    let token = input.split_whitespace().next()?;
    if !token.starts_with('/') || resolve_slash_command(input).is_some() {
        return None;
    }
    SLASH_TOKENS
        .iter()
        .map(|(name, _)| *name)
        .min_by_key(|candidate| edit_distance(token, candidate))
}

pub fn unknown_command_message(input: &str) -> Option<String> {
    let token = input.split_whitespace().next()?;
    let hint = suggest_slash_command(input)?;
    Some(format!(
        "Unknown command: {token}. Did you mean {hint}? Type /help for available commands."
    ))
}

fn edit_distance(left: &str, right: &str) -> usize {
    if left.len() != right.len() {
        return left.len().abs_diff(right.len()) + 5;
    }
    left.chars()
        .zip(right.chars())
        .filter(|(a, b)| a != b)
        .count()
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
        for command in ["/h", "/c", "/s", "/p", "/q"] {
            assert_eq!(
                suggest_slash_command(command),
                None,
                "expected {command} to be valid"
            );
        }
    }

    #[test]
    fn unknown_slash_command_suggests_closest_token() {
        assert!(suggest_slash_command("/setings").is_some());
        assert_eq!(suggest_slash_command("plain prompt"), None);
        let message = unknown_command_message("/setings").expect("message for unknown command");
        assert!(message.contains("Did you mean"));
        assert_eq!(unknown_command_message("/help"), None);
    }

    #[test]
    fn resolve_maps_aliases_to_canonical_command() {
        assert_eq!(resolve_slash_command("/c"), Some(SlashCommand::Config));
        assert_eq!(resolve_slash_command("/quit"), Some(SlashCommand::Exit));
        assert_eq!(resolve_slash_command("/nope"), None);
    }
}
