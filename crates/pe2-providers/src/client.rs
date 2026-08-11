use pe2_core::errors::CliError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProviderKind {
    OpenAI,
    Anthropic,
    Google,
    OpenRouter,
    Ollama,
}

impl ProviderKind {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openai" => Some(ProviderKind::OpenAI),
            "anthropic" => Some(ProviderKind::Anthropic),
            "google" => Some(ProviderKind::Google),
            "openrouter" => Some(ProviderKind::OpenRouter),
            "ollama" => Some(ProviderKind::Ollama),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderKind::OpenAI => "openai",
            ProviderKind::Anthropic => "anthropic",
            ProviderKind::Google => "google",
            ProviderKind::OpenRouter => "openrouter",
            ProviderKind::Ollama => "ollama",
        }
    }

    pub fn default_model(&self) -> &'static str {
        pe2_core::constants::default_model_for_provider(self.as_str())
    }

    pub fn models(&self) -> &[&'static str] {
        pe2_core::constants::models_for_provider(self.as_str())
    }

    pub fn all() -> &'static [ProviderKind] {
        &[
            ProviderKind::OpenAI,
            ProviderKind::Anthropic,
            ProviderKind::Google,
            ProviderKind::OpenRouter,
            ProviderKind::Ollama,
        ]
    }
}

impl std::str::FromStr for ProviderKind {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| CliError::Config(format!("Unknown provider: {s}")))
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

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }
}
