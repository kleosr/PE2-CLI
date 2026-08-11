use crate::analysis::ComplexityResult;
use crate::constants;
use std::fmt::Write as _;

pub fn get_initial_template(raw_prompt: &str) -> String {
    constants::INITIAL_PROMPT_TEMPLATE.replace("{raw_prompt}", raw_prompt)
}

pub fn get_refinement_template(current_prompt_json: &str, iteration_num: u32) -> String {
    constants::REFINEMENT_PROMPT_TEMPLATE
        .replace("{current_prompt_json}", current_prompt_json)
        .replace("{iteration_num}", &iteration_num.to_string())
}

pub fn format_markdown_output(
    pe2_prompt: &str,
    history: &[(u32, String)],
    analysis: &ComplexityResult,
    iterations: usize,
) -> String {
    let mut output = String::with_capacity(2048);
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let _ = writeln!(output, "# PE² Optimized Prompt\n");
    let _ = writeln!(output, "**Generated:** {now}  ");
    let _ = writeln!(
        output,
        "**Difficulty:** {} (Score: {}/{})  \n",
        analysis.difficulty.as_str(),
        analysis.score,
        constants::COMPLEXITY_SCORE_MAX
    );
    output.push_str("---\n\n## Optimized Prompt\n\n```markdown\n");
    output.push_str(pe2_prompt);
    output.push_str("\n```\n\n---\n\n## Refinement History\n\n");
    for (iteration, edit) in history {
        let _ = writeln!(output, "### Iteration {iteration}\n\n{edit}\n");
    }
    output.push_str("---\n\n## Run Metrics\n\n| Metric | Value |\n|--------|-------|\n");
    let _ = writeln!(output, "| Difficulty | {} |", analysis.difficulty.as_str());
    let _ = writeln!(
        output,
        "| Complexity Score | {}/{} |",
        analysis.score,
        constants::COMPLEXITY_SCORE_MAX
    );
    let _ = writeln!(output, "| Iterations Applied | {iterations} |");
    output
}
