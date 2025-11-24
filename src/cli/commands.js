import fs from 'fs';
import path from 'path';
import inquirer from 'inquirer';
import { setTerminalTitle, formatApiKeyDisplay, formatContentPreview, formatProcessingPromptDisplay, displayInteractiveBanner } from '../ui.js';
import { promptForConfig } from '../configPrompt.js';
import { saveConfig } from '../config.js';
import { PROVIDERS } from '../providers/index.js';
import { PROMPT_LIMITS, UI_CONFIG } from '../constants.js';
import { createTable, copyToClipboard, COMMANDS } from '../utils/index.js';

const PROMPTS_DIR = path.join(process.cwd(), 'pe2-prompts');

export async function handleCommand(command, rl, config, themeManager, sessionManager, userPreferences, lastResult) {
    switch (command) {
        case '/settings':
            setTerminalTitle('KleoSr PE2-CLI - Settings Configuration');
            config = await promptForConfig(rl);
            setTerminalTitle('KleoSr PE2-CLI - Interactive Mode');
            return config;
            
        case '/config':
            setTerminalTitle('KleoSr PE2-CLI - Current Configuration');
            console.log('\n' + themeManager.color('info')('Current Configuration:'));
            
            const configTerminalWidth = process.stdout.columns || UI_CONFIG.terminalWidth.default;
            const useMinimalConfig = configTerminalWidth < UI_CONFIG.terminalWidth.compactThreshold;
            
            const configTable = createTable(
                ['Setting', 'Value'],
                [
                    ['Provider', config.provider || 'Not set'],
                    ['Model', config.model || 'Not set'],
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
            break;
            
        case '/model':
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
            
        case '/showkey':
            if (config.apiKey) {
                console.log('\n' + themeManager.color('warning')('⚠️  Full API Key:'));
                console.log(themeManager.color('text')(config.apiKey));
                console.log(themeManager.color('muted')('(Keep this secure and don\'t share it)'));
            } else {
                console.log(themeManager.color('warning')('No API key configured.'));
            }
            break;
            
        case '/clear':
            const { clearConsole } = await import('../ui.js');
            clearConsole();
            displayInteractiveBanner();
            break;
            
        case '/history':
            const sessions = sessionManager.loadHistory();
            if (sessions.length === 0) {
                console.log(themeManager.color('warning')('No history found.'));
            } else {
                console.log('\n' + themeManager.color('info')('Recent Sessions:'));
                sessions.slice(0, 5).forEach((session, idx) => {
                    console.log(`\n${idx + 1}. Session ${session.id} - ${new Date(session.timestamp).toLocaleString()}`);
                    console.log(`   Prompts: ${session.prompts.length}`);
                });
            }
            break;
            
        case '/export':
            const exportPath = path.join(process.cwd(), `pe2-export-${Date.now()}.json`);
            fs.writeFileSync(exportPath, JSON.stringify(sessionManager.currentSession, null, 2));
            console.log(themeManager.color('success')(`✓ Session exported to: ${exportPath}`));
            break;
            
        case '/import':
            const { importPath } = await inquirer.prompt([
                {
                    type: 'input',
                    name: 'importPath',
                    message: 'Enter file path to import:',
                    validate: (input) => {
                        if (!input.trim()) return 'Path required';
                        if (!fs.existsSync(input)) return 'File not found';
                        return true;
                    }
                }
            ]);
            
            try {
                const content = fs.readFileSync(importPath, 'utf-8');
                console.log(themeManager.color('success')('✓ File imported successfully'));
                const previewLength = 300;
                console.log(themeManager.color('info')(`📄 Content preview: ${formatContentPreview(content, previewLength)}`));
                return content;
            } catch (error) {
                console.log(themeManager.color('error')(`✗ Import failed: ${error.message}`));
            }
            break;
            
        case '/theme':
            const newTheme = themeManager.currentTheme === 'dark' ? 'light' : 'dark';
            themeManager.setTheme(newTheme);
            userPreferences.set('theme', newTheme);
            console.log(themeManager.color('success')(`✓ Theme changed to: ${newTheme}`));
            return config;
            
        case '/preferences':
        case '/prefs':
            console.log('\n' + themeManager.color('info')('User Preferences:'));
            
            const prefsTerminalWidth = process.stdout.columns || UI_CONFIG.terminalWidth.default;
            const useMinimalPrefs = prefsTerminalWidth < UI_CONFIG.terminalWidth.compactThreshold;
            
            const prefsTable = createTable(
                ['Setting', 'Value'],
                [
                    ['Theme', userPreferences.get('theme')],
                    ['Compact Mode', userPreferences.get('compactMode') ? 'Yes' : 'No'],
                    ['Show Borders', userPreferences.get('showBorders') ? 'Yes' : 'No'],
                    ['Auto Save', userPreferences.get('autoSave') ? 'Yes' : 'No'],
                    ['Max History', userPreferences.get('maxHistoryItems')],
                    ['Default Provider', userPreferences.get('defaultProvider')]
                ],
                { minimal: useMinimalPrefs, compact: true }
            );
            console.log(prefsTable);
            
            console.log(themeManager.color('muted')('\n💡 Tips:'));
            console.log(themeManager.color('muted')('  • Use /theme to toggle theme'));
            console.log(themeManager.color('muted')('  • Use /compact to toggle compact mode'));
            console.log(themeManager.color('muted')('  • Preferences auto-save when changed'));
            break;
            
        case '/compact':
            const currentCompact = userPreferences.get('compactMode');
            userPreferences.set('compactMode', !currentCompact);
            console.log(themeManager.color('success')(`✓ Compact mode ${!currentCompact ? 'enabled' : 'disabled'}`));
            
            const { clearConsole: clearConsoleCompact } = await import('../ui.js');
            clearConsoleCompact();
            displayInteractiveBanner();
            break;
            
        case '/batch':
            const { batchPath } = await inquirer.prompt([
                {
                    type: 'input',
                    name: 'batchPath',
                    message: 'Enter file path containing prompts (one per line):',
                    validate: (input) => {
                        if (!input.trim()) return 'Path required';
                        if (!fs.existsSync(input)) return 'File not found';
                        return true;
                    }
                }
            ]);
            
            try {
                const prompts = fs.readFileSync(batchPath, 'utf-8').split('\n').filter(p => p.trim());
                console.log(themeManager.color('info')(`Found ${prompts.length} prompts to process.`));
                
                if (prompts.length > 0) {
                    console.log(themeManager.color('muted')('\n📝 Prompt previews:'));
                    prompts.slice(0, 3).forEach((prompt, idx) => {
                        console.log(`  ${idx + 1}. ${formatProcessingPromptDisplay(prompt, 80)}`);
                    });
                    if (prompts.length > 3) {
                        console.log(`  ... and ${prompts.length - 3} more prompts`);
                    }
                }
                
                return { batch: prompts };
            } catch (error) {
                console.log(themeManager.color('error')(`✗ Batch load failed: ${error.message}`));
            }
            break;
            
        case '/copy':
            if (lastResult) {
                await copyToClipboard(lastResult);
            } else {
                console.log(themeManager.color('warning')('No result to copy.'));
            }
            break;

        case '/clearall':
            if (fs.existsSync(PROMPTS_DIR)) {
                fs.readdirSync(PROMPTS_DIR).forEach(f => {
                    fs.unlinkSync(path.join(PROMPTS_DIR, f));
                });
                console.log(themeManager.color('success')('✓ All saved prompts cleared.'));
            } else {
                console.log(themeManager.color('warning')('No prompts folder to clear.'));
            }
            break;
            
        case '/help':
        default:
            const helpTerminalWidth = process.stdout.columns || 80;
            const useMinimalHelp = helpTerminalWidth < 70;
            const separator = themeManager.color('muted')('─'.repeat(Math.min(helpTerminalWidth - 4, 60)));
            
            console.log();
            console.log(themeManager.color('info')('╔' + '═'.repeat(Math.min(helpTerminalWidth - 4, 58)) + '╗'));
            console.log(themeManager.color('info')('║' + ' '.repeat(Math.floor((Math.min(helpTerminalWidth - 4, 58) - 20) / 2)) + 'Available Commands' + ' '.repeat(Math.ceil((Math.min(helpTerminalWidth - 4, 58) - 20) / 2)) + '║'));
            console.log(themeManager.color('info')('╚' + '═'.repeat(Math.min(helpTerminalWidth - 4, 58)) + '╝'));
            console.log();
            
            const helpTable = createTable(
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
            break;
    }
    
    return config;
}

