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

function buildConfigRows(config, themeManager) {
    return [
        ['Provider', config.provider ?? 'Not set'],
        ['Model', config.model ?? 'Not set'],
        [config.provider === 'ollama' ? 'Base URL' : 'API Key', formatApiKeyDisplay(config.apiKey)],
        ['Theme', themeManager.currentTheme]
    ];
}

function displayConfigTips(themeManager, hasApiKey) {
    if (hasApiKey) {
        console.log(themeManager.color('muted')('\n💡 Tips:'));
        console.log(themeManager.color('muted')('  • Use /showkey to reveal full API key'));
        console.log(themeManager.color('muted')('  • Use /model to quickly switch models'));
        console.log(themeManager.color('muted')('  • Use /theme to toggle light/dark mode'));
    } else {
        console.log(themeManager.color('warning')('\n⚠️  No API key configured. Use /settings to configure.'));
    }
}

export async function handleConfig(ctx) {
    const { config, themeManager } = ctx;
    setTerminalTitle('KleoSr PE2-CLI - Current Configuration');
    console.log('\n' + themeManager.color('info')('Current Configuration:'));

    const terminalWidth = process.stdout.columns || UI_CONFIG.terminalWidth.default;
    const useMinimal = terminalWidth < UI_CONFIG.terminalWidth.compactThreshold;

    const configTable = createTable(
        themeManager,
        ['Setting', 'Value'],
        buildConfigRows(config, themeManager),
        { minimal: useMinimal, compact: true }
    );
    console.log(configTable);

    displayConfigTips(themeManager, !!config.apiKey);
    setTerminalTitle('KleoSr PE2-CLI - Interactive Mode');
}

async function promptForCustomModel() {
    const { customModel } = await inquirer.prompt([{
        type: 'input',
        name: 'customModel',
        message: 'Enter custom model name:',
        validate: (input) => input.trim() ? true : 'Model name required'
    }]);
    return customModel.trim();
}

function buildModelChoices(providerConfig) {
    return providerConfig.models.map(modelName => ({
        name: modelName,
        value: modelName
    }));
}

async function selectFromModelList(providerConfig) {
    const { selectedModel } = await inquirer.prompt([{
        type: 'list',
        name: 'selectedModel',
        message: 'Select a model:',
        choices: [
            ...buildModelChoices(providerConfig),
            { name: 'Enter Custom Model', value: 'custom' }
        ]
    }]);
    return selectedModel;
}

async function resolveModelChoice(providerConfig) {
    const selected = await selectFromModelList(providerConfig);
    return selected === 'custom' ? await promptForCustomModel() : selected;
}

async function handleModelSelection(config) {
    const providerConfig = PROVIDERS[config.provider];
    if (!providerConfig) return;
    config.model = await resolveModelChoice(providerConfig);
}

export async function handleModel(ctx) {
    const { config, themeManager } = ctx;
    process.stdout.write('\r\x1b[K');
    setTerminalTitle('KleoSr PE2-CLI - Model Selection');

    const { quickModel } = await inquirer.prompt([{
        type: 'input',
        name: 'quickModel',
        message: 'Enter model name (or press Enter to select from list):',
    }]);

    if (quickModel.trim()) {
        config.model = quickModel.trim();
    } else {
        await handleModelSelection(config);
    }

    saveConfig(config);
    console.log(themeManager.color('success')(`Model changed to: ${config.model}`));
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
