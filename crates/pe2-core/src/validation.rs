use crate::constants;

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

pub fn validate_and_suggest_command(command: &str) -> CommandValidation {
    let trimmed = command.trim().to_lowercase();
    if !trimmed.starts_with('/') {
        return CommandValidation {
            valid: false,
            is_command: false,
            message: String::new(),
            suggestion: None,
        };
    }

    let cmd = trimmed.split_whitespace().next().unwrap_or("");
    let known_commands = [
        "/help", "/config", "/settings", "/model", "/showkey",
        "/session", "/history", "/clear", "/theme", "/compact",
        "/preferences", "/copy", "/batch", "/import", "/export",
        "/exit", "/quit",
    ];

    if known_commands.contains(&cmd) {
        return CommandValidation {
            valid: true,
            is_command: true,
            message: String::new(),
            suggestion: None,
        };
    }

    // Find closest match
    let suggestion = known_commands
        .iter()
        .min_by_key(|known| str_similarity(cmd, known))
        .copied();

    CommandValidation {
        valid: false,
        is_command: true,
        message: format!("Unknown command: {}. Type /help for available commands.", cmd),
        suggestion,
    }
}

#[derive(Debug, Clone)]
pub struct CommandValidation {
    pub valid: bool,
    pub is_command: bool,
    pub message: String,
    pub suggestion: Option<&'static str>,
}

fn str_similarity(a: &str, b: &str) -> usize {
    if a.len() != b.len() {
        return a.len().abs_diff(b.len()) + 5;
    }
    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}

pub fn parse_slash_command(input: &str) -> Option<&str> {
    let trimmed = input.trim();
    if trimmed.starts_with('/') {
        Some(trimmed.split_whitespace().next().unwrap_or(""))
    } else {
        None
    }
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
        let r = validate_and_suggest_command("/help");
        assert!(r.valid);
    }
}
