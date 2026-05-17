use async_trait::async_trait;
use pe2_core::constants;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse};
use crate::headers::build_openrouter_headers;

pub struct OpenRouterClient {
    client: reqwest::Client,
    api_key: String,
}

impl OpenRouterClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let api_key = config.api_key()
            .ok_or_else(|| CliError::Auth("OpenRouter API key is required".to_string()))?;
        Ok(Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
        })
    }
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
        let headers = build_openrouter_headers(&self.api_key);
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": max_tokens,
            "temperature": temperature,
        });

        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .headers(headers)
            .json(&body)
            .timeout(std::time::Duration::from_millis(constants::REQUEST_TIMEOUT_MS))
            .send()
            .await
            .map_err(|e| CliError::Network(e.to_string()))?;

        let status = response.status();
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CliError::Json(e))?;

        if !status.is_success() {
            let err_msg = json["error"]["message"].as_str().unwrap_or("Unknown error");
            return Err(CliError::Provider {
                provider: "openrouter".to_string(),
                message: err_msg.to_string(),
            });
        }

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
            provider: crate::client::ProviderKind::OpenRouter,
        })
    }
}
