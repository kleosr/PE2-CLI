use reqwest::header::{HeaderMap, HeaderValue, REFERER};
use pe2_core::constants;

pub fn build_openrouter_headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        REFERER,
        HeaderValue::from_static(constants::HTTP_REFERER),
    );
    headers.insert(
        "X-Title",
        HeaderValue::from_static(constants::HTTP_TITLE),
    );
    headers
}

pub fn build_bearer_header(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/json"),
    );
    headers
}
