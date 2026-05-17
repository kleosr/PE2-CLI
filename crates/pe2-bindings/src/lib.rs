#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use pe2_core::analysis::{self, ComplexityResult, Difficulty};
use pe2_core::config;
use pe2_core::engine::{Pipeline, StructuredPrompt};
use pe2_core::errors::CliError;

// ============================================================
// Config bindings
// ============================================================

#[napi]
pub fn load_config() -> String {
    let cfg = config::load_config();
    serde_json::to_string(&cfg).unwrap_or_default()
}

#[napi]
pub fn save_config(config_json: String) -> Result<()> {
    let cfg: config::Config = serde_json::from_str(&config_json)
        .map_err(|e| Error::from_reason(format!("Invalid config JSON: {}", e)))?;
    config::save_config(&cfg)
        .map_err(|e| Error::from_reason(format!("Failed to save config: {}", e)))?;
    Ok(())
}

#[napi]
pub fn get_config_path() -> String {
    config::config_file_path()
        .to_string_lossy()
        .to_string()
}

// ============================================================
// Analysis bindings
// ============================================================

#[napi]
pub fn analyze_prompt_complexity(raw_prompt: String) -> String {
    let result = analysis::analyze_prompt_complexity(&raw_prompt);
    serde_json::json!({
        "score": result.score,
        "difficulty": result.difficulty.as_str(),
        "difficulty_label": result.difficulty.label(),
        "iterations": result.iterations,
        "word_count": result.word_count,
    })
    .to_string()
}

// ============================================================
// Prompt processing bindings
// ============================================================

#[napi]
pub async fn process_prompt(
    raw_prompt: String,
    provider: String,
    model: String,
    api_key: Option<String>,
) -> Result<String> {
    let kind = pe2_providers::client::ProviderKind::from_str_result(&provider)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let provider_config = pe2_providers::client::ProviderConfig {
        kind,
        api_key,
        base_url: None,
    };

    let client = pe2_providers::factory::create_client(&provider_config)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    use async_trait::async_trait;
    use pe2_core::engine::EngineLlmProvider;
    use pe2_core::messages::Message;

    struct NapiClientAdapter {
        inner: Box<dyn pe2_providers::client::LlmClient>,
    }

    #[async_trait]
    impl EngineLlmProvider for NapiClientAdapter {
        async fn chat(
            &self,
            model: &str,
            messages: &[Message],
            max_tokens: u32,
            temperature: f64,
        ) -> std::result::Result<String, CliError> {
            let resp = self.inner.chat(model, messages, max_tokens, temperature).await?;
            Ok(resp.content)
        }
    }

    let cfg = config::Config {
        provider,
        model,
        api_key: None,
        output_file: None,
    };

    let mut pipeline = Pipeline::new(Box::new(NapiClientAdapter { inner: client }), cfg);
    let result = pipeline.run(&raw_prompt)
        .await
        .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(serde_json::json!({
        "prompt": {
            "context": result.prompt.context,
            "role": result.prompt.role,
            "task": result.prompt.task,
            "constraints": result.prompt.constraints,
            "output": result.prompt.output,
        },
        "output_file": result.output_file,
        "metrics": {
            "accuracy_gain": result.metrics.accuracy_gain,
            "optimization_level": result.metrics.optimization_level,
            "quality_score": result.metrics.quality_score,
            "iterations_applied": result.metrics.iterations_applied,
        },
        "analysis": {
            "score": result.analysis.score,
            "difficulty": result.analysis.difficulty.as_str(),
            "iterations": result.analysis.iterations,
            "word_count": result.analysis.word_count,
        },
    })
    .to_string())
}

// ============================================================
// Validation bindings
// ============================================================

#[napi]
pub fn validate_prompt(prompt: String) -> Option<String> {
    pe2_core::validation::validate_prompt(&prompt)
}

#[napi]
pub fn parse_slash_command(input: String) -> Option<String> {
    pe2_core::validation::parse_slash_command(&input)
        .map(|s| s.to_string())
}
