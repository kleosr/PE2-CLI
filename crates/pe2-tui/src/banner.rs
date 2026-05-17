use colored::Colorize;

const BANNER_LINES: &[&str] = &[
    "  _____ _____     _  __   ___   _     _____ ",
    " |  __ \\  _  |___| |/ /  / _ \\ | |   |_   _|",
    " | |__/ / |_| / __| ' /  / /_\\ \\| |     | |  ",
    " |  __/ \\__  \\__ \\ . \\  |  _  || |     | |  ",
    " | |     __/ |___) |_\\ \\ | | | || |____ | |_ ",
    " |_|    |___/____/\\__\\/  \\_| |_/\\_____/ \\___/",
];

const TAGLINE: &str = "Structured Prompt Generation v4 — KleoSr Pro Edition";

pub fn print_banner() {
    let version = "v4.0.2";
    let padded = format!("{:>60}", format!("[ {} ]", version).bright_black());

    for line in BANNER_LINES {
        println!("  {}", line.bright_cyan());
    }
    println!();
    println!("  {} {}", "|>".bright_green(), TAGLINE.bright_white());
    println!("  {}", padded);
    println!();
}

pub fn print_banner_brief() {
    let version = "v4.0.2";
    println!(
        "{} {} ({} {})",
        "|>".bright_green(),
        "PE2-CLI".bright_cyan(),
        version.bright_white(),
        "— interactive mode".dimmed()
    );
}
