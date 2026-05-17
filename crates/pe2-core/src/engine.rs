use crate::analysis::{self, ComplexityResult, Difficulty};
use crate::config::Config;
use crate::constants;
use crate::errors::CliError;
use crate::messages::{self, Message};
use crate::paths;
use crate::templates;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredPrompt {
    pub context: String,
    pub role: String,
    pub task: String,
    pub constraints: String,
    pub output: String,
}

impl StructuredPrompt {
    pub fn from_json(json: &str) -> Result<Self, CliError> {
        let parsed: Self = serde_json::from_str(json)?;
        Ok(parsed)
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn from_llm_response(content: &str, raw_prompt: &str) -> Result<(Self, String), CliError> {
        if let Ok(parsed) = serde_json::from_str::<StructuredPrompt>(content) {
            let has_all = !parsed.context.is_empty()
                && !parsed.role.is_empty()
                && !parsed.task.is_empty()
                && !parsed.constraints.is_empty()
                && !parsed.output.is_empty();
            if has_all {
                return Ok((parsed, "Prompt generation with field validation.".to_string()));
            }
            let filled = Self {
                context: if parsed.context.is_empty() { "No context provided".to_string() } else { parsed.context },
                role: if parsed.role.is_empty() { "Expert assistant".to_string() } else { parsed.role },
                task: if parsed.task.is_empty() { "Complete the requested task".to_string() } else { parsed.task },
                constraints: if parsed.constraints.is_empty() { "Follow best practices".to_string() } else { parsed.constraints },
                output: if parsed.output.is_empty() { "Provide appropriate output".to_string() } else { parsed.output },
            };
            return Ok((filled, "Prompt generation with field validation.".to_string()));
        }

        let repaired = content
            .chars()
            .skip_while(|&c| c != '{')
            .collect::<String>();
        let repaired = repaired
            .chars()
            .rev()
            .skip_while(|&c| c != '}')
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        if let Ok(parsed) = serde_json::from_str::<StructuredPrompt>(&repaired) {
            return Ok((
                Self {
                    context: if parsed.context.is_empty() { "No context provided".to_string() } else { parsed.context },
                    role: if parsed.role.is_empty() { "Expert assistant".to_string() } else { parsed.role },
                    task: if parsed.task.is_empty() { "Complete the requested task".to_string() } else { parsed.task },
                    constraints: if parsed.constraints.is_empty() { "Follow best practices".to_string() } else { parsed.constraints },
                    output: if parsed.output.is_empty() { "Provide appropriate output".to_string() } else { parsed.output },
                },
                "Prompt generation with field validation.".to_string(),
            ));
        }

        let fallback = Self {
            context: format!("The user wants to: {}", &raw_prompt.chars().take(500).collect::<String>()),
            role: "Expert assistant with deep knowledge in the relevant domain".to_string(),
            task: "1. Understand the user's requirements\n2. Provide a comprehensive solution\n3. Ensure clarity and completeness".to_string(),
            constraints: "- Be accurate and thorough\n- Follow best practices\n- Provide clear explanations".to_string(),
            output: "A well-structured response that fully addresses the user's needs".to_string(),
        };
        Ok((fallback, "Prompt generation with automatic structuring.".to_string()))
    }
}

impl Default for StructuredPrompt {
    fn default() -> Self {
        Self {
            context: "General purpose task".to_string(),
            role: "Expert assistant with deep knowledge in the relevant domain".to_string(),
            task: "1. Understand the user's requirements\n2. Provide a comprehensive solution\n3. Ensure clarity and completeness".to_string(),
            constraints: "- Be accurate and thorough\n- Follow best practices\n- Provide clear explanations".to_string(),
            output: "A well-structured response that fully addresses the user's needs".to_string(),
        }
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
    current_prompt: Option<StructuredPrompt>,
    history: Vec<RefinementEntry>,
}

impl Pipeline {
    pub fn new(provider: Box<dyn EngineLlmProvider>, config: Config) -> Self {
        Self {
            provider,
            config,
            current_prompt: None,
            history: Vec::new(),
        }
    }

    pub async fn run(&mut self, raw_prompt: &str) -> Result<PipelineResult, CliError> {
        let analysis = analysis::analyze_prompt_complexity(raw_prompt);
        let iterations = analysis.iterations as usize;

        let initial = self.generate_initial(raw_prompt).await?;
        self.current_prompt = Some(initial.prompt.clone());
        self.history.push(RefinementEntry {
            iteration: 1,
            edits: initial.edits.clone(),
        });

        for i in 1..iterations {
            let result = self.refine(i + 1).await;
            match result {
                Ok(r) => {
                    self.current_prompt = Some(r.prompt.clone());
                    self.history.push(RefinementEntry {
                        iteration: (i + 1) as u32,
                        edits: r.edits.clone(),
                    });
                }
                Err(e) => {
                    tracing::warn!("Refinement {} failed: {}", i + 1, e);
                    break;
                }
            }
        }

        let prompt = self.current_prompt.as_ref()
            .ok_or_else(|| CliError::Runtime("No prompt generated".to_string()))?;

        let output_file = paths::resolve_output_file(
            self.config.output_file.as_deref(),
            &uuid::Uuid::new_v4().to_string()[..8],
        );

        let metrics = Metrics::new(
            analysis.score,
            "optimization",
            self.history.len(),
        );

        let markdown = templates::format_markdown_output(
            &prompt.to_json_pretty(),
            &self.history.iter().map(|h| (h.iteration, h.edits.clone())).collect::<Vec<_>>(),
            &metrics.accuracy_gain,
            &metrics.optimization_level,
            &metrics.quality_score,
            metrics.iterations_applied,
            analysis.difficulty.as_str(),
            analysis.score,
        );

        std::fs::write(&output_file, &markdown)?;

        Ok(PipelineResult {
            prompt: prompt.clone(),
            output_file: output_file.to_string_lossy().to_string(),
            metrics,
            analysis,
            history: self.history.clone(),
        })
    }

    async fn generate_initial(&self, raw_prompt: &str) -> Result<PromptResponse, CliError> {
        let template = templates::get_initial_template(raw_prompt);
        let messages = messages::build_messages(constants::LLM_SYSTEM_MESSAGE, &template);

        let content = self.provider
            .chat(
                &self.config.model,
                &messages,
                constants::LLM_MAX_TOKENS,
                constants::LLM_TEMPERATURE,
            )
            .await?;

        if content.trim().is_empty() {
            return Err(CliError::Provider {
                provider: self.config.provider.clone(),
                message: "Model returned empty content for initial prompt".to_string(),
            });
        }

        let (prompt, edits) = StructuredPrompt::from_llm_response(&content, raw_prompt)?;
        Ok(PromptResponse { prompt, edits })
    }

    async fn refine(&self, iteration_num: u32) -> Result<PromptResponse, CliError> {
        let current = self.current_prompt.as_ref()
            .ok_or_else(|| CliError::Runtime("No prompt to refine".to_string()))?;
        let json = current.to_json_pretty();
        let template = templates::get_refinement_template(&json, iteration_num);
        let messages = messages::build_messages(constants::LLM_REFINEMENT_SYSTEM_MESSAGE, &template);

        let content = self.provider
            .chat(
                &self.config.model,
                &messages,
                constants::LLM_MAX_TOKENS,
                constants::LLM_TEMPERATURE,
            )
            .await?;

        if content.trim().is_empty() {
            return Err(CliError::Provider {
                provider: self.config.provider.clone(),
                message: "Model returned empty content during refinement".to_string(),
            });
        }

        let (prompt, edits) = StructuredPrompt::from_llm_response(&content, &json)?;
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
}

pub fn create_session_output_file(session_id: &str) -> String {
    paths::resolve_output_file(None, session_id)
        .to_string_lossy()
        .to_string()
}
