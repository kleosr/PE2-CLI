use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

/// Default model configuration
pub const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";
pub const DEFAULT_PROVIDER: &str = "openrouter";

/// LLM configuration
pub const LLM_MAX_TOKENS: u32 = 1024;
pub const LLM_TEMPERATURE: f64 = 0.3;
pub const LLM_SYSTEM_MESSAGE: &str =
    "You are a precise prompt optimizer. Follow the instructions and return JSON only.";
pub const LLM_REFINEMENT_SYSTEM_MESSAGE: &str =
    "You are a precise prompt optimizer. Return JSON only.";

/// Request timeout in milliseconds
pub const REQUEST_TIMEOUT_MS: u64 = 30000;

/// Prompt limits
pub const PROMPT_MIN_LENGTH: usize = 10;
pub const PROMPT_MAX_LENGTH: usize = 10000;
pub const PROMPT_CACHE_KEY_PREFIX_LENGTH: usize = 100;
pub const PROMPT_PREVIEW_MAX_LENGTH: usize = 200;
pub const PROMPT_PROCESSING_DISPLAY_MAX_LENGTH: usize = 100;
pub const API_KEY_DISPLAY_LENGTH: usize = 8;
pub const API_KEY_MASK_LENGTH: usize = 8;
pub const SHORT_API_KEY_THRESHOLD: usize = 12;
pub const SHORT_API_KEY_PREFIX: usize = 4;
pub const SHORT_API_KEY_SUFFIX: usize = 4;

/// Complexity thresholds
pub const COMPLEXITY_WORD_VERY_HIGH: usize = 400;
pub const COMPLEXITY_WORD_HIGH: usize = 250;
pub const COMPLEXITY_WORD_MEDIUM: usize = 120;
pub const COMPLEXITY_WORD_LOW: usize = 60;
pub const MAX_TECH_INDICATORS: usize = 4;
pub const MAX_DOMAIN_INDICATORS: usize = 3;
pub const MAX_STRUCTURAL_MATCHES: usize = 4;
pub const MAX_LOGIC_MATCHES: usize = 3;
pub const SPECIAL_CHARS_HIGH: usize = 5;
pub const SPECIAL_CHARS_MEDIUM: usize = 2;

/// Session configuration
pub const MAX_HISTORY_ITEMS: usize = 50;
pub const MAX_FAVORITE_MODELS: usize = 5;
pub const MAX_LAST_USED_COMMANDS: usize = 10;
pub const COMMAND_SUGGESTIONS_LIMIT: usize = 5;
pub const PRIORITY_COMMANDS_LIMIT: usize = 5;

/// File permissions (0o600)
pub const CONFIG_FILE_MODE: u32 = 0o600;

/// Performance metrics
pub const DEFAULT_QUALITY_SCORE: f64 = 8.0;
pub const COMPLEXITY_SCORE_MAX: u32 = 20;

/// Required prompt fields for PE2 structured output
pub const REQUIRED_PROMPT_FIELDS: &[&str] = &["context", "role", "task", "constraints", "output"];

/// HTTP headers for OpenRouter compatibility
pub const HTTP_REFERER: &str = "https://pe2-cli-tool.local";
pub const HTTP_TITLE: &str = "KleoSr PE2-CLI Tool";

/// Provider environment variable names
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

/// Default provider models
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

/// Lazy-compiled regex patterns for analysis
pub static TECH_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"\b(api|json|rest|graphql|grpc|restful)\b").unwrap(),
        Regex::new(r"\b(sql|nosql|database|postgres|mongo|redis|mysql)\b").unwrap(),
        Regex::new(r"\b(docker|kubernetes|k8s|container|orchestrat)\b").unwrap(),
        Regex::new(r"\b(aws|gcp|azure|cloud|deploy|serverless)\b").unwrap(),
        Regex::new(r"\b(microservice|distributed|message.queue|event.driven)\b").unwrap(),
        Regex::new(r"\b(auth|oauth|jwt|saml|oidc|authentication|authorization)\b").unwrap(),
        Regex::new(r"\b(testing|tdd|unit.test|integration.test|e2e|mock|assert)\b").unwrap(),
        Regex::new(r"\b(ci/cd|pipeline|devops|deploy|monitoring|observability)\b").unwrap(),
        Regex::new(r"\b(caching|redis|memcached|cdn|performance)\b").unwrap(),
        Regex::new(r"\b(security|encrypt|hash|ssl|tls|certificate)\b").unwrap(),
        Regex::new(r"\b(async|await|promise|callback|concurren|parallel|thread)\b").unwrap(),
        Regex::new(r"\b(stream|kafka|rabbitmq|pub.sub|event)\b").unwrap(),
    ]
});

pub static DOMAIN_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"\b(frontend|react|vue|angular|svelte|ui|ux)\b").unwrap(),
        Regex::new(r"\b(backend|server|node|express|fastapi|django|spring)\b").unwrap(),
        Regex::new(r"\b(data|analytics|machine.learning|ai|deep.learning)\b").unwrap(),
        Regex::new(r"\b(mobile|ios|android|flutter|react.native|swift)\b").unwrap(),
        Regex::new(r"\b(blockchain|web3|smart.contract|solidity|nft|defi)\b").unwrap(),
        Regex::new(r"\b(devops|sre|reliability|scalability|infrastructure)\b").unwrap(),
        Regex::new(r"\b(security|pen.test|vulnerability|compliance|audit)\b").unwrap(),
        Regex::new(r"\b(gaming|unity|unreal|3d|game.dev)\b").unwrap(),
        Regex::new(r"\b(embedded|iot|firmware|hardware|rtos)\b").unwrap(),
        Regex::new(r"\b(scientific|research|bioinformatics|computational)\b").unwrap(),
    ]
});

pub static STRUCTURAL_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\n\s*\d+\.|\n\s*\-|```|#)").unwrap());

pub static LOGIC_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(if|then|when|unless|until|depending|while)\b").unwrap());

pub static SPECIAL_CHARS_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[;\{\[]").unwrap());
