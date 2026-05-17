use async_trait::async_trait;
use pe2_core::constants;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse};

pub struct GoogleClient {
    client: reqwest::Client,
    api_key: String,
}

impl GoogleClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let api_key = config.api_key()
            .ok_or_else(|| CliError::Auth("Google API key is required".to_string()))?;
        Ok(Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
        })
    }
}

fn flatten_messages(messages: &[Message]) -> String {
    let mut parts = Vec::new();
    for msg in messages {
        parts.push(format!("{}: {}", msg.role, msg.content));
    }
    parts.join("\n")
}

#[async_trait]
impl LlmClient for GoogleClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        _max_tokens: u32,
        temperature: f64,
    ) -> Result<ProviderResponse, CliError> {
        let prompt_text = flatten_messages(messages);

        let body = serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": prompt_text
                }]
            }],
            "generationConfig": {
                "temperature": temperature,
                "maxOutputTokens": 8192
            }
        });

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, self.api_key
        );

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
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
                provider: "google".to_string(),
                message: err_msg.to_string(),
            });
        }

        let content = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| CliError::Provider {
                provider: "google".to_string(),
                message: "Empty response from model".to_string(),
            })?
            .to_string();

        Ok(ProviderResponse {
            content,
            model: model.to_string(),
            provider: crate::client::ProviderKind::Google,
        })
    }
}
