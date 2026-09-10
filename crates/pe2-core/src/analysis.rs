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
    fn t(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Difficulty::Novice => ("NOVICE", "Novice", "🟢"),
            Difficulty::Intermediate => ("INTERMEDIATE", "Intermediate", "🟡"),
            Difficulty::Advanced => ("ADVANCED", "Advanced", "🟠"),
            Difficulty::Expert => ("EXPERT", "Expert", "🔴"),
            Difficulty::Master => ("MASTER", "Master", "🟣"),
        }
    }
    pub fn as_str(&self) -> &'static str {
        self.t().0
    }
    pub fn label(&self) -> &'static str {
        self.t().1
    }
    pub fn emoji(&self) -> &'static str {
        self.t().2
    }
}
fn word_score(w: usize) -> u32 {
    match w {
        w if w >= constants::COMPLEXITY_WORD_VERY_HIGH => 4,
        w if w >= constants::COMPLEXITY_WORD_HIGH => 3,
        w if w >= constants::COMPLEXITY_WORD_MEDIUM => 2,
        w if w >= constants::COMPLEXITY_WORD_LOW => 1,
        _ => 0,
    }
}
fn pattern_scores(r: &str, l: &str) -> u32 {
    let c = |n: usize, m: usize| n.min(m) as u32;
    let t = c(
        constants::TECH_PATTERNS
            .iter()
            .filter(|x| x.is_match(r))
            .count(),
        constants::MAX_TECH_INDICATORS,
    );
    let d = c(
        constants::DOMAIN_PATTERNS
            .iter()
            .filter(|x| x.is_match(r))
            .count(),
        constants::MAX_DOMAIN_INDICATORS,
    );
    let s = c(
        constants::STRUCTURAL_PATTERN.find_iter(r).count(),
        constants::MAX_STRUCTURAL_MATCHES,
    );
    let g = c(
        constants::LOGIC_PATTERN.find_iter(l).count(),
        constants::MAX_LOGIC_MATCHES,
    );
    let n = constants::SPECIAL_CHARS_PATTERN.find_iter(r).count();
    t + d
        + s
        + g
        + if n >= constants::SPECIAL_CHARS_HIGH {
            2
        } else if n >= constants::SPECIAL_CHARS_MEDIUM {
            1
        } else {
            0
        }
}
fn difficulty_from_score(s: u32) -> (Difficulty, u32) {
    match s {
        0..=4 => (Difficulty::Novice, 1),
        5..=8 => (Difficulty::Intermediate, 2),
        9..=12 => (Difficulty::Advanced, 3),
        13..=16 => (Difficulty::Expert, 4),
        _ => (Difficulty::Master, 5),
    }
}
pub fn analyze_prompt_complexity(r: &str) -> ComplexityResult {
    let w = r.split_whitespace().count();
    let l = r.to_lowercase();
    let score = (word_score(w) + pattern_scores(r, &l)).min(constants::COMPLEXITY_SCORE_MAX);
    let (difficulty, iterations) = difficulty_from_score(score);
    ComplexityResult {
        score,
        difficulty,
        iterations,
        word_count: w,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_prompt() {
        let p = format!("{} api", "word ".repeat(20));
        let r = analyze_prompt_complexity(&p);
        assert!(r.score > 0);
        assert!(r.iterations >= 1);
    }
    #[test]
    fn test_technical_keywords_increase_score() {
        let plain = "word ".repeat(20);
        let tech = format!("{plain} python api docker ml algorithm framework database");
        let pr = analyze_prompt_complexity(&plain);
        let tr = analyze_prompt_complexity(&tech);
        assert!(tr.score >= pr.score);
    }
    #[test]
    fn test_difficulty_mapping() {
        let r = analyze_prompt_complexity("short");
        assert_eq!(r.difficulty, Difficulty::Novice);
        assert_eq!(r.iterations, 1);
    }
}
