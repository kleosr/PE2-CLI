use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "pe2",
    version = "4.0.2",
    about = "Convert raw prompts to PE²-structured optimized prompts",
    long_about = r#"PE²-CLI: Structured Prompt Generation v4 — KleoSr Pro Edition

Takes a rough prompt (text or file), calls a configured LLM,
and returns a structured PE²-style prompt with refinement iterations.

Examples:
  pe2 "Write a blog post about AI"
  pe2 --config
  pe2 prompt.txt --iterations 3
  pe2 --provider ollama --model llama3.2 "Explain quantum computing"
"#
)]
pub struct Args {
    /// Raw prompt text or path to prompt file (omit for interactive mode)
    #[arg()]
    pub prompt: Option<String>,

    /// Open interactive configuration mode
    #[arg(long)]
    pub config: bool,

    /// LLM provider (openai, anthropic, google, openrouter, ollama)
    #[arg(long, short = 'p')]
    pub provider: Option<String>,

    /// Model identifier for the selected provider
    #[arg(long, short = 'm')]
    pub model: Option<String>,

    /// API key for the provider
    #[arg(long)]
    pub api_key: Option<String>,

    /// Output file path
    #[arg(long, short = 'o')]
    pub output_file: Option<String>,

    /// Number of refinement iterations (overrides auto-detection)
    #[arg(long, short = 'i')]
    pub iterations: Option<u32>,

    /// Enable auto-difficulty detection
    #[arg(long)]
    pub auto_difficulty: bool,

    /// Max tokens for LLM response
    #[arg(long, default_value = "1024")]
    pub max_tokens: u32,

    /// Temperature for LLM sampling
    #[arg(long, default_value_t = 0.3)]
    pub temperature: f64,
}
