use pe2_core::errors::CliError;
use reqwest::StatusCode;

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
    let text = response
        .text()
        .await
        .map_err(|e| CliError::Network(e.to_string()))?;
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
            other => panic!("expected Provider error, got {other}"),
        }
    }

    #[test]
    fn parse_json_body_accepts_empty_json_object() {
        let (_, json) = parse_json_body(StatusCode::OK, "{}", "test").unwrap();
        assert!(json.is_object());
    }
}
