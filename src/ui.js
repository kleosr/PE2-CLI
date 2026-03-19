import chalk from 'chalk';
import figlet from 'figlet';
import { displayStatusBar } from './utils/display.js';
import { UI_CONFIG, PROMPT_LIMITS, DIFFICULTY_INDICATORS } from './constants.js';

export function clearConsole() {
  process.stdout.write(process.platform === 'win32' ? '\x1Bc' : '\x1B[2J\x1B[3J\x1B[H');
}

export function setTerminalTitle(title) {
  process.stdout.write(`\x1b]0;${title}\x07`);
}

export function renderAnsiShadowFiglet(text) {
  const figletText = figlet.textSync(text, { font: 'ANSI Shadow' });
  return chalk.white(figletText);
}

export function displayBanner({ themeManager, userPreferences, config, interactive = false }) {
  const title = interactive ? 'KleoSr PE2-CLI - Interactive Mode' : 'KleoSr PE2-CLI';
  setTerminalTitle(title);
  clearConsole();

  const terminalWidth = process.stdout.columns || UI_CONFIG.terminalWidth.default;
  const isNarrow = terminalWidth < UI_CONFIG.terminalWidth.narrowThreshold;

  if (isNarrow) {
    console.log(themeManager.color('primary')('\n╔═══════════════════════════════╗'));
    console.log(themeManager.color('primary')('║      PE²-CLI v3.4.6          ║'));
    console.log(themeManager.color('primary')('╚═══════════════════════════════╝'));
    console.log(themeManager.color('secondary')('  ⚡ Prompt Engineering 2.0'));
  } else {
    const figletText = renderAnsiShadowFiglet('KLEOSR PE2');
    console.log(figletText);
    console.log(themeManager.color('muted')(' '.repeat(Math.max(0, Math.floor((terminalWidth - 30) / 2))) + 'v3.4.6 • Prompt Engineering 2.0'));
  }

  console.log();

  const useCompact = userPreferences.shouldUseCompactMode() || isNarrow;
  const showBorder = userPreferences.shouldShowBorders() && !isNarrow;
  displayStatusBar(themeManager, config, { compact: useCompact, showBorder });
  console.log();

  if (isNarrow) {
    console.log(themeManager.color('info')('Quick: /help /settings /config'));
  } else {
    const separator = themeManager.color('muted')('─'.repeat(Math.min(terminalWidth - 4, 60)));
    console.log(separator);
    console.log(themeManager.color('info')('  Essential Commands:'));
    console.log(themeManager.color('muted')('    /settings    Configure API provider, model, and key'));
    console.log(themeManager.color('muted')('    /config      View current settings'));
    console.log(themeManager.color('muted')('    /help        Show all available commands'));
    console.log(separator);
  }

  console.log();
  console.log(themeManager.color('text')('  💡 Type your prompt or use a command to begin.'));
  console.log();
}


export function formatApiKeyDisplay(apiKey, options = {}) {
  const { visibility = 'masked' } = options;
  if (!apiKey) return 'Not set';
  if (visibility === 'full') return apiKey;
  const keyLength = apiKey.length;
  const { shortApiKeyThreshold, shortApiKeyPrefix, shortApiKeySuffix, apiKeyDisplayLength, apiKeyMaskLength } = PROMPT_LIMITS;
  if (keyLength <= shortApiKeyThreshold) {
    return apiKey.substring(0, shortApiKeyPrefix) + '•'.repeat(Math.max(shortApiKeyPrefix, keyLength - (shortApiKeyPrefix + shortApiKeySuffix))) + apiKey.substring(keyLength - shortApiKeySuffix);
  }
  return apiKey.substring(0, apiKeyDisplayLength) + '•'.repeat(apiKeyMaskLength) + apiKey.substring(keyLength - shortApiKeySuffix);
}

function truncateText(text, maxLength, suffix = '...') {
  if (!text || text.length <= maxLength) return text;
  const truncated = text.substring(0, maxLength);
  const lastSpace = truncated.lastIndexOf(' ');
  const cleanTruncated = lastSpace > maxLength * 0.7 ? truncated.substring(0, lastSpace) : truncated;
  return `${cleanTruncated}${suffix}`;
}

export function formatContentPreview(content, options = {}) {
  const { maxLength = PROMPT_LIMITS.previewMaxLength, mode = 'truncated' } = options;
  if (!content || mode === 'full') return content;
  const truncated = truncateText(content, maxLength);
  return `${truncated}... [${content.length - truncated.length} more characters]`;
}

export function formatProcessingPromptDisplay(prompt, maxLength = PROMPT_LIMITS.processingDisplayMaxLength) {
  return truncateText(prompt, maxLength, `... [${prompt?.length || 0} chars total]`);
}

function getScoreDisplay(score, maxScore = 20) {
  const scoreBar = '█'.repeat(Math.floor((score / maxScore) * 20)) + '░'.repeat(20 - Math.floor((score / maxScore) * 20));
  const scoreColor = score <= 4 ? 'success' : score <= 8 ? 'warning' : score <= 12 ? 'secondary' : 'error';
  return { scoreBar, scoreColor };
}

export function displayComplexityAnalysis({ themeManager, difficulty, iterations, score, rawPrompt }) {
  const indicator = DIFFICULTY_INDICATORS[difficulty];
  const terminalWidth = Math.min(process.stdout.columns || UI_CONFIG.terminalWidth.default, UI_CONFIG.terminalWidth.max);
  const separatorLength = Math.min(UI_CONFIG.separator.defaultLength, terminalWidth - 10);
  const wordCount = rawPrompt.split(/\s+/).length;
  const { scoreBar, scoreColor } = getScoreDisplay(score);

  console.log();
  console.log(themeManager.color('info')('╔' + '═'.repeat(separatorLength - 2) + '╗'));
  console.log(themeManager.color('info')('║' + ' '.repeat(Math.floor((separatorLength - 30) / 2)) + '🔍 PROMPT COMPLEXITY ANALYSIS' + ' '.repeat(Math.ceil((separatorLength - 30) / 2)) + '║'));
  console.log(themeManager.color('info')('╚' + '═'.repeat(separatorLength - 2) + '╝'));
  console.log();

  console.log(themeManager.color('text')('  📊 Complexity Score:'), themeManager.color(scoreColor)(`${score}/20`));
  console.log(themeManager.color('muted')(`    ${scoreBar}`));
  console.log(themeManager.color('text')(`  🎚️  Difficulty Level: ${indicator} ${difficulty}`));
  console.log(themeManager.color('text')(`  🔄 Recommended Iterations: ${themeManager.color('primary')(iterations)}`));
  console.log(themeManager.color('text')(`  📝 Word Count: ${wordCount} words`));
  console.log();
  console.log(themeManager.color('muted')('─'.repeat(separatorLength)));
}

export function displayAdaptiveAnalysis({
  themeManager,
  context,
  strategy,
  difficulty,
  complexityScore,
  recommendedIterations,
  maxScore = 20
}) {
  const { scoreBar, scoreColor } = getScoreDisplay(complexityScore, maxScore);

  console.log(themeManager.color('info')('  📊 Adaptive Analysis:'));
  console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
  console.log(`     ${themeManager.color('text')('Domain')}: ${themeManager.color('primary')(context.domain)}`);
  console.log(`     ${themeManager.color('text')('Difficulty')}: ${DIFFICULTY_INDICATORS[difficulty]} ${themeManager.color('info')(difficulty)}`);
  console.log(`     ${themeManager.color('text')('Score')}: ${themeManager.color(scoreColor)(`${complexityScore}/${maxScore}`)}`);
  console.log(themeManager.color('muted')(`     ${scoreBar}`));
  console.log(`     ${themeManager.color('text')('Iterations')}: ${themeManager.color('primary')(recommendedIterations)} ${themeManager.color('muted')(`(adapted for ${context.domain})`)}`);
  if (strategy.adaptiveFeatures.length > 0) {
    console.log(`     ${themeManager.color('text')('Features')}: ${themeManager.color('info')(strategy.adaptiveFeatures.join(', '))}`);
  }
  console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
  console.log();
}
