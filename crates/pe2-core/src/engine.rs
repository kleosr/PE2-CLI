use crate::analysis::{self, ComplexityResult};
use crate::config::Config;
use crate::constants;
use crate::errors::CliError;
use crate::messages::{self, Message};
use crate::paths;
use crate::templates;
use crate::validation;
use crate::write_atomic;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const FIELD_VALIDATION_EDITS: &str = "Prompt generation with field validation.";
const AUTO_STRUCTURING_EDITS: &str = "Prompt generation with automatic structuring.";

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

fn extract_json_object(content: &str) -> String {
    let trimmed = content
        .chars()
        .skip_while(|&c| c != '{')
        .collect::<String>();
    trimmed
        .chars()
        .rev()
        .skip_while(|&c| c != '}')
        .collect::<String>()
        .chars()
        .rev()
        .collect()
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
    pub fn from_json(json: &str) -> Result<Self, CliError> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn to_json_pretty(&self) -> Result<String, CliError> {
        serde_json::to_string_pretty(self).map_err(CliError::Json)
    }

    pub fn from_llm_response(content: &str, raw_prompt: &str) -> Result<(Self, String), CliError> {
        if let Ok(parsed) = serde_json::from_str::<Self>(content) {
            if has_all_fields(&parsed) {
                return Ok((parsed, FIELD_VALIDATION_EDITS.to_string()));
            }
            return Ok((fill_empty_fields(parsed), FIELD_VALIDATION_EDITS.to_string()));
        }

        let repaired = extract_json_object(content);
        if let Ok(parsed) = serde_json::from_str::<Self>(&repaired) {
            return Ok((fill_empty_fields(parsed), FIELD_VALIDATION_EDITS.to_string()));
        }

        Ok((fallback_prompt(raw_prompt), AUTO_STRUCTURING_EDITS.to_string()))
    }
}

impl Default for StructuredPrompt {
    fn default() -> Self {
        fallback_prompt("General purpose task")
    }
}

#[async_trait]
pub trait EngineLlmProvider: Send + Sync {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        max_tokens: u32,
        temperature: f64,
    ) -> Result<String, CliError>;
}

#[derive(Debug, Clone)]
pub struct RefinementEntry {
    pub iteration: u32,
    pub edits: String,
}

#[derive(Debug, Clone)]
pub struct Metrics {
    pub accuracy_gain: String,
    pub optimization_level: String,
    pub quality_score: String,
    pub iterations_applied: usize,
}

impl Metrics {
    pub fn new(complexity_score: u32, strategy_focus: &str, history_len: usize) -> Self {
        let gain = constants::DEFAULT_QUALITY_SCORE as u32 + complexity_score * 3;
        Self {
            accuracy_gain: format!("Estimated {}% improvement", gain),
            optimization_level: strategy_focus.to_string(),
            quality_score: format!("{:.1}", constants::DEFAULT_QUALITY_SCORE),
            iterations_applied: history_len,
        }
    }
}

pub struct Pipeline {
    provider: Box<dyn EngineLlmProvider>,
    config: Config,
    options: PipelineRunOptions,
    current_prompt: Option<StructuredPrompt>,
    history: Vec<RefinementEntry>,
    refinement_note: Option<String>,
}

#[derive(Debug, Clone)]
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
        self.write_result(raw_prompt, &analysis).await
    }

    fn resolve_iterations(&self, analysis: &ComplexityResult) -> usize {
        self.options
            .iterations_override
            .unwrap_or(analysis.iterations)
            .max(1) as usize
    }

    async fn run_refinements(&mut self, raw_prompt: &str, iterations: usize) -> Result<(), CliError> {
        let initial = self.generate_initial(raw_prompt).await?;
        self.current_prompt = Some(initial.prompt.clone());
        self.history.push(RefinementEntry {
            iteration: 1,
            edits: initial.edits.clone(),
        });

        for i in 1..iterations {
            match self.refine((i + 1) as u32).await {
                Ok(r) => {
                    self.current_prompt = Some(r.prompt.clone());
                    self.history.push(RefinementEntry {
                        iteration: (i + 1) as u32,
                        edits: r.edits.clone(),
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

    async fn write_result(
        &self,
        _raw_prompt: &str,
        analysis: &ComplexityResult,
    ) -> Result<PipelineResult, CliError> {
        let prompt = self
            .current_prompt
            .as_ref()
            .ok_or_else(|| CliError::Runtime("No prompt generated".to_string()))?;

        let output_file = paths::resolve_output_file(
            self.config.output_file.as_deref(),
            &uuid::Uuid::new_v4().to_string()[..8],
        )?;

        let metrics = Metrics::new(analysis.score, "optimization", self.history.len());
        let markdown = templates::format_markdown_output(
            &prompt.to_json_pretty()?,
            &self
                .history
                .iter()
                .map(|h| (h.iteration, h.edits.clone()))
                .collect::<Vec<_>>(),
            &templates::MarkdownMetrics {
                accuracy: &metrics.accuracy_gain,
                optimization: &metrics.optimization_level,
                quality: &metrics.quality_score,
                iterations: metrics.iterations_applied,
                difficulty: analysis.difficulty.as_str(),
                complexity_score: analysis.score,
            },
        );

        write_atomic::write_text_atomic(&output_file, &markdown)?;

        Ok(PipelineResult {
            prompt: prompt.clone(),
            output_file: output_file.to_string_lossy().to_string(),
            metrics,
            analysis: analysis.clone(),
            history: self.history.clone(),
            refinement_note: self.refinement_note.clone(),
        })
    }

    async fn generate_initial(&self, raw_prompt: &str) -> Result<PromptResponse, CliError> {
        let template = templates::get_initial_template(raw_prompt);
        let messages = messages::build_messages(constants::LLM_SYSTEM_MESSAGE, &template);
        let content = self
            .call_provider(&messages)
            .await?;
        self.parse_provider_content(&content, raw_prompt)
    }

    async fn refine(&self, iteration_num: u32) -> Result<PromptResponse, CliError> {
        let current = self
            .current_prompt
            .as_ref()
            .ok_or_else(|| CliError::Runtime("No prompt to refine".to_string()))?;
        let json = current.to_json_pretty()?;
        let template = templates::get_refinement_template(&json, iteration_num);
        let messages =
            messages::build_messages(constants::LLM_REFINEMENT_SYSTEM_MESSAGE, &template);
        let content = self.call_provider(&messages).await?;
        self.parse_provider_content(&content, &json)
    }

    async fn call_provider(&self, messages: &[Message]) -> Result<String, CliError> {
        let content = self
            .provider
            .chat(
                &self.config.model,
                messages,
                self.options.max_tokens,
                self.options.temperature,
            )
            .await?;
        if content.trim().is_empty() {
            return Err(CliError::Provider {
                provider: self.config.provider.clone(),
                message: "Model returned empty content".to_string(),
            });
        }
        Ok(content)
    }

    fn parse_provider_content(
        &self,
        content: &str,
        raw_prompt: &str,
    ) -> Result<PromptResponse, CliError> {
        let (prompt, edits) = StructuredPrompt::from_llm_response(content, raw_prompt)?;
        Ok(PromptResponse { prompt, edits })
    }
}

#[derive(Debug, Clone)]
pub struct PromptResponse {
    pub prompt: StructuredPrompt,
    pub edits: String,
}

#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub prompt: StructuredPrompt,
    pub output_file: String,
    pub metrics: Metrics,
    pub analysis: ComplexityResult,
    pub history: Vec<RefinementEntry>,
    pub refinement_note: Option<String>,
}
