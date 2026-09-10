use crate::analysis::ComplexityResult;
use crate::constants;
use std::fmt::Write as _;

pub fn get_initial_template(r: &str) -> String {
    constants::INITIAL_PROMPT_TEMPLATE.replace("{raw_prompt}", r)
}
pub fn get_refinement_template(j: &str, n: u32) -> String {
    constants::REFINEMENT_PROMPT_TEMPLATE
        .replace("{current_prompt_json}", j)
        .replace("{iteration_num}", &n.to_string())
}
pub fn format_markdown_output(
    p: &str,
    h: &[(u32, String)],
    a: &ComplexityResult,
    n: usize,
) -> String {
    let mut o = String::with_capacity(2048);
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let _ = writeln!(
        o,
        "# PE² Optimized Prompt\n\n**Generated:** {now}  \n**Difficulty:** {} (Score: {}/{})  \n",
        a.difficulty.as_str(),
        a.score,
        constants::COMPLEXITY_SCORE_MAX
    );
    let _ = writeln!(
        o,
        "---\n\n## Optimized Prompt\n\n```markdown\n{p}\n```\n\n---\n\n## Refinement History\n"
    );
    for (i, e) in h {
        let _ = writeln!(o, "### Iteration {i}\n\n{e}\n");
    }
    let _ = write!(
        o,
        "---\n\n## Run Metrics\n\n| Metric | Value |\n|--------|-------|\n| Difficulty | {} |\n| Complexity Score | {}/{} |\n| Iterations Applied | {n} |\n",
        a.difficulty.as_str(),
        a.score,
        constants::COMPLEXITY_SCORE_MAX
    );
    o
}
