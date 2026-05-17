use colored::Colorize;

pub struct Theme {
    pub primary: fn(String) -> colored::ColoredString,
    pub secondary: fn(String) -> colored::ColoredString,
    pub success: fn(String) -> colored::ColoredString,
    pub error: fn(String) -> colored::ColoredString,
    pub warning: fn(String) -> colored::ColoredString,
    pub muted: fn(String) -> colored::ColoredString,
    pub highlight: fn(String) -> colored::ColoredString,
    pub label: fn(String) -> colored::ColoredString,
    pub value: fn(String) -> colored::ColoredString,
    pub border: fn(String) -> colored::ColoredString,
}

pub const PE2_THEME: Theme = Theme {
    primary: |s| s.bright_cyan().to_colored_string(),
    secondary: |s| s.bright_magenta().to_colored_string(),
    success: |s| s.bright_green().to_colored_string(),
    error: |s| s.bright_red().to_colored_string(),
    warning: |s| s.bright_yellow().to_colored_string(),
    muted: |s| s.dimmed().to_colored_string(),
    highlight: |s| s.bright_white().bold().to_colored_string(),
    label: |s| s.bright_blue().to_colored_string(),
    value: |s| s.white().to_colored_string(),
    border: |s| s.dimmed().to_colored_string(),
};

pub fn styled_label(label: &str) -> String {
    (PE2_THEME.label)(label.to_string()).to_string()
}

pub fn styled_value(value: &str) -> String {
    (PE2_THEME.value)(value.to_string()).to_string()
}
