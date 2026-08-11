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
    cfg: Config,
    options: PipelineRunOptions,
    raw_prompt: &str,
) -> Result<PipelineResult, CliError> {
    let spinner = create_spinner("Generating prompt...")?;
    let result = run_pipeline(cfg, options, raw_prompt).await?;
    spinner.finish_and_clear();
    Ok(result)
}

pub fn render_generation_result(result: &PipelineResult) {
    print_success("Prompt generation complete!");
    print_prompt_result(&result.prompt, &result.output_file);
    print_refinement_history(&result.history);
    print_metrics(&result.analysis, result.history.len());
}
