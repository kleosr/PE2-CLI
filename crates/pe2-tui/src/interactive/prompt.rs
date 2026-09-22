use super::InteractiveSession;
use crate::display::print_error;
use crate::prompt_flow::generate_and_render;
use pe2_core::errors::CliError;
use pe2_core::session::SessionEntry;
use pe2_core::validation;

pub async fn run_prompt_input(session: &mut InteractiveSession, raw: &str) -> Result<(), CliError> {
    if let Some(message) = validation::validate_prompt(raw) {
        print_error(&message);
        return Ok(());
    }
    let result = generate_and_render(session.config.clone(), session.pipeline_options, raw).await?;
    session.session_store.add_entry(SessionEntry {
        prompt: raw.to_string(),
        output: result.output_file.clone(),
        model: session.config.model.clone(),
        provider: session.config.provider.clone(),
        difficulty: result.analysis.difficulty.label().to_string(),
        score: result.analysis.score,
        timestamp: chrono::Utc::now().to_rfc3339(),
    });
    if session.preferences.track_usage() {
        session
            .stats
            .record_usage(&session.config.provider, Some(result.analysis.score));
    }
    Ok(())
}
