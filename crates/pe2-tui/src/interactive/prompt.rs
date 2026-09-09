use super::InteractiveSession;
use crate::display::print_error;
use crate::prompt_flow::generate_and_render;
use pe2_core::errors::CliError;
use pe2_core::validation;

pub async fn run_prompt_input(
    state: &mut InteractiveSession,
    raw_prompt: &str,
) -> Result<(), CliError> {
    if let Some(msg) = validation::validate_prompt(raw_prompt) {
        print_error(&msg);
        return Ok(());
    }

    let result =
        generate_and_render(state.config.clone(), state.pipeline_options, raw_prompt).await?;
    persist_prompt_outcome(state, raw_prompt, &result);
    Ok(())
}

fn persist_prompt_outcome(
    state: &mut InteractiveSession,
    raw_prompt: &str,
    result: &pe2_core::engine::PipelineResult,
) {
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
        });

    if state.preferences.track_usage() {
        state
            .stats
            .record_usage(&state.config.provider, Some(result.analysis.score));
    }
}
