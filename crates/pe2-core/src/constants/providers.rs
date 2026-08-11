pub fn provider_env_var(provider: &str) -> &'static str {
    match provider {
        "openai" => "OPENAI_API_KEY",
        "anthropic" => "ANTHROPIC_API_KEY",
        "google" => "GOOGLE_API_KEY",
        "openrouter" => "OPENROUTER_API_KEY",
        "ollama" => "OLLAMA_BASE_URL",
        _ => "OPENROUTER_API_KEY",
    }
}

pub fn default_model_for_provider(provider: &str) -> &'static str {
    match provider {
        "openai" => "gpt-4o-mini",
        "anthropic" => "claude-sonnet-4-20250514",
        "google" => "gemini-2.0-flash",
        "openrouter" => "openai/gpt-4o-mini",
        "ollama" => "llama3.2",
        _ => "openai/gpt-4o-mini",
    }
}

pub fn models_for_provider(provider: &str) -> &'static [&'static str] {
    match provider {
        "openai" => &["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-3.5-turbo"],
        "anthropic" => &[
            "claude-sonnet-4-20250514",
            "claude-3-5-sonnet-20241022",
            "claude-3-5-haiku-20241022",
        ],
        "google" => &[
            "gemini-2.0-flash",
            "gemini-2.0-pro",
            "gemini-1.5-pro",
            "gemini-1.5-flash",
        ],
        "openrouter" => &[
            "openai/gpt-4o",
            "openai/gpt-4o-mini",
            "openai/gpt-4-turbo",
            "anthropic/claude-sonnet-4-20250514",
            "anthropic/claude-3.5-sonnet",
            "google/gemini-2.0-flash-001",
            "google/gemini-2.0-pro",
        ],
        "ollama" => &[
            "llama3.2",
            "llama3.1",
            "mistral",
            "mixtral",
            "codellama",
            "phi4",
        ],
        _ => &["openai/gpt-4o-mini"],
    }
}
