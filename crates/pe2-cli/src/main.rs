use anyhow::Context as AnyhowContext;
use async_trait::async_trait;
use clap::Parser;
use colored::Colorize;
use pe2_cli::args::Args;
use pe2_core::config;
use pe2_core::engine::EngineLlmProvider;
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use pe2_providers::client::{LlmClient, ProviderConfig, ProviderKind};
use pe2_tui::banner::print_banner;
use pe2_tui::display::{
    create_spinner, print_complexity_analysis, print_error, print_info, print_metrics,
    print_prompt_result, print_refinement_history, print_success,
};
use pe2_tui::interactive::setup_and_run_interactive;
use std::path::Path;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(args).await {
        print_error(&format!("{}", e));
        std::process::exit(exit_code(&e));
    }
}

fn exit_code(err: &anyhow::Error) -> i32 {
    if let Some(cli_err) = err.downcast_ref::<CliError>() {
        cli_err.exit_code()
    } else {
        1
    }
}

async fn run(args: Args) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    if args.config || args.prompt.is_none() {
        return setup_and_run_interactive().await;
    }

    let prompt = args.prompt.as_ref().unwrap();
    let prompt = if Path::new(prompt).exists() {
        std::fs::read_to_string(prompt)
            .with_context(|| format!("Failed to read prompt file: {}", prompt))?
    } else {
        prompt.clone()
    };

    if prompt.trim().is_empty() {
        anyhow::bail!("Prompt cannot be empty");
    }

    run_single_prompt(args, &prompt).await
}

async fn run_single_prompt(args: Args, raw_prompt: &str) -> anyhow::Result<()> {
    print_banner();

    let analysis = pe2_core::analysis::analyze_prompt_complexity(raw_prompt);
    print_complexity_analysis(&analysis);

    let mut cfg = config::load_config();
    if let Some(provider) = &args.provider {
        cfg.provider = provider.clone();
    }
    if let Some(model) = &args.model {
        cfg.model = model.clone();
    }
    if let Some(key) = &args.api_key {
        cfg.api_key = Some(key.clone());
    }
    cfg.output_file = args.output_file.clone();

    let kind = ProviderKind::from_str_result(&cfg.provider)?;
    let provider_config = ProviderConfig {
        kind,
        base_url: None,
        api_key: config::resolve_api_key(&cfg.provider, cfg.api_key.as_deref()),
    };

    print_info(&format!(
        "Using {} / {}",
        cfg.provider.bright_cyan(),
        cfg.model.bright_white()
    ));

    let spinner = create_spinner("Generating prompt...");

    let raw_client = pe2_providers::factory::create_client(&provider_config)?;

    struct SingleClientAdapter {
        inner: Box<dyn LlmClient>,
    }

    #[async_trait]
    impl EngineLlmProvider for SingleClientAdapter {
        async fn chat(
            &self,
            model: &str,
            messages: &[Message],
            max_tokens: u32,
            temperature: f64,
        ) -> Result<String, CliError> {
            let resp = self.inner.chat(model, messages, max_tokens, temperature).await?;
            Ok(resp.content)
        }
    }

    let adapter = SingleClientAdapter { inner: raw_client };
    let mut pipeline = pe2_core::engine::Pipeline::new(Box::new(adapter), cfg.clone());
    let result = pipeline.run(raw_prompt).await?;

    spinner.finish_and_clear();

    print_success("Prompt generation complete!");
    print_prompt_result(&result.prompt, &result.output_file);
    print_refinement_history(&result.history);
    print_metrics(&result.metrics);

    Ok(())
}


