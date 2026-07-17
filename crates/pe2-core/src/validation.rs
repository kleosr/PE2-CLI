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

pub fn resolve_slash_command(input: &str) -> Option<SlashCommand> {
    let token = input.split_whitespace().next()?;
    match token {
        "/help" | "/h" => Some(SlashCommand::Help),
        "/config" | "/c" => Some(SlashCommand::Config),
        "/session" | "/s" => Some(SlashCommand::Session),
        "/prefs" | "/p" => Some(SlashCommand::Prefs),
        "/stats" => Some(SlashCommand::Stats),
        "/clear" => Some(SlashCommand::Clear),
        "/exit" | "/quit" | "/q" => Some(SlashCommand::Exit),
        _ => None,
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandValidation {
    NotCommand,
    Valid,
    Unknown {
        command: String,
        suggestion: Option<&'static str>,
    },
}

impl CommandValidation {
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }

    pub fn is_command(&self) -> bool {
        !matches!(self, Self::NotCommand)
    }

    pub fn suggestion(&self) -> Option<&'static str> {
        match self {
            Self::Unknown { suggestion, .. } => *suggestion,
            _ => None,
        }
    }
}

pub fn unknown_command_message(validation: &CommandValidation) -> Option<String> {
    match validation {
        CommandValidation::Unknown {
            command,
            suggestion: Some(hint),
        } => Some(format!(
            "Unknown command: {command}. Did you mean {hint}? Type /help for available commands."
        )),
        CommandValidation::Unknown { command, .. } => Some(format!(
            "Unknown command: {command}. Type /help for available commands."
        )),
        _ => None,
    }
}

pub fn validate_and_suggest_command(command: &str) -> CommandValidation {
    if !command
        .split_whitespace()
        .next()
        .is_some_and(|t| t.starts_with('/'))
    {
        return CommandValidation::NotCommand;
    }

    if resolve_slash_command(command).is_some() {
        return CommandValidation::Valid;
    }

    let Some(cmd) = command.split_whitespace().next().map(str::to_string) else {
        return CommandValidation::NotCommand;
    };
    let suggestion = known_slash_tokens()
        .iter()
        .min_by_key(|known| str_similarity(&cmd, known))
        .copied();

    CommandValidation::Unknown {
        command: cmd,
        suggestion,
    }
}

fn known_slash_tokens() -> &'static [&'static str] {
    &[
        "/help", "/h", "/config", "/c", "/session", "/s", "/prefs", "/p", "/stats", "/clear",
        "/exit", "/quit", "/q",
    ]
}

fn str_similarity(a: &str, b: &str) -> usize {
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
        let short = "x".repeat(constants::PROMPT_MIN_LENGTH - 1);
        assert!(validate_prompt(&short).is_some());
    }

    #[test]
    fn test_valid_prompt() {
        let valid = "x".repeat(constants::PROMPT_MIN_LENGTH);
        assert!(validate_prompt(&valid).is_none());
    }

    #[test]
    fn test_long_prompt() {
        let long = "x".repeat(constants::PROMPT_MAX_LENGTH + 1);
        assert!(validate_prompt(&long).is_some());
    }

    #[test]
    fn test_command_validation() {
        assert_eq!(
            validate_and_suggest_command("/help"),
            CommandValidation::Valid
        );
    }

    #[test]
    fn test_tui_aliases_are_known() {
        for cmd in ["/h", "/c", "/s", "/p", "/q"] {
            assert_eq!(
                validate_and_suggest_command(cmd),
                CommandValidation::Valid,
                "expected {cmd} to be valid"
            );
        }
    }

    #[test]
    fn resolve_maps_aliases_to_canonical_command() {
        assert_eq!(resolve_slash_command("/c"), Some(SlashCommand::Config));
        assert_eq!(resolve_slash_command("/quit"), Some(SlashCommand::Exit));
        assert_eq!(resolve_slash_command("/nope"), None);
    }
}
