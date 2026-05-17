use async_trait::async_trait;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum ProviderKind {
    OpenAI,
    Anthropic,
    Google,
    OpenRouter,
    Ollama,
}

impl ProviderKind {
    pub fn from_str_result(s: &str) -> Result<Self, CliError> {
        Self::from_str(s).ok_or_else(|| CliError::Config(format!("Unknown provider: {}", s)))
    }

    pub fn from_str(s: &str) -> Option<Self> {
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
        match self {
            ProviderKind::OpenAI => "gpt-4o-mini",
            ProviderKind::Anthropic => "claude-sonnet-4-20250514",
            ProviderKind::Google => "gemini-2.0-flash",
            ProviderKind::OpenRouter => "openai/gpt-4o-mini",
            ProviderKind::Ollama => "llama3.2",
        }
    }

    pub fn models(&self) -> &[&'static str] {
        match self {
            ProviderKind::OpenAI => &["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-3.5-turbo"],
            ProviderKind::Anthropic => &["claude-sonnet-4-20250514", "claude-3-5-sonnet-20241022", "claude-3-5-haiku-20241022"],
            ProviderKind::Google => &["gemini-2.0-flash", "gemini-2.0-pro", "gemini-1.5-pro", "gemini-1.5-flash"],
            ProviderKind::OpenRouter => &[
                "openai/gpt-4o", "openai/gpt-4o-mini", "openai/gpt-4-turbo",
                "anthropic/claude-sonnet-4-20250514", "anthropic/claude-3.5-sonnet",
                "google/gemini-2.0-flash-001", "google/gemini-2.0-pro",
            ],
            ProviderKind::Ollama => &["llama3.2", "llama3.1", "mistral", "mixtral", "codellama", "phi4"],
        }
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
