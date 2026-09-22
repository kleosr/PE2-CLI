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

const FIELD_NOTE: &str = "Prompt generation with field validation.";
const AUTO_NOTE: &str = "Prompt generation with automatic structuring.";

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub fn build_messages(system: &str, user: &str) -> Vec<Message> {
    vec![
        Message {
            role: "system".to_string(),
            content: system.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: user.to_string(),
        },
    ]
}

fn working_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn resolve_output_file(explicit: Option<&str>, stamp: &str) -> Result<PathBuf, std::io::Error> {
    if let Some(file) = explicit {
        let path = if Path::new(file).is_absolute() {
            PathBuf::from(file)
        } else {
            working_dir().join(file)
        };
        if path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "output path must not contain '..'",
            ));
        }
        return Ok(path);
    }
    let dir = working_dir().join("pe2-prompts");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(format!("pe2-session-{stamp}.md")))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredPrompt {
    pub context: String,
    pub role: String,
    pub task: String,
    pub constraints: String,
    pub output: String,
}

fn fill_if_empty(field: &mut String, fallback: &str) {
    if field.is_empty() {
        *field = fallback.to_string();
    }
}

fn fill(mut prompt: StructuredPrompt) -> StructuredPrompt {
    fill_if_empty(&mut prompt.context, "No context provided");
    fill_if_empty(&mut prompt.role, "Expert assistant");
    fill_if_empty(&mut prompt.task, "Complete the requested task");
    fill_if_empty(&mut prompt.constraints, "Follow best practices");
    fill_if_empty(&mut prompt.output, "Provide appropriate output");
    prompt
}

fn json_object_slice(content: &str) -> &str {
    match (content.find('{'), content.rfind('}')) {
        (Some(start), Some(end)) if start <= end => &content[start..=end],
        _ => "",
    }
}

fn fallback_prompt(raw: &str) -> StructuredPrompt {
    let preview: String = raw.chars().take(500).collect();
    StructuredPrompt {
        context: format!("The user wants to: {preview}"),
        role: "Expert assistant with deep knowledge in the relevant domain".to_string(),
        task: "1. Understand the user's requirements\n2. Provide a comprehensive solution\n3. Ensure clarity and completeness".to_string(),
        constraints: "- Be accurate and thorough\n- Follow best practices\n- Provide clear explanations".to_string(),
        output: "A well-structured response that fully addresses the user's needs".to_string(),
    }
}

impl StructuredPrompt {
    pub fn to_json_pretty(&self) -> Result<String, CliError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_llm_response(content: &str, raw: &str) -> Result<(Self, String), CliError> {
        let parsed = serde_json::from_str::<Self>(content)
            .or_else(|_| serde_json::from_str::<Self>(json_object_slice(content)));
        Ok(match parsed {
            Ok(prompt) => (fill(prompt), FIELD_NOTE.to_string()),
            Err(_) => (fallback_prompt(raw), AUTO_NOTE.to_string()),
        })
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

pub struct Pipeline {
    provider: Box<dyn EngineLlmProvider>,
    config: Config,
    options: PipelineRunOptions,
}

#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub prompt: StructuredPrompt,
    pub output_file: String,
    pub analysis: ComplexityResult,
    pub history: Vec<RefinementEntry>,
    pub refinement_note: Option<String>,
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
        }
    }

    pub async fn run(&mut self, raw: &str) -> Result<PipelineResult, CliError> {
        if let Some(message) = validation::validate_prompt(raw) {
            return Err(CliError::Validation(message));
        }
        let analysis = analysis::analyze_prompt_complexity(raw);
        let passes = self
            .options
            .iterations_override
            .unwrap_or(analysis.iterations)
            .max(1);
        let (prompt, history, note) = self.refine(raw, passes).await?;
        self.write_output(prompt, history, note, &analysis)
    }

    async fn refine(
        &self,
        raw: &str,
        passes: u32,
    ) -> Result<(StructuredPrompt, Vec<RefinementEntry>, Option<String>), CliError> {
        let initial = templates::get_initial_template(raw);
        let (mut prompt, edits) = self
            .complete(constants::LLM_SYSTEM_MESSAGE, &initial, raw)
            .await?;
        let mut history = vec![RefinementEntry {
            iteration: 1,
            edits,
        }];
        let mut note = None;
        for iteration in 2..=passes {
            let current = prompt.to_json_pretty()?;
            let template = templates::get_refinement_template(&current, iteration);
            match self
                .complete(
                    constants::LLM_REFINEMENT_SYSTEM_MESSAGE,
                    &template,
                    &current,
                )
                .await
            {
                Ok((next, edits)) => {
                    prompt = next;
                    history.push(RefinementEntry { iteration, edits });
                }
                Err(error) => {
                    tracing::warn!("Refinement {iteration} failed: {error}");
                    note = Some(error.to_string());
                    break;
                }
            }
        }
        Ok((prompt, history, note))
    }

    fn write_output(
        &self,
        prompt: StructuredPrompt,
        history: Vec<RefinementEntry>,
        note: Option<String>,
        analysis: &ComplexityResult,
    ) -> Result<PipelineResult, CliError> {
        let id = uuid::Uuid::new_v4().to_string();
        let path = resolve_output_file(self.config.output_file.as_deref(), &id[..8])?;
        let pairs: Vec<(u32, String)> = history
            .iter()
            .map(|entry| (entry.iteration, entry.edits.clone()))
            .collect();
        let markdown = templates::format_markdown_output(
            &prompt.to_json_pretty()?,
            &pairs,
            analysis,
            history.len(),
        );
        write_atomic::write_text_atomic(&path, &markdown)?;
        Ok(PipelineResult {
            prompt,
            output_file: path.to_string_lossy().to_string(),
            analysis: analysis.clone(),
            history,
            refinement_note: note,
        })
    }

    async fn complete(
        &self,
        system: &str,
        user: &str,
        raw: &str,
    ) -> Result<(StructuredPrompt, String), CliError> {
        let content = self.ask(&build_messages(system, user)).await?;
        StructuredPrompt::from_llm_response(&content, raw)
    }

    async fn ask(&self, messages: &[Message]) -> Result<String, CliError> {
        let options = ChatOptions {
            max_tokens: self.options.max_tokens,
            temperature: self.options.temperature,
        };
        let content = self
            .provider
            .chat(&self.config.model, messages, &options)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_llm_response_parses_valid_json() {
        let (prompt, _) = StructuredPrompt::from_llm_response(
            r#"{"context":"c","role":"r","task":"t","constraints":"x","output":"o"}"#,
            "raw",
        )
        .unwrap();
        assert_eq!(prompt.context, "c");
        assert_eq!(prompt.role, "r");
    }

    #[test]
    fn from_llm_response_falls_back_on_garbage() {
        let (prompt, note) =
            StructuredPrompt::from_llm_response("not json", "do the thing").unwrap();
        assert!(prompt.context.contains("do the thing"));
        assert!(note.contains("automatic"));
    }

    #[test]
    fn rejects_parent_dir_in_output_file() {
        let error = resolve_output_file(Some("../escape.md"), "abc").unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    }

    #[test]
    fn build_messages_has_system_and_user() {
        let messages = build_messages("sys", "user");
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
    }
}
