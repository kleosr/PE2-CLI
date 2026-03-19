import { createTable } from '../../utils/display.js';
import { COMMANDS } from '../../utils/validation.js';

export async function handleHelp(ctx) {
    const { themeManager } = ctx;
    const helpTerminalWidth = process.stdout.columns || 80;
    const useMinimalHelp = helpTerminalWidth < 70;
    const separator = themeManager.color('muted')('─'.repeat(Math.min(helpTerminalWidth - 4, 60)));

    console.log();
    console.log(themeManager.color('info')('╔' + '═'.repeat(Math.min(helpTerminalWidth - 4, 58)) + '╗'));
    console.log(themeManager.color('info')('║' + ' '.repeat(Math.floor((Math.min(helpTerminalWidth - 4, 58) - 20) / 2)) + 'Available Commands' + ' '.repeat(Math.ceil((Math.min(helpTerminalWidth - 4, 58) - 20) / 2)) + '║'));
    console.log(themeManager.color('info')('╚' + '═'.repeat(Math.min(helpTerminalWidth - 4, 58)) + '╝'));
    console.log();

    const helpTable = createTable(
        themeManager,
        ['Command', 'Description'],
        Object.entries(COMMANDS).map(([cmd, desc]) => [
            themeManager.color('primary')(cmd),
            useMinimalHelp && desc.length > 30 ? desc.substring(0, 27) + '...' : desc
        ]),
        { minimal: useMinimalHelp, compact: true }
    );
    console.log(helpTable);

    console.log();
    console.log(separator);
    console.log(themeManager.color('info')('  💡 Quick Tips:'));
    console.log(themeManager.color('muted')('    • Type any prompt to start optimization'));
    console.log(themeManager.color('muted')('    • Use /exit or /quit to leave'));
    console.log(themeManager.color('muted')('    • Results are auto-saved to pe2-prompts/'));
    console.log(separator);
}
