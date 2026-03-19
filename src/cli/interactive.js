import fs from 'fs';
import readline from 'readline';
import { PROVIDERS, getProviderClient } from '../providers/index.js';
import { getDefaultConfig, loadConfig, resolveApiKey } from '../config.js';
import { promptForConfig } from '../configPrompt.js';
import { analyzePromptComplexity } from '../analysis.js';
import { displayBanner, formatProcessingPromptDisplay, displayComplexityAnalysis } from '../ui.js';
import { PROMPT_LIMITS } from '../constants.js';
import { ConfigError } from '../errorHandler.js';
import { runPromptOptimizationPipeline } from './promptProcessing.js';
import { handleInteractiveUserLine } from './interactiveLineLoop.js';

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

export async function interactiveMode(interactiveSession) {
    const {
        initialInput,
        cliOptions,
        themeManager,
        sessionManager,
        statsTracker,
        userPreferences
    } = interactiveSession;

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
        displayComplexityAnalysis({
            themeManager,
            difficulty,
            iterations: recIter,
            score: compScore,
            rawPrompt
        });
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
        config = await promptForConfig(themeManager);
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

    const loopState = {
        rl,
        cliOptions,
        themeManager,
        sessionManager,
        userPreferences,
        statsTracker,
        config,
        client,
        sessionCounter: 1,
        lastResult: null
    };

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
            displayComplexityAnalysis({
                themeManager,
                difficulty,
                iterations: recIter,
                score: compScore,
                rawPrompt: prompt
            });

            const result = await runPromptOptimizationPipeline({
                prompt,
                client,
                config: { ...config, _cliOptions: cliOptions },
                sessionId: loopState.sessionCounter++,
                themeManager,
                statsTracker,
                previousOptimizedMarkdown: loopState.lastResult
            });
            if (result) loopState.lastResult = result.lastResult;

            if (!cliOptions.interactive) {
                rl.close();
                return;
            }
        }
    }

    rl.prompt();

    rl.on('line', async (input) => {
        await handleInteractiveUserLine(loopState, input.trim());
    });

    rl.on('close', () => {
        console.log(themeManager.color('success')('\n✨ Session ended. Have a great day!'));
        process.exit(0);
    });
}
