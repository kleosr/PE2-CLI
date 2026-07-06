use async_trait::async_trait;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse, ProviderKind};
use crate::headers::build_bearer_header;
use crate::http::{build_http_client, check_success, post_json, validate_base_url};

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

fn extract_content(json: &serde_json::Value, model: &str) -> Result<ProviderResponse, CliError> {
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| CliError::Provider {
            provider: "openai".to_string(),
            message: "Empty response from model".to_string(),
        })?
        .to_string();
    Ok(ProviderResponse {
        content,
        model: json["model"].as_str().unwrap_or(model).to_string(),
        provider: ProviderKind::OpenAI,
    })
}

#[async_trait]
impl LlmClient for OpenAIClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        max_tokens: u32,
        temperature: f64,
    ) -> Result<ProviderResponse, CliError> {
        let headers = build_bearer_header(&self.api_key)?;
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": max_tokens,
            "temperature": temperature,
        });
        let url = format!("{}/chat/completions", self.base_url);
        let (status, json) = post_json(&self.client, &url, headers, &body, "openai").await?;
        check_success(status, &json, "openai")?;
        extract_content(&json, model)
    }
}
