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
    primary: |s| s.bright_cyan(),
    secondary: |s| s.bright_magenta(),
    success: |s| s.bright_green(),
    error: |s| s.bright_red(),
    warning: |s| s.bright_yellow(),
    muted: |s| s.dimmed(),
    highlight: |s| s.bright_white().bold(),
    label: |s| s.bright_blue(),
    value: |s| s.white(),
    border: |s| s.dimmed(),
};

pub fn styled_label(label: &str) -> String {
    (PE2_THEME.label)(label.to_string()).to_string()
}

pub fn styled_value(value: &str) -> String {
    (PE2_THEME.value)(value.to_string()).to_string()
}
