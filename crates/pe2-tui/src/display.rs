use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use pe2_core::analysis::ComplexityResult;
use pe2_core::engine::{RefinementEntry, StructuredPrompt};
use pe2_core::errors::CliError;

const SPINNER_TEMPLATE: &str = "{spinner:.green} {msg}";

fn row(label: &str, value: impl std::fmt::Display) {
    println!("  {} {}: {}", "◇".bright_blue(), label.bright_blue(), value);
}

pub fn print_complexity_analysis(analysis: &ComplexityResult) {
    println!();
    row(
        "Difficulty",
        format!(
            "{} {} ({} iterations)",
            analysis.difficulty.emoji(),
            analysis.difficulty.label().bold(),
            analysis.iterations
        ),
    );
    row(
        "Complexity Score",
        format!(
            "{}/{}",
            analysis.score,
            pe2_core::constants::COMPLEXITY_SCORE_MAX
        )
        .white(),
    );
    row(
        "Word Count",
        format!("{} words", analysis.word_count).white(),
    );
    println!();
}

fn field(prefix: &str, label: &str, value: &str) {
    println!("  {} {}", prefix, label.bright_blue());
    for line in value.lines() {
        println!("  {} {}", "│  ".dimmed(), line.white());
    }
    println!();
}

pub fn print_prompt_result(prompt: &StructuredPrompt, path: &str) {
    println!();
    println!(
        "  {} {}",
        "┌".dimmed(),
        "Optimized Prompt".bright_white().bold()
    );
    for (label, value) in [
        ("Context:", &prompt.context),
        ("Role:", &prompt.role),
        ("Task:", &prompt.task),
        ("Constraints:", &prompt.constraints),
        ("Output:", &prompt.output),
    ] {
        field("├─", label, value);
    }
    println!("  {} {}", "└─".dimmed(), "Saved to:".bright_blue());
    println!("  {}   {}", " ".dimmed(), path.bright_cyan().underline());
    println!();
}

pub fn print_refinement_history(history: &[RefinementEntry]) {
    if history.len() <= 1 {
        return;
    }
    println!(
        "  {} {}",
        "◆".bright_cyan(),
        "Refinement History".bright_white().bold()
    );
    for entry in history {
        let preview: String = entry.edits.chars().take(120).collect();
        let label = format!("Iteration {}", entry.iteration).bright_magenta();
        println!(
            "  {} {} {} {}",
            " ".dimmed(),
            label,
            "·".dimmed(),
            preview.dimmed()
        );
    }
    println!();
}

pub fn print_metrics(analysis: &ComplexityResult, iterations: usize) {
    use comfy_table::Table;
    let mut table = Table::new();
    table
        .set_header(vec!["Metric".bold(), "Value".bold()])
        .add_row(vec!["Difficulty", analysis.difficulty.as_str()])
        .add_row(vec!["Complexity Score", &analysis.score.to_string()])
        .add_row(vec!["Iterations", &iterations.to_string()]);
    println!("  {}", "Run Metrics".bright_white().bold());
    for line in table.to_string().lines() {
        println!("  {} {}", " ".dimmed(), line.dimmed());
    }
    println!();
}

pub fn print_error(message: &str) {
    eprintln!("  ✖ {}", message.bright_red());
}

pub fn print_success(message: &str) {
    println!("  ✔ {}", message.bright_green());
}

pub fn print_info(message: &str) {
    println!("  ℹ {}", message.bright_cyan());
}

pub fn print_separator() {
    println!("  {}", "─".repeat(60).dimmed());
}

pub fn create_spinner(message: &str) -> Result<ProgressBar, CliError> {
    let style = ProgressStyle::with_template(SPINNER_TEMPLATE)
        .map_err(|error| CliError::Runtime(error.to_string()))?
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(style);
    spinner.set_message(format!("  {message}"));
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    Ok(spinner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indicatif::ProgressStyle;

    #[test]
    fn spinner_template_is_valid() {
        assert!(ProgressStyle::with_template(SPINNER_TEMPLATE).is_ok());
    }
}
