import fs from 'fs';
import readline from 'readline';
import chalk from 'chalk';
import { PROVIDERS, getProviderClient } from '../providers/index.js';
import { getDefaultConfig, loadConfig, resolveApiKey } from '../config.js';
import { promptForConfig } from '../configPrompt.js';
import { analyzePromptComplexity } from '../analysis.js';
import { displayBanner, displayInteractiveBanner, formatProcessingPromptDisplay, displayComplexityAnalysis } from '../ui.js';
import { PROMPT_LIMITS } from '../constants.js';
import { validateAndSuggestCommand } from '../utils/index.js';
import { ConfigError } from '../errorHandler.js';
import { handleCommand } from './commands.js';
import { processPrompt } from './promptProcessing.js';

export async function interactiveMode(initialInput, cliOptions, themeManager, sessionManager, statsTracker, userPreferences) {
    if (initialInput && cliOptions.autoDifficulty) {
        const rawPrompt = (() => {
            try {
                if (!cliOptions.text && !cliOptions.file && fs.existsSync(initialInput)) {
                    return fs.readFileSync(initialInput, 'utf-8').trim();
                }
            } catch {}
            return initialInput;
        })();
        const { difficulty, iterations: recIter, score: compScore } = analyzePromptComplexity(rawPrompt);
        displayBanner({ themeManager, userPreferences, config: loadConfig(), interactive: false });
        console.log(themeManager.color('info')(`📝 Input: ${formatProcessingPromptDisplay(rawPrompt, PROMPT_LIMITS.processingDisplayMaxLength)}`));
        displayComplexityAnalysis({ themeManager }, difficulty, recIter, compScore, rawPrompt);
        return;
    }

    displayInteractiveBanner({ themeManager, userPreferences, config: loadConfig() });
    
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
    let resolvedApiKey = resolveApiKey(config.provider, config.apiKey);
    if (!resolvedApiKey && config.provider === 'ollama') {
        resolvedApiKey = PROVIDERS.ollama.baseURL;
    }
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
        config = await promptForConfig(rl);
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
        client = getProviderClient(config.provider || 'openrouter', apiKeyInitial);
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
        let rawPrompt = initialInput;
        let inputSource = 'direct text';

        if (!cliOptions.text && !cliOptions.file && fs.existsSync(initialInput)) {
            rawPrompt = fs.readFileSync(initialInput, 'utf-8').trim();
            inputSource = `file: ${initialInput}`;
        } else if (cliOptions.file) {
            if (fs.existsSync(initialInput)) {
                rawPrompt = fs.readFileSync(initialInput, 'utf-8').trim();
                inputSource = `file: ${initialInput}`;
            } else {
                console.log(themeManager.color('error')(`❌ Error: File not found at ${initialInput}`));
            }
        }

        if (rawPrompt) {
            console.log(themeManager.color('info')(`📝 Initial input: ${inputSource}`));
            const { difficulty, iterations: recIter, score: compScore } = analyzePromptComplexity(rawPrompt);
            displayComplexityAnalysis({ themeManager }, difficulty, recIter, compScore, rawPrompt);

            if (cliOptions.autoDifficulty) {
                rl.close();
                return;
            }

            if (!cliOptions.interactive) {
                const result = await processPrompt(rawPrompt, client, { ...config, _cliOptions: cliOptions }, sessionCounter++, themeManager, statsTracker, userPreferences, lastResult);
                if (result) lastResult = result.lastResult;
                rl.close();
                return;
            }

            const result = await processPrompt(rawPrompt, client, { ...config, _cliOptions: cliOptions }, sessionCounter++, themeManager, statsTracker, userPreferences, lastResult);
            if (result) lastResult = result.lastResult;
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
            
            const result = await handleCommand(command, rl, config, themeManager, sessionManager, userPreferences, lastResult);
            
            if (result && typeof result === 'object') {
                config = { ...config, ...result };
                if (result.provider || result.apiKey) {
                    const newApiKey = resolveApiKey(config.provider, config.apiKey);
                    const newClient = config.provider ? getProviderClient(config.provider, newApiKey) : getProviderClient('openrouter', newApiKey);
                    client = newClient;
                }

                displayInteractiveBanner({ themeManager, userPreferences, config: loadConfig() });
            }
            
            if (typeof result === 'string') {
                const processResult = await processPrompt(result, client, config, sessionCounter++, themeManager, statsTracker, userPreferences, lastResult);
                if (processResult) lastResult = processResult.lastResult;
            } else if (result && result.batch) {
                for (const prompt of result.batch) {
                    console.log(`\n${themeManager.color('info')('Processing prompt:')} ${formatProcessingPromptDisplay(prompt, 80)}`);
                    const processResult = await processPrompt(prompt, client, config, sessionCounter++, themeManager, statsTracker, userPreferences, lastResult);
                    if (processResult) lastResult = processResult.lastResult;
                }
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
            await handleCommand('/help', rl, config, themeManager, sessionManager, userPreferences, lastResult);
            rl.prompt();
            return;
        }
        
        if (trimmedInput === 'config') {
            await handleCommand('/settings', rl, config, themeManager, sessionManager, userPreferences, lastResult);
            rl.prompt();
            return;
        }
        
        if (trimmedInput === 'status') {
            await handleCommand('/config', rl, config, themeManager, sessionManager, userPreferences, lastResult);
            rl.prompt();
            return;
        }
        
        const processResult = await processPrompt(trimmedInput, client, config, sessionCounter++, themeManager, statsTracker, userPreferences, lastResult);
        if (processResult) lastResult = processResult.lastResult;
        rl.prompt();
    });

    rl.on('close', () => {
        console.log(themeManager.color('success')('\n✨ Session ended. Have a great day!'));
        process.exit(0);
    });
}

