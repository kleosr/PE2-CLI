use pe2_core::errors::CliError;
use crate::client::{LlmClient, ProviderConfig, ProviderKind};
use crate::openai::OpenAIClient;
use crate::anthropic::AnthropicClient;
use crate::google::GoogleClient;
use crate::ollama::OllamaClient;
use crate::openrouter::OpenRouterClient;

pub fn create_client(config: &ProviderConfig) -> Result<Box<dyn LlmClient>, CliError> {
    match config.kind {
        ProviderKind::OpenAI => Ok(Box::new(OpenAIClient::new(config)?)),
        ProviderKind::Anthropic => Ok(Box::new(AnthropicClient::new(config)?)),
        ProviderKind::Google => Ok(Box::new(GoogleClient::new(config)?)),
        ProviderKind::Ollama => Ok(Box::new(OllamaClient::new(config)?)),
        ProviderKind::OpenRouter => Ok(Box::new(OpenRouterClient::new(config)?)),
    }
}
