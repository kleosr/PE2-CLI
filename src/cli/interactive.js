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
        if (!exists) {
            return { prompt: null, source: `file: ${initialInput}`, error: `File not found: ${initialInput}` };
        }
        const fileContents = fs.readFileSync(initialInput, 'utf-8').trim();
        return { prompt: fileContents, source: `file: ${initialInput}` };
    }

    if (!exists) return { prompt: initialInput, source: 'direct text' };
    const fileContents = fs.readFileSync(initialInput, 'utf-8').trim();
    return { prompt: fileContents, source: `file: ${initialInput}` };
}

async function handleAutoDifficultyMode(options) {
    const { initialInput, cliOptions, themeManager, userPreferences } = options;
    const { prompt, error } = resolveInputPrompt(initialInput, cliOptions);
    if (error) {
        console.log(themeManager.color('error')(error));
        return true;
    }

    const { difficulty, iterations: recIter, score: compScore } = analyzePromptComplexity(prompt);
    displayBanner({ themeManager, userPreferences, config: loadConfig(), interactive: false });
    const displayText = formatProcessingPromptDisplay(prompt, PROMPT_LIMITS.processingDisplayMaxLength);
    console.log(themeManager.color('info')(`Input: ${displayText}`));
    displayComplexityAnalysis({ themeManager, difficulty, iterations: recIter, score: compScore, rawPrompt: prompt });
    return true;
}

function mergeConfigWithCliOptions(baseConfig, cliOptions) {
    const merged = { ...baseConfig };
    if (cliOptions.provider) merged.provider = cliOptions.provider;
    if (cliOptions.model) merged.model = cliOptions.model;
    const resolvedKey = resolveApiKey(merged.provider, merged.apiKey);
    const fallbackKey = merged.provider === 'ollama' ? PROVIDERS.ollama.baseURL : null;
    if (resolvedKey ?? fallbackKey) merged.apiKey = resolvedKey ?? fallbackKey;
    return merged;
}

async function promptForInitialConfig(themeManager, rl) {
    console.log(themeManager.color('warning')('First time setup required.'));
    console.log(themeManager.color('muted')('Please configure your API provider and key to continue.\n'));
    const newConfig = await promptForConfig(themeManager);
    if (!newConfig || !newConfig.apiKey) {
        console.log(themeManager.color('error')('\nConfiguration cancelled or incomplete.'));
        rl.close();
        return null;
    }
    return newConfig;
}

async function createApiClient(config, themeManager, rl) {
    try {
        const apiKey = resolveApiKey(config.provider, config.apiKey);
        if (!apiKey && config.provider !== 'ollama') {
            throw new ConfigError('API key is required. Configure with /settings or set environment variable.');
        }
        return getProviderClient(config.provider ?? 'openrouter', apiKey);
    } catch (error) {
        if (error instanceof ConfigError) {
            console.log(themeManager.color('error')(`\n${error.message}`));
        } else {
            console.log(themeManager.color('error')(`\nFailed to initialize API client: ${error.message}`));
        }
        console.log(themeManager.color('muted')('Please check your configuration with /settings'));
        rl.close();
        return null;
    }
}

async function initializeApiClient(config, themeManager, initialInput, cliOptions, rl) {
    if (!config.apiKey) {
        if (initialInput && !cliOptions.interactive) {
            console.log(themeManager.color('error')('No API key configured. Set environment variable or run with --config.'));
            rl.close();
            return null;
        }
        return promptForInitialConfig(themeManager, rl);
    }
    return createApiClient(config, themeManager, rl);
}

function createReadlineInterface(themeManager) {
    return readline.createInterface({
        input: process.stdin,
        output: process.stdout,
        prompt: themeManager.color('primary')('> ')
    });
}

async function processInitialInput(loopState, initialInput) {
    const { prompt, source, error } = resolveInputPrompt(initialInput, loopState.cliOptions);
    if (error) {
        console.log(loopState.themeManager.color('error')(error));
        if (!loopState.cliOptions.interactive) {
            loopState.rl.close();
            return false;
        }
    }

    if (!prompt) return true;

    console.log(loopState.themeManager.color('info')(`Initial input: ${source}`));
    const { difficulty, iterations: recIter, score: compScore } = analyzePromptComplexity(prompt);
    displayComplexityAnalysis({
        themeManager: loopState.themeManager,
        difficulty,
        iterations: recIter,
        score: compScore,
        rawPrompt: prompt
    });

    const result = await runPromptOptimizationPipeline({
        prompt,
        client: loopState.client,
        config: { ...loopState.config, _cliOptions: loopState.cliOptions },
        sessionId: loopState.sessionCounter++,
        themeManager: loopState.themeManager,
        statsTracker: loopState.statsTracker,
        previousOptimizedMarkdown: loopState.lastResult
    });
    if (result) loopState.lastResult = result.lastResult;

    if (!loopState.cliOptions.interactive) {
        loopState.rl.close();
        return false;
    }
    return true;
}

function setupEventListeners(loopState) {
    loopState.rl.prompt();

    loopState.rl.on('line', async (input) => {
        await handleInteractiveUserLine(loopState, input.trim());
    });

    loopState.rl.on('close', () => {
        console.log(loopState.themeManager.color('success')('\nSession ended.'));
        process.exit(0);
    });
}

export async function interactiveMode(session) {
    if (session.initialInput && session.cliOptions.autoDifficulty) {
        await handleAutoDifficultyMode({
            initialInput: session.initialInput,
            cliOptions: session.cliOptions,
            themeManager: session.themeManager,
            userPreferences: session.userPreferences
        });
        return;
    }

    displayBanner({ themeManager: session.themeManager, userPreferences: session.userPreferences, config: loadConfig(), interactive: true });
    const rl = createReadlineInterface(session.themeManager);

    const config = mergeConfigWithCliOptions({ ...getDefaultConfig(), ...loadConfig() }, session.cliOptions);
    const client = await initializeApiClient(config, session.themeManager, session.initialInput, session.cliOptions, rl);
    if (!client) return;

    const loopState = {
        rl,
        cliOptions: session.cliOptions,
        themeManager: session.themeManager,
        sessionManager: session.sessionManager,
        userPreferences: session.userPreferences,
        statsTracker: session.statsTracker,
        config,
        client,
        sessionCounter: 1,
        lastResult: null
    };

    if (session.initialInput) {
        const shouldContinue = await processInitialInput(loopState, session.initialInput);
        if (!shouldContinue) return;
    }

    setupEventListeners(loopState);
}
