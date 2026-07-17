use async_trait::async_trait;
use pe2_core::engine::{ChatOptions, EngineLlmProvider};
use pe2_core::errors::CliError;
use pe2_core::engine::Message;
use crate::client::ProviderConfig;
use crate::headers::build_openrouter_headers;
use crate::http::{build_http_client, check_success, post_json};

pub struct OpenRouterClient {
    client: reqwest::Client,
    api_key: String,
}

impl OpenRouterClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let api_key = config
            .api_key()
            .ok_or_else(|| CliError::Auth("OpenRouter API key is required".to_string()))?;
        Ok(Self {
            client: build_http_client()?,
            api_key: api_key.to_string(),
        })
    }
}

fn extract_content(json: &serde_json::Value) -> Result<String, CliError> {
    json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| CliError::Provider {
            provider: "openrouter".to_string(),
            message: "Empty response from model".to_string(),
        })
        .map(str::to_string)
}

#[async_trait]
impl EngineLlmProvider for OpenRouterClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        options: &ChatOptions,
    ) -> Result<String, CliError> {
        let headers = build_openrouter_headers(&self.api_key)?;
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": options.max_tokens,
            "temperature": options.temperature,
        });
        let (status, json) = post_json(
            &self.client,
            "https://openrouter.ai/api/v1/chat/completions",
            headers,
            &body,
            "openrouter",
        )
        .await?;
        check_success(status, &json, "openrouter")?;
        extract_content(&json)
    }
}
