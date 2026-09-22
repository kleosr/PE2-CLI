use crate::display::{
    create_spinner, print_complexity_analysis, print_info, print_metrics, print_prompt_result,
    print_refinement_history, print_success,
};
use pe2_core::analysis::ComplexityResult;
use pe2_core::config::Config;
use pe2_core::engine::{PipelineResult, PipelineRunOptions};
use pe2_core::errors::CliError;
use pe2_providers::runner::run_pipeline;

pub fn render_complexity_preflight(analysis: &ComplexityResult, provider: &str, model: &str) {
    print_complexity_analysis(analysis);
    print_info(&format!("Using {provider} / {model}"));
}

pub async fn generate_prompt_with_spinner(
    config: Config,
    options: PipelineRunOptions,
    raw: &str,
) -> Result<PipelineResult, CliError> {
    let spinner = create_spinner("Generating prompt...")?;
    let result = run_pipeline(config, options, raw).await?;
    spinner.finish_and_clear();
    Ok(result)
}

pub fn render_generation_result(result: &PipelineResult) {
    print_success("Prompt generation complete!");
    print_prompt_result(&result.prompt, &result.output_file);
    print_refinement_history(&result.history);
    print_metrics(&result.analysis, result.history.len());
    if let Some(note) = &result.refinement_note {
        print_info(&format!("Refinement note: {note}"));
    }
}

pub async fn generate_and_render(
    config: Config,
    options: PipelineRunOptions,
    raw: &str,
) -> Result<PipelineResult, CliError> {
    let analysis = pe2_core::analysis::analyze_prompt_complexity(raw);
    render_complexity_preflight(&analysis, &config.provider, &config.model);
    let result = generate_prompt_with_spinner(config, options, raw).await?;
    render_generation_result(&result);
    Ok(result)
}
