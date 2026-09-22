use crate::analysis::ComplexityResult;
use crate::constants;
use std::fmt::Write as _;

pub fn get_initial_template(raw: &str) -> String {
    constants::INITIAL_PROMPT_TEMPLATE.replace("{raw_prompt}", raw)
}

pub fn get_refinement_template(current_json: &str, iteration: u32) -> String {
    constants::REFINEMENT_PROMPT_TEMPLATE
        .replace("{current_prompt_json}", current_json)
        .replace("{iteration_num}", &iteration.to_string())
}

pub fn format_markdown_output(
    prompt_json: &str,
    history: &[(u32, String)],
    analysis: &ComplexityResult,
    iterations_applied: usize,
) -> String {
    let mut output = String::with_capacity(2048);
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let _ = writeln!(
        output,
        "# PE² Optimized Prompt\n\n**Generated:** {now}  \n**Difficulty:** {} (Score: {}/{})  \n",
        analysis.difficulty.as_str(),
        analysis.score,
        constants::COMPLEXITY_SCORE_MAX
    );
    let _ = writeln!(
        output,
        "---\n\n## Optimized Prompt\n\n```markdown\n{prompt_json}\n```\n\n---\n\n## Refinement History\n"
    );
    for (iteration, edits) in history {
        let _ = writeln!(output, "### Iteration {iteration}\n\n{edits}\n");
    }
    let _ = write!(
        output,
        "---\n\n## Run Metrics\n\n| Metric | Value |\n|--------|-------|\n| Difficulty | {} |\n| Complexity Score | {}/{} |\n| Iterations Applied | {iterations_applied} |\n",
        analysis.difficulty.as_str(),
        analysis.score,
        constants::COMPLEXITY_SCORE_MAX
    );
    output
}
