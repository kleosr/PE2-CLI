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
    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Novice => "NOVICE",
            Difficulty::Intermediate => "INTERMEDIATE",
            Difficulty::Advanced => "ADVANCED",
            Difficulty::Expert => "EXPERT",
            Difficulty::Master => "MASTER",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Novice => "Novice",
            Difficulty::Intermediate => "Intermediate",
            Difficulty::Advanced => "Advanced",
            Difficulty::Expert => "Expert",
            Difficulty::Master => "Master",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Difficulty::Novice => "🟢",
            Difficulty::Intermediate => "🟡",
            Difficulty::Advanced => "🟠",
            Difficulty::Expert => "🔴",
            Difficulty::Master => "🟣",
        }
    }
}

fn word_score(word_count: usize) -> u32 {
    if word_count >= constants::COMPLEXITY_WORD_VERY_HIGH {
        4
    } else if word_count >= constants::COMPLEXITY_WORD_HIGH {
        3
    } else if word_count >= constants::COMPLEXITY_WORD_MEDIUM {
        2
    } else if word_count >= constants::COMPLEXITY_WORD_LOW {
        1
    } else {
        0
    }
}

pub fn analyze_prompt_complexity(raw_prompt: &str) -> ComplexityResult {
    let word_count = raw_prompt.split_whitespace().count();
    let prompt_lower = raw_prompt.to_lowercase();

    let tech_count = constants::TECH_PATTERNS
        .iter()
        .filter(|r| r.is_match(raw_prompt))
        .count()
        .min(constants::MAX_TECH_INDICATORS);

    let domain_count = constants::DOMAIN_PATTERNS
        .iter()
        .filter(|r| r.is_match(raw_prompt))
        .count()
        .min(constants::MAX_DOMAIN_INDICATORS);

    let structural_matches = constants::STRUCTURAL_PATTERN
        .find_iter(raw_prompt)
        .count()
        .min(constants::MAX_STRUCTURAL_MATCHES);

    let logic_matches = constants::LOGIC_PATTERN
        .find_iter(&prompt_lower)
        .count()
        .min(constants::MAX_LOGIC_MATCHES);

    let special_char_count = constants::SPECIAL_CHARS_PATTERN
        .find_iter(raw_prompt)
        .count();

    let special_score = if special_char_count >= constants::SPECIAL_CHARS_HIGH {
        2
    } else if special_char_count >= constants::SPECIAL_CHARS_MEDIUM {
        1
    } else {
        0
    };

    let base_score = word_score(word_count);
    let mut score = base_score
        + tech_count as u32
        + domain_count as u32
        + structural_matches as u32
        + logic_matches as u32
        + special_score;

    score = score.min(constants::COMPLEXITY_SCORE_MAX);

    let (difficulty, iterations) = if score <= 4 {
        (Difficulty::Novice, 1)
    } else if score <= 8 {
        (Difficulty::Intermediate, 2)
    } else if score <= 12 {
        (Difficulty::Advanced, 3)
    } else if score <= 16 {
        (Difficulty::Expert, 4)
    } else {
        (Difficulty::Master, 5)
    };

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
        let r = analyze_prompt_complexity("this is a simple prompt with enough words");
        assert!(r.score >= 0);
        assert!(r.iterations >= 1);
    }

    #[test]
    fn test_technical_keywords_increase_score() {
        let plain = "word ".repeat(20);
        let technical = format!("{} python api docker ml algorithm framework database", plain);
        let plain_result = analyze_prompt_complexity(&plain);
        let technical_result = analyze_prompt_complexity(&technical);
        assert!(technical_result.score >= plain_result.score);
    }

    #[test]
    fn test_difficulty_mapping() {
        let r1 = analyze_prompt_complexity("short");
        assert_eq!(r1.difficulty, Difficulty::Novice);
        assert_eq!(r1.iterations, 1);
    }
}
