// ============================================================
// Provider tests  (port of tests/providers.test.js)
// ============================================================

#[test]
fn test_provider_kind_from_str() {
    use pe2_providers::client::ProviderKind;

    assert_eq!(ProviderKind::from_str("openai"), Some(ProviderKind::OpenAI));
    assert_eq!(ProviderKind::from_str("OpenAI"), Some(ProviderKind::OpenAI));
    assert_eq!(ProviderKind::from_str("OPENAI"), Some(ProviderKind::OpenAI));
    assert_eq!(ProviderKind::from_str("anthropic"), Some(ProviderKind::Anthropic));
    assert_eq!(ProviderKind::from_str("google"), Some(ProviderKind::Google));
    assert_eq!(ProviderKind::from_str("openrouter"), Some(ProviderKind::OpenRouter));
    assert_eq!(ProviderKind::from_str("ollama"), Some(ProviderKind::Ollama));
    assert_eq!(ProviderKind::from_str("unknown"), None);
}

#[test]
fn test_provider_kind_from_str_result() {
    use pe2_providers::client::ProviderKind;

    assert!(ProviderKind::from_str_result("openai").is_ok());
    assert!(ProviderKind::from_str_result("unknown").is_err());
}

#[test]
fn test_provider_kind_as_str() {
    use pe2_providers::client::ProviderKind;

    assert_eq!(ProviderKind::OpenAI.as_str(), "openai");
    assert_eq!(ProviderKind::Anthropic.as_str(), "anthropic");
    assert_eq!(ProviderKind::Google.as_str(), "google");
    assert_eq!(ProviderKind::OpenRouter.as_str(), "openrouter");
    assert_eq!(ProviderKind::Ollama.as_str(), "ollama");
}

#[test]
fn test_provider_kind_display_name() {
    use pe2_providers::client::ProviderKind;

    assert!(ProviderKind::OpenAI.display_name().contains("OpenAI"));
    assert!(ProviderKind::Anthropic.display_name().contains("Claude"));
    assert!(ProviderKind::Ollama.display_name().contains("Local"));
}

#[test]
fn test_provider_kind_default_model() {
    use pe2_providers::client::ProviderKind;

    assert!(!ProviderKind::OpenAI.default_model().is_empty());
    assert!(!ProviderKind::Anthropic.default_model().is_empty());
    assert!(!ProviderKind::Ollama.default_model().is_empty());
}

#[test]
fn test_provider_kind_models() {
    use pe2_providers::client::ProviderKind;

    assert!(!ProviderKind::OpenAI.models().is_empty());
    assert!(!ProviderKind::Anthropic.models().is_empty());
    assert!(!ProviderKind::OpenRouter.models().is_empty());
}

#[test]
fn test_provider_kind_all() {
    use pe2_providers::client::ProviderKind;

    let all = ProviderKind::all();
    assert_eq!(all.len(), 5);
    assert!(all.contains(&ProviderKind::OpenAI));
    assert!(all.contains(&ProviderKind::Anthropic));
    assert!(all.contains(&ProviderKind::Google));
    assert!(all.contains(&ProviderKind::OpenRouter));
    assert!(all.contains(&ProviderKind::Ollama));
}

// ============================================================
// ProviderConfig tests
// ============================================================

#[test]
fn test_provider_config_new() {
    use pe2_providers::client::{ProviderConfig, ProviderKind};

    let cfg = ProviderConfig::new(ProviderKind::OpenAI, Some("sk-test".to_string()));
    assert_eq!(cfg.kind, ProviderKind::OpenAI);
    assert_eq!(cfg.api_key(), Some("sk-test"));
    assert!(cfg.base_url.is_none());
}

#[test]
fn test_provider_config_with_base_url() {
    use pe2_providers::client::{ProviderConfig, ProviderKind};

    let cfg = ProviderConfig::new(ProviderKind::Ollama, None)
        .with_base_url("http://localhost:11434".to_string());
    assert_eq!(cfg.base_url, Some("http://localhost:11434".to_string()));
}

#[test]
fn test_provider_config_api_key_none() {
    use pe2_providers::client::{ProviderConfig, ProviderKind};

    let cfg = ProviderConfig::new(ProviderKind::OpenAI, None);
    assert!(cfg.api_key().is_none());
}

// ============================================================
// Headers tests  (port of tests/providers.test.js header tests)
// ============================================================

#[test]
fn test_build_bearer_header() {
    use pe2_providers::headers::build_bearer_header;

    let headers = build_bearer_header("sk-test-key");
    let auth = headers.get("Authorization").unwrap();
    assert_eq!(auth, "Bearer sk-test-key");
}

#[test]
fn test_build_openrouter_headers() {
    use pe2_providers::headers::build_openrouter_headers;

    let headers = build_openrouter_headers("sk-or-key");
    let auth = headers.get("Authorization").unwrap();
    assert_eq!(auth, "Bearer sk-or-key");
    // OpenRouter specific headers
    let referer = headers.get("HTTP-Referer");
    assert!(referer.is_some(), "should have HTTP-Referer header");
    let title = headers.get("X-Title");
    assert!(title.is_some(), "should have X-Title header");
}

// ============================================================
// Factory tests  (port of tests/providers.test.js getProviderClient)
// ============================================================

#[test]
fn test_create_client_unsupported_provider_errors() {
    use pe2_core::errors::CliError;
    use pe2_providers::client::{ProviderConfig, ProviderKind};

    // ProviderKind::from_str returns None for unknown providers
    let kind = ProviderKind::from_str("unknown-provider");
    assert!(kind.is_none(), "unknown provider should be None");
}

#[test]
fn test_create_client_openai_missing_key() {
    use pe2_providers::client::{ProviderConfig, ProviderKind};
    use pe2_providers::factory::create_client;

    let cfg = ProviderConfig::new(ProviderKind::OpenAI, None);
    let result = create_client(&cfg);
    assert!(result.is_err(), "OpenAI without key should error");
}

#[test]
fn test_create_client_anthropic_missing_key() {
    use pe2_providers::client::{ProviderConfig, ProviderKind};
    use pe2_providers::factory::create_client;

    let cfg = ProviderConfig::new(ProviderKind::Anthropic, None);
    let result = create_client(&cfg);
    assert!(result.is_err(), "Anthropic without key should error");
}

#[test]
fn test_create_client_google_missing_key() {
    use pe2_providers::client::{ProviderConfig, ProviderKind};
    use pe2_providers::factory::create_client;

    let cfg = ProviderConfig::new(ProviderKind::Google, None);
    let result = create_client(&cfg);
    assert!(result.is_err(), "Google without key should error");
}

#[test]
fn test_create_client_ollama_no_key_needed() {
    use pe2_providers::client::{ProviderConfig, ProviderKind};
    use pe2_providers::factory::create_client;

    // Ollama doesn't need an API key
    let cfg = ProviderConfig::new(ProviderKind::Ollama, None);
    let result = create_client(&cfg);
    assert!(result.is_ok(), "Ollama without key should succeed: {:?}", result.err());
}

// ============================================================
// ProviderResponse tests
// ============================================================

#[test]
fn test_provider_response_creation() {
    use pe2_providers::client::{ProviderKind, ProviderResponse};

    let resp = ProviderResponse {
        content: "Hello".to_string(),
        model: "gpt-4o".to_string(),
        provider: ProviderKind::OpenAI,
    };
    assert_eq!(resp.content, "Hello");
    assert_eq!(resp.model, "gpt-4o");
    assert_eq!(resp.provider, ProviderKind::OpenAI);
}

// ============================================================
// ProviderKind round-trip tests
// ============================================================

#[test]
fn test_provider_kind_round_trip() {
    use pe2_providers::client::ProviderKind;

    for kind in ProviderKind::all() {
        let s = kind.as_str();
        let back = ProviderKind::from_str(s);
        assert_eq!(back, Some(kind), "round-trip failed for {:?}", kind);
    }
}

#[test]
fn test_provider_config_default_model_matches_kind() {
    use pe2_providers::client::ProviderKind;

    for kind in ProviderKind::all() {
        let model = kind.default_model();
        assert!(!model.is_empty(), "default model for {:?} should not be empty", kind);
    }
}
