use async_trait::async_trait;
use pe2_core::constants;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse};

pub struct AnthropicClient {
    client: reqwest::Client,
    api_key: String,
}

impl AnthropicClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let api_key = config.api_key()
            .ok_or_else(|| CliError::Auth("Anthropic API key is required".to_string()))?;
        Ok(Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
        })
    }
}

fn extract_system(messages: &[Message]) -> (Option<String>, Vec<serde_json::Value>) {
    let mut system = None;
    let mut msgs = Vec::new();
    for msg in messages {
        if msg.role == "system" {
            system = Some(msg.content.clone());
        } else {
            msgs.push(serde_json::json!({
                "role": msg.role,
                "content": msg.content
            }));
        }
    }
    (system, msgs)
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        max_tokens: u32,
        temperature: f64,
    ) -> Result<ProviderResponse, CliError> {
        let (system_text, anthropic_messages) = extract_system(messages);

        let mut body = serde_json::json!({
            "model": model,
            "messages": anthropic_messages,
            "max_tokens": max_tokens,
            "temperature": temperature,
        });

        if let Some(sys) = system_text {
            body["system"] = serde_json::Value::String(sys);
        }

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
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
                provider: "anthropic".to_string(),
                message: err_msg.to_string(),
            });
        }

        let content = json["content"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|block| block["text"].as_str())
            .ok_or_else(|| CliError::Provider {
                provider: "anthropic".to_string(),
                message: "Empty response from model".to_string(),
            })?
            .to_string();

        Ok(ProviderResponse {
            content,
            model: model.to_string(),
            provider: crate::client::ProviderKind::Anthropic,
        })
    }
}
