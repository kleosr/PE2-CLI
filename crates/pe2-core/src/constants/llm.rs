pub const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";
pub const DEFAULT_PROVIDER: &str = "openrouter";
pub const LLM_MAX_TOKENS: u32 = 1024;
pub const LLM_TEMPERATURE: f64 = 0.3;
pub const LLM_SYSTEM_MESSAGE: &str =
    "You are a precise prompt optimizer. Follow the instructions and return JSON only.";
pub const LLM_REFINEMENT_SYSTEM_MESSAGE: &str =
    "You are a precise prompt optimizer. Return JSON only.";
pub const REQUEST_TIMEOUT_MS: u64 = 30000;
