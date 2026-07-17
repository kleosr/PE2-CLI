use crate::analysis::ComplexityResult;
use crate::constants;

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
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    output.push_str("# PE² Optimized Prompt\n\n");
    output.push_str(&format!("**Generated:** {}  \n", now));
    output.push_str(&format!(
        "**Difficulty:** {} (Score: {}/{})  \n\n",
        analysis.difficulty.as_str(),
        analysis.score,
        constants::COMPLEXITY_SCORE_MAX
    ));
    output.push_str("---\n\n");
    output.push_str("## Optimized Prompt\n\n");
    output.push_str("```markdown\n");
    output.push_str(pe2_prompt);
    output.push_str("\n```\n\n");
    output.push_str("---\n\n");
    output.push_str("## Refinement History\n\n");
    for (iteration, edit) in history {
        output.push_str(&format!("### Iteration {}\n\n{}\n\n", iteration, edit));
    }
    output.push_str("---\n\n");
    output.push_str("## Run Metrics\n\n");
    output.push_str("| Metric | Value |\n");
    output.push_str("|--------|-------|\n");
    output.push_str(&format!("| Difficulty | {} |\n", analysis.difficulty.as_str()));
    output.push_str(&format!(
        "| Complexity Score | {}/{} |\n",
        analysis.score,
        constants::COMPLEXITY_SCORE_MAX
    ));
    output.push_str(&format!("| Iterations Applied | {} |\n", iterations));
    output
}
