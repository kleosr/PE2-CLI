use async_trait::async_trait;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse, ProviderKind};
use crate::http::{build_http_client, check_success, post_json};

pub struct AnthropicClient {
    client: reqwest::Client,
    api_key: String,
}

impl AnthropicClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let api_key = config
            .api_key()
            .ok_or_else(|| CliError::Auth("Anthropic API key is required".to_string()))?;
        Ok(Self {
            client: build_http_client()?,
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

fn build_body(
    model: &str,
    messages: &[Message],
    max_tokens: u32,
    temperature: f64,
) -> serde_json::Value {
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
    body
}

fn extract_content(json: &serde_json::Value, model: &str) -> Result<ProviderResponse, CliError> {
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
        provider: ProviderKind::Anthropic,
    })
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
        let body = build_body(model, messages, max_tokens, temperature);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", self.api_key.parse().map_err(|_| {
            CliError::Auth("Invalid Anthropic API key format".to_string())
        })?);
        headers.insert(
            "anthropic-version",
            reqwest::header::HeaderValue::from_static("2023-06-01"),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let (status, json) = post_json(
            &self.client,
            "https://api.anthropic.com/v1/messages",
            headers,
            &body,
            "anthropic",
        )
        .await?;
        check_success(status, &json, "anthropic")?;
        extract_content(&json, model)
    }
}
