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
    let a = Args::parse();
    if let Err(e) = run(a).await {
        print_error(&format!("{e}"));
        std::process::exit(exit_code(&e));
    }
}
fn exit_code(e: &anyhow::Error) -> i32 {
    e.downcast_ref::<CliError>()
        .map(|c| c.exit_code())
        .unwrap_or(1)
}
async fn run(a: Args) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();
    if a.config || a.prompt.is_none() {
        return setup_and_run_interactive(opt(&a)).await.map_err(Into::into);
    }
    let r = load(a.prompt.as_ref().unwrap())?;
    print_banner();
    generate_and_render(cfg(&a), opt(&a), &r).await?;
    Ok(())
}
fn load(p: &str) -> anyhow::Result<String> {
    if Path::new(p).is_file() {
        std::fs::read_to_string(p).with_context(|| format!("Failed to read prompt file: {p}"))
    } else {
        Ok(p.to_string())
    }
}
fn cfg(a: &Args) -> Config {
    let mut c = config::load_config_or_default();
    if let Some(p) = &a.provider {
        c.provider = p.clone();
    }
    if let Some(m) = &a.model {
        c.model = m.clone();
    }
    if let Some(k) = &a.api_key {
        c.api_key = Some(k.clone());
    }
    c.output_file = a.output_file.clone();
    c
}
fn opt(a: &Args) -> PipelineRunOptions {
    PipelineRunOptions {
        iterations_override: a
            .iterations
            .or(if a.auto_difficulty { None } else { Some(1) }),
        max_tokens: a.max_tokens,
        temperature: a.temperature,
    }
}
