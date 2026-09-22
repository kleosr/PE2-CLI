use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("[{provider}] {message}")]
    Provider { provider: String, message: String },
    #[error("Network error: {0}")]
    Network(String),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Runtime error: {0}")]
    Runtime(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Other(String),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Validation(_) => 2,
            Self::Config(_) => 3,
            Self::Provider { .. } => 4,
            Self::Network(_) => 5,
            Self::Auth(_) => 6,
            Self::Runtime(_) => 7,
            _ => 1,
        }
    }
}

impl From<String> for CliError {
    fn from(message: String) -> Self {
        Self::Other(message)
    }
}

impl From<&str> for CliError {
    fn from(message: &str) -> Self {
        Self::Other(message.to_string())
    }
}
