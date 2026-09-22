use crate::constants;

#[derive(Debug, Clone)]
pub struct ComplexityResult {
    pub score: u32,
    pub difficulty: Difficulty,
    pub iterations: u32,
    pub word_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Novice,
    Intermediate,
    Advanced,
    Expert,
    Master,
}

impl Difficulty {
    pub fn as_str(self) -> &'static str {
        self.presentation().0
    }

    pub fn label(self) -> &'static str {
        self.presentation().1
    }

    pub fn emoji(self) -> &'static str {
        self.presentation().2
    }

    fn presentation(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Novice => ("NOVICE", "Novice", "🟢"),
            Self::Intermediate => ("INTERMEDIATE", "Intermediate", "🟡"),
            Self::Advanced => ("ADVANCED", "Advanced", "🟠"),
            Self::Expert => ("EXPERT", "Expert", "🔴"),
            Self::Master => ("MASTER", "Master", "🟣"),
        }
    }
}

fn word_score(words: usize) -> u32 {
    if words >= constants::COMPLEXITY_WORD_VERY_HIGH {
        4
    } else if words >= constants::COMPLEXITY_WORD_HIGH {
        3
    } else if words >= constants::COMPLEXITY_WORD_MEDIUM {
        2
    } else if words >= constants::COMPLEXITY_WORD_LOW {
        1
    } else {
        0
    }
}

fn capped(count: usize, max: usize) -> u32 {
    count.min(max) as u32
}

fn pattern_score(raw: &str, lower: &str) -> u32 {
    let tech = capped(
        constants::TECH_PATTERNS
            .iter()
            .filter(|pattern| pattern.is_match(raw))
            .count(),
        constants::MAX_TECH_INDICATORS,
    );
    let domain = capped(
        constants::DOMAIN_PATTERNS
            .iter()
            .filter(|pattern| pattern.is_match(raw))
            .count(),
        constants::MAX_DOMAIN_INDICATORS,
    );
    let structure = capped(
        constants::STRUCTURAL_PATTERN.find_iter(raw).count(),
        constants::MAX_STRUCTURAL_MATCHES,
    );
    let logic = capped(
        constants::LOGIC_PATTERN.find_iter(lower).count(),
        constants::MAX_LOGIC_MATCHES,
    );
    let special = constants::SPECIAL_CHARS_PATTERN.find_iter(raw).count();
    let special_score = if special >= constants::SPECIAL_CHARS_HIGH {
        2
    } else if special >= constants::SPECIAL_CHARS_MEDIUM {
        1
    } else {
        0
    };
    tech + domain + structure + logic + special_score
}

fn tier(score: u32) -> (Difficulty, u32) {
    match score {
        0..=4 => (Difficulty::Novice, 1),
        5..=8 => (Difficulty::Intermediate, 2),
        9..=12 => (Difficulty::Advanced, 3),
        13..=16 => (Difficulty::Expert, 4),
        _ => (Difficulty::Master, 5),
    }
}

pub fn analyze_prompt_complexity(raw: &str) -> ComplexityResult {
    let word_count = raw.split_whitespace().count();
    let lower = raw.to_lowercase();
    let score =
        (word_score(word_count) + pattern_score(raw, &lower)).min(constants::COMPLEXITY_SCORE_MAX);
    let (difficulty, iterations) = tier(score);
    ComplexityResult {
        score,
        difficulty,
        iterations,
        word_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_prompt() {
        let prompt = format!("{} api", "word ".repeat(20));
        let result = analyze_prompt_complexity(&prompt);
        assert!(result.score > 0);
        assert!(result.iterations >= 1);
    }

    #[test]
    fn test_technical_keywords_increase_score() {
        let plain = "word ".repeat(20);
        let tech = format!("{plain} python api docker ml algorithm framework database");
        let plain_result = analyze_prompt_complexity(&plain);
        let tech_result = analyze_prompt_complexity(&tech);
        assert!(tech_result.score >= plain_result.score);
    }

    #[test]
    fn test_difficulty_mapping() {
        let result = analyze_prompt_complexity("short");
        assert_eq!(result.difficulty, Difficulty::Novice);
        assert_eq!(result.iterations, 1);
    }
}
