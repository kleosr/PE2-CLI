use crate::client::{default_base, ProviderConfig, ProviderKind};
use crate::http::{
    build_http_client, check_success, need_key, post_json, ptr, validate_base_url,
    validate_model_id,
};
use async_trait::async_trait;
use pe2_core::constants;
use pe2_core::engine::{ChatOptions, EngineLlmProvider, Message};
use pe2_core::errors::CliError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, REFERER};

pub struct Client {
    kind: ProviderKind,
    http: reqwest::Client,
    key: String,
    base: String,
}

impl Client {
    pub fn new(config: &ProviderConfig) -> Result<Self, CliError> {
        let base = config
            .base_url
            .clone()
            .unwrap_or_else(|| default_base(config.kind));
        if matches!(config.kind, ProviderKind::OpenAI | ProviderKind::Ollama) {
            validate_base_url(&base)?;
        }
        let key = if config.kind == ProviderKind::Ollama {
            String::new()
        } else {
            need_key(&config.api_key, config.kind.label())?.to_string()
        };
        Ok(Self {
            kind: config.kind,
            http: build_http_client()?,
            key,
            base,
        })
    }

    fn request(
        &self,
        model: &str,
        messages: &[Message],
        options: &ChatOptions,
    ) -> Result<(String, serde_json::Value, &'static str), CliError> {
        match self.kind {
            ProviderKind::OpenAI => Ok((
                format!("{}/chat/completions", self.base),
                openai_body(model, messages, options),
                "/choices/0/message/content",
            )),
            ProviderKind::OpenRouter => Ok((
                "https://openrouter.ai/api/v1/chat/completions".to_string(),
                openai_body(model, messages, options),
                "/choices/0/message/content",
            )),
            ProviderKind::Anthropic => Ok((
                "https://api.anthropic.com/v1/messages".to_string(),
                anthropic_body(model, messages, options),
                "/content/0/text",
            )),
            ProviderKind::Google => {
                validate_model_id(model)?;
                Ok((
                    format!(
                        "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
                    ),
                    google_body(messages, options),
                    "/candidates/0/content/parts/0/text",
                ))
            }
            ProviderKind::Ollama => Ok((
                format!("{}/api/chat", self.base),
                ollama_body(model, messages, options),
                "/message/content",
            )),
        }
    }
}

fn openai_body(model: &str, messages: &[Message], options: &ChatOptions) -> serde_json::Value {
    serde_json::json!({
        "model": model,
        "messages": messages,
        "max_tokens": options.max_tokens,
        "temperature": options.temperature
    })
}

fn anthropic_body(model: &str, messages: &[Message], options: &ChatOptions) -> serde_json::Value {
    let mut system = None;
    let mut turns = Vec::new();
    for message in messages {
        if message.role == "system" {
            system = Some(message.content.clone());
        } else {
            turns.push(serde_json::json!({"role": message.role, "content": message.content}));
        }
    }
    let mut body = serde_json::json!({
        "model": model,
        "messages": turns,
        "max_tokens": options.max_tokens,
        "temperature": options.temperature
    });
    if let Some(system) = system {
        body["system"] = serde_json::Value::String(system);
    }
    body
}

fn google_body(messages: &[Message], options: &ChatOptions) -> serde_json::Value {
    let text = messages
        .iter()
        .map(|message| format!("{}: {}", message.role, message.content))
        .collect::<Vec<_>>()
        .join("\n");
    serde_json::json!({
        "contents": [{"parts": [{"text": text}]}],
        "generationConfig": {
            "temperature": options.temperature,
            "maxOutputTokens": options.max_tokens
        }
    })
}

fn ollama_body(model: &str, messages: &[Message], options: &ChatOptions) -> serde_json::Value {
    let mut body = serde_json::json!({
        "model": model,
        "messages": messages,
        "stream": false
    });
    if options.max_tokens > 0 {
        body["options"] = serde_json::json!({
            "num_predict": options.max_tokens,
            "temperature": options.temperature
        });
    }
    body
}

fn header_value(value: &str) -> Result<HeaderValue, CliError> {
    HeaderValue::from_str(value).map_err(|_| CliError::Auth("Invalid API key format".to_string()))
}

fn json_content(headers: &mut HeaderMap) {
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
}

pub fn headers(kind: ProviderKind, key: &str) -> Result<HeaderMap, CliError> {
    let mut headers = HeaderMap::new();
    match kind {
        ProviderKind::Anthropic => {
            headers.insert(
                "x-api-key",
                header_value(key)
                    .map_err(|_| CliError::Auth("Invalid Anthropic API key format".to_string()))?,
            );
            headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
            json_content(&mut headers);
        }
        ProviderKind::Google => {
            headers.insert("x-goog-api-key", header_value(key)?);
            json_content(&mut headers);
        }
        ProviderKind::Ollama => {}
        ProviderKind::OpenAI | ProviderKind::OpenRouter => {
            headers.insert(AUTHORIZATION, header_value(&format!("Bearer {key}"))?);
            json_content(&mut headers);
            if kind == ProviderKind::OpenRouter {
                headers.insert(REFERER, HeaderValue::from_static(constants::HTTP_REFERER));
                headers.insert("X-Title", HeaderValue::from_static(constants::HTTP_TITLE));
            }
        }
    }
    Ok(headers)
}

#[async_trait]
impl EngineLlmProvider for Client {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        options: &ChatOptions,
    ) -> Result<String, CliError> {
        let provider = self.kind.as_str();
        let (url, body, pointer) = self.request(model, messages, options)?;
        let (status, json) = post_json(
            &self.http,
            &url,
            headers(self.kind, &self.key)?,
            &body,
            provider,
        )
        .await?;
        check_success(status, &json, provider)?;
        ptr(&json, pointer, provider)
    }
}
