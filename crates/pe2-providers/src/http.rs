use pe2_core::constants;
use pe2_core::errors::CliError;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;

pub const MAX_RESPONSE_BYTES: usize = 10 * 1024 * 1024;

pub fn build_http_client() -> Result<reqwest::Client, CliError> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(constants::REQUEST_TIMEOUT_MS))
        .build()
        .map_err(|e| CliError::Network(e.to_string()))
}

pub fn validate_base_url(base_url: &str) -> Result<(), CliError> {
    if base_url.starts_with("http://") || base_url.starts_with("https://") {
        Ok(())
    } else {
        Err(CliError::Validation(format!("Invalid base URL: {base_url}")))
    }
}

pub fn validate_model_id(model: &str) -> Result<(), CliError> {
    if model
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '/'))
    {
        Ok(())
    } else {
        Err(CliError::Validation(format!("Invalid model id: {model}")))
    }
}

pub fn provider_error_message(json: &serde_json::Value) -> String {
    if let Some(msg) = json["error"]["message"].as_str() {
        return msg.to_string();
    }
    if let Ok(raw) = serde_json::to_string(json) {
        return format!("Provider error: {raw}");
    }
    "Provider returned an error without a message".to_string()
}

pub fn check_success(
    status: StatusCode,
    json: &serde_json::Value,
    provider: &str,
) -> Result<(), CliError> {
    if status.is_success() {
        Ok(())
    } else {
        Err(CliError::Provider {
            provider: provider.to_string(),
            message: provider_error_message(json),
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
        .map_err(|e| CliError::Network(e.to_string()))?;
    parse_json_response(response, provider).await
}

pub fn parse_json_body(
    status: StatusCode,
    text: &str,
    provider: &str,
) -> Result<(StatusCode, serde_json::Value), CliError> {
    let json: serde_json::Value = serde_json::from_str(text).map_err(|_| CliError::Provider {
        provider: provider.to_string(),
        message: format!(
            "Non-JSON response (HTTP {}): {}",
            status.as_u16(),
            text.chars().take(200).collect::<String>()
        ),
    })?;
    Ok((status, json))
}

pub async fn parse_json_response(
    response: reqwest::Response,
    provider: &str,
) -> Result<(StatusCode, serde_json::Value), CliError> {
    let status = response.status();
    let bytes = response
        .bytes()
        .await
        .map_err(|e| CliError::Network(e.to_string()))?;
    if bytes.len() > MAX_RESPONSE_BYTES {
        return Err(CliError::Provider {
            provider: provider.to_string(),
            message: format!("Response body exceeds {} bytes", MAX_RESPONSE_BYTES),
        });
    }
    let text = String::from_utf8_lossy(&bytes);
    parse_json_body(status, &text, provider)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    #[test]
    fn parse_json_body_accepts_valid_json() {
        let (status, json) =
            parse_json_body(StatusCode::OK, r#"{"ok":true}"#, "test").unwrap();
        assert_eq!(status, StatusCode::OK);
        assert_eq!(json["ok"], true);
    }

    #[test]
    fn parse_json_body_rejects_non_json() {
        let err = parse_json_body(StatusCode::BAD_GATEWAY, "not-json", "test").unwrap_err();
        match err {
            CliError::Provider { provider, message } => {
                assert_eq!(provider, "test");
                assert!(message.contains("Non-JSON response"));
                assert!(message.contains("502"));
            }
            other => assert!(false, "expected Provider error, got {other}"),
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
