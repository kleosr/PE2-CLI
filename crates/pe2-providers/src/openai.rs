use crate::client::ProviderConfig;
use crate::headers::build_bearer_header;
use crate::http::{build_http_client, check_success, post_json, validate_base_url};
use async_trait::async_trait;
use pe2_core::engine::Message;
use pe2_core::engine::{ChatOptions, EngineLlmProvider};
use pe2_core::errors::CliError;

pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl OpenAIClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let api_key = config
            .api_key()
            .ok_or_else(|| CliError::Auth("OpenAI API key is required".to_string()))?;
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
        validate_base_url(&base_url)?;
        Ok(Self {
            client: build_http_client()?,
            api_key: api_key.to_string(),
            base_url,
        })
    }
}

fn extract_content(json: &serde_json::Value) -> Result<String, CliError> {
    json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| CliError::Provider {
            provider: "openai".to_string(),
            message: "Empty response from model".to_string(),
        })
        .map(str::to_string)
}

#[async_trait]
impl EngineLlmProvider for OpenAIClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        options: &ChatOptions,
    ) -> Result<String, CliError> {
        let headers = build_bearer_header(&self.api_key)?;
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": options.max_tokens,
            "temperature": options.temperature,
        });
        let url = format!("{}/chat/completions", self.base_url);
        let (status, json) = post_json(&self.client, &url, headers, &body, "openai").await?;
        check_success(status, &json, "openai")?;
        extract_content(&json)
    }
}
