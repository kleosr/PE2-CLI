#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::Status;
use pe2_core::analysis;
use pe2_core::config;
use pe2_core::engine::PipelineRunOptions;
use pe2_core::errors::CliError;
use pe2_core::validation;
use pe2_providers::runner::run_pipeline;

#[napi]
pub fn load_config() -> Result<String> {
    let cfg = config::load_config().map_err(map_cli_error)?;
    serde_json::to_string(&cfg).map_err(|e| map_cli_error(CliError::Json(e)))
}

#[napi]
pub fn save_config(config_json: String) -> Result<()> {
    let cfg: config::Config =
        serde_json::from_str(&config_json).map_err(|e| map_cli_error(CliError::Json(e)))?;
    config::save_config(&cfg).map_err(map_cli_error)
}

#[napi]
pub fn get_config_path() -> String {
    config::config_file_path()
        .to_string_lossy()
        .to_string()
}

#[napi]
pub fn analyze_prompt_complexity(raw_prompt: String) -> Result<String> {
    if let Some(msg) = validation::validate_prompt(&raw_prompt) {
        return Err(map_cli_error(CliError::Validation(msg)));
    }
    let result = analysis::analyze_prompt_complexity(&raw_prompt);
    serde_json::to_string(&serde_json::json!({
        "score": result.score,
        "difficulty": result.difficulty.as_str(),
        "difficulty_label": result.difficulty.label(),
        "iterations": result.iterations,
        "word_count": result.word_count,
    }))
    .map_err(|e| map_cli_error(CliError::Json(e)))
}

#[napi]
pub async fn execute_prompt(
    raw_prompt: String,
    provider: String,
    model: String,
    api_key: Option<String>,
) -> Result<String> {
    if let Some(msg) = validation::validate_prompt(&raw_prompt) {
        return Err(map_cli_error(CliError::Validation(msg)));
    }
    let cfg = config::Config {
        provider,
        model,
        api_key,
        output_file: None,
    };
    let result = run_pipeline(cfg, PipelineRunOptions::default(), &raw_prompt)
        .await
        .map_err(map_cli_error)?;
    Ok(serialize_pipeline_result(&result))
}

fn serialize_pipeline_result(result: &pe2_core::engine::PipelineResult) -> String {
    serde_json::json!({
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
        "refinement_note": result.refinement_note,
    })
    .to_string()
}

#[napi]
pub fn validate_prompt(prompt: String) -> Option<String> {
    validation::validate_prompt(&prompt)
}

#[napi]
pub fn parse_slash_command(input: String) -> Option<String> {
    validation::parse_slash_command(&input).map(|s| s.to_string())
}

fn cli_error_code(err: &CliError) -> &'static str {
    match err {
        CliError::Validation(_) => "VALIDATION_ERROR",
        CliError::Config(_) => "CONFIG_ERROR",
        CliError::Auth(_) => "AUTH_ERROR",
        CliError::Provider { .. } => "PROVIDER_ERROR",
        CliError::Network(_) => "NETWORK_ERROR",
        CliError::Runtime(_) => "RUNTIME_ERROR",
        CliError::Json(_) => "JSON_ERROR",
        CliError::Io(_) => "IO_ERROR",
        CliError::General(_) => "GENERAL_ERROR",
        CliError::Other(_) => "UNKNOWN_ERROR",
    }
}

fn map_cli_error(err: CliError) -> Error {
    let code = cli_error_code(&err);
    let status = match &err {
        CliError::Validation(_) => Status::InvalidArg,
        CliError::Auth(_) => Status::GenericFailure,
        _ => Status::GenericFailure,
    };
    Error::new(status, format!("{code}: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_prompt_rejects_short_input() {
        assert!(validate_prompt("short".to_string()).is_some());
    }

    #[test]
    fn load_config_returns_parseable_json() {
        let json = load_config().expect("load_config should serialize");
        assert!(serde_json::from_str::<serde_json::Value>(&json).is_ok());
    }

    #[test]
    fn cli_error_codes_are_stable() {
        assert_eq!(cli_error_code(&CliError::Validation("x".into())), "VALIDATION_ERROR");
        assert_eq!(cli_error_code(&CliError::Auth("x".into())), "AUTH_ERROR");
        assert_eq!(cli_error_code(&CliError::Provider {
            provider: "openai".into(),
            message: "fail".into(),
        }), "PROVIDER_ERROR");
    }

    #[test]
    fn analyze_prompt_complexity_rejects_short_prompt() {
        let err = analyze_prompt_complexity("short".to_string()).unwrap_err();
        assert!(err.to_string().contains("VALIDATION_ERROR"));
    }
}
