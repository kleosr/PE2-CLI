import {
  COMPLEXITY_THRESHOLDS,
  DIFFICULTY_LEVELS
} from './constants.js';

const TECH_INDICATORS = ['api', 'algorithm', 'framework', 'database', 'ml', 'ai', 'docker', 'python', 'javascript'];
const DOMAIN_INDICATORS = ['compliance', 'strategy', 'analytics', 'finance', 'healthcare', 'enterprise'];
const STRUCTURAL_PATTERN = /(\n\s*\d+\.|\n\s*\-|```|#)/g;
const LOGIC_PATTERN = /\b(if|then|when|unless|until|depending|while)\b/g;
const SPECIAL_CHARS_PATTERN = /[;\{\[]/g;

export function analyzePromptComplexity(rawPrompt) {
  const words = rawPrompt.split(/\s+/).length;
  const promptLower = rawPrompt.toLowerCase();
  let score = 0;

  const { wordCount, wordCountScores, maxTechIndicators, maxDomainIndicators, maxStructuralMatches, maxLogicMatches, specialCharsHigh, specialCharsMedium, specialCharsHighScore, specialCharsMediumScore } = COMPLEXITY_THRESHOLDS;

  score += words > wordCount.veryHigh ? wordCountScores.veryHigh
    : words > wordCount.high ? wordCountScores.high
    : words > wordCount.medium ? wordCountScores.medium
    : words > wordCount.low ? wordCountScores.low
    : 0;

  score += Math.min(TECH_INDICATORS.filter(keyword => promptLower.includes(keyword)).length, maxTechIndicators);
  score += Math.min(DOMAIN_INDICATORS.filter(keyword => promptLower.includes(keyword)).length, maxDomainIndicators);

  const structuralMatches = (rawPrompt.match(STRUCTURAL_PATTERN) || []).length;
  score += Math.min(structuralMatches, maxStructuralMatches);

  const logicMatches = (promptLower.match(LOGIC_PATTERN) || []).length;
  score += Math.min(logicMatches, maxLogicMatches);

  const specialChars = (rawPrompt.match(SPECIAL_CHARS_PATTERN) || []).length;
  score += specialChars >= specialCharsHigh ? specialCharsHighScore
    : specialChars >= specialCharsMedium ? specialCharsMediumScore
    : 0;

  const result = DIFFICULTY_LEVELS.find(difficulty => score <= difficulty.max);
  return { difficulty: result.level, iterations: result.iterations, score };
}
