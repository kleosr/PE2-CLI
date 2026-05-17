use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================
// Analysis tests  (port of tests/analysis.test.js)
// ============================================================

#[test]
fn test_analysis_simple_prompt() {
    use pe2_core::analysis::analyze_prompt_complexity;
    let r = analyze_prompt_complexity("this is a simple prompt with enough words");
    assert!(r.score > 0, "score should be positive");
    assert!(r.difficulty as u32 >= 1, "difficulty should be set");
    assert!(r.iterations >= 1, "iterations should be at least 1");
}

#[test]
fn test_analysis_technical_keywords_increase_score() {
    use pe2_core::analysis::analyze_prompt_complexity;
    let plain = "word ".repeat(20);
    let technical = format!("{} python api docker ml algorithm framework database", plain.trim());
    let plain_result = analyze_prompt_complexity(&plain);
    let technical_result = analyze_prompt_complexity(&technical);
    assert!(
        technical_result.score >= plain_result.score,
        "technical keywords should increase or equal plain score ({} >= {})",
        technical_result.score,
        plain_result.score
    );
}

#[test]
fn test_analysis_difficulty_mapping_novice() {
    use pe2_core::analysis::{analyze_prompt_complexity, Difficulty};
    let r = analyze_prompt_complexity("short prompt");
    assert_eq!(r.difficulty, Difficulty::Novice);
    assert_eq!(r.iterations, 1);
}

// ============================================================
// Config tests  (port of tests/config.test.js)
// ============================================================

#[test]
fn test_get_default_config() {
    use pe2_core::config::Config;
    let c = Config::default();
    assert!(!c.provider.is_empty(), "provider should be non-empty");
    assert!(!c.model.is_empty(), "model should be non-empty");
}

#[test]
fn test_resolve_api_key_prefers_configured() {
    use pe2_core::config::resolve_api_key;
    assert_eq!(
        resolve_api_key("openai", Some("sk-from-config")),
        Some("sk-from-config".to_string())
    );
}

#[test]
fn test_resolve_api_key_uses_env_when_config_empty() {
    use pe2_core::config::resolve_api_key;
    // The function checks env var when config key is empty
    // We don't set env in test to avoid side effects, so it should return None
    assert_eq!(resolve_api_key("openai", None), None);
    assert_eq!(resolve_api_key("openai", Some("")), None);
}

#[test]
fn test_provider_env_var_names() {
    use pe2_core::constants::provider_env_var;
    assert_eq!(provider_env_var("openai"), "OPENAI_API_KEY");
    assert_eq!(provider_env_var("anthropic"), "ANTHROPIC_API_KEY");
    assert_eq!(provider_env_var("google"), "GOOGLE_API_KEY");
    assert_eq!(provider_env_var("openrouter"), "OPENROUTER_API_KEY");
    assert_eq!(provider_env_var("ollama"), "OLLAMA_BASE_URL");
}

// ============================================================
// Validation tests  (port of tests/validation.test.js)
// ============================================================

#[test]
fn test_validate_prompt_rejects_empty() {
    use pe2_core::validation::validate_prompt;
    assert!(validate_prompt("").is_some(), "empty prompt should be rejected");
    assert!(validate_prompt("   ").is_some(), "whitespace prompt should be rejected");
}

#[test]
fn test_validate_prompt_rejects_short() {
    use pe2_core::validation::validate_prompt;
    let short = "x";
    assert!(validate_prompt(short).is_some(), "short prompt should be rejected");
}

#[test]
fn test_validate_prompt_accepts_minimum() {
    use pe2_core::constants::PROMPT_MIN_LENGTH;
    use pe2_core::validation::validate_prompt;
    let ok = "x".repeat(PROMPT_MIN_LENGTH);
    assert!(validate_prompt(&ok).is_none(), "minimum length prompt should be accepted");
}

#[test]
fn test_validate_prompt_rejects_too_long() {
    use pe2_core::constants::PROMPT_MAX_LENGTH;
    use pe2_core::validation::validate_prompt;
    let too_long = "x".repeat(PROMPT_MAX_LENGTH + 1);
    assert!(
        validate_prompt(&too_long).is_some(),
        "over-max prompt should be rejected"
    );
}

#[test]
fn test_validate_and_suggest_command_accepts_help() {
    use pe2_core::validation::validate_and_suggest_command;
    let v = validate_and_suggest_command("/help");
    assert!(v.valid, "/help should be valid");
    assert!(v.is_command, "/help should be recognized as command");
}

#[test]
fn test_validate_and_suggest_command_rejects_unknown() {
    use pe2_core::validation::validate_and_suggest_command;
    let v = validate_and_suggest_command("/setings");
    assert!(!v.valid, "typo should be invalid");
    assert!(v.is_command, "should still be recognized as command attempt");
    assert!(v.suggestion.is_some(), "should have a suggestion for typo");
}

#[test]
fn test_parse_slash_command() {
    use pe2_core::validation::parse_slash_command;
    assert_eq!(parse_slash_command("/help"), Some("/help"));
    assert_eq!(parse_slash_command("/help more args"), Some("/help"));
    assert_eq!(parse_slash_command("not a command"), None);
    assert_eq!(parse_slash_command(""), None);
}

// ============================================================
// Atomic write tests  (port of tests/writeJsonFileAtomically.test.js)
// ============================================================

#[test]
fn test_write_json_atomic_writes_valid_json() {
    use pe2_core::write_atomic::write_json_atomic;
    use std::collections::HashMap;
    use std::fs;

    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("data.json");

    let mut data = HashMap::new();
    data.insert("n".to_string(), serde_json::Value::Number(42.into()));
    data.insert("s".to_string(), serde_json::Value::String("ok".to_string()));

    write_json_atomic(&file_path, &data).unwrap();

    let content = fs::read_to_string(&file_path).unwrap();
    let parsed: HashMap<String, serde_json::Value> = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed["n"], serde_json::Value::Number(42.into()));
    assert_eq!(parsed["s"], serde_json::Value::String("ok".to_string()));
}

#[test]
fn test_write_json_atomic_creates_file() {
    use pe2_core::write_atomic::write_json_atomic;

    let dir = TempDir::new().unwrap();
    let file_path = dir.path().join("test.json");

    write_json_atomic(&file_path, &serde_json::json!({"key": "value"})).unwrap();

    assert!(file_path.exists(), "file should exist after atomic write");
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("key"));
    assert!(content.contains("value"));
}

// ============================================================
// Preferences tests
// ============================================================

#[test]
fn test_preferences_defaults() {
    use pe2_core::preferences::UserPreferences;
    let prefs = UserPreferences::new();
    assert_eq!(prefs.theme(), "default");
    assert!(!prefs.compact());
    assert!(prefs.track_usage());
}

#[test]
fn test_preferences_setters() {
    use pe2_core::preferences::UserPreferences;
    let mut prefs = UserPreferences::new();
    prefs.set_theme("dark".to_string());
    assert_eq!(prefs.theme(), "dark");
    prefs.set_compact(true);
    assert!(prefs.compact());
    prefs.set_track_usage(false);
    assert!(!prefs.track_usage());
}

// ============================================================
// Session tests
// ============================================================

#[test]
fn test_session_manager_new() {
    use pe2_core::session::SessionManager;
    let sm = SessionManager::new();
    assert!(!sm.session_id().is_empty());
    assert_eq!(sm.prompt_count(), 0);
}

#[test]
fn test_session_add_prompt() {
    use pe2_core::session::SessionManager;
    let mut sm = SessionManager::new();
    sm.add_prompt("test prompt".to_string());
    assert_eq!(sm.prompt_count(), 1);
    assert_eq!(sm.prompts()[0], "test prompt");
}

// ============================================================
// Stats tests
// ============================================================

#[test]
fn test_stats_tracker_new() {
    use pe2_core::stats::StatsTracker;
    let st = StatsTracker::new();
    assert_eq!(st.stats().total_prompts, 0);
}

#[test]
fn test_stats_track_increments() {
    use pe2_core::stats::StatsTracker;
    let mut st = StatsTracker::new();
    st.track("test-model", 10);
    assert_eq!(st.stats().total_prompts, 1);
    assert!(st.stats().running_avg_complexity > 0.0);
}

// ============================================================
// Constants tests
// ============================================================

#[test]
fn test_constants_have_sensible_values() {
    use pe2_core::constants;
    assert!(constants::PROMPT_MIN_LENGTH >= 3);
    assert!(constants::PROMPT_MAX_LENGTH >= constants::PROMPT_MIN_LENGTH);
    assert!(constants::REQUEST_TIMEOUT_MS >= 1000);
    assert!(constants::LLM_MAX_TOKENS >= 128);
    assert!(constants::COMPLEXITY_SCORE_MAX >= 10);
}

#[test]
fn test_default_model_for_provider() {
    use pe2_core::constants::default_model_for_provider;
    assert!(!default_model_for_provider("openai").is_empty());
    assert!(!default_model_for_provider("anthropic").is_empty());
    assert!(!default_model_for_provider("google").is_empty());
    assert!(!default_model_for_provider("openrouter").is_empty());
    assert!(!default_model_for_provider("ollama").is_empty());
    // Unknown provider falls back to a sensible default
    assert!(!default_model_for_provider("unknown").is_empty());
}

// ============================================================
// Engine / Pipeline tests  (port of tests/engine.test.js)
// ============================================================

/// A mock provider that simulates an LLM returning structured JSON
struct MockProvider {
    response: String,
    should_fail: bool,
}

#[async_trait::async_trait]
impl pe2_core::engine::EngineLlmProvider for MockProvider {
    async fn chat(
        &self,
        _model: &str,
        _messages: &[pe2_core::messages::Message],
        _max_tokens: u32,
        _temperature: f64,
    ) -> Result<String, pe2_core::errors::CliError> {
        if self.should_fail {
            return Err(pe2_core::errors::CliError::Network("network failure".to_string()));
        }
        if self.response.trim().is_empty() {
            return Err(pe2_core::errors::CliError::Provider {
                provider: "mock".to_string(),
                message: "Model returned empty content".to_string(),
            });
        }
        Ok(self.response.clone())
    }
}

#[tokio::test]
async fn test_engine_refine_throws_on_network_failure() {
    let provider = MockProvider {
        response: String::new(),
        should_fail: true,
    };
    let cfg = pe2_core::config::Config::default();
    let mut pipeline = pe2_core::engine::Pipeline::new(Box::new(provider), cfg);

    let result = pipeline.run("test prompt").await;
    assert!(result.is_err(), "engine should error on network failure");
    let err = result.unwrap_err();
    match err {
        pe2_core::errors::CliError::Network(_) => {}
        other => panic!("Expected Network error, got: {}", other),
    }
}

#[tokio::test]
async fn test_engine_refine_throws_on_empty_content() {
    let provider = MockProvider {
        response: "   ".to_string(),
        should_fail: false,
    };
    let cfg = pe2_core::config::Config::default();
    let mut pipeline = pe2_core::engine::Pipeline::new(Box::new(provider), cfg);

    let result = pipeline.run("test prompt").await;
    assert!(result.is_err(), "engine should error on empty content");
}

#[tokio::test]
async fn test_engine_generate_initial_success() {
    let valid_json = r#"{
        "context": "Test context",
        "role": "Test role",
        "task": "1. Do this\n2. Do that",
        "constraints": "- Be good",
        "output": "JSON output"
    }"#;

    let provider = MockProvider {
        response: valid_json.to_string(),
        should_fail: false,
    };
    let cfg = pe2_core::config::Config::default();
    let mut pipeline = pe2_core::engine::Pipeline::new(Box::new(provider), cfg);

    let result = pipeline.run("Write a test").await;
    assert!(result.is_ok(), "pipeline should succeed: {:?}", result.err());
    let pipeline_result = result.unwrap();
    assert_eq!(pipeline_result.prompt.context, "Test context");
    assert_eq!(pipeline_result.prompt.role, "Test role");
    assert!(pipeline_result.output_file.contains("pe2-session-"));
}

// ============================================================
// StructuredPrompt tests
// ============================================================

#[test]
fn test_structured_prompt_default() {
    use pe2_core::engine::StructuredPrompt;
    let sp = StructuredPrompt::default();
    assert!(!sp.context.is_empty());
    assert!(!sp.role.is_empty());
    assert!(!sp.task.is_empty());
    assert!(!sp.constraints.is_empty());
    assert!(!sp.output.is_empty());
}

#[test]
fn test_structured_prompt_from_json() {
    use pe2_core::engine::StructuredPrompt;
    let json = r#"{
        "context": "ctx",
        "role": "role",
        "task": "task",
        "constraints": "constraints",
        "output": "output"
    }"#;
    let sp = StructuredPrompt::from_json(json).unwrap();
    assert_eq!(sp.context, "ctx");
    assert_eq!(sp.role, "role");
}

#[test]
fn test_structured_prompt_to_json_roundtrip() {
    use pe2_core::engine::StructuredPrompt;
    let sp = StructuredPrompt {
        context: "c".to_string(),
        role: "r".to_string(),
        task: "t".to_string(),
        constraints: "cn".to_string(),
        output: "o".to_string(),
    };
    let json = sp.to_json_pretty();
    let parsed = StructuredPrompt::from_json(&json).unwrap();
    assert_eq!(parsed.context, "c");
    assert_eq!(parsed.role, "r");
    assert_eq!(parsed.task, "t");
    assert_eq!(parsed.constraints, "cn");
    assert_eq!(parsed.output, "o");
}

#[test]
fn test_structured_prompt_from_llm_response_valid_json() {
    use pe2_core::engine::StructuredPrompt;
    let content = r#"{
        "context": "test c",
        "role": "test r",
        "task": "test t",
        "constraints": "test cn",
        "output": "test o"
    }"#;
    let (sp, edits) = StructuredPrompt::from_llm_response(content, "raw").unwrap();
    assert_eq!(sp.context, "test c");
    assert!(edits.contains("field validation"));
}

#[test]
fn test_structured_prompt_from_llm_response_fallback_on_invalid() {
    use pe2_core::engine::StructuredPrompt;
    let content = "this is not json at all";
    let (sp, edits) = StructuredPrompt::from_llm_response(content, "raw prompt").unwrap();
    // Should produce a fallback with the raw prompt embedded
    assert!(!sp.context.is_empty());
    assert!(edits.contains("automatic structuring"));
}

// ============================================================
// Template tests
// ============================================================

#[test]
fn test_initial_template_contains_raw_prompt() {
    use pe2_core::templates::get_initial_template;
    let template = get_initial_template("Hello world");
    assert!(template.contains("Hello world"));
    assert!(template.contains("context"));
    assert!(template.contains("role"));
}

#[test]
fn test_refinement_template_contains_iteration() {
    use pe2_core::templates::get_refinement_template;
    let template = get_refinement_template("{\"key\":\"val\"}", 3);
    assert!(template.contains("3"));
    assert!(template.contains("context"));
}

#[test]
fn test_markdown_output_format() {
    use pe2_core::templates::format_markdown_output;
    let history = vec![(1u32, "Initial generation".to_string())];
    let output = format_markdown_output(
        "{\"test\": true}",
        &history,
        "90%",
        "balanced",
        "8.5",
        1,
        "NOVICE",
        5,
    );
    assert!(output.contains("PE² Optimized Prompt"));
    assert!(output.contains("NOVICE"));
    assert!(output.contains("Initial generation"));
    assert!(output.contains("Performance Metrics"));
}

// ============================================================
// Path tests
// ============================================================

#[test]
fn test_resolve_output_file_default() {
    use pe2_core::paths::resolve_output_file;
    let p = resolve_output_file(None, "abc123");
    assert!(p.to_string_lossy().contains("pe2-session-abc123"));
    assert!(p.to_string_lossy().ends_with(".md"));
}

#[test]
fn test_resolve_output_file_absolute() {
    use pe2_core::paths::resolve_output_file;
    let p = resolve_output_file(Some("C:\\absolute\\path.md"), "ignored");
    assert!(p.to_string_lossy().ends_with("path.md"));
}

// ============================================================
// Message tests
// ============================================================

#[test]
fn test_build_messages() {
    use pe2_core::messages::{build_messages, Message};
    let msgs = build_messages("system msg", "user msg");
    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[0].role, "system");
    assert_eq!(msgs[0].content, "system msg");
    assert_eq!(msgs[1].role, "user");
    assert_eq!(msgs[1].content, "user msg");
}

#[test]
fn test_build_messages_with_system_none() {
    use pe2_core::messages::build_messages_with_system;
    let msgs = build_messages_with_system(None, "user only");
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].role, "user");
}
