use super::InteractiveSession;
use crate::display::print_error;
use crate::prompt_flow::generate_and_render;
use pe2_core::errors::CliError;
use pe2_core::validation;

pub async fn run_prompt_input(s: &mut InteractiveSession, r: &str) -> Result<(), CliError> {
    if let Some(m) = validation::validate_prompt(r) {
        print_error(&m);
        return Ok(());
    }
    let x = generate_and_render(s.config.clone(), s.pipeline_options, r).await?;
    s.session_store.add_entry(pe2_core::session::SessionEntry {
        prompt: r.to_string(),
        output: x.output_file.clone(),
        model: s.config.model.clone(),
        provider: s.config.provider.clone(),
        difficulty: x.analysis.difficulty.label().to_string(),
        score: x.analysis.score,
        timestamp: chrono::Utc::now().to_rfc3339(),
    });
    if s.preferences.track_usage() {
        s.stats
            .record_usage(&s.config.provider, Some(x.analysis.score));
    }
    Ok(())
}
