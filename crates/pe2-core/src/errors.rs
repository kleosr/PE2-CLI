use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("{0}")]
    General(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("[{provider}] {message}")]
    Provider {
        provider: String,
        message: String,
    },

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
            CliError::General(_) => 1,
            CliError::Validation(_) => 2,
            CliError::Config(_) => 3,
            CliError::Provider { .. } => 4,
            CliError::Network(_) => 5,
            CliError::Auth(_) => 6,
            CliError::Runtime(_) => 7,
            CliError::Io(_) => 1,
            CliError::Json(_) => 1,
            CliError::Other(_) => 1,
        }
    }
}

impl From<String> for CliError {
    fn from(s: String) -> Self {
        CliError::Other(s)
    }
}

impl From<&str> for CliError {
    fn from(s: &str) -> Self {
        CliError::Other(s.to_string())
    }
}
