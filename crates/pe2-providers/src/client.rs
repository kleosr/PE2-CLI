use crate::anthropic::AnthropicClient;
use crate::google::GoogleClient;
use crate::ollama::OllamaClient;
use crate::openai::OpenAIClient;
use crate::openrouter::OpenRouterClient;
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

pub fn create_client(config: &ProviderConfig) -> Result<Box<dyn EngineLlmProvider>, CliError> {
    match config.kind {
        ProviderKind::OpenAI => Ok(Box::new(OpenAIClient::new(config)?)),
        ProviderKind::Anthropic => Ok(Box::new(AnthropicClient::new(config)?)),
        ProviderKind::Google => Ok(Box::new(GoogleClient::new(config)?)),
        ProviderKind::Ollama => Ok(Box::new(OllamaClient::new(config)?)),
        ProviderKind::OpenRouter => Ok(Box::new(OpenRouterClient::new(config)?)),
    }
}
