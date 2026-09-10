use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use pe2_core::analysis::ComplexityResult;
use pe2_core::engine::{RefinementEntry, StructuredPrompt};
use pe2_core::errors::CliError;

const SPINNER_TEMPLATE: &str = "{spinner:.green} {msg}";

macro_rules! row {
    ($l:expr, $v:expr) => {
        println!("  {} {}: {}", "◇".bright_blue(), $l.bright_blue(), $v);
    };
}
pub fn print_complexity_analysis(a: &ComplexityResult) {
    println!();
    row!(
        "Difficulty",
        format!(
            "{} {} ({} iterations)",
            a.difficulty.emoji(),
            a.difficulty.label().bold(),
            a.iterations
        )
    );
    row!(
        "Complexity Score",
        format!("{}/{}", a.score, pe2_core::constants::COMPLEXITY_SCORE_MAX).white()
    );
    row!("Word Count", format!("{} words", a.word_count).white());
    println!();
}
fn field(pre: &str, l: &str, v: &str) {
    println!("  {} {}", pre, l.bright_blue());
    for x in v.lines() {
        println!("  {} {}", "│  ".dimmed(), x.white());
    }
    println!();
}
pub fn print_prompt_result(p: &StructuredPrompt, f: &str) {
    println!();
    println!(
        "  {} {}",
        "┌".dimmed(),
        "Optimized Prompt".bright_white().bold()
    );
    for (l, v) in [
        ("Context:", &p.context),
        ("Role:", &p.role),
        ("Task:", &p.task),
        ("Constraints:", &p.constraints),
        ("Output:", &p.output),
    ] {
        field("├─", l, v);
    }
    println!("  {} {}", "└─".dimmed(), "Saved to:".bright_blue());
    println!("  {}   {}", " ".dimmed(), f.bright_cyan().underline());
    println!();
}
pub fn print_refinement_history(h: &[RefinementEntry]) {
    if h.len() <= 1 {
        return;
    }
    println!(
        "  {} {}",
        "◆".bright_cyan(),
        "Refinement History".bright_white().bold()
    );
    for e in h {
        println!(
            "  {} {} {} {}",
            " ".dimmed(),
            format!("Iteration {}", e.iteration).bright_magenta(),
            "·".dimmed(),
            e.edits.chars().take(120).collect::<String>().dimmed()
        );
    }
    println!();
}
pub fn print_metrics(a: &ComplexityResult, n: usize) {
    use comfy_table::Table;
    let mut t = Table::new();
    t.set_header(vec!["Metric".bold(), "Value".bold()])
        .add_row(vec!["Difficulty", a.difficulty.as_str()])
        .add_row(vec!["Complexity Score", &a.score.to_string()])
        .add_row(vec!["Iterations", &n.to_string()]);
    println!("  {}", "Run Metrics".bright_white().bold());
    for l in t.to_string().lines() {
        println!("  {} {}", " ".dimmed(), l.dimmed());
    }
    println!();
}
pub fn print_error(m: &str) {
    eprintln!("  ✖ {}", m.bright_red());
}
pub fn print_success(m: &str) {
    println!("  ✔ {}", m.bright_green());
}
pub fn print_info(m: &str) {
    println!("  ℹ {}", m.bright_cyan());
}
pub fn print_separator() {
    println!("  {}", "─".repeat(60).dimmed());
}
pub fn create_spinner(m: &str) -> Result<ProgressBar, CliError> {
    let s = ProgressStyle::with_template(SPINNER_TEMPLATE)
        .map_err(|e| CliError::Runtime(e.to_string()))?
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    let b = ProgressBar::new_spinner();
    b.set_style(s);
    b.set_message(format!("  {m}"));
    b.enable_steady_tick(std::time::Duration::from_millis(80));
    Ok(b)
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
