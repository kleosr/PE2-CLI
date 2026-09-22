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
    let version = env!("CARGO_PKG_VERSION");
    let badge = format!("[ v{version} ]").bright_black();
    for line in BANNER_LINES {
        println!("  {}", line.bright_cyan());
    }
    println!(
        "\n  {} {}\n  {:>60}\n",
        "|>".bright_green(),
        TAGLINE.bright_white(),
        badge
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
