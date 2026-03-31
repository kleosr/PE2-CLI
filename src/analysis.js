import {
  COMPLEXITY_THRESHOLDS,
  DIFFICULTY_LEVELS
} from './constants.js';

const TECH_INDICATORS = ['api', 'algorithm', 'framework', 'database', 'ml', 'ai', 'docker', 'python', 'javascript'];
const DOMAIN_INDICATORS = ['compliance', 'strategy', 'analytics', 'finance', 'healthcare', 'enterprise'];
const STRUCTURAL_PATTERN = /(\n\s*\d+\.|\n\s*\-|```|#)/g;
const LOGIC_PATTERN = /\b(if|then|when|unless|until|depending|while)\b/g;
const SPECIAL_CHARS_PATTERN = /[;\{\[]/g;

function calculateWordScore(words, thresholds) {
  const { wordCount, wordCountScores } = thresholds;
  if (words > wordCount.veryHigh) return wordCountScores.veryHigh;
  if (words > wordCount.high) return wordCountScores.high;
  if (words > wordCount.medium) return wordCountScores.medium;
  if (words > wordCount.low) return wordCountScores.low;
  return 0;
}

function countKeywordMatches(text, keywords, maxCount) {
  const matches = keywords.filter(keyword => text.includes(keyword)).length;
  return Math.min(matches, maxCount);
}

function calculateSpecialCharScore(charCount, thresholds) {
  const { specialCharsHigh, specialCharsMedium, specialCharsHighScore, specialCharsMediumScore } = thresholds;
  if (charCount >= specialCharsHigh) return specialCharsHighScore;
  if (charCount >= specialCharsMedium) return specialCharsMediumScore;
  return 0;
}

export function analyzePromptComplexity(rawPrompt) {
  const words = rawPrompt.split(/\s+/).length;
  const promptLower = rawPrompt.toLowerCase();
  const thresholds = COMPLEXITY_THRESHOLDS;
  let score = 0;

  score += calculateWordScore(words, thresholds);
  score += countKeywordMatches(promptLower, TECH_INDICATORS, thresholds.maxTechIndicators);
  score += countKeywordMatches(promptLower, DOMAIN_INDICATORS, thresholds.maxDomainIndicators);

  const structuralMatches = (rawPrompt.match(STRUCTURAL_PATTERN) || []).length;
  score += Math.min(structuralMatches, thresholds.maxStructuralMatches);

  const logicMatches = (promptLower.match(LOGIC_PATTERN) || []).length;
  score += Math.min(logicMatches, thresholds.maxLogicMatches);

  const specialChars = (rawPrompt.match(SPECIAL_CHARS_PATTERN) || []).length;
  score += calculateSpecialCharScore(specialChars, thresholds);

  const difficulty = DIFFICULTY_LEVELS.find(level => score <= level.max);
  return { difficulty: difficulty.level, iterations: difficulty.iterations, score };
}
