use async_trait::async_trait;
use colored::Colorize;
use crossterm::{
    cursor,
    execute,
    terminal::{self, Clear, ClearType},
};
use pe2_core::config::{self, Config};
use pe2_core::engine::{self, EngineLlmProvider, Pipeline};
use pe2_core::errors::CliError;
use pe2_core::messages::Message;
use pe2_core::preferences::UserPreferences;
use pe2_core::session::SessionManager;
use pe2_core::stats::StatsTracker;
use pe2_core::validation;
use pe2_providers::client::{LlmClient, ProviderConfig, ProviderKind};
use pe2_providers::factory::create_client;
use std::io::{self, Write};
use crate::banner::{print_banner, print_banner_brief};
use crate::display::{
    create_spinner, print_complexity_analysis, print_error, print_info, print_metrics,
    print_prompt_result, print_refinement_history, print_separator, print_success, print_warning,
};

struct ClientAdapter {
    inner: Box<dyn LlmClient>,
}

#[async_trait]
impl EngineLlmProvider for ClientAdapter {
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

const HELP_TEXT: &str = r#"
  Available Commands:
    /help, /h        Show this help
    /config, /c      Open configuration
    /session, /s     Show session info
    /prefs, /p       Show preferences
    /stats           Show usage statistics
    /clear           Clear screen
    /exit, /quit, /q Exit

  Tips:
    - Type a prompt to generate a PE²-optimized prompt
    - Prompts are automatically analyzed for complexity
    - Use --iterations flag for more refinement passes
"#;

pub struct InteractiveSession {
    config: Config,
    session: SessionManager,
    stats: StatsTracker,
    preferences: UserPreferences,
}

impl InteractiveSession {
    pub fn new(
        config: Config,
        session: SessionManager,
        stats: StatsTracker,
        preferences: UserPreferences,
    ) -> Self {
        Self {
            config,
            session,
            stats,
            preferences,
        }
    }

    pub async fn run(&mut self) -> Result<(), CliError> {
        print_banner();
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("  {} ", ">>>".bright_green().bold());
            stdout.flush()?;

            let mut input = String::new();
            stdin.read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            if let Some(cmd) = validation::parse_slash_command(input) {
                match cmd {
                    "/help" | "/h" => {
                        print_info(HELP_TEXT);
                    }
                    "/config" | "/c" => {
                        self.handle_config().await?;
                    }
                    "/session" | "/s" => {
                        self.handle_session().await;
                    }
                    "/prefs" | "/p" => {
                        self.handle_preferences().await;
                    }
                    "/stats" => {
                        self.handle_stats().await;
                    }
                    "/clear" => {
                        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
                        print_banner_brief();
                    }
                    "/exit" | "/quit" | "/q" => {
                        print_info("Goodbye!");
                        break;
                    }
                    _ => {
                        print_warning(&format!("Unknown command: {}. Type /help for available commands.", cmd));
                    }
                }
                continue;
            }

            self.process_input(input).await?;
        }

        Ok(())
    }

    async fn process_input(&mut self, raw_prompt: &str) -> Result<(), CliError> {
        if let Err(e) = validation::validate_prompt(raw_prompt) {
            print_error(&e.to_string());
            return Ok(());
        }

        let analysis = pe2_core::analysis::analyze_prompt_complexity(raw_prompt);
        print_complexity_analysis(&analysis);

        let kind = ProviderKind::from_str_result(&self.config.provider)?;
        let provider_config = ProviderConfig {
            kind,
            base_url: None,
            api_key: config::resolve_api_key(
                &self.config.provider,
                self.config.api_key.as_deref(),
            ),
        };

        print_info(&format!(
            "Using {} / {}",
            self.config.provider.bright_cyan(),
            self.config.model.bright_white()
        ));

        let spinner = create_spinner("Generating prompt...");

        let raw_client = create_client(&provider_config)?;
        let adapter = ClientAdapter { inner: raw_client };
        let mut pipeline = Pipeline::new(Box::new(adapter), self.config.clone());

        let result = pipeline.run(raw_prompt).await?;

        spinner.finish_and_clear();

        print_success("Prompt generation complete!");
        print_prompt_result(&result.prompt, &result.output_file);
        print_refinement_history(&result.history);
        print_metrics(&result.metrics);

        self.session.add_entry(pe2_core::session::SessionEntry {
            prompt: raw_prompt.to_string(),
            output: result.output_file.clone(),
            model: self.config.model.clone(),
            provider: self.config.provider.clone(),
            difficulty: result.analysis.difficulty.label().to_string(),
            score: result.analysis.score,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }).await;
        self.session.save().await;

        self.stats.record_usage(&self.config.provider);
        self.stats.save().await;

        Ok(())
    }

    async fn handle_config(&mut self) -> Result<(), CliError> {
        println!();
        print_info("Configuration (press Enter to keep current value):");
        print_separator();

        let current_provider = &self.config.provider;
        print!("  {} [{}]: ", "Provider".bright_white(), current_provider.dimmed());
        io::stdout().flush()?;
        let mut provider = String::new();
        io::stdin().read_line(&mut provider)?;
        let provider = provider.trim();
        if !provider.is_empty() {
            self.config.provider = provider.to_string();
        }

        let current_model = &self.config.model;
        print!("  {} [{}]: ", "Model".bright_white(), current_model.dimmed());
        io::stdout().flush()?;
        let mut model = String::new();
        io::stdin().read_line(&mut model)?;
        let model = model.trim();
        if !model.is_empty() {
            self.config.model = model.to_string();
        }

        let masked = self.config.api_key.as_deref()
            .map(|k| {
                let len = k.len();
                if len > 12 {
                    format!("{}...{}", &k[..4], &k[len-4..])
                } else {
                    "****".to_string()
                }
            })
            .unwrap_or_else(|| "not set".to_string());
        print!("  {} [{}]: ", "API Key".bright_white(), masked.dimmed());
        io::stdout().flush()?;
        let mut key = String::new();
        io::stdin().read_line(&mut key)?;
        let key = key.trim();
        if !key.is_empty() {
            self.config.api_key = Some(key.to_string());
        }

        config::save_config(&self.config)?;
        print_success("Configuration saved!");
        println!();
        Ok(())
    }

    async fn handle_session(&self) {
        let entries = self.session.entries.lock().await;
        if entries.is_empty() {
            println!("  {}", "No sessions recorded yet.".dimmed());
            return;
        }
        println!();
        println!("  {} {}", "◆".bright_cyan(), "Session History".bright_white().bold());
        println!();
        for (i, entry) in entries.iter().rev().take(10).enumerate() {
            let preview: String = entry.prompt.chars().take(60).collect();
            println!(
                "  {} {}. {} {}",
                " ".dimmed(),
                (i + 1).to_string().bright_blue(),
                preview.dimmed(),
                format!("[{}]", entry.difficulty).dimmed(),
            );
        }
        println!();
    }

    async fn handle_preferences(&self) {
        println!();
        println!("  {} {}", "◆".bright_yellow(), "Preferences".bright_white().bold());
        println!();
        println!("  {} {}", "  Theme:".dimmed(), self.preferences.theme().bright_white());
        println!("  {} {}", "  Compact:".dimmed(), format!("{}", self.preferences.compact()).bright_white());
        println!("  {} {}", "  Track Usage:".dimmed(), format!("{}", self.preferences.track_usage()).bright_white());
        println!();
    }

    async fn handle_stats(&self) {
        let daily_stats = self.stats.daily_usage.lock().await;
        if daily_stats.is_empty() {
            println!("  {}", "No usage statistics yet.".dimmed());
            return;
        }
        println!();
        println!("  {} {}", "◆".bright_green(), "Usage Statistics".bright_white().bold());
        println!();
        let mut sorted: Vec<_> = daily_stats.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (provider, count) in sorted.iter().take(10) {
            println!("  {} {} {} {}", " ".dimmed(), provider.bright_cyan(), "·".dimmed(), count.to_string().bright_white());
        }
        println!();
    }
}

pub async fn setup_and_run_interactive() -> Result<(), CliError> {
    let cfg = config::load_config();
    let session = SessionManager::new();
    let stats = StatsTracker::new();
    let preferences = UserPreferences::new();

    let mut interactive = InteractiveSession::new(cfg, session, stats, preferences);
    interactive.run().await
}
