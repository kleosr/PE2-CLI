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
        .map_err(|error| CliError::Network(error.to_string()))
}

pub fn validate_base_url(url: &str) -> Result<(), CliError> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(())
    } else {
        Err(CliError::Validation(format!("Invalid base URL: {url}")))
    }
}

pub fn validate_model_id(model: &str) -> Result<(), CliError> {
    let ok = model
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_' | '/'));
    if ok {
        Ok(())
    } else {
        Err(CliError::Validation(format!("Invalid model id: {model}")))
    }
}

pub fn need_key<'a>(key: &'a Option<String>, provider: &str) -> Result<&'a str, CliError> {
    key.as_deref()
        .ok_or_else(|| CliError::Auth(format!("{provider} API key is required")))
}

pub fn ptr(body: &serde_json::Value, path: &str, provider: &str) -> Result<String, CliError> {
    body.pointer(path)
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .ok_or_else(|| CliError::Provider {
            provider: provider.to_string(),
            message: "Empty response from model".to_string(),
        })
}

pub fn provider_error_message(body: &serde_json::Value) -> String {
    if let Some(message) = body["error"]["message"].as_str() {
        return message.to_string();
    }
    match serde_json::to_string(body) {
        Ok(raw) => format!("Provider error: {raw}"),
        Err(_) => "Provider returned an error without a message".to_string(),
    }
}

pub fn check_success(
    status: StatusCode,
    body: &serde_json::Value,
    provider: &str,
) -> Result<(), CliError> {
    if status.is_success() {
        Ok(())
    } else {
        Err(CliError::Provider {
            provider: provider.to_string(),
            message: provider_error_message(body),
        })
    }
}

pub async fn post_json(
    client: &reqwest::Client,
    url: &str,
    headers: HeaderMap,
    body: &serde_json::Value,
    provider: &str,
) -> Result<(StatusCode, serde_json::Value), CliError> {
    let response = client
        .post(url)
        .headers(headers)
        .json(body)
        .send()
        .await
        .map_err(|error| CliError::Network(error.to_string()))?;
    parse_json_response(response, provider).await
}

pub fn parse_json_body(
    status: StatusCode,
    bytes: &[u8],
    provider: &str,
) -> Result<(StatusCode, serde_json::Value), CliError> {
    match serde_json::from_slice(bytes) {
        Ok(body) => Ok((status, body)),
        Err(_) => {
            let preview: String = String::from_utf8_lossy(bytes).chars().take(200).collect();
            Err(CliError::Provider {
                provider: provider.to_string(),
                message: format!("Non-JSON response (HTTP {}): {preview}", status.as_u16()),
            })
        }
    }
}

pub async fn parse_json_response(
    response: reqwest::Response,
    provider: &str,
) -> Result<(StatusCode, serde_json::Value), CliError> {
    let status = response.status();
    let bytes = response
        .bytes()
        .await
        .map_err(|error| CliError::Network(error.to_string()))?;
    if bytes.len() > MAX_RESPONSE_BYTES {
        return Err(CliError::Provider {
            provider: provider.to_string(),
            message: format!("Response body exceeds {MAX_RESPONSE_BYTES} bytes"),
        });
    }
    parse_json_body(status, &bytes, provider)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_json_body_accepts_valid_json() {
        let (status, body) = parse_json_body(StatusCode::OK, br#"{"ok":true}"#, "test").unwrap();
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["ok"], true);
    }

    #[test]
    fn parse_json_body_rejects_non_json() {
        let error = parse_json_body(StatusCode::BAD_GATEWAY, b"not-json", "test").unwrap_err();
        match error {
            CliError::Provider { provider, message } => {
                assert_eq!(provider, "test");
                assert!(message.contains("Non-JSON response"));
                assert!(message.contains("502"));
            }
            other => panic!("expected Provider error, got {other}"),
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
