use regex::Regex;
use std::sync::LazyLock;

fn rx(p: &str) -> Regex {
    Regex::new(p).expect("hardcoded analysis regex pattern is valid")
}
fn v(ps: &[&str]) -> Vec<Regex> {
    ps.iter().map(|p| rx(p)).collect()
}
const TECH: &[&str] = &[
    r"\b(api|json|rest|graphql|grpc|restful)\b",
    r"\b(sql|nosql|database|postgres|mongo|redis|mysql)\b",
    r"\b(docker|kubernetes|k8s|container|orchestrat)\b",
    r"\b(aws|gcp|azure|cloud|deploy|serverless)\b",
    r"\b(microservice|distributed|message.queue|event.driven)\b",
    r"\b(auth|oauth|jwt|saml|oidc|authentication|authorization)\b",
    r"\b(testing|tdd|unit.test|integration.test|e2e|mock|assert)\b",
    r"\b(ci/cd|pipeline|devops|deploy|monitoring|observability)\b",
    r"\b(caching|redis|memcached|cdn|performance)\b",
    r"\b(security|encrypt|hash|ssl|tls|certificate)\b",
    r"\b(async|await|promise|callback|concurren|parallel|thread)\b",
    r"\b(stream|kafka|rabbitmq|pub.sub|event)\b",
];
const DOMAIN: &[&str] = &[
    r"\b(frontend|react|vue|angular|svelte|ui|ux)\b",
    r"\b(backend|server|node|express|fastapi|django|spring)\b",
    r"\b(data|analytics|machine.learning|ai|deep.learning)\b",
    r"\b(mobile|ios|android|flutter|react.native|swift)\b",
    r"\b(blockchain|web3|smart.contract|solidity|nft|defi)\b",
    r"\b(devops|sre|reliability|scalability|infrastructure)\b",
    r"\b(security|pen.test|vulnerability|compliance|audit)\b",
    r"\b(gaming|unity|unreal|3d|game.dev)\b",
    r"\b(embedded|iot|firmware|hardware|rtos)\b",
    r"\b(scientific|research|bioinformatics|computational)\b",
];
pub static TECH_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| v(TECH));
pub static DOMAIN_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| v(DOMAIN));
pub static STRUCTURAL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| rx(r"(\n\s*\d+\.|\n\s*\-|```|#)"));
pub static LOGIC_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| rx(r"\b(if|then|when|unless|until|depending|while)\b"));
pub static SPECIAL_CHARS_PATTERN: LazyLock<Regex> = LazyLock::new(|| rx(r"[;\{\[]"));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_static_regex_patterns_compile() {
        let _ = &*TECH_PATTERNS;
        let _ = &*DOMAIN_PATTERNS;
        let _ = &*STRUCTURAL_PATTERN;
        let _ = &*LOGIC_PATTERN;
        let _ = &*SPECIAL_CHARS_PATTERN;
    }
}
