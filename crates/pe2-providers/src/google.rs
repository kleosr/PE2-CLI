use async_trait::async_trait;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::{LlmClient, ProviderConfig, ProviderResponse, ProviderKind};
use crate::http::{build_http_client, check_success, post_json, validate_model_id};
use crate::headers::build_google_headers;

pub struct GoogleClient {
    client: reqwest::Client,
    api_key: String,
}

impl GoogleClient {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let api_key = config
            .api_key()
            .ok_or_else(|| CliError::Auth("Google API key is required".to_string()))?;
        Ok(Self {
            client: build_http_client()?,
            api_key: api_key.to_string(),
        })
    }
}

fn flatten_messages(messages: &[Message]) -> String {
    messages
        .iter()
        .map(|msg| format!("{}: {}", msg.role, msg.content))
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_content(json: &serde_json::Value, model: &str) -> Result<ProviderResponse, CliError> {
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
        provider: ProviderKind::Google,
    })
}

#[async_trait]
impl LlmClient for GoogleClient {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        max_tokens: u32,
        temperature: f64,
    ) -> Result<ProviderResponse, CliError> {
        validate_model_id(model)?;
        let prompt_text = flatten_messages(messages);
        let body = serde_json::json!({
            "contents": [{
                "parts": [{ "text": prompt_text }]
            }],
            "generationConfig": {
                "temperature": temperature,
                "maxOutputTokens": max_tokens
            }
        });
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            model
        );
        let headers = build_google_headers(&self.api_key)?;
        let (status, json) = post_json(
            &self.client,
            &url,
            headers,
            &body,
            "google",
        )
        .await?;
        check_success(status, &json, "google")?;
        extract_content(&json, model)
    }
}
