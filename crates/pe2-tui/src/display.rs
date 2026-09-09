use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use pe2_core::analysis::ComplexityResult;
use pe2_core::engine::{RefinementEntry, StructuredPrompt};
use pe2_core::errors::CliError;

const SPINNER_TEMPLATE: &str = "{spinner:.green} {msg}";

pub fn print_complexity_analysis(analysis: &ComplexityResult) {
    println!();
    println!(
        "  {} {}: {} {} ({} iterations)",
        "◇".bright_blue(),
        "Difficulty".bright_blue(),
        analysis.difficulty.emoji(),
        analysis.difficulty.label().bold(),
        analysis.iterations,
    );
    println!(
        "  {} {}: {}",
        "◇".bright_blue(),
        "Complexity Score".bright_blue(),
        format!(
            "{}/{}",
            analysis.score,
            pe2_core::constants::COMPLEXITY_SCORE_MAX
        )
        .white(),
    );
    println!(
        "  {} {}: {} words",
        "◇".bright_blue(),
        "Word Count".bright_blue(),
        analysis.word_count.to_string().white(),
    );
    println!();
}

fn print_prompt_field(prefix: &str, label: &str, value: &str) {
    println!("  {} {}", prefix, label.bright_blue());
    for line in value.lines() {
        println!("  {} {}", "│  ".dimmed(), line.white());
    }
    println!();
}

pub fn print_prompt_result(prompt: &StructuredPrompt, output_file: &str) {
    println!();
    println!(
        "  {} {}",
        "┌".dimmed(),
        "Optimized Prompt".bright_white().bold()
    );
    print_prompt_field("├─", "Context:", &prompt.context);
    print_prompt_field("├─", "Role:", &prompt.role);
    print_prompt_field("├─", "Task:", &prompt.task);
    print_prompt_field("├─", "Constraints:", &prompt.constraints);
    print_prompt_field("├─", "Output:", &prompt.output);
    println!("  {} {}", "└─".dimmed(), "Saved to:".bright_blue());
    println!(
        "  {}   {}",
        " ".dimmed(),
        output_file.bright_cyan().underline()
    );
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
        let label = format!("Iteration {}", entry.iteration);
        let short = entry.edits.chars().take(120).collect::<String>();
        println!(
            "  {} {} {} {}",
            " ".dimmed(),
            label.bright_magenta(),
            "·".dimmed(),
            short.dimmed(),
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

pub fn print_error(msg: &str) {
    eprintln!("  ✖ {}", msg.bright_red());
}

pub fn print_success(msg: &str) {
    println!("  ✔ {}", msg.bright_green());
}

pub fn print_info(msg: &str) {
    println!("  ℹ {}", msg.bright_cyan());
}

pub fn print_separator() {
    println!("  {}", "─".repeat(60).dimmed());
}

pub fn create_spinner(msg: &str) -> Result<ProgressBar, CliError> {
    let style = ProgressStyle::with_template(SPINNER_TEMPLATE)
        .map_err(|e| CliError::Runtime(e.to_string()))?
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    let pb = ProgressBar::new_spinner();
    pb.set_style(style);
    pb.set_message(format!("  {}", msg));
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    Ok(pb)
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
