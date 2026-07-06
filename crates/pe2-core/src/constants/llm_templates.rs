pub const INITIAL_PROMPT_TEMPLATE: &str = r#"You are an expert prompt engineer. Convert the following raw prompt into a structured, optimized PE² prompt.

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
{
  "context": "Building a REST API endpoint",
  "role": "Senior backend developer",
  "task": "1. Design the endpoint\n2. Implement error handling\n3. Add validation",
  "constraints": "- Follow RESTful conventions\n- Use proper HTTP status codes",
  "output": "Production-ready code with documentation"
}

JSON:"#;

pub const REFINEMENT_PROMPT_TEMPLATE: &str = r#"You are refining a PE² prompt. Improve clarity, specificity, and completeness.

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

JSON:"#;
