use pe2_core::config::{self, Config};
use pe2_core::engine::{Pipeline, PipelineResult, PipelineRunOptions};
use pe2_core::errors::CliError;
use crate::adapter::LlmClientAdapter;
use crate::client::{ProviderConfig, ProviderKind};
use crate::factory::create_client;

pub async fn run_pipeline(
    cfg: Config,
    options: PipelineRunOptions,
    raw_prompt: &str,
) -> Result<PipelineResult, CliError> {
    let kind = ProviderKind::from_str_result(&cfg.provider)?;
    let provider_config = ProviderConfig {
        kind,
        base_url: None,
        api_key: config::resolve_api_key(&cfg.provider, cfg.api_key.as_deref()),
    };
    let client = create_client(&provider_config)?;
    let mut pipeline = Pipeline::with_options(
        Box::new(LlmClientAdapter::new(client)),
        cfg,
        options,
    );
    pipeline.run(raw_prompt).await
}
