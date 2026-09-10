use crate::client::{create_client, ProviderConfig, ProviderKind};
use pe2_core::config::{self, Config};
use pe2_core::constants;
use pe2_core::engine::{Pipeline, PipelineResult, PipelineRunOptions};
use pe2_core::errors::CliError;

pub async fn run_pipeline(
    cfg: Config,
    o: PipelineRunOptions,
    p: &str,
) -> Result<PipelineResult, CliError> {
    let k: ProviderKind = cfg.provider.parse()?;
    let mut c = ProviderConfig::new(
        k,
        config::resolve_api_key(&cfg.provider, cfg.api_key.as_deref()),
    );
    if k == ProviderKind::Ollama {
        if let Ok(u) = std::env::var(constants::provider_env_var("ollama")) {
            if !u.trim().is_empty() {
                c = c.with_base_url(u);
            }
        }
    }
    let mut x = Pipeline::with_options(create_client(&c)?, cfg, o);
    x.run(p).await
}
