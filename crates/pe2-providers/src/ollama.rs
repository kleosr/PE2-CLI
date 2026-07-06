use async_trait::async_trait;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse, ProviderKind};
use crate::http::{build_http_client, post_json, validate_base_url};

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

fn extract_content(json: &serde_json::Value, model: &str) -> Result<ProviderResponse, CliError> {
    let content = json["message"]["content"]
        .as_str()
        .ok_or_else(|| CliError::Provider {
            provider: "ollama".to_string(),
            message: "Empty response from model".to_string(),
        })?
        .to_string();
    Ok(ProviderResponse {
        content,
        model: json["model"].as_str().unwrap_or(model).to_string(),
        provider: ProviderKind::Ollama,
    })
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        max_tokens: u32,
        temperature: f64,
    ) -> Result<ProviderResponse, CliError> {
        let mut body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": false,
        });
        if max_tokens > 0 {
            body["options"] = serde_json::json!({
                "num_predict": max_tokens,
                "temperature": temperature,
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
        if !status.is_success() {
            return Err(CliError::Provider {
                provider: "ollama".to_string(),
                message: format!("HTTP {}", status),
            });
        }
        extract_content(&json, model)
    }
}
