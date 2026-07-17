use clap::Parser;
use pe2_tui::banner::TAGLINE;

fn long_help() -> &'static str {
    static HELP: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    HELP.get_or_init(|| {
        format!(
            "PE²-CLI: {TAGLINE}\n\n\
             Takes a rough prompt (text or file), calls a configured LLM,\n\
             and returns a structured PE²-style prompt with refinement iterations.\n\n\
             Examples:\n\
               pe2 \"Write a blog post about AI\"\n\
               pe2 --config\n\
               pe2 --provider openai --model gpt-4o-mini \"My prompt\"\n\
               pe2 -o output.md \"Save to file\""
        )
    })
}

#[derive(Parser, Debug)]
#[command(
    name = "pe2",
    version = env!("CARGO_PKG_VERSION"),
    about = "Convert raw prompts to PE²-structured optimized prompts",
    long_about = long_help()
)]
pub struct Args {
    #[arg(help = "Raw prompt text or path to prompt file (omit for interactive mode)")]
    pub prompt: Option<String>,

    #[arg(long, help = "Open interactive REPL (same as omitting prompt)")]
    pub config: bool,

    #[arg(long, short = 'p', help = "LLM provider (openai, anthropic, google, openrouter, ollama)")]
    pub provider: Option<String>,

    #[arg(long, short = 'm', help = "Model identifier for the selected provider")]
    pub model: Option<String>,

    #[arg(long, help = "API key for the provider")]
    pub api_key: Option<String>,

    #[arg(long, short = 'o', help = "Output file path")]
    pub output_file: Option<String>,

    #[arg(long, short = 'i', help = "Number of refinement iterations (overrides auto-detection)")]
    pub iterations: Option<u32>,

    #[arg(long, default_value_t = true, help = "Enable auto-difficulty detection")]
    pub auto_difficulty: bool,

    #[arg(long, default_value = "1024", help = "Max tokens for LLM response")]
    pub max_tokens: u32,

    #[arg(long, default_value_t = 0.3, help = "Temperature for LLM sampling")]
    pub temperature: f64,
}
