import { createTable } from '../../utils/display.js';
import { COMMANDS } from '../../utils/validation.js';

function buildHelpRows(themeManager, commands, truncate) {
    return Object.entries(commands).map(([cmd, desc]) => [
        themeManager.color('primary')(cmd),
        truncate && desc.length > 30 ? desc.substring(0, 27) + '...' : desc
    ]);
}

function displayHelpHeader(themeManager, width) {
    const boxWidth = Math.min(width - 4, 58);
    const padding = Math.floor((boxWidth - 20) / 2);
    console.log();
    console.log(themeManager.color('info')('╔' + '═'.repeat(boxWidth) + '╗'));
    console.log(themeManager.color('info')('║' + ' '.repeat(padding) + 'Available Commands' + ' '.repeat(boxWidth - 20 - padding) + '║'));
    console.log(themeManager.color('info')('╚' + '═'.repeat(boxWidth) + '╝'));
    console.log();
}

function displayHelpFooter(themeManager, width) {
    const sep = themeManager.color('muted')('─'.repeat(Math.min(width - 4, 60)));
    console.log();
    console.log(sep);
    console.log(themeManager.color('info')('  Quick Tips:'));
    console.log(themeManager.color('muted')('    • Type any prompt to start optimization'));
    console.log(themeManager.color('muted')('    • Use /exit or /quit to leave'));
    console.log(themeManager.color('muted')('    • Results auto-saved to pe2-prompts/'));
    console.log(sep);
}

export async function handleHelp(ctx) {
    const { themeManager } = ctx;
    const width = process.stdout.columns || 80;
    const truncate = width < 70;

    displayHelpHeader(themeManager, width);

    const helpTable = createTable(
        themeManager,
        ['Command', 'Description'],
        buildHelpRows(themeManager, COMMANDS, truncate),
        { minimal: truncate, compact: true }
    );
    console.log(helpTable);

    displayHelpFooter(themeManager, width);
}
