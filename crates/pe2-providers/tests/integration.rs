
#[test]
fn test_provider_kind_from_str() {
    use pe2_providers::client::ProviderKind;

    assert_eq!(ProviderKind::parse("openai"), Some(ProviderKind::OpenAI));
    assert_eq!(ProviderKind::parse("OpenAI"), Some(ProviderKind::OpenAI));
    assert_eq!(ProviderKind::parse("OPENAI"), Some(ProviderKind::OpenAI));
    assert_eq!(ProviderKind::parse("anthropic"), Some(ProviderKind::Anthropic));
    assert_eq!(ProviderKind::parse("google"), Some(ProviderKind::Google));
    assert_eq!(ProviderKind::parse("openrouter"), Some(ProviderKind::OpenRouter));
    assert_eq!(ProviderKind::parse("ollama"), Some(ProviderKind::Ollama));
    assert_eq!(ProviderKind::parse("unknown"), None);
}

#[test]
fn test_provider_kind_parse_result() {
    use pe2_providers::client::ProviderKind;

    assert!("openai".parse::<ProviderKind>().is_ok());
    assert!("unknown".parse::<ProviderKind>().is_err());
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


#[test]
fn test_build_bearer_header() {
    use pe2_providers::headers::build_bearer_header;

    let headers = build_bearer_header("sk-test-key").unwrap();
    let auth = headers.get("Authorization").unwrap();
    assert_eq!(auth, "Bearer sk-test-key");
}

#[test]
fn test_build_openrouter_headers() {
    use pe2_providers::headers::build_openrouter_headers;

    let headers = build_openrouter_headers("sk-or-key").unwrap();
    let auth = headers.get("authorization").unwrap();
    assert_eq!(auth, "Bearer sk-or-key");
    let referer = headers.get("referer");
    assert!(referer.is_some(), "should have referer header");
    let title = headers.get("X-Title");
    assert!(title.is_some(), "should have X-Title header");
}


#[test]
fn test_create_client_unsupported_provider_errors() {
    use pe2_providers::client::ProviderKind;

    let kind = ProviderKind::parse("unknown-provider");
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

    let cfg = ProviderConfig::new(ProviderKind::Ollama, None);
    let result = create_client(&cfg);
    assert!(result.is_ok(), "Ollama without key should succeed: {:?}", result.err());
}


#[test]
fn test_build_anthropic_headers() {
    use pe2_providers::headers::build_anthropic_headers;

    let headers = build_anthropic_headers("sk-ant-key").unwrap();
    assert_eq!(headers.get("x-api-key").unwrap(), "sk-ant-key");
    assert!(headers.get("anthropic-version").is_some());
}

#[test]
fn test_provider_kind_round_trip() {
    use pe2_providers::client::ProviderKind;

    for kind in ProviderKind::all() {
        let s = kind.as_str();
        let back = ProviderKind::parse(s);
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
