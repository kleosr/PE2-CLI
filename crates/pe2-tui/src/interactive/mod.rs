mod slash_commands;
mod prompt;

use colored::Colorize;
use crossterm::{
    cursor,
    execute,
    terminal::{Clear, ClearType},
};
use pe2_core::config;
use pe2_core::engine::PipelineRunOptions;
use pe2_core::errors::CliError;
use pe2_core::preferences::UserPreferences;
use pe2_core::session::SessionStore;
use pe2_core::stats::StatsTracker;
use pe2_core::validation::{
    resolve_slash_command, unknown_command_message, validate_and_suggest_command, CommandValidation,
    SlashCommand,
};
use std::io::{self, Write};
use crate::banner::{print_banner, print_banner_brief};
use crate::display::{print_error, print_info};

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
    - Launch with -i/--max-tokens/--temperature to apply those options here
"#;

pub struct InteractiveSession {
    pub(crate) config: pe2_core::config::Config,
    pub(crate) pipeline_options: PipelineRunOptions,
    pub(crate) session_store: SessionStore,
    pub(crate) stats: StatsTracker,
    pub(crate) preferences: UserPreferences,
}

impl InteractiveSession {
    pub fn new(
        config: pe2_core::config::Config,
        pipeline_options: PipelineRunOptions,
        session_store: SessionStore,
        stats: StatsTracker,
        preferences: UserPreferences,
    ) -> Self {
        Self {
            config,
            pipeline_options,
            session_store,
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

            if let Some(cmd) = resolve_slash_command(input) {
                if self.dispatch_slash_command(cmd, &mut stdout).await? {
                    break;
                }
                continue;
            }

            if input.starts_with('/') {
                let validation = validate_and_suggest_command(input);
                if matches!(validation, CommandValidation::Unknown { .. }) {
                    if let Some(msg) = unknown_command_message(&validation) {
                        print_error(&msg);
                    }
                    continue;
                }
            }

            prompt::run_prompt_input(self, input).await?;
        }

        Ok(())
    }

    async fn dispatch_slash_command(
        &mut self,
        cmd: SlashCommand,
        stdout: &mut io::Stdout,
    ) -> Result<bool, CliError> {
        match cmd {
            SlashCommand::Help => print_info(HELP_TEXT),
            SlashCommand::Config => slash_commands::edit_config(&mut self.config).await?,
            SlashCommand::Session => slash_commands::show_session(&self.session_store).await,
            SlashCommand::Prefs => slash_commands::show_preferences(&self.preferences).await,
            SlashCommand::Stats => slash_commands::show_stats(&self.stats).await,
            SlashCommand::Clear => {
                execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
                print_banner_brief();
            }
            SlashCommand::Exit => {
                print_info("Goodbye!");
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub async fn setup_and_run_interactive(options: PipelineRunOptions) -> Result<(), CliError> {
    let cfg = config::load_config_or_default();
    let mut interactive = InteractiveSession::new(
        cfg,
        options,
        SessionStore::new(),
        StatsTracker::new(),
        UserPreferences::new(),
    );
    interactive.run().await
}

#[cfg(test)]
mod tests {
    use pe2_core::validation::{resolve_slash_command, SlashCommand};

    #[test]
    fn slash_command_aliases_resolve() {
        assert_eq!(resolve_slash_command("/c"), Some(SlashCommand::Config));
        assert_eq!(resolve_slash_command("/stats"), Some(SlashCommand::Stats));
        assert_eq!(resolve_slash_command("/q"), Some(SlashCommand::Exit));
    }
}
