use crate::analysis::{self, ComplexityResult};
use crate::config::Config;
use crate::constants;
use crate::errors::CliError;
use crate::templates;
use crate::validation;
use crate::write_atomic;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};

const FIELD_VALIDATION_EDITS: &str = "Prompt generation with field validation.";
const AUTO_STRUCTURING_EDITS: &str = "Prompt generation with automatic structuring.";

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub fn build_messages(system_content: &str, user_content: &str) -> Vec<Message> {
    vec![
        Message {
            role: "system".to_string(),
            content: system_content.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: user_content.to_string(),
        },
    ]
}

fn local_prompts_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("pe2-prompts")
}

fn rejects_traversal(path: &Path) -> bool {
    path.components().any(|c| matches!(c, Component::ParentDir))
}

pub fn resolve_output_file(
    output_file: Option<&str>,
    session_id: &str,
) -> Result<PathBuf, std::io::Error> {
    if let Some(file) = output_file {
        let path = if PathBuf::from(file).is_absolute() {
            PathBuf::from(file)
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(file)
        };
        if rejects_traversal(&path) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "output path must not contain '..'",
            ));
        }
        return Ok(path);
    }
    let dir = local_prompts_dir();
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(format!("pe2-session-{}.md", session_id)))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredPrompt {
    pub context: String,
    pub role: String,
    pub task: String,
    pub constraints: String,
    pub output: String,
}

fn fill_empty_fields(parsed: StructuredPrompt) -> StructuredPrompt {
    StructuredPrompt {
        context: if parsed.context.is_empty() {
            "No context provided".to_string()
        } else {
            parsed.context
        },
        role: if parsed.role.is_empty() {
            "Expert assistant".to_string()
        } else {
            parsed.role
        },
        task: if parsed.task.is_empty() {
            "Complete the requested task".to_string()
        } else {
            parsed.task
        },
        constraints: if parsed.constraints.is_empty() {
            "Follow best practices".to_string()
        } else {
            parsed.constraints
        },
        output: if parsed.output.is_empty() {
            "Provide appropriate output".to_string()
        } else {
            parsed.output
        },
    }
}

fn has_all_fields(prompt: &StructuredPrompt) -> bool {
    !prompt.context.is_empty()
        && !prompt.role.is_empty()
        && !prompt.task.is_empty()
        && !prompt.constraints.is_empty()
        && !prompt.output.is_empty()
}

fn extract_json_object(content: &str) -> &str {
    match (content.find('{'), content.rfind('}')) {
        (Some(start), Some(end)) if start <= end => &content[start..=end],
        _ => "",
    }
}

fn fallback_prompt(raw_prompt: &str) -> StructuredPrompt {
    StructuredPrompt {
        context: format!(
            "The user wants to: {}",
            raw_prompt.chars().take(500).collect::<String>()
        ),
        role: "Expert assistant with deep knowledge in the relevant domain".to_string(),
        task: "1. Understand the user's requirements\n2. Provide a comprehensive solution\n3. Ensure clarity and completeness".to_string(),
        constraints: "- Be accurate and thorough\n- Follow best practices\n- Provide clear explanations".to_string(),
        output: "A well-structured response that fully addresses the user's needs".to_string(),
    }
}

impl StructuredPrompt {
    pub fn to_json_pretty(&self) -> Result<String, CliError> {
        serde_json::to_string_pretty(self).map_err(CliError::Json)
    }

    pub fn from_llm_response(content: &str, raw_prompt: &str) -> Result<(Self, String), CliError> {
        if let Ok(parsed) = serde_json::from_str::<Self>(content) {
            if has_all_fields(&parsed) {
                return Ok((parsed, FIELD_VALIDATION_EDITS.to_string()));
            }
            return Ok((
                fill_empty_fields(parsed),
                FIELD_VALIDATION_EDITS.to_string(),
            ));
        }

        let repaired = extract_json_object(content);
        if let Ok(parsed) = serde_json::from_str::<Self>(repaired) {
            return Ok((
                fill_empty_fields(parsed),
                FIELD_VALIDATION_EDITS.to_string(),
            ));
        }

        Ok((
            fallback_prompt(raw_prompt),
            AUTO_STRUCTURING_EDITS.to_string(),
        ))
    }
}

impl Default for StructuredPrompt {
    fn default() -> Self {
        fallback_prompt("General purpose task")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChatOptions {
    pub max_tokens: u32,
    pub temperature: f64,
}

#[async_trait]
pub trait EngineLlmProvider: Send + Sync {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        options: &ChatOptions,
    ) -> Result<String, CliError>;
}

#[derive(Debug, Clone)]
pub struct RefinementEntry {
    pub iteration: u32,
    pub edits: String,
}

struct PromptTurn {
    prompt: StructuredPrompt,
    edits: String,
}

pub struct Pipeline {
    provider: Box<dyn EngineLlmProvider>,
    config: Config,
    options: PipelineRunOptions,
    current_prompt: Option<StructuredPrompt>,
    history: Vec<RefinementEntry>,
    refinement_note: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct PipelineRunOptions {
    pub iterations_override: Option<u32>,
    pub max_tokens: u32,
    pub temperature: f64,
}

impl Default for PipelineRunOptions {
    fn default() -> Self {
        Self {
            iterations_override: None,
            max_tokens: constants::LLM_MAX_TOKENS,
            temperature: constants::LLM_TEMPERATURE,
        }
    }
}

impl Pipeline {
    pub fn new(provider: Box<dyn EngineLlmProvider>, config: Config) -> Self {
        Self::with_options(provider, config, PipelineRunOptions::default())
    }

    pub fn with_options(
        provider: Box<dyn EngineLlmProvider>,
        config: Config,
        options: PipelineRunOptions,
    ) -> Self {
        Self {
            provider,
            config,
            options,
            current_prompt: None,
            history: Vec::new(),
            refinement_note: None,
        }
    }

    pub async fn run(&mut self, raw_prompt: &str) -> Result<PipelineResult, CliError> {
        if let Some(msg) = validation::validate_prompt(raw_prompt) {
            return Err(CliError::Validation(msg));
        }

        let analysis = analysis::analyze_prompt_complexity(raw_prompt);
        let iterations = self.resolve_iterations(&analysis);
        self.run_refinements(raw_prompt, iterations).await?;
        self.write_result(&analysis)
    }

    fn resolve_iterations(&self, analysis: &ComplexityResult) -> usize {
        self.options
            .iterations_override
            .unwrap_or(analysis.iterations)
            .max(1) as usize
    }

    async fn run_refinements(
        &mut self,
        raw_prompt: &str,
        iterations: usize,
    ) -> Result<(), CliError> {
        let initial = self.generate_initial(raw_prompt).await?;
        self.current_prompt = Some(initial.prompt);
        self.history.push(RefinementEntry {
            iteration: 1,
            edits: initial.edits,
        });

        for i in 1..iterations {
            match self.refine((i + 1) as u32).await {
                Ok(r) => {
                    self.current_prompt = Some(r.prompt);
                    self.history.push(RefinementEntry {
                        iteration: (i + 1) as u32,
                        edits: r.edits,
                    });
                }
                Err(e) => {
                    tracing::warn!("Refinement {} failed: {}", i + 1, e);
                    self.refinement_note = Some(e.to_string());
                    break;
                }
            }
        }
        Ok(())
    }

    fn write_result(&self, analysis: &ComplexityResult) -> Result<PipelineResult, CliError> {
        let prompt = self
            .current_prompt
            .as_ref()
            .ok_or_else(|| CliError::Runtime("No prompt generated".to_string()))?;

        let output_file = resolve_output_file(
            self.config.output_file.as_deref(),
            &uuid::Uuid::new_v4().to_string()[..8],
        )?;

        let markdown = templates::format_markdown_output(
            &prompt.to_json_pretty()?,
            &self
                .history
                .iter()
                .map(|h| (h.iteration, h.edits.clone()))
                .collect::<Vec<_>>(),
            analysis,
            self.history.len(),
        );

        write_atomic::write_text_atomic(&output_file, &markdown)?;

        Ok(PipelineResult {
            prompt: prompt.clone(),
            output_file: output_file.to_string_lossy().to_string(),
            analysis: analysis.clone(),
            history: self.history.clone(),
            refinement_note: self.refinement_note.clone(),
        })
    }

    async fn generate_initial(&self, raw_prompt: &str) -> Result<PromptTurn, CliError> {
        let template = templates::get_initial_template(raw_prompt);
        let messages = build_messages(constants::LLM_SYSTEM_MESSAGE, &template);
        let content = self.call_provider(&messages).await?;
        let (prompt, edits) = StructuredPrompt::from_llm_response(&content, raw_prompt)?;
        Ok(PromptTurn { prompt, edits })
    }

    async fn refine(&self, iteration_num: u32) -> Result<PromptTurn, CliError> {
        let current = self
            .current_prompt
            .as_ref()
            .ok_or_else(|| CliError::Runtime("No prompt to refine".to_string()))?;
        let json = current.to_json_pretty()?;
        let template = templates::get_refinement_template(&json, iteration_num);
        let messages = build_messages(constants::LLM_REFINEMENT_SYSTEM_MESSAGE, &template);
        let content = self.call_provider(&messages).await?;
        let (prompt, edits) = StructuredPrompt::from_llm_response(&content, &json)?;
        Ok(PromptTurn { prompt, edits })
    }

    async fn call_provider(&self, messages: &[Message]) -> Result<String, CliError> {
        let chat_opts = ChatOptions {
            max_tokens: self.options.max_tokens,
            temperature: self.options.temperature,
        };
        let content = self
            .provider
            .chat(&self.config.model, messages, &chat_opts)
            .await?;
        if content.trim().is_empty() {
            return Err(CliError::Provider {
                provider: self.config.provider.clone(),
                message: "Model returned empty content".to_string(),
            });
        }
        Ok(content)
    }
}

#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub prompt: StructuredPrompt,
    pub output_file: String,
    pub analysis: ComplexityResult,
    pub history: Vec<RefinementEntry>,
    pub refinement_note: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_llm_response_parses_valid_json() {
        let json = r#"{
            "context": "c",
            "role": "r",
            "task": "t",
            "constraints": "x",
            "output": "o"
        }"#;
        let (prompt, _) = StructuredPrompt::from_llm_response(json, "raw").unwrap();
        assert_eq!(prompt.context, "c");
        assert_eq!(prompt.role, "r");
    }

    #[test]
    fn from_llm_response_falls_back_on_garbage() {
        let (prompt, edits) =
            StructuredPrompt::from_llm_response("not json", "do the thing").unwrap();
        assert!(prompt.context.contains("do the thing"));
        assert!(edits.contains("automatic"));
    }

    #[test]
    fn rejects_parent_dir_in_output_file() {
        let err = resolve_output_file(Some("../escape.md"), "abc").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    }

    #[test]
    fn build_messages_has_system_and_user() {
        let msgs = build_messages("sys", "user");
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].role, "system");
        assert_eq!(msgs[1].role, "user");
    }
}
