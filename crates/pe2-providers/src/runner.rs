use crate::client::{ProviderConfig, ProviderKind};
use crate::factory::create_client;
use pe2_core::config::{self, Config};
use pe2_core::constants;
use pe2_core::engine::{Pipeline, PipelineResult, PipelineRunOptions};
use pe2_core::errors::CliError;

pub async fn run_pipeline(
    cfg: Config,
    options: PipelineRunOptions,
    raw_prompt: &str,
) -> Result<PipelineResult, CliError> {
    let kind: ProviderKind = cfg.provider.parse()?;
    let mut provider_config = ProviderConfig::new(
        kind,
        config::resolve_api_key(&cfg.provider, cfg.api_key.as_deref()),
    );
    if kind == ProviderKind::Ollama {
        if let Ok(url) = std::env::var(constants::provider_env_var("ollama")) {
            if !url.trim().is_empty() {
                provider_config = provider_config.with_base_url(url);
            }
        }
    }
    let client = create_client(&provider_config)?;
    let mut pipeline = Pipeline::with_options(client, cfg, options);
    pipeline.run(raw_prompt).await
}
