use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use pe2_core::analysis::ComplexityResult;
use pe2_core::engine::{Metrics, RefinementEntry, StructuredPrompt};
use pe2_core::errors::CliError;
use crate::theme::{styled_label, styled_value, PE2_THEME};

const SPINNER_TEMPLATE: &str = "{spinner:.green} {msg}";

pub fn print_complexity_analysis(analysis: &ComplexityResult) {
    println!();
    println!(
        "  {} {}: {} {} ({} iterations)",
        "◇".bright_blue(),
        styled_label("Difficulty"),
        analysis.difficulty.emoji(),
        analysis.difficulty.label().bold(),
        analysis.iterations,
    );
    println!(
        "  {} {}: {}",
        "◇".bright_blue(),
        styled_label("Complexity Score"),
        styled_value(&format!("{}/{}", analysis.score, pe2_core::constants::COMPLEXITY_SCORE_MAX)),
    );
    println!(
        "  {} {}: {} words",
        "◇".bright_blue(),
        styled_label("Word Count"),
        styled_value(&analysis.word_count.to_string()),
    );
    println!();
}

fn print_prompt_field(prefix: &str, label: &str, value: &str) {
    println!("  {} {}", prefix, styled_label(label));
    for line in value.lines() {
        println!("  {} {}", "│  ".dimmed(), styled_value(line));
    }
    println!();
}

pub fn print_prompt_result(prompt: &StructuredPrompt, output_file: &str) {
    println!();
    println!("  {} {}", "┌".dimmed(), "Optimized Prompt".bright_white().bold());
    print_prompt_field("├─", "Context:", &prompt.context);
    print_prompt_field("├─", "Role:", &prompt.role);
    print_prompt_field("├─", "Task:", &prompt.task);
    print_prompt_field("├─", "Constraints:", &prompt.constraints);
    print_prompt_field("├─", "Output:", &prompt.output);
    println!("  {} {}", "└─".dimmed(), styled_label("Saved to:"));
    println!("  {}   {}", " ".dimmed(), output_file.bright_cyan().underline());
    println!();
}

pub fn print_refinement_history(history: &[RefinementEntry]) {
    if history.len() <= 1 {
        return;
    }
    println!(
        "  {} {}",
        (PE2_THEME.primary)("◆".to_string()),
        (PE2_THEME.highlight)("Refinement History".to_string())
    );
    for entry in history {
        let label = format!("Iteration {}", entry.iteration);
        let short = entry.edits.chars().take(120).collect::<String>();
        println!(
            "  {} {} {} {}",
            (PE2_THEME.muted)(" ".to_string()),
            (PE2_THEME.secondary)(label),
            (PE2_THEME.muted)("·".to_string()),
            (PE2_THEME.muted)(short),
        );
    }
    println!();
}

pub fn print_metrics(metrics: &Metrics) {
    use comfy_table::Table;
    let mut table = Table::new();
    table
        .set_header(vec!["Metric".bold(), "Value".bold()])
        .add_row(vec!["Accuracy Gain", &metrics.accuracy_gain])
        .add_row(vec!["Optimization", &metrics.optimization_level])
        .add_row(vec!["Quality Score", &metrics.quality_score])
        .add_row(vec!["Iterations", &metrics.iterations_applied.to_string()]);

    println!("  {}", (PE2_THEME.highlight)("Performance Metrics".to_string()));
    for line in table.to_string().lines() {
        println!(
            "  {} {}",
            (PE2_THEME.muted)(" ".to_string()),
            (PE2_THEME.muted)(line.to_string())
        );
    }
    println!();
}

pub fn print_error(msg: &str) {
    eprintln!("  ✖ {}", (PE2_THEME.error)(msg.to_string()));
}

pub fn print_success(msg: &str) {
    println!("  ✔ {}", (PE2_THEME.success)(msg.to_string()));
}

pub fn print_info(msg: &str) {
    println!("  ℹ {}", (PE2_THEME.primary)(msg.to_string()));
}

pub fn print_warning(msg: &str) {
    println!("  ⚠ {}", (PE2_THEME.warning)(msg.to_string()));
}

pub fn print_separator() {
    println!("  {}", (PE2_THEME.border)("─".repeat(60)));
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
