use crate::client::{ProviderConfig, ProviderKind};
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
    k: ProviderKind,
    c: reqwest::Client,
    key: String,
    base: String,
}
impl Client {
    pub fn new(cfg: &ProviderConfig) -> Result<Self, CliError> {
        let base = cfg.base_url.clone().unwrap_or_else(|| match cfg.kind {
            ProviderKind::OpenAI => "https://api.openai.com/v1".to_string(),
            ProviderKind::Ollama => "http://localhost:11434".to_string(),
            _ => String::new(),
        });
        if matches!(cfg.kind, ProviderKind::OpenAI | ProviderKind::Ollama) {
            validate_base_url(&base)?;
        }
        let name = match cfg.kind {
            ProviderKind::OpenAI => "OpenAI",
            ProviderKind::Anthropic => "Anthropic",
            ProviderKind::Google => "Google",
            ProviderKind::OpenRouter => "OpenRouter",
            ProviderKind::Ollama => "Ollama",
        };
        let key = if cfg.kind == ProviderKind::Ollama {
            String::new()
        } else {
            need_key(&cfg.api_key, name)?.to_string()
        };
        Ok(Self {
            k: cfg.kind,
            c: build_http_client()?,
            key,
            base,
        })
    }
    fn req(
        &self,
        m: &str,
        ms: &[Message],
        o: &ChatOptions,
    ) -> Result<(String, serde_json::Value, &'static str), CliError> {
        match self.k {
            ProviderKind::OpenAI => Ok((
                format!("{}/chat/completions", self.base),
                oj(m, ms, o),
                "/choices/0/message/content",
            )),
            ProviderKind::OpenRouter => Ok((
                "https://openrouter.ai/api/v1/chat/completions".to_string(),
                oj(m, ms, o),
                "/choices/0/message/content",
            )),
            ProviderKind::Anthropic => Ok((
                "https://api.anthropic.com/v1/messages".to_string(),
                ab(m, ms, o),
                "/content/0/text",
            )),
            ProviderKind::Google => {
                validate_model_id(m)?;
                let t = ms
                    .iter()
                    .map(|x| format!("{}: {}", x.role, x.content))
                    .collect::<Vec<_>>()
                    .join("\n");
                let b = serde_json::json!({"contents": [{"parts": [{"text": t}]}], "generationConfig": {"temperature": o.temperature, "maxOutputTokens": o.max_tokens}});
                Ok((format!("https://generativelanguage.googleapis.com/v1beta/models/{m}:generateContent"), b, "/candidates/0/content/parts/0/text"))
            }
            ProviderKind::Ollama => {
                let mut b = serde_json::json!({"model": m, "messages": ms, "stream": false});
                if o.max_tokens > 0 {
                    b["options"] = serde_json::json!({"num_predict": o.max_tokens, "temperature": o.temperature});
                }
                Ok((format!("{}/api/chat", self.base), b, "/message/content"))
            }
        }
    }
}
fn oj(m: &str, ms: &[Message], o: &ChatOptions) -> serde_json::Value {
    serde_json::json!({"model": m, "messages": ms, "max_tokens": o.max_tokens, "temperature": o.temperature})
}
fn ab(m: &str, ms: &[Message], o: &ChatOptions) -> serde_json::Value {
    let mut sys = None;
    let mut v = Vec::new();
    for x in ms {
        if x.role == "system" {
            sys = Some(x.content.clone());
        } else {
            v.push(serde_json::json!({"role": x.role, "content": x.content}));
        }
    }
    let mut b = serde_json::json!({"model": m, "messages": v, "max_tokens": o.max_tokens, "temperature": o.temperature});
    if let Some(s) = sys {
        b["system"] = serde_json::Value::String(s);
    }
    b
}
fn hv(s: &str) -> Result<HeaderValue, CliError> {
    HeaderValue::from_str(s).map_err(|_| CliError::Auth("Invalid API key format".to_string()))
}
fn js(h: &mut HeaderMap) {
    h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
}
pub fn headers(k: ProviderKind, key: &str) -> Result<HeaderMap, CliError> {
    let mut h = HeaderMap::new();
    match k {
        ProviderKind::Anthropic => {
            h.insert(
                "x-api-key",
                hv(key)
                    .map_err(|_| CliError::Auth("Invalid Anthropic API key format".to_string()))?,
            );
            h.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
            js(&mut h);
        }
        ProviderKind::Google => {
            h.insert("x-goog-api-key", hv(key)?);
            js(&mut h);
        }
        ProviderKind::Ollama => {}
        _ => {
            h.insert(AUTHORIZATION, hv(&format!("Bearer {key}"))?);
            js(&mut h);
            if k == ProviderKind::OpenRouter {
                h.insert(REFERER, HeaderValue::from_static(constants::HTTP_REFERER));
                h.insert("X-Title", HeaderValue::from_static(constants::HTTP_TITLE));
            }
        }
    }
    Ok(h)
}
#[async_trait]
impl EngineLlmProvider for Client {
    async fn chat(&self, m: &str, ms: &[Message], o: &ChatOptions) -> Result<String, CliError> {
        let p = self.k.as_str();
        let (u, b, e) = self.req(m, ms, o)?;
        let (s, j) = post_json(&self.c, &u, headers(self.k, &self.key)?, &b, p).await?;
        check_success(s, &j, p)?;
        ptr(&j, e, p)
    }
}
