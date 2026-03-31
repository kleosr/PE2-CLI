import chalk from 'chalk';
import cliProgress from 'cli-progress';
import clipboardy from 'clipboardy';
import Table from 'cli-table3';
import { PROGRESS_BAR_CONFIG, UI_CONFIG } from '../constants.js';

function calculateBarSize() {
    const columns = process.stdout.columns || UI_CONFIG.terminalWidth.default;
    const size = columns - PROGRESS_BAR_CONFIG.columnOffset;
    return Math.min(PROGRESS_BAR_CONFIG.maxBarSize, Math.max(PROGRESS_BAR_CONFIG.minBarSize, size));
}

function buildProgressBarOptions(userOptions) {
    return {
        format: PROGRESS_BAR_CONFIG.format,
        barCompleteChar: PROGRESS_BAR_CONFIG.barCompleteChar,
        barIncompleteChar: PROGRESS_BAR_CONFIG.barIncompleteChar,
        hideCursor: PROGRESS_BAR_CONFIG.hideCursor,
        clearOnComplete: PROGRESS_BAR_CONFIG.clearOnComplete,
        stopOnComplete: PROGRESS_BAR_CONFIG.stopOnComplete,
        barsize: calculateBarSize(),
        forceRedraw: true,
        linewrap: false,
        fps: PROGRESS_BAR_CONFIG.fps,
        noTTYOutput: false,
        notTTYSchedule: PROGRESS_BAR_CONFIG.notTTYSchedule,
        synchronousUpdate: false,
        ...userOptions
    };
}

function wrapProgressBarStop(progressBar) {
    const originalStop = progressBar.stop.bind(progressBar);
    progressBar.stop = function () {
        try {
            originalStop();
            process.stdout.write('\r\x1b[K\x1b[?25h');
        } catch {
        }
    };
}

export function createProgressBar(options = {}) {
    const progressBar = new cliProgress.SingleBar(buildProgressBarOptions(options));
    wrapProgressBarStop(progressBar);
    return progressBar;
}

function buildCompactStatusText(themeManager, provider, displayModel, status) {
    return ` ${themeManager.color('primary')(provider)} | ${themeManager.color('info')(displayModel)} | ${status} `;
}

function buildFullStatusText(themeManager, provider, displayModel, status) {
    const providerLabel = themeManager.color('primary')('Provider');
    const modelLabel = themeManager.color('info')('Model');
    const statusLabel = themeManager.color('text')('Status');
    return ` ${providerLabel}: ${provider} | ${modelLabel}: ${displayModel} | ${statusLabel}: ${status} `;
}

function truncateStatusText(text, maxWidth) {
    if (text.length <= maxWidth) return text;
    return text.substring(0, maxWidth - 3) + '...';
}

function renderBorderedStatus(borderColor, width, truncatedText, padding) {
    const border = '─'.repeat(width - 2);
    console.log(borderColor('┌' + border + '┐'));
    console.log(borderColor('│') + truncatedText + ' '.repeat(padding) + borderColor('│'));
    console.log(borderColor('└' + border + '┘'));
}

export function displayStatusBar(themeManager, config, options = {}) {
    const { terminalWidth, statusBar } = UI_CONFIG;
    const width = Math.min(process.stdout.columns || terminalWidth.default, terminalWidth.max);
    const showBorder = options.showBorder !== false;
    const compact = options.compact || false;

    const status = config.apiKey
        ? themeManager.color('success')('Connected')
        : themeManager.color('error')('Not Connected');
    const provider = config.provider ?? 'Not Set';
    const model = config.model ?? 'Default';

    const displayModel = model.length > statusBar.modelTruncateLength
        ? model.substring(0, statusBar.modelDisplayLength) + statusBar.modelTruncateSuffix
        : model;

    const statusText = compact
        ? buildCompactStatusText(themeManager, provider, displayModel, status)
        : buildFullStatusText(themeManager, provider, displayModel, status);

    const maxWidth = width - statusBar.borderPadding;
    const truncatedText = truncateStatusText(statusText, maxWidth);
    const padding = Math.max(0, width - truncatedText.length - 2);

    if (showBorder) {
        renderBorderedStatus(chalk.hex(themeManager.colors.border), width, truncatedText, padding);
    } else {
        console.log(themeManager.color('muted')(truncatedText));
    }
}

export async function copyToClipboard(text) {
    try {
        await clipboardy.write(text);
        console.log(chalk.green('Copied to clipboard. Press Ctrl+V to paste.'));
        return true;
    } catch (error) {
        if (error.code === 'ENOENT') {
            console.log(chalk.red('Clipboard utility not found.'));
            console.log(chalk.yellow('Linux: install xclip or xsel. macOS: pbcopy built-in. Windows: ensure clip.exe.'));
        } else {
            console.log(chalk.red(`Failed to copy to clipboard: ${error.message}`));
        }
        return false;
    }
}

function buildTableStyle(themeManager, minimal) {
    return {
        head: [themeManager.currentTheme === 'dark' ? 'cyan' : 'blue'],
        border: minimal ? [] : [themeManager.currentTheme === 'dark' ? 'gray' : 'black'],
        compact: true
    };
}

function applyMinimalChars(tableConfig) {
    tableConfig.chars = {
        top: '', 'top-mid': '', 'top-left': '', 'top-right': '',
        bottom: '', 'bottom-mid': '', 'bottom-left': '', 'bottom-right': '',
        left: '', 'left-mid': '', 'mid': '', 'mid-mid': '',
        right: '', 'right-mid': '', 'middle': '|'
    };
}

export function createTable(themeManager, headers, rows, options = {}) {
    const minimal = options.minimal ?? false;
    const tableConfig = {
        head: headers,
        style: buildTableStyle(themeManager, minimal),
        compact: options.compact ?? false
    };

    if (minimal) {
        applyMinimalChars(tableConfig);
    }

    const table = new Table(tableConfig);
    rows.forEach(row => table.push(row));
    return table.toString();
}
