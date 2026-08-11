use crate::client::ProviderConfig;
use crate::http::{build_http_client, check_success, post_json, validate_base_url};
use async_trait::async_trait;
use pe2_core::engine::Message;
use pe2_core::engine::{ChatOptions, EngineLlmProvider};
use pe2_core::errors::CliError;

pub struct OllamaClient {
    client: reqwest::Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| "http://localhost:11434".to_string());
        validate_base_url(&base_url)?;
        Ok(Self {
            client: build_http_client()?,
            base_url,
        })
    }
}

fn extract_content(json: &serde_json::Value) -> Result<String, CliError> {
    json["message"]["content"]
        .as_str()
        .ok_or_else(|| CliError::Provider {
            provider: "ollama".to_string(),
            message: "Empty response from model".to_string(),
        })
        .map(str::to_string)
}

#[async_trait]
impl EngineLlmProvider for OllamaClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        options: &ChatOptions,
    ) -> Result<String, CliError> {
        let mut body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": false,
        });
        if options.max_tokens > 0 {
            body["options"] = serde_json::json!({
                "num_predict": options.max_tokens,
                "temperature": options.temperature,
            });
        }
        let url = format!("{}/api/chat", self.base_url);
        let (status, json) = post_json(
            &self.client,
            &url,
            reqwest::header::HeaderMap::new(),
            &body,
            "ollama",
        )
        .await?;
        check_success(status, &json, "ollama")?;
        extract_content(&json)
    }
}
