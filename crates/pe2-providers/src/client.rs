use crate::adapters::Client;
use pe2_core::engine::EngineLlmProvider;
use pe2_core::errors::CliError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderKind {
    OpenAI,
    Anthropic,
    Google,
    OpenRouter,
    Ollama,
}
const MAP: &[(&str, ProviderKind)] = &[
    ("openai", ProviderKind::OpenAI),
    ("anthropic", ProviderKind::Anthropic),
    ("google", ProviderKind::Google),
    ("openrouter", ProviderKind::OpenRouter),
    ("ollama", ProviderKind::Ollama),
];
impl ProviderKind {
    pub fn parse(s: &str) -> Option<Self> {
        MAP.iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(s))
            .map(|(_, k)| *k)
    }
    pub fn as_str(&self) -> &'static str {
        MAP.iter()
            .find(|(_, k)| k == self)
            .map(|(n, _)| *n)
            .unwrap_or("openrouter")
    }
}
impl std::str::FromStr for ProviderKind {
    type Err = CliError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| CliError::Config(format!("Unknown provider: {s}")))
    }
}
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub kind: ProviderKind,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}
impl ProviderConfig {
    pub fn new(kind: ProviderKind, api_key: Option<String>) -> Self {
        Self {
            kind,
            api_key,
            base_url: None,
        }
    }
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = Some(url);
        self
    }
}
pub fn create_client(c: &ProviderConfig) -> Result<Box<dyn EngineLlmProvider>, CliError> {
    Ok(Box::new(Client::new(c)?))
}
