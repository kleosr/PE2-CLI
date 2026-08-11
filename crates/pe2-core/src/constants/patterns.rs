use regex::Regex;
use std::sync::LazyLock;

fn static_regex(pattern: &'static str) -> Regex {
    Regex::new(pattern).expect("hardcoded analysis regex pattern is valid")
}

pub static TECH_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        static_regex(r"\b(api|json|rest|graphql|grpc|restful)\b"),
        static_regex(r"\b(sql|nosql|database|postgres|mongo|redis|mysql)\b"),
        static_regex(r"\b(docker|kubernetes|k8s|container|orchestrat)\b"),
        static_regex(r"\b(aws|gcp|azure|cloud|deploy|serverless)\b"),
        static_regex(r"\b(microservice|distributed|message.queue|event.driven)\b"),
        static_regex(r"\b(auth|oauth|jwt|saml|oidc|authentication|authorization)\b"),
        static_regex(r"\b(testing|tdd|unit.test|integration.test|e2e|mock|assert)\b"),
        static_regex(r"\b(ci/cd|pipeline|devops|deploy|monitoring|observability)\b"),
        static_regex(r"\b(caching|redis|memcached|cdn|performance)\b"),
        static_regex(r"\b(security|encrypt|hash|ssl|tls|certificate)\b"),
        static_regex(r"\b(async|await|promise|callback|concurren|parallel|thread)\b"),
        static_regex(r"\b(stream|kafka|rabbitmq|pub.sub|event)\b"),
    ]
});

pub static DOMAIN_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        static_regex(r"\b(frontend|react|vue|angular|svelte|ui|ux)\b"),
        static_regex(r"\b(backend|server|node|express|fastapi|django|spring)\b"),
        static_regex(r"\b(data|analytics|machine.learning|ai|deep.learning)\b"),
        static_regex(r"\b(mobile|ios|android|flutter|react.native|swift)\b"),
        static_regex(r"\b(blockchain|web3|smart.contract|solidity|nft|defi)\b"),
        static_regex(r"\b(devops|sre|reliability|scalability|infrastructure)\b"),
        static_regex(r"\b(security|pen.test|vulnerability|compliance|audit)\b"),
        static_regex(r"\b(gaming|unity|unreal|3d|game.dev)\b"),
        static_regex(r"\b(embedded|iot|firmware|hardware|rtos)\b"),
        static_regex(r"\b(scientific|research|bioinformatics|computational)\b"),
    ]
});

pub static STRUCTURAL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| static_regex(r"(\n\s*\d+\.|\n\s*\-|```|#)"));

pub static LOGIC_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| static_regex(r"\b(if|then|when|unless|until|depending|while)\b"));

pub static SPECIAL_CHARS_PATTERN: LazyLock<Regex> = LazyLock::new(|| static_regex(r"[;\{\[]"));

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
