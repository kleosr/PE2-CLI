import chalk from 'chalk';
import cliProgress from 'cli-progress';
import clipboardy from 'clipboardy';
import Table from 'cli-table3';
import { PROGRESS_BAR_CONFIG, UI_CONFIG } from '../constants.js';

export function createProgressBar(options = {}) {
    const defaultOptions = {
        format: PROGRESS_BAR_CONFIG.format,
        barCompleteChar: PROGRESS_BAR_CONFIG.barCompleteChar,
        barIncompleteChar: PROGRESS_BAR_CONFIG.barIncompleteChar,
        hideCursor: PROGRESS_BAR_CONFIG.hideCursor,
        clearOnComplete: PROGRESS_BAR_CONFIG.clearOnComplete,
        stopOnComplete: PROGRESS_BAR_CONFIG.stopOnComplete,
        barsize: Math.min(PROGRESS_BAR_CONFIG.maxBarSize, Math.max(PROGRESS_BAR_CONFIG.minBarSize, (process.stdout.columns || UI_CONFIG.terminalWidth.default) - PROGRESS_BAR_CONFIG.columnOffset)),
        forceRedraw: true,
        linewrap: false,
        fps: PROGRESS_BAR_CONFIG.fps,
        noTTYOutput: false,
        notTTYSchedule: PROGRESS_BAR_CONFIG.notTTYSchedule,
        synchronousUpdate: false,
        ...options
    };

    const progressBar = new cliProgress.SingleBar(defaultOptions);

    const originalStop = progressBar.stop.bind(progressBar);
    progressBar.stop = function () {
        try {
            originalStop();
            process.stdout.write('\r\x1b[K\x1b[?25h');
        } catch {
        }
    };

    return progressBar;
}

export function displayStatusBar(themeManager, config, options = {}) {
    const { terminalWidth, statusBar } = UI_CONFIG;
    const width = Math.min(process.stdout.columns || terminalWidth.default, terminalWidth.max);
    const showBorder = options.showBorder !== false;
    const compact = options.compact || false;

    const status = config.apiKey ? themeManager.color('success')('✓ Connected') : themeManager.color('error')('✗ Not Connected');
    const provider = config.provider ?? 'Not Set';
    const model = config.model ?? 'Default';

    const displayModel = model.length > statusBar.modelTruncateLength
        ? model.substring(0, statusBar.modelDisplayLength) + statusBar.modelTruncateSuffix
        : model;

    const statusText = compact
        ? ` ${themeManager.color('primary')(provider)} | ${themeManager.color('info')(displayModel)} | ${status} `
        : ` ${themeManager.color('primary')('Provider')}: ${provider} | ${themeManager.color('info')('Model')}: ${displayModel} | ${themeManager.color('text')('Status')}: ${status} `;

    const maxContentWidth = width - statusBar.borderPadding;
    const truncatedText = statusText.length > maxContentWidth
        ? statusText.substring(0, maxContentWidth - 3) + '...'
        : statusText;

    const padding = Math.max(0, width - truncatedText.length - 2);

    if (showBorder) {
        const borderColor = chalk.hex(themeManager.colors.border);

        console.log(borderColor('┌' + '─'.repeat(width - 2) + '┐'));
        console.log(borderColor('│') + truncatedText + ' '.repeat(padding) + borderColor('│'));
        console.log(borderColor('└' + '─'.repeat(width - 2) + '┘'));
    } else {
        console.log(themeManager.color('muted')(truncatedText));
    }
}

export async function copyToClipboard(text) {
    try {
        await clipboardy.write(text);
        console.log(chalk.green('✓ Copied to clipboard! Press Ctrl+V to paste.'));
        return true;
    } catch (error) {
        if (error.code === 'ENOENT') {
            console.log(chalk.red('✗ Clipboard utility not found on your system.'));
            console.log(chalk.yellow('  • Linux: install xclip or xsel  |  macOS: pbcopy/pbpaste are built-in  |  Windows: ensure clip.exe is accessible'));
        } else {
            console.log(chalk.red(`✗ Failed to copy to clipboard: ${error.message}`));
        }
        return false;
    }
}

export function createTable(themeManager, headers, rows, options = {}) {
    const minimal = options.minimal ?? false;
    const compact = options.compact ?? false;

    const tableConfig = {
        head: headers,
        style: {
            head: [themeManager.currentTheme === 'dark' ? 'cyan' : 'blue'],
            border: minimal ? [] : [themeManager.currentTheme === 'dark' ? 'gray' : 'black'],
            compact: compact
        }
    };

    if (minimal) {
        tableConfig.chars = {
            'top': '', 'top-mid': '', 'top-left': '', 'top-right': '',
            'bottom': '', 'bottom-mid': '', 'bottom-left': '', 'bottom-right': '',
            'left': '', 'left-mid': '', 'mid': '', 'mid-mid': '',
            'right': '', 'right-mid': '', 'middle': '│'
        };
    }

    const table = new Table(tableConfig);
    rows.forEach(row => table.push(row));
    return table.toString();
}
