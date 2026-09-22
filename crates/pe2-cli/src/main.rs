use anyhow::Context as AnyhowContext;
use clap::Parser;
use pe2_cli::args::Args;
use pe2_core::config::{self, Config};
use pe2_core::engine::PipelineRunOptions;
use pe2_core::errors::CliError;
use pe2_tui::banner::print_banner;
use pe2_tui::display::print_error;
use pe2_tui::interactive::setup_and_run_interactive;
use pe2_tui::prompt_flow::generate_and_render;
use std::path::Path;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(error) = run(args).await {
        print_error(&error.to_string());
        std::process::exit(exit_code(&error));
    }
}

fn exit_code(error: &anyhow::Error) -> i32 {
    error
        .downcast_ref::<CliError>()
        .map(CliError::exit_code)
        .unwrap_or(1)
}

async fn run(args: Args) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();
    if args.config || args.prompt.is_none() {
        setup_and_run_interactive(run_options(&args)).await?;
        return Ok(());
    }
    let raw = load_prompt(args.prompt.as_deref().expect("prompt is set"))?;
    print_banner();
    generate_and_render(merged_config(&args), run_options(&args), &raw).await?;
    Ok(())
}

fn load_prompt(prompt: &str) -> anyhow::Result<String> {
    if Path::new(prompt).is_file() {
        std::fs::read_to_string(prompt)
            .with_context(|| format!("Failed to read prompt file: {prompt}"))
    } else {
        Ok(prompt.to_string())
    }
}

fn merged_config(args: &Args) -> Config {
    let mut config = config::load_config_or_default();
    if let Some(provider) = &args.provider {
        config.provider = provider.clone();
    }
    if let Some(model) = &args.model {
        config.model = model.clone();
    }
    if let Some(key) = &args.api_key {
        config.api_key = Some(key.clone());
    }
    config.output_file = args.output_file.clone();
    config
}

fn run_options(args: &Args) -> PipelineRunOptions {
    PipelineRunOptions {
        iterations_override: args
            .iterations
            .or(if args.auto_difficulty { None } else { Some(1) }),
        max_tokens: args.max_tokens,
        temperature: args.temperature,
    }
}
