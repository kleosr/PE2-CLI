use crate::constants;

pub struct MarkdownMetrics<'a> {
    pub accuracy: &'a str,
    pub optimization: &'a str,
    pub quality: &'a str,
    pub iterations: usize,
    pub difficulty: &'a str,
    pub complexity_score: u32,
}

pub fn get_initial_template(raw_prompt: &str) -> String {
    constants::INITIAL_PROMPT_TEMPLATE.replace("{raw_prompt}", raw_prompt)
}

pub fn get_refinement_template(current_prompt_json: &str, iteration_num: u32) -> String {
    constants::REFINEMENT_PROMPT_TEMPLATE
        .replace("{current_prompt_json}", current_prompt_json)
        .replace("{iteration_num}", &iteration_num.to_string())
}

fn append_header(output: &mut String, metrics: &MarkdownMetrics<'_>) {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    output.push_str("# PE² Optimized Prompt\n\n");
    output.push_str(&format!("**Generated:** {}  \n", now));
    output.push_str(&format!(
        "**Difficulty:** {} (Score: {}/{})  \n\n",
        metrics.difficulty,
        metrics.complexity_score,
        constants::COMPLEXITY_SCORE_MAX
    ));
    output.push_str("---\n\n");
}

fn append_prompt_body(output: &mut String, pe2_prompt: &str) {
    output.push_str("## Optimized Prompt\n\n");
    output.push_str("```markdown\n");
    output.push_str(pe2_prompt);
    output.push_str("\n```\n\n");
    output.push_str("---\n\n");
}

fn append_history(output: &mut String, history: &[(u32, String)]) {
    output.push_str("## Refinement History\n\n");
    for (iteration, edit) in history {
        output.push_str(&format!("### Iteration {}\n\n{}\n\n", iteration, edit));
    }
    output.push_str("---\n\n");
}

fn append_metrics_table(output: &mut String, metrics: &MarkdownMetrics<'_>) {
    output.push_str("## Performance Metrics\n\n");
    output.push_str("| Metric | Value |\n");
    output.push_str("|--------|-------|\n");
    output.push_str(&format!("| Accuracy Gain | {} |\n", metrics.accuracy));
    output.push_str(&format!("| Optimization | {} |\n", metrics.optimization));
    output.push_str(&format!("| Quality Score | {} |\n", metrics.quality));
    output.push_str(&format!("| Iterations Applied | {} |\n", metrics.iterations));
}

pub fn format_markdown_output(
    pe2_prompt: &str,
    history: &[(u32, String)],
    metrics: &MarkdownMetrics<'_>,
) -> String {
    let mut output = String::with_capacity(2048);
    append_header(&mut output, metrics);
    append_prompt_body(&mut output, pe2_prompt);
    append_history(&mut output, history);
    append_metrics_table(&mut output, metrics);
    output
}
