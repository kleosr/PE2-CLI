use colored::Colorize;

const BANNER_LINES: &[&str] = &[
    "  _____ _____     _  __   ___   _     _____ ",
    " |  __ \\  _  |___| |/ /  / _ \\ | |   |_   _|",
    " | |__/ / |_| / __| ' /  / /_\\ \\| |     | |  ",
    " |  __/ \\__  \\__ \\ . \\  |  _  || |     | |  ",
    " | |     __/ |___) |_\\ \\ | | | || |____ | |_ ",
    " |_|    |___/____/\\__\\/  \\_| |_/\\_____/ \\___/",
];
pub const TAGLINE: &str = "Structured Prompt Generation v4 — KleoSr Pro Edition";

pub fn print_banner() {
    let v = format!("v{}", env!("CARGO_PKG_VERSION"));
    for l in BANNER_LINES {
        println!("  {}", l.bright_cyan());
    }
    println!(
        "\n  {} {}\n  {}\n",
        "|>".bright_green(),
        TAGLINE.bright_white(),
        format!("{:>60}", format!("[ {v} ]").bright_black())
    );
}
pub fn print_banner_brief() {
    println!(
        "{} {} ({} {})",
        "|>".bright_green(),
        "PE2-CLI".bright_cyan(),
        env!("CARGO_PKG_VERSION").bright_white(),
        "— interactive mode".dimmed()
    );
}
