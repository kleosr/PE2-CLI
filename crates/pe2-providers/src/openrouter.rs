use async_trait::async_trait;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse, ProviderKind};
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

fn extract_content(json: &serde_json::Value, model: &str) -> Result<ProviderResponse, CliError> {
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| CliError::Provider {
            provider: "openrouter".to_string(),
            message: "Empty response from model".to_string(),
        })?
        .to_string();
    Ok(ProviderResponse {
        content,
        model: model.to_string(),
        provider: ProviderKind::OpenRouter,
    })
}

#[async_trait]
impl LlmClient for OpenRouterClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        max_tokens: u32,
        temperature: f64,
    ) -> Result<ProviderResponse, CliError> {
        let headers = build_openrouter_headers(&self.api_key)?;
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": max_tokens,
            "temperature": temperature,
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
        extract_content(&json, model)
    }
}
