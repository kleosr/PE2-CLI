import fs from 'fs';
import readline from 'readline';
import { PROVIDERS, getProviderClient } from '../providers/index.js';
import { getDefaultConfig, loadConfig, resolveApiKey } from '../config.js';
import { promptForConfig } from '../configPrompt.js';
import { analyzePromptComplexity } from '../analysis.js';
import { displayBanner, formatProcessingPromptDisplay, displayComplexityAnalysis } from '../ui.js';
import { PROMPT_LIMITS } from '../constants.js';
import { validateAndSuggestCommand } from '../utils/validation.js';
import { ConfigError } from '../errorHandler.js';
import { handleCommand } from './commands.js';
import { processPrompt } from './promptProcessing.js';

function resolveInputPrompt(initialInput, cliOptions) {
    if (!initialInput) return { prompt: null, source: null };
    if (cliOptions.text) return { prompt: initialInput, source: 'direct text' };

    const exists = fs.existsSync(initialInput);
    if (cliOptions.file) {
        if (!exists) return { prompt: null, source: `file: ${initialInput}`, error: `❌ Error: File not found at ${initialInput}` };
        return { prompt: fs.readFileSync(initialInput, 'utf-8').trim(), source: `file: ${initialInput}` };
    }

    if (!exists) return { prompt: initialInput, source: 'direct text' };
    return { prompt: fs.readFileSync(initialInput, 'utf-8').trim(), source: `file: ${initialInput}` };
}

export async function interactiveMode(initialInput, cliOptions, themeManager, sessionManager, statsTracker, userPreferences) {
    if (initialInput && cliOptions.autoDifficulty) {
        const { prompt, error } = resolveInputPrompt(initialInput, cliOptions);
        if (error) {
            console.log(themeManager.color('error')(error));
            return;
        }

        const rawPrompt = prompt;
        const { difficulty, iterations: recIter, score: compScore } = analyzePromptComplexity(rawPrompt);
        displayBanner({ themeManager, userPreferences, config: loadConfig(), interactive: false });
        console.log(themeManager.color('info')(`📝 Input: ${formatProcessingPromptDisplay(rawPrompt, PROMPT_LIMITS.processingDisplayMaxLength)}`));
        displayComplexityAnalysis({ themeManager }, difficulty, recIter, compScore, rawPrompt);
        return;
    }

    displayBanner({ themeManager, userPreferences, config: loadConfig(), interactive: true });
    
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
        prompt: themeManager.color('primary')('⚡ ') + themeManager.color('text')('> ')
    });

    let config = { ...getDefaultConfig(), ...loadConfig() };
    if (cliOptions.provider) {
        config.provider = cliOptions.provider;
    }
    if (cliOptions.model) {
        config.model = cliOptions.model;
    }
    const resolvedApiKey = resolveApiKey(config.provider, config.apiKey) ?? (config.provider === 'ollama' ? PROVIDERS.ollama.baseURL : null);
    if (resolvedApiKey) {
        config.apiKey = resolvedApiKey;
    }
    
    if (!config.apiKey) {
        if (initialInput && !cliOptions.interactive) {
            console.log(themeManager.color('error')('✗ No API key configured. Set the proper environment variable or run with --config.'));
            rl.close();
            return;
        }
        console.log(themeManager.color('warning')('⚠️  First time setup required.'));
        console.log(themeManager.color('muted')('Please configure your API provider and key to continue.\n'));
        config = await promptForConfig();
        if (!config || !config.apiKey) {
            console.log(themeManager.color('error')('\n✗ Configuration cancelled or incomplete.'));
            rl.close();
            return;
        }
    }

    let client;
    try {
        const apiKeyInitial = resolveApiKey(config.provider, config.apiKey);
        if (!apiKeyInitial && config.provider !== 'ollama') {
            throw new ConfigError('API key is required. Please configure with /settings or set environment variable.');
        }
        client = getProviderClient(config.provider ?? 'openrouter', apiKeyInitial);
    } catch (error) {
        if (error instanceof ConfigError) {
            console.log(themeManager.color('error')(`\n✗ ${error.message}`));
        } else {
            console.log(themeManager.color('error')(`\n✗ Failed to initialize API client: ${error.message}`));
        }
        console.log(themeManager.color('muted')('Please check your configuration with /settings'));
        rl.close();
        return;
    }
    
    let sessionCounter = 1;
    let lastResult = null;

    if (initialInput) {
        const { prompt, source, error } = resolveInputPrompt(initialInput, cliOptions);
        if (error) {
            console.log(themeManager.color('error')(error));
            if (!cliOptions.interactive) {
                rl.close();
                return;
            }
        }

        if (prompt) {
            console.log(themeManager.color('info')(`📝 Initial input: ${source}`));
            const { difficulty, iterations: recIter, score: compScore } = analyzePromptComplexity(prompt);
            displayComplexityAnalysis({ themeManager }, difficulty, recIter, compScore, prompt);

            if (cliOptions.autoDifficulty) {
                rl.close();
                return;
            }

            const result = await processPrompt(prompt, client, { ...config, _cliOptions: cliOptions }, sessionCounter++, themeManager, statsTracker, lastResult);
            if (result) lastResult = result.lastResult;
            
            if (!cliOptions.interactive) {
                rl.close();
                return;
            }
        }
    }

    rl.prompt();

    rl.on('line', async (input) => {
        const trimmedInput = input.trim();
        
        if (trimmedInput === '') {
            rl.prompt();
            return;
        }
        
        if (trimmedInput.startsWith('/')) {
            const command = trimmedInput.toLowerCase().split(' ')[0];
            
            const validation = validateAndSuggestCommand(command);
            
            if (!validation.valid && validation.isCommand) {
                console.log(themeManager.color('error')(`✗ ${validation.message}`));
                rl.prompt();
                return;
            }
            
            if (validation.valid) {
                userPreferences.trackCommand(command);
            }
            
            if (command === '/exit' || command === '/quit') {
                console.log(themeManager.color('success')('\n✨ Thanks for using KleoSr PE2-CLI! Goodbye!'));
                rl.close();
                return;
            }
            
            process.stdout.write('\r\x1b[K');
            
            const result = await handleCommand(command, config, themeManager, sessionManager, userPreferences, lastResult);
            
            if (result?.batch) {
                for (const prompt of result.batch) {
                    console.log(`\n${themeManager.color('info')('Processing prompt:')} ${formatProcessingPromptDisplay(prompt, 80)}`);
                    const processResult = await processPrompt(prompt, client, config, sessionCounter++, themeManager, statsTracker, lastResult);
                    if (processResult) lastResult = processResult.lastResult;
                }
            } else if (result && typeof result === 'object') {
                config = { ...config, ...result };
                if (result.provider || result.apiKey) {
                    client = getProviderClient(config.provider || 'openrouter', resolveApiKey(config.provider, config.apiKey));
                }
                displayBanner({ themeManager, userPreferences, config: loadConfig(), interactive: true });
            } else if (typeof result === 'string') {
                const processResult = await processPrompt(result, client, config, sessionCounter++, themeManager, statsTracker, lastResult);
                if (processResult) lastResult = processResult.lastResult;
            }
            
            rl.prompt();
            return;
        }
        
        if (trimmedInput === 'exit' || trimmedInput === 'quit') {
            console.log(themeManager.color('success')('\n✨ Thanks for using KleoSr PE2-CLI! Goodbye!'));
            rl.close();
            return;
        }
        
        if (trimmedInput === 'help') {
            await handleCommand('/help', config, themeManager, sessionManager, userPreferences, lastResult);
            rl.prompt();
            return;
        }
        
        if (trimmedInput === 'config') {
            await handleCommand('/settings', config, themeManager, sessionManager, userPreferences, lastResult);
            rl.prompt();
            return;
        }
        
        if (trimmedInput === 'status') {
            await handleCommand('/config', config, themeManager, sessionManager, userPreferences, lastResult);
            rl.prompt();
            return;
        }
        
        const processResult = await processPrompt(trimmedInput, client, config, sessionCounter++, themeManager, statsTracker, lastResult);
        if (processResult) lastResult = processResult.lastResult;
        rl.prompt();
    });

    rl.on('close', () => {
        console.log(themeManager.color('success')('\n✨ Session ended. Have a great day!'));
        process.exit(0);
    });
}
