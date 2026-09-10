use pe2_core::constants;
use pe2_core::errors::CliError;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;

pub const MAX_RESPONSE_BYTES: usize = 10 * 1024 * 1024;

pub fn build_http_client() -> Result<reqwest::Client, CliError> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(
            constants::REQUEST_TIMEOUT_MS,
        ))
        .build()
        .map_err(|e| CliError::Network(e.to_string()))
}
pub fn validate_base_url(u: &str) -> Result<(), CliError> {
    (u.starts_with("http://") || u.starts_with("https://"))
        .then_some(())
        .ok_or_else(|| CliError::Validation(format!("Invalid base URL: {u}")))
}
pub fn validate_model_id(m: &str) -> Result<(), CliError> {
    m.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '/'))
        .then_some(())
        .ok_or_else(|| CliError::Validation(format!("Invalid model id: {m}")))
}
pub fn need_key<'a>(k: &'a Option<String>, p: &str) -> Result<&'a str, CliError> {
    k.as_deref()
        .ok_or_else(|| CliError::Auth(format!("{p} API key is required")))
}
pub fn ptr(j: &serde_json::Value, p: &str, pv: &str) -> Result<String, CliError> {
    j.pointer(p)
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| CliError::Provider {
            provider: pv.to_string(),
            message: "Empty response from model".to_string(),
        })
}
pub fn provider_error_message(j: &serde_json::Value) -> String {
    if let Some(m) = j["error"]["message"].as_str() {
        return m.to_string();
    }
    serde_json::to_string(j)
        .map(|r| format!("Provider error: {r}"))
        .unwrap_or_else(|_| "Provider returned an error without a message".to_string())
}
pub fn check_success(s: StatusCode, j: &serde_json::Value, p: &str) -> Result<(), CliError> {
    s.is_success()
        .then_some(())
        .ok_or_else(|| CliError::Provider {
            provider: p.to_string(),
            message: provider_error_message(j),
        })
}
pub async fn post_json(
    c: &reqwest::Client,
    u: &str,
    h: HeaderMap,
    b: &serde_json::Value,
    p: &str,
) -> Result<(StatusCode, serde_json::Value), CliError> {
    let r = c
        .post(u)
        .headers(h)
        .json(b)
        .send()
        .await
        .map_err(|e| CliError::Network(e.to_string()))?;
    parse_json_response(r, p).await
}
pub fn parse_json_body(
    s: StatusCode,
    b: &[u8],
    p: &str,
) -> Result<(StatusCode, serde_json::Value), CliError> {
    serde_json::from_slice(b)
        .map(|j| (s, j))
        .map_err(|_| CliError::Provider {
            provider: p.to_string(),
            message: format!(
                "Non-JSON response (HTTP {}): {}",
                s.as_u16(),
                String::from_utf8_lossy(b)
                    .chars()
                    .take(200)
                    .collect::<String>()
            ),
        })
}
pub async fn parse_json_response(
    r: reqwest::Response,
    p: &str,
) -> Result<(StatusCode, serde_json::Value), CliError> {
    let s = r.status();
    let b = r
        .bytes()
        .await
        .map_err(|e| CliError::Network(e.to_string()))?;
    if b.len() > MAX_RESPONSE_BYTES {
        return Err(CliError::Provider {
            provider: p.to_string(),
            message: format!("Response body exceeds {MAX_RESPONSE_BYTES} bytes"),
        });
    }
    parse_json_body(s, &b, p)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_json_body_accepts_valid_json() {
        let (s, j) = parse_json_body(StatusCode::OK, br#"{"ok":true}"#, "test").unwrap();
        assert_eq!(s, StatusCode::OK);
        assert_eq!(j["ok"], true);
    }
    #[test]
    fn parse_json_body_rejects_non_json() {
        let e = parse_json_body(StatusCode::BAD_GATEWAY, b"not-json", "test").unwrap_err();
        match e {
            CliError::Provider { provider, message } => {
                assert_eq!(provider, "test");
                assert!(message.contains("Non-JSON response"));
                assert!(message.contains("502"));
            }
            o => panic!("expected Provider error, got {o}"),
        }
    }
    #[test]
    fn validate_base_url_rejects_invalid_scheme() {
        assert!(validate_base_url("ftp://bad").is_err());
    }
    #[test]
    fn validate_model_id_rejects_spaces() {
        assert!(validate_model_id("bad model").is_err());
    }
}
