use crate::adapters::Client;
use pe2_core::engine::EngineLlmProvider;
use pe2_core::errors::CliError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderKind {
    OpenAI,
    Anthropic,
    Google,
    OpenRouter,
    Ollama,
}

const KINDS: &[(&str, ProviderKind)] = &[
    ("openai", ProviderKind::OpenAI),
    ("anthropic", ProviderKind::Anthropic),
    ("google", ProviderKind::Google),
    ("openrouter", ProviderKind::OpenRouter),
    ("ollama", ProviderKind::Ollama),
];

impl ProviderKind {
    pub fn parse(name: &str) -> Option<Self> {
        KINDS
            .iter()
            .find(|(label, _)| label.eq_ignore_ascii_case(name))
            .map(|(_, kind)| *kind)
    }

    pub fn as_str(self) -> &'static str {
        KINDS
            .iter()
            .find(|(_, kind)| *kind == self)
            .map(|(label, _)| *label)
            .unwrap_or("openrouter")
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::OpenAI => "OpenAI",
            Self::Anthropic => "Anthropic",
            Self::Google => "Google",
            Self::OpenRouter => "OpenRouter",
            Self::Ollama => "Ollama",
        }
    }
}

impl std::str::FromStr for ProviderKind {
    type Err = CliError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Self::parse(name).ok_or_else(|| CliError::Config(format!("Unknown provider: {name}")))
    }
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub kind: ProviderKind,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

impl ProviderConfig {
    pub fn new(kind: ProviderKind, api_key: Option<String>) -> Self {
        Self {
            kind,
            api_key,
            base_url: None,
        }
    }

    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = Some(url);
        self
    }
}

pub fn create_client(config: &ProviderConfig) -> Result<Box<dyn EngineLlmProvider>, CliError> {
    Ok(Box::new(Client::new(config)?))
}

pub(crate) fn default_base(kind: ProviderKind) -> String {
    match kind {
        ProviderKind::OpenAI => "https://api.openai.com/v1".to_string(),
        ProviderKind::Ollama => "http://localhost:11434".to_string(),
        _ => String::new(),
    }
}
