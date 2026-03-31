import { clearConsole, setTerminalTitle, displayBanner } from '../../ui.js';
import { UI_CONFIG } from '../../constants.js';
import { createTable } from '../../utils/display.js';

export async function handleClear(ctx) {
    const { themeManager, userPreferences, config } = ctx;
    clearConsole();
    displayBanner({ themeManager, userPreferences, config, interactive: true });
}

export async function handleTheme(ctx) {
    const { config, themeManager, userPreferences } = ctx;
    const newTheme = themeManager.currentTheme === 'dark' ? 'light' : 'dark';
    themeManager.setTheme(newTheme);
    userPreferences.set('theme', newTheme);
    console.log(themeManager.color('success')(`✓ Theme changed to: ${newTheme}`));
    return config;
}

function formatBool(value) {
    return value ? 'Yes' : 'No';
}

function buildPreferenceRows(userPreferences) {
    return [
        ['Theme', userPreferences.get('theme')],
        ['Compact Mode', formatBool(userPreferences.get('compactMode') ?? false)],
        ['Show Borders', formatBool(userPreferences.get('showBorders') ?? false)],
        ['Auto Save', formatBool(userPreferences.get('autoSave') ?? false)],
        ['Max History', userPreferences.get('maxHistoryItems')],
        ['Default Provider', userPreferences.get('defaultProvider')]
    ];
}

export async function handlePreferences(ctx) {
    const { themeManager, userPreferences } = ctx;
    console.log('\n' + themeManager.color('info')('User Preferences:'));

    const terminalWidth = process.stdout.columns || UI_CONFIG.terminalWidth.default;
    const useMinimal = terminalWidth < UI_CONFIG.terminalWidth.compactThreshold;

    const prefsTable = createTable(
        themeManager,
        ['Setting', 'Value'],
        buildPreferenceRows(userPreferences),
        { minimal: useMinimal, compact: true }
    );
    console.log(prefsTable);

    console.log(themeManager.color('muted')('\n💡 Tips:'));
    console.log(themeManager.color('muted')('  • Use /theme to toggle theme'));
    console.log(themeManager.color('muted')('  • Use /compact to toggle compact mode'));
    console.log(themeManager.color('muted')('  • Preferences auto-save when changed'));
}

export async function handleCompact(ctx) {
    const { themeManager, userPreferences, config } = ctx;
    const currentCompact = userPreferences.get('compactMode');
    userPreferences.set('compactMode', !currentCompact);
    console.log(themeManager.color('success')(`✓ Compact mode ${!currentCompact ? 'enabled' : 'disabled'}`));

    clearConsole();
    displayBanner({ themeManager, userPreferences, config, interactive: true });
}
