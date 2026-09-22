use crate::client::{create_client, ProviderConfig, ProviderKind};
use pe2_core::config::{self, Config};
use pe2_core::constants;
use pe2_core::engine::{Pipeline, PipelineResult, PipelineRunOptions};
use pe2_core::errors::CliError;

pub async fn run_pipeline(
    config: Config,
    options: PipelineRunOptions,
    prompt: &str,
) -> Result<PipelineResult, CliError> {
    let kind: ProviderKind = config.provider.parse()?;
    let mut provider = ProviderConfig::new(
        kind,
        config::resolve_api_key(&config.provider, config.api_key.as_deref()),
    );
    if kind == ProviderKind::Ollama {
        if let Ok(url) = std::env::var(constants::provider_env_var("ollama")) {
            if !url.trim().is_empty() {
                provider = provider.with_base_url(url);
            }
        }
    }
    Pipeline::with_options(create_client(&provider)?, config, options)
        .run(prompt)
        .await
}
