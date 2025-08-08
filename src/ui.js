import chalk from 'chalk';
import figlet from 'figlet';
import { displayStatusBar, createTable } from './utils.js';

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

  const terminalWidth = process.stdout.columns || 80;
  const isNarrow = terminalWidth < 60;

  if (isNarrow) {
    console.log(themeManager.color('primary')('\n PE²-CLI'));
    console.log(themeManager.color('secondary')(' ⚡ Prompt Engineering 2.0'));
  } else {
    console.log(renderAnsiShadowFiglet('KLEOSR PE2'));
  }

  console.log();

  const useCompact = userPreferences.shouldUseCompactMode() || isNarrow;
  const showBorder = userPreferences.shouldShowBorders() && !isNarrow;
  displayStatusBar(config, { compact: useCompact, showBorder });
  console.log();

  if (isNarrow) {
    console.log(themeManager.color('info')('Quick commands: /help /settings /config'));
  } else {
    console.log(themeManager.color('info')('Essential Commands:'));
    console.log(themeManager.color('muted')('  /settings    Configure API provider, model, and key'));
    console.log(themeManager.color('muted')('  /config      View current settings'));
    console.log(themeManager.color('muted')('  /help        Show all commands'));
  }

  console.log();
  console.log(themeManager.color('text')('Type your prompt or use a command to begin.'));
  console.log();
}

export function displayInteractiveBanner({ themeManager, userPreferences, config }) {
  displayBanner({ themeManager, userPreferences, config, interactive: true });
}

export function formatApiKeyDisplay(apiKey, showFullKey = false) {
  if (!apiKey) return 'Not set';
  if (showFullKey) return apiKey;
  const keyLength = apiKey.length;
  if (keyLength <= 12) {
    return apiKey.substring(0, 4) + '•'.repeat(Math.max(4, keyLength - 8)) + apiKey.substring(keyLength - 4);
  }
  return apiKey.substring(0, 8) + '•'.repeat(8) + apiKey.substring(keyLength - 4);
}

export function formatContentPreview(content, maxLength = 200, showFullLength = false) {
  if (!content) return '';
  if (content.length <= maxLength || showFullLength) return content;
  const truncated = content.substring(0, maxLength);
  const lastSpace = truncated.lastIndexOf(' ');
  const cleanTruncated = lastSpace > maxLength * 0.7 ? truncated.substring(0, lastSpace) : truncated;
  return `${cleanTruncated}... [${content.length - cleanTruncated.length} more characters]`;
}

export function formatProcessingPromptDisplay(prompt, maxLength = 100) {
  if (!prompt) return '';
  if (prompt.length <= maxLength) return prompt;
  const truncated = prompt.substring(0, maxLength);
  const lastSpace = truncated.lastIndexOf(' ');
  const cleanTruncated = lastSpace > maxLength * 0.7 ? truncated.substring(0, lastSpace) : truncated;
  return `${cleanTruncated}... [${prompt.length} chars total]`;
}

export const DIFFICULTY_INDICATORS = {
  NOVICE: '🟢',
  INTERMEDIATE: '🟡',
  ADVANCED: '🟠',
  EXPERT: '🔴',
  MASTER: '🟣'
};

export function displayComplexityAnalysis({ themeManager }, difficulty, iterations, score, rawPrompt) {
  const indicator = DIFFICULTY_INDICATORS[difficulty];
  const terminalWidth = Math.min(process.stdout.columns || 80, 100);
  const separatorLength = Math.min(50, terminalWidth - 10);
  console.log(themeManager.color('info')('\n🔍 PROMPT COMPLEXITY ANALYSIS'));
  console.log(themeManager.color('muted')('─'.repeat(separatorLength)));
  console.log(themeManager.color('text')(`📊 Complexity Score: ${score}/20`));
  console.log(themeManager.color('text')(`🎚️  Difficulty Level: ${indicator} ${difficulty}`));
  console.log(themeManager.color('text')(`🔄 Recommended Iterations: ${iterations}`));
  console.log(themeManager.color('text')(`📝 Word Count: ${rawPrompt.split(/\s+/).length} words`));
  console.log(themeManager.color('muted')('─'.repeat(separatorLength)));
}


