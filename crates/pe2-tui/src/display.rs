use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use pe2_core::analysis::{ComplexityResult, Difficulty};
use pe2_core::engine::{Metrics, RefinementEntry, StructuredPrompt};
use crate::theme::{styled_label, styled_value, PE2_THEME};

pub fn print_complexity_analysis(analysis: &ComplexityResult) {
    let emoji = match analysis.difficulty {
        Difficulty::Novice => "🟢",
        Difficulty::Intermediate => "🟡",
        Difficulty::Advanced => "🟠",
        Difficulty::Expert => "🔴",
        Difficulty::Master => "🟣",
    };

    println!();
    println!(
        "  {} {}: {} {} ({} iterations)",
        "◇".bright_blue(),
        styled_label("Difficulty"),
        emoji,
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

pub fn print_prompt_result(prompt: &StructuredPrompt, output_file: &str) {
    println!();
    println!("  {} {}", "┌".dimmed(), "Optimized Prompt".bright_white().bold());
    println!("  {} {}", "├─".dimmed(), styled_label("Context:"));
    println!("  {} {}", "│  ".dimmed(), styled_value(&prompt.context));
    println!();
    println!("  {} {}", "├─".dimmed(), styled_label("Role:"));
    println!("  {} {}", "│  ".dimmed(), styled_value(&prompt.role));
    println!();
    println!("  {} {}", "├─".dimmed(), styled_label("Task:"));
    for line in prompt.task.lines() {
        println!("  {} {}", "│  ".dimmed(), styled_value(line));
    }
    println!();
    println!("  {} {}", "├─".dimmed(), styled_label("Constraints:"));
    for line in prompt.constraints.lines() {
        println!("  {} {}", "│  ".dimmed(), styled_value(line));
    }
    println!();
    println!("  {} {}", "├─".dimmed(), styled_label("Output:"));
    println!("  {} {}", "│  ".dimmed(), styled_value(&prompt.output));
    println!();
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
        "◆".bright_magenta(),
        "Refinement History".bright_white().bold()
    );
    for entry in history {
        let label = format!("Iteration {}", entry.iteration);
        let short = entry.edits.chars().take(120).collect::<String>();
        println!(
            "  {} {} {} {}",
            " ".dimmed(),
            (PE2_THEME.secondary)(label),
            "·".dimmed(),
            short.dimmed(),
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

    println!("  {}", "Performance Metrics".bright_white().bold());
    let table_str = table.to_string();
    for line in table_str.lines() {
        println!("  {} {}", " ".dimmed(), line.dimmed());
    }
    println!();
}

pub fn print_error(msg: &str) {
    eprintln!("  {} {}", "✖".bright_red(), msg.bright_red());
}

pub fn print_success(msg: &str) {
    println!("  {} {}", "✔".bright_green(), msg.bright_green());
}

pub fn print_info(msg: &str) {
    println!("  {} {}", "ℹ".bright_blue(), msg.bright_white());
}

pub fn print_warning(msg: &str) {
    println!("  {} {}", "⚠".bright_yellow(), msg.bright_yellow());
}

pub fn print_separator() {
    println!("  {}", "─".repeat(60).dimmed());
}

pub fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("  {}", msg));
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}
