use pe2_core::errors::CliError;
use pe2_core::validation;
use crate::display::print_error;
use crate::prompt_flow::{generate_prompt_with_spinner, render_complexity_preflight, render_generation_result};
use super::InteractiveSession;

pub async fn run_prompt_input(state: &mut InteractiveSession, raw_prompt: &str) -> Result<(), CliError> {
    if let Some(msg) = validation::validate_prompt(raw_prompt) {
        print_error(&msg);
        return Ok(());
    }

    let analysis = pe2_core::analysis::analyze_prompt_complexity(raw_prompt);
    render_complexity_preflight(&analysis, &state.config.provider, &state.config.model);

    let result = generate_prompt_with_spinner(
        state.config.clone(),
        state.pipeline_options.clone(),
        raw_prompt,
    )
    .await?;

    render_generation_result(&result);
    persist_prompt_outcome(state, raw_prompt, &result).await
}

async fn persist_prompt_outcome(
    state: &mut InteractiveSession,
    raw_prompt: &str,
    result: &pe2_core::engine::PipelineResult,
) -> Result<(), CliError> {
    state
        .session_store
        .add_entry(pe2_core::session::SessionEntry {
            prompt: raw_prompt.to_string(),
            output: result.output_file.clone(),
            model: state.config.model.clone(),
            provider: state.config.provider.clone(),
            difficulty: result.analysis.difficulty.label().to_string(),
            score: result.analysis.score,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
        .await;
    state.session_store.save().await?;

    if state.preferences.track_usage() {
        state
            .stats
            .record_usage(&state.config.provider, Some(result.analysis.score));
        state.stats.save().await?;
    }

    Ok(())
}
