import inquirer from 'inquirer';
import { setTerminalTitle, formatApiKeyDisplay } from '../../ui.js';
import { promptForConfig } from '../../configPrompt.js';
import { saveConfig } from '../../config.js';
import { PROVIDERS } from '../../providers/index.js';
import { UI_CONFIG } from '../../constants.js';
import { createTable } from '../../utils/display.js';

export async function handleSettings(ctx) {
    const { themeManager } = ctx;
    setTerminalTitle('KleoSr PE2-CLI - Settings Configuration');
    const config = await promptForConfig(themeManager);
    setTerminalTitle('KleoSr PE2-CLI - Interactive Mode');
    return config;
}

export async function handleConfig(ctx) {
    const { config, themeManager } = ctx;
    setTerminalTitle('KleoSr PE2-CLI - Current Configuration');
    console.log('\n' + themeManager.color('info')('Current Configuration:'));

    const configTerminalWidth = process.stdout.columns || UI_CONFIG.terminalWidth.default;
    const useMinimalConfig = configTerminalWidth < UI_CONFIG.terminalWidth.compactThreshold;

    const configTable = createTable(
        themeManager,
        ['Setting', 'Value'],
        [
            ['Provider', config.provider ?? 'Not set'],
            ['Model', config.model ?? 'Not set'],
            [config.provider === 'ollama' ? 'Base URL' : 'API Key', formatApiKeyDisplay(config.apiKey)],
            ['Theme', themeManager.currentTheme]
        ],
        { minimal: useMinimalConfig, compact: true }
    );
    console.log(configTable);

    if (config.apiKey) {
        console.log(themeManager.color('muted')('\n💡 Tips:'));
        console.log(themeManager.color('muted')('  • Use /showkey to reveal full API key'));
        console.log(themeManager.color('muted')('  • Use /model to quickly switch models'));
        console.log(themeManager.color('muted')('  • Use /theme to toggle light/dark mode'));
    } else {
        console.log(themeManager.color('warning')('\n⚠️  No API key configured. Use /settings to configure.'));
    }
    setTerminalTitle('KleoSr PE2-CLI - Interactive Mode');
}

export async function handleModel(ctx) {
    const { config, themeManager } = ctx;
    process.stdout.write('\r\x1b[K');
    setTerminalTitle('KleoSr PE2-CLI - Model Selection');
    const { quickModel } = await inquirer.prompt([
        {
            type: 'input',
            name: 'quickModel',
            message: 'Enter model name (or press Enter to select from list):',
        }
    ]);

    if (quickModel.trim()) {
        config.model = quickModel.trim();
        saveConfig(config);
        console.log(themeManager.color('success')(`✓ Model changed to: ${config.model}`));
    } else {
        const providerConfig = PROVIDERS[config.provider];
        if (providerConfig) {
            const { selectedModel } = await inquirer.prompt([
                {
                    type: 'list',
                    name: 'selectedModel',
                    message: 'Select a model:',
                    choices: [
                        ...providerConfig.models.map(m => ({ name: m, value: m })),
                        { name: '📝 Enter Custom Model', value: 'custom' }
                    ]
                }
            ]);

            if (selectedModel === 'custom') {
                const { customModel } = await inquirer.prompt([
                    {
                        type: 'input',
                        name: 'customModel',
                        message: 'Enter custom model name:',
                        validate: (input) => input.trim() ? true : 'Model name required'
                    }
                ]);
                config.model = customModel.trim();
            } else {
                config.model = selectedModel;
            }

            saveConfig(config);
            console.log(themeManager.color('success')(`✓ Model changed to: ${config.model}`));
        }
    }
    setTerminalTitle('KleoSr PE2-CLI - Interactive Mode');
    return config;
}

export async function handleShowkey(ctx) {
    const { config, themeManager } = ctx;
    if (config.apiKey) {
        console.log('\n' + themeManager.color('warning')('⚠️  Full API Key:'));
        console.log(themeManager.color('text')(config.apiKey));
        console.log(themeManager.color('muted')('(Keep this secure and don\'t share it)'));
    } else {
        console.log(themeManager.color('warning')('No API key configured.'));
    }
}
