use anyhow::Context as AnyhowContext;
use clap::Parser;
use pe2_cli::args::Args;
use pe2_core::config::{self, Config};
use pe2_core::engine::PipelineRunOptions;
use pe2_core::errors::CliError;
use pe2_tui::banner::print_banner;
use pe2_tui::display::print_error;
use pe2_tui::interactive::setup_and_run_interactive;
use pe2_tui::prompt_flow::{
    generate_prompt_with_spinner, render_complexity_preflight, render_generation_result,
};
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

    if args.config {
        return setup_and_run_interactive(pipeline_options(&args))
            .await
            .map_err(Into::into);
    }

    let Some(prompt_arg) = args.prompt.as_ref() else {
        return setup_and_run_interactive(pipeline_options(&args))
            .await
            .map_err(Into::into);
    };
    let prompt = load_prompt_text(prompt_arg)?;
    run_single_prompt(&args, &prompt).await
}

fn load_prompt_text(prompt: &str) -> anyhow::Result<String> {
    let text = if Path::new(prompt).exists() {
        std::fs::read_to_string(prompt)
            .with_context(|| format!("Failed to read prompt file: {prompt}"))?
    } else {
        prompt.to_string()
    };
    Ok(text)
}

async fn run_single_prompt(args: &Args, raw_prompt: &str) -> anyhow::Result<()> {
    print_banner();
    let cfg = build_config_from_args(args);
    let analysis = pe2_core::analysis::analyze_prompt_complexity(raw_prompt);
    render_complexity_preflight(&analysis, &cfg.provider, &cfg.model);

    let result = generate_prompt_with_spinner(cfg, pipeline_options(args), raw_prompt).await?;
    render_generation_result(&result);
    Ok(())
}

fn build_config_from_args(args: &Args) -> Config {
    let mut cfg = config::load_config_or_default();
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
    cfg
}

fn pipeline_options(args: &Args) -> PipelineRunOptions {
    PipelineRunOptions {
        iterations_override: resolve_iterations(args),
        max_tokens: args.max_tokens,
        temperature: args.temperature,
    }
}

fn resolve_iterations(args: &Args) -> Option<u32> {
    if args.iterations.is_some() {
        args.iterations
    } else if args.auto_difficulty {
        None
    } else {
        Some(1)
    }
}
