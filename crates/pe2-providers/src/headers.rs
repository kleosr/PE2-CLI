use pe2_core::constants;
use pe2_core::errors::CliError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, REFERER};

fn bearer_value(api_key: &str) -> Result<HeaderValue, CliError> {
    HeaderValue::from_str(&format!("Bearer {api_key}"))
        .map_err(|_| CliError::Auth("Invalid API key format".to_string()))
}

pub fn build_openrouter_headers(api_key: &str) -> Result<HeaderMap, CliError> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, bearer_value(api_key)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(REFERER, HeaderValue::from_static(constants::HTTP_REFERER));
    headers.insert("X-Title", HeaderValue::from_static(constants::HTTP_TITLE));
    Ok(headers)
}

pub fn build_google_headers(api_key: &str) -> Result<HeaderMap, CliError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-goog-api-key",
        HeaderValue::from_str(api_key)
            .map_err(|_| CliError::Auth("Invalid API key format".to_string()))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(headers)
}

pub fn build_bearer_header(api_key: &str) -> Result<HeaderMap, CliError> {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, bearer_value(api_key)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(headers)
}

pub fn build_anthropic_headers(api_key: &str) -> Result<HeaderMap, CliError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-api-key",
        HeaderValue::from_str(api_key)
            .map_err(|_| CliError::Auth("Invalid Anthropic API key format".to_string()))?,
    );
    headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(headers)
}
