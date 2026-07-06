use async_trait::async_trait;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
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
    pub fn from_str_result(s: &str) -> Result<Self, CliError> {
        s.parse()
    }

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

    pub fn display_name(&self) -> &'static str {
        match self {
            ProviderKind::OpenAI => "OpenAI",
            ProviderKind::Anthropic => "Anthropic (Claude)",
            ProviderKind::Google => "Google (Gemini)",
            ProviderKind::OpenRouter => "OpenRouter (Multi-Provider)",
            ProviderKind::Ollama => "Ollama (Local)",
        }
    }

    pub fn default_model(&self) -> &'static str {
        pe2_core::constants::default_model_for_provider(self.as_str())
    }

    pub fn models(&self) -> &[&'static str] {
        pe2_core::constants::models_for_provider(self.as_str())
    }

    pub fn all() -> Vec<ProviderKind> {
        vec![
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResponse {
    pub content: String,
    pub model: String,
    pub provider: ProviderKind,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        max_tokens: u32,
        temperature: f64,
    ) -> Result<ProviderResponse, CliError>;
}
