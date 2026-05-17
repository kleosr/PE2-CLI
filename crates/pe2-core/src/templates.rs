use crate::analysis::{ComplexityResult, Difficulty};
use crate::constants;

pub fn get_initial_template(raw_prompt: &str) -> String {
    format!(
        r#"You are an expert prompt engineer. Convert the following raw prompt into a structured, optimized PE² prompt.

Raw prompt to optimize:
---
{raw_prompt}
---

Return ONLY a valid JSON object with these exact fields:
- "context": Brief context about the task
- "role": Expert role that should handle this
- "task": Specific task description (numbered steps)
- "constraints": Key constraints and requirements
- "output": Expected output format

Example:
{{
  "context": "Building a REST API endpoint",
  "role": "Senior backend developer",
  "task": "1. Design the endpoint\n2. Implement error handling\n3. Add validation",
  "constraints": "- Follow RESTful conventions\n- Use proper HTTP status codes",
  "output": "Production-ready code with documentation"
}}

JSON:"#
    )
}

pub fn get_refinement_template(current_prompt_json: &str, iteration_num: u32) -> String {
    format!(
        r#"You are refining a PE² prompt. Improve clarity, specificity, and completeness.

Current prompt:
---
{current_prompt_json}
---

Iteration {iteration_num}: Analyze and enhance this prompt. Consider:
1. Is the context sufficiently detailed?
2. Are constraints comprehensive?
3. Is the output format clearly specified?
4. Can any instructions be more precise?

Return the improved prompt as a JSON object with the same fields: context, role, task, constraints, output.

JSON:"#
    )
}

pub fn format_markdown_output(
    pe2_prompt: &str,
    history: &[(u32, String)],
    metric_accuracy: &str,
    metric_optimization: &str,
    metric_quality: &str,
    metric_iterations: usize,
    difficulty: &str,
    complexity_score: u32,
) -> String {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut output = String::with_capacity(2048);

    output.push_str("# PE² Optimized Prompt\n\n");
    output.push_str(&format!("**Generated:** {}  \n", now));
    output.push_str(&format!("**Difficulty:** {} (Score: {}/{})  \n\n", difficulty, complexity_score, constants::COMPLEXITY_SCORE_MAX));
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
    output.push_str("## Performance Metrics\n\n");
    output.push_str("| Metric | Value |\n");
    output.push_str("|--------|-------|\n");
    output.push_str(&format!("| Accuracy Gain | {} |\n", metric_accuracy));
    output.push_str(&format!("| Optimization | {} |\n", metric_optimization));
    output.push_str(&format!("| Quality Score | {} |\n", metric_quality));
    output.push_str(&format!("| Iterations Applied | {} |\n", metric_iterations));

    output
}

/// Build a default fallback prompt when LLM response cannot be parsed
pub fn default_prompt() -> String {
    r#"{
  "context": "General purpose task",
  "role": "Expert assistant with deep knowledge in the relevant domain",
  "task": "1. Understand the user's requirements\n2. Provide a comprehensive solution\n3. Ensure clarity and completeness",
  "constraints": "- Be accurate and thorough\n- Follow best practices\n- Provide clear explanations",
  "output": "A well-structured response that fully addresses the user's needs"
}"#
    .to_string()
}
