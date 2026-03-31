import chalk from 'chalk';
import figlet from 'figlet';
import { displayStatusBar } from './utils/display.js';
import { UI_CONFIG, PROMPT_LIMITS, DIFFICULTY_INDICATORS } from './constants.js';
import { PE2_CODE_GENERATION, cliBannerSubtitle, cliVersionWithPrefix } from './versionInfo.js';

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

function displayNarrowBanner(themeManager) {
  console.log(themeManager.color('primary')('\n' + '═'.repeat(31)));
  console.log(themeManager.color('primary')(`  PE²-CLI ${cliVersionWithPrefix()}`));
  console.log(themeManager.color('primary')('═'.repeat(31)));
  console.log(themeManager.color('secondary')(`  Prompt Engineering 2.0 • Code ${PE2_CODE_GENERATION}`));
}

function displayWideBanner(themeManager, terminalWidth) {
  const figletText = renderAnsiShadowFiglet('KLEOSR PE2');
  console.log(figletText);
  const padding = Math.max(0, Math.floor((terminalWidth - 30) / 2));
  console.log(themeManager.color('muted')(' '.repeat(padding) + cliBannerSubtitle()));
}

function displayCommandHints(themeManager, terminalWidth) {
  const separator = themeManager.color('muted')('─'.repeat(Math.min(terminalWidth - 4, 60)));
  console.log(separator);
  console.log(themeManager.color('info')('  Essential Commands:'));
  console.log(themeManager.color('muted')('    /settings    Configure API and model'));
  console.log(themeManager.color('muted')('    /config      View current settings'));
  console.log(themeManager.color('muted')('    /help        Show all commands'));
  console.log(separator);
}

export function displayBanner(bannerOptions) {
  const { themeManager, userPreferences, config, interactive = false } = bannerOptions;
  const title = interactive ? 'KleoSr PE2-CLI - Interactive Mode' : 'KleoSr PE2-CLI';
  setTerminalTitle(title);
  clearConsole();

  const terminalWidth = process.stdout.columns || UI_CONFIG.terminalWidth.default;
  const isNarrow = terminalWidth < UI_CONFIG.terminalWidth.narrowThreshold;

  if (isNarrow) {
    displayNarrowBanner(themeManager);
  } else {
    displayWideBanner(themeManager, terminalWidth);
  }

  console.log();

  const useCompact = userPreferences.shouldUseCompactMode() || isNarrow;
  const showBorder = userPreferences.shouldShowBorders() && !isNarrow;
  displayStatusBar(themeManager, config, { compact: useCompact, showBorder });
  console.log();

  if (isNarrow) {
    console.log(themeManager.color('info')('Quick: /help /settings /config'));
  } else {
    displayCommandHints(themeManager, terminalWidth);
  }

  console.log();
  console.log(themeManager.color('text')('  Type your prompt or use a command to begin.'));
  console.log();
}

function buildShortKeyDisplay(apiKey, limits) {
  const { shortApiKeyPrefix, shortApiKeySuffix, keyLength } = limits;
  const dots = Math.max(shortApiKeyPrefix, keyLength - (shortApiKeyPrefix + shortApiKeySuffix));
  const prefix = apiKey.substring(0, shortApiKeyPrefix);
  const suffix = apiKey.substring(keyLength - shortApiKeySuffix);
  return prefix + '•'.repeat(dots) + suffix;
}

function buildLongKeyDisplay(apiKey, limits) {
  const { apiKeyDisplayLength, apiKeyMaskLength, keyLength, shortApiKeySuffix } = limits;
  const prefix = apiKey.substring(0, apiKeyDisplayLength);
  const suffix = apiKey.substring(keyLength - shortApiKeySuffix);
  return prefix + '•'.repeat(apiKeyMaskLength) + suffix;
}

export function formatApiKeyDisplay(apiKey, options = {}) {
  const { visibility = 'masked' } = options;
  if (!apiKey) return 'Not set';
  if (visibility === 'full') return apiKey;

  const limits = {
    ...PROMPT_LIMITS,
    keyLength: apiKey.length
  };

  if (apiKey.length <= PROMPT_LIMITS.shortApiKeyThreshold) {
    return buildShortKeyDisplay(apiKey, limits);
  }
  return buildLongKeyDisplay(apiKey, limits);
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

function getScoreColor(score) {
  if (score <= 4) return 'success';
  if (score <= 8) return 'warning';
  if (score <= 12) return 'secondary';
  return 'error';
}

function buildScoreBar(score, maxScore = 20) {
  const filled = Math.floor((score / maxScore) * 20);
  return '█'.repeat(filled) + '░'.repeat(20 - filled);
}

export function displayComplexityAnalysis(analysisOptions) {
  const { themeManager, difficulty, iterations, score, rawPrompt } = analysisOptions;
  const indicator = DIFFICULTY_INDICATORS[difficulty];
  const terminalWidth = Math.min(process.stdout.columns || UI_CONFIG.terminalWidth.default, UI_CONFIG.terminalWidth.max);
  const separatorLength = Math.min(UI_CONFIG.separator.defaultLength, terminalWidth - 10);
  const wordCount = rawPrompt.split(/\s+/).length;
  const scoreBar = buildScoreBar(score);
  const scoreColor = getScoreColor(score);

  console.log();
  console.log(themeManager.color('info')('╔' + '═'.repeat(separatorLength - 2) + '╗'));
  const titlePadding = Math.floor((separatorLength - 30) / 2);
  console.log(themeManager.color('info')('║' + ' '.repeat(titlePadding) + 'PROMPT COMPLEXITY ANALYSIS' + ' '.repeat(separatorLength - 30 - titlePadding) + '║'));
  console.log(themeManager.color('info')('╚' + '═'.repeat(separatorLength - 2) + '╝'));
  console.log();

  console.log(themeManager.color('text')('  Complexity Score:'), themeManager.color(scoreColor)(`${score}/20`));
  console.log(themeManager.color('muted')(`    ${scoreBar}`));
  console.log(themeManager.color('text')(`  Difficulty Level: ${indicator} ${difficulty}`));
  console.log(themeManager.color('text')(`  Recommended Iterations: ${themeManager.color('primary')(iterations)}`));
  console.log(themeManager.color('text')(`  Word Count: ${wordCount} words`));
  console.log();
  console.log(themeManager.color('muted')('─'.repeat(separatorLength)));
}

export function displayAdaptiveAnalysis(analysisOptions) {
  const { themeManager, context, strategy, difficulty, complexityScore, recommendedIterations, maxScore = 20 } = analysisOptions;
  const scoreBar = buildScoreBar(complexityScore, maxScore);
  const scoreColor = getScoreColor(complexityScore);

  console.log(themeManager.color('info')('  Adaptive Analysis:'));
  console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
  console.log(`     Domain: ${themeManager.color('primary')(context.domain)}`);
  console.log(`     Difficulty: ${DIFFICULTY_INDICATORS[difficulty]} ${themeManager.color('info')(difficulty)}`);
  console.log(`     Score: ${themeManager.color(scoreColor)(`${complexityScore}/${maxScore}`)}`);
  console.log(themeManager.color('muted')(`     ${scoreBar}`));
  console.log(`     Iterations: ${themeManager.color('primary')(recommendedIterations)} ${themeManager.color('muted')(`(adapted for ${context.domain})`)}`);
  if (strategy.adaptiveFeatures.length > 0) {
    console.log(`     Features: ${themeManager.color('info')(strategy.adaptiveFeatures.join(', '))}`);
  }
  console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
  console.log();
}
