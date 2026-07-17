pub const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";
pub const DEFAULT_PROVIDER: &str = "openrouter";
pub const LLM_MAX_TOKENS: u32 = 1024;
pub const LLM_TEMPERATURE: f64 = 0.3;
pub const LLM_SYSTEM_MESSAGE: &str =
    "You are a precise prompt optimizer. Follow the instructions and return JSON only.";
pub const LLM_REFINEMENT_SYSTEM_MESSAGE: &str =
    "You are a precise prompt optimizer. Return JSON only.";
pub const REQUEST_TIMEOUT_MS: u64 = 30000;

pub const HTTP_REFERER: &str = "https://pe2-cli-tool.local";
pub const HTTP_TITLE: &str = "KleoSr PE2-CLI Tool";

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
