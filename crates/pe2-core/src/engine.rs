use crate::analysis::{self, ComplexityResult};
use crate::config::Config;
use crate::constants;
use crate::errors::CliError;
use crate::templates;
use crate::validation;
use crate::write_atomic;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::{Component, PathBuf};

const FV: &str = "Prompt generation with field validation.";
const AU: &str = "Prompt generation with automatic structuring.";

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}
pub fn build_messages(s: &str, u: &str) -> Vec<Message> {
    vec![
        Message {
            role: "system".to_string(),
            content: s.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: u.to_string(),
        },
    ]
}
fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
pub fn resolve_output_file(o: Option<&str>, s: &str) -> Result<PathBuf, std::io::Error> {
    if let Some(f) = o {
        let q = if PathBuf::from(f).is_absolute() {
            PathBuf::from(f)
        } else {
            cwd().join(f)
        };
        if q.components().any(|c| matches!(c, Component::ParentDir)) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "output path must not contain '..'",
            ));
        }
        return Ok(q);
    }
    let d = cwd().join("pe2-prompts");
    std::fs::create_dir_all(&d)?;
    Ok(d.join(format!("pe2-session-{s}.md")))
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredPrompt {
    pub context: String,
    pub role: String,
    pub task: String,
    pub constraints: String,
    pub output: String,
}
fn fill(mut p: StructuredPrompt) -> StructuredPrompt {
    macro_rules! d {
        ($f:ident, $v:expr) => {
            if p.$f.is_empty() {
                p.$f = $v.to_string();
            }
        };
    }
    d!(context, "No context provided");
    d!(role, "Expert assistant");
    d!(task, "Complete the requested task");
    d!(constraints, "Follow best practices");
    d!(output, "Provide appropriate output");
    p
}
fn js(c: &str) -> &str {
    match (c.find('{'), c.rfind('}')) {
        (Some(a), Some(b)) if a <= b => &c[a..=b],
        _ => "",
    }
}
fn fb(r: &str) -> StructuredPrompt {
    StructuredPrompt {
        context: format!("The user wants to: {}", r.chars().take(500).collect::<String>()),
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
    pub fn from_llm_response(c: &str, r: &str) -> Result<(Self, String), CliError> {
        Ok(serde_json::from_str::<Self>(c)
            .or_else(|_| serde_json::from_str::<Self>(js(c)))
            .map(|p| (fill(p), FV.to_string()))
            .unwrap_or_else(|_| (fb(r), AU.to_string())))
    }
}
impl Default for StructuredPrompt {
    fn default() -> Self {
        fb("General purpose task")
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
pub struct Pipeline {
    pv: Box<dyn EngineLlmProvider>,
    cfg: Config,
    opt: PipelineRunOptions,
    cur: Option<StructuredPrompt>,
    hist: Vec<RefinementEntry>,
    note: Option<String>,
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
    pub fn new(pv: Box<dyn EngineLlmProvider>, cfg: Config) -> Self {
        Self::with_options(pv, cfg, PipelineRunOptions::default())
    }
    pub fn with_options(
        pv: Box<dyn EngineLlmProvider>,
        cfg: Config,
        opt: PipelineRunOptions,
    ) -> Self {
        Self {
            pv,
            cfg,
            opt,
            cur: None,
            hist: Vec::new(),
            note: None,
        }
    }
    pub async fn run(&mut self, r: &str) -> Result<PipelineResult, CliError> {
        if let Some(m) = validation::validate_prompt(r) {
            return Err(CliError::Validation(m));
        }
        let a = analysis::analyze_prompt_complexity(r);
        self.run_refinements(
            r,
            self.opt.iterations_override.unwrap_or(a.iterations).max(1) as usize,
        )
        .await?;
        self.write_result(&a)
    }
    async fn run_refinements(&mut self, r: &str, n: usize) -> Result<(), CliError> {
        let t = templates::get_initial_template(r);
        let (p, e) = self.step(constants::LLM_SYSTEM_MESSAGE, &t, r).await?;
        self.cur = Some(p);
        self.hist.push(RefinementEntry {
            iteration: 1,
            edits: e,
        });
        for i in 2..=n as u32 {
            let j = self
                .cur
                .as_ref()
                .ok_or_else(|| CliError::Runtime("No prompt to refine".to_string()))?
                .to_json_pretty()?;
            let t = templates::get_refinement_template(&j, i);
            match self
                .step(constants::LLM_REFINEMENT_SYSTEM_MESSAGE, &t, &j)
                .await
            {
                Ok((p, e)) => {
                    self.cur = Some(p);
                    self.hist.push(RefinementEntry {
                        iteration: i,
                        edits: e,
                    });
                }
                Err(e) => {
                    tracing::warn!("Refinement {i} failed: {e}");
                    self.note = Some(e.to_string());
                    break;
                }
            }
        }
        Ok(())
    }
    fn write_result(&self, a: &ComplexityResult) -> Result<PipelineResult, CliError> {
        let p = self
            .cur
            .as_ref()
            .ok_or_else(|| CliError::Runtime("No prompt generated".to_string()))?;
        let f = resolve_output_file(
            self.cfg.output_file.as_deref(),
            &uuid::Uuid::new_v4().to_string()[..8],
        )?;
        let m = templates::format_markdown_output(
            &p.to_json_pretty()?,
            &self
                .hist
                .iter()
                .map(|h| (h.iteration, h.edits.clone()))
                .collect::<Vec<_>>(),
            a,
            self.hist.len(),
        );
        write_atomic::write_text_atomic(&f, &m)?;
        Ok(PipelineResult {
            prompt: p.clone(),
            output_file: f.to_string_lossy().to_string(),
            analysis: a.clone(),
            history: self.hist.clone(),
            refinement_note: self.note.clone(),
        })
    }
    async fn step(
        &self,
        s: &str,
        t: &str,
        r: &str,
    ) -> Result<(StructuredPrompt, String), CliError> {
        StructuredPrompt::from_llm_response(&self.call(&build_messages(s, t)).await?, r)
    }
    async fn call(&self, m: &[Message]) -> Result<String, CliError> {
        let c = self
            .pv
            .chat(
                &self.cfg.model,
                m,
                &ChatOptions {
                    max_tokens: self.opt.max_tokens,
                    temperature: self.opt.temperature,
                },
            )
            .await?;
        if c.trim().is_empty() {
            return Err(CliError::Provider {
                provider: self.cfg.provider.clone(),
                message: "Model returned empty content".to_string(),
            });
        }
        Ok(c)
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
        let (p, _) = StructuredPrompt::from_llm_response(
            r#"{"context":"c","role":"r","task":"t","constraints":"x","output":"o"}"#,
            "raw",
        )
        .unwrap();
        assert_eq!(p.context, "c");
        assert_eq!(p.role, "r");
    }
    #[test]
    fn from_llm_response_falls_back_on_garbage() {
        let (p, e) = StructuredPrompt::from_llm_response("not json", "do the thing").unwrap();
        assert!(p.context.contains("do the thing"));
        assert!(e.contains("automatic"));
    }
    #[test]
    fn rejects_parent_dir_in_output_file() {
        let e = resolve_output_file(Some("../escape.md"), "abc").unwrap_err();
        assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput);
    }
    #[test]
    fn build_messages_has_system_and_user() {
        let m = build_messages("sys", "user");
        assert_eq!(m.len(), 2);
        assert_eq!(m[0].role, "system");
        assert_eq!(m[1].role, "user");
    }
}
