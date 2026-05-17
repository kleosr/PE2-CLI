use async_trait::async_trait;
use pe2_core::constants;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse};
use crate::headers::build_bearer_header;

pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl OpenAIClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let api_key = config.api_key()
            .ok_or_else(|| CliError::Auth("OpenAI API key is required".to_string()))?;
        let base_url = config.base_url.clone().unwrap_or_else(|| "https://api.openai.com/v1".to_string());
        Ok(Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            base_url,
        })
    }
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
        let headers = build_bearer_header(&self.api_key);
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": max_tokens,
            "temperature": temperature,
        });

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
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
                provider: "openai".to_string(),
                message: err_msg.to_string(),
            });
        }

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| CliError::Provider {
                provider: "openai".to_string(),
                message: "Empty response from model".to_string(),
            })?
            .to_string();

        let model_used = json["model"].as_str().unwrap_or(model).to_string();

        Ok(ProviderResponse {
            content,
            model: model_used,
            provider: crate::client::ProviderKind::OpenAI,
        })
    }
}
