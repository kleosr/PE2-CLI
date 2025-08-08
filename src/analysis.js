export function analyzePromptComplexity(rawPrompt) {
  const words = rawPrompt.split(/\s+/).length;
  const promptLower = rawPrompt.toLowerCase();
  let score = 0;

  score += words > 400 ? 4 : words > 250 ? 3 : words > 120 ? 2 : words > 60 ? 1 : 0;

  const techIndicators = ['api', 'algorithm', 'framework', 'database', 'ml', 'ai', 'docker', 'python', 'javascript'];
  const domainIndicators = ['compliance', 'strategy', 'analytics', 'finance', 'healthcare', 'enterprise'];
  score += Math.min(techIndicators.filter(kw => promptLower.includes(kw)).length, 4);
  score += Math.min(domainIndicators.filter(kw => promptLower.includes(kw)).length, 3);

  const structuralPattern = /(\n\s*\d+\.|\n\s*\-|```|#)/g;
  const structuralMatches = (rawPrompt.match(structuralPattern) || []).length;
  score += Math.min(structuralMatches, 4);

  const logicPattern = /\b(if|then|when|unless|until|depending|while)\b/g;
  const logicMatches = (promptLower.match(logicPattern) || []).length;
  score += Math.min(logicMatches, 3);

  const specialChars = (rawPrompt.match(/[;\{\[]/g) || []).length;
  score += specialChars >= 5 ? 2 : specialChars >= 2 ? 1 : 0;

  const difficultyMap = [
    { max: 4, level: 'NOVICE', iterations: 1 },
    { max: 8, level: 'INTERMEDIATE', iterations: 2 },
    { max: 12, level: 'ADVANCED', iterations: 3 },
    { max: 16, level: 'EXPERT', iterations: 4 },
    { max: Infinity, level: 'MASTER', iterations: 5 }
  ];

  const result = difficultyMap.find(d => score <= d.max);
  return { difficulty: result.level, iterations: result.iterations, score };
}


