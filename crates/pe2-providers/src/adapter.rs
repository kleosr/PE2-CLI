use async_trait::async_trait;
use pe2_core::engine::EngineLlmProvider;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use crate::client::LlmClient;

pub struct LlmClientAdapter {
    inner: Box<dyn LlmClient>,
}

impl LlmClientAdapter {
    pub fn new(inner: Box<dyn LlmClient>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl EngineLlmProvider for LlmClientAdapter {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        max_tokens: u32,
        temperature: f64,
    ) -> Result<String, CliError> {
        let resp = self
            .inner
            .chat(model, messages, max_tokens, temperature)
            .await?;
        Ok(resp.content)
    }
}
