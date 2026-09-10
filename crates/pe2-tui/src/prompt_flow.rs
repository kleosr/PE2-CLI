use crate::display::{
    create_spinner, print_complexity_analysis, print_info, print_metrics, print_prompt_result,
    print_refinement_history, print_success,
};
use pe2_core::analysis::ComplexityResult;
use pe2_core::config::Config;
use pe2_core::engine::{PipelineResult, PipelineRunOptions};
use pe2_core::errors::CliError;
use pe2_providers::runner::run_pipeline;

pub fn render_complexity_preflight(a: &ComplexityResult, p: &str, m: &str) {
    print_complexity_analysis(a);
    print_info(&format!("Using {p} / {m}"));
}
pub async fn generate_prompt_with_spinner(
    c: Config,
    o: PipelineRunOptions,
    r: &str,
) -> Result<PipelineResult, CliError> {
    let s = create_spinner("Generating prompt...")?;
    let x = run_pipeline(c, o, r).await?;
    s.finish_and_clear();
    Ok(x)
}
pub fn render_generation_result(r: &PipelineResult) {
    print_success("Prompt generation complete!");
    print_prompt_result(&r.prompt, &r.output_file);
    print_refinement_history(&r.history);
    print_metrics(&r.analysis, r.history.len());
    if let Some(n) = &r.refinement_note {
        print_info(&format!("Refinement note: {n}"));
    }
}
pub async fn generate_and_render(
    c: Config,
    o: PipelineRunOptions,
    r: &str,
) -> Result<PipelineResult, CliError> {
    let a = pe2_core::analysis::analyze_prompt_complexity(r);
    render_complexity_preflight(&a, &c.provider, &c.model);
    let x = generate_prompt_with_spinner(c, o, r).await?;
    render_generation_result(&x);
    Ok(x)
}
