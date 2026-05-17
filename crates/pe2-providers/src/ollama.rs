use async_trait::async_trait;
use pe2_core::constants;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse};

pub struct OllamaClient {
    client: reqwest::Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let base_url = config.base_url.clone()
            .unwrap_or_else(|| "http://localhost:11434".to_string());
        Ok(Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_millis(constants::REQUEST_TIMEOUT_MS))
                .build()
                .map_err(|e| CliError::Network(e.to_string()))?,
            base_url,
        })
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        _max_tokens: u32,
        _temperature: f64,
    ) -> Result<ProviderResponse, CliError> {
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": false,
        });

        let response = self.client
            .post(format!("{}/api/chat", self.base_url))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| CliError::Network(e.to_string()))?;

        let status = response.status();
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CliError::Json(e))?;

        if !status.is_success() {
            return Err(CliError::Provider {
                provider: "ollama".to_string(),
                message: format!("HTTP {}", status),
            });
        }

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
            provider: crate::client::ProviderKind::Ollama,
        })
    }
}
