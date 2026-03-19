import { loadConfig, resolveApiKey } from '../config.js';
import { displayBanner, formatProcessingPromptDisplay } from '../ui.js';
import { validateAndSuggestCommand } from '../utils/validation.js';
import { getProviderClient } from '../providers/index.js';
import { handleCommand } from './commands.js';
import { runPromptOptimizationPipeline } from './promptProcessing.js';

function buildCommandContext(state) {
    return {
        config: state.config,
        themeManager: state.themeManager,
        sessionManager: state.sessionManager,
        userPreferences: state.userPreferences,
        lastResult: state.lastResult
    };
}

function configWithCliOptions(config, cliOptions) {
    return { ...config, _cliOptions: cliOptions };
}

async function runOptimizationForPrompt(state, promptText) {
    const processResult = await runPromptOptimizationPipeline({
        prompt: promptText,
        client: state.client,
        config: configWithCliOptions(state.config, state.cliOptions),
        sessionId: state.sessionCounter++,
        themeManager: state.themeManager,
        statsTracker: state.statsTracker,
        previousOptimizedMarkdown: state.lastResult
    });
    if (processResult) {
        state.lastResult = processResult.lastResult;
    }
}

async function handleSlashCommandLine(state, command) {
    const validation = validateAndSuggestCommand(command);

    if (!validation.valid && validation.isCommand) {
        console.log(state.themeManager.color('error')(`✗ ${validation.message}`));
        state.rl.prompt();
        return;
    }

    if (validation.valid) {
        state.userPreferences.trackCommand(command);
    }

    if (command === '/exit' || command === '/quit') {
        console.log(state.themeManager.color('success')('\n✨ Thanks for using KleoSr PE2-CLI! Goodbye!'));
        state.rl.close();
        return;
    }

    process.stdout.write('\r\x1b[K');

    const result = await handleCommand(command, buildCommandContext(state));

    if (result?.batch) {
        for (const prompt of result.batch) {
            console.log(`\n${state.themeManager.color('info')('Processing prompt:')} ${formatProcessingPromptDisplay(prompt, 80)}`);
            await runOptimizationForPrompt(state, prompt);
        }
    } else if (result && typeof result === 'object') {
        state.config = { ...state.config, ...result };
        if (result.provider || result.apiKey) {
            state.client = getProviderClient(
                state.config.provider || 'openrouter',
                resolveApiKey(state.config.provider, state.config.apiKey)
            );
        }
        displayBanner({
            themeManager: state.themeManager,
            userPreferences: state.userPreferences,
            config: loadConfig(),
            interactive: true
        });
    } else if (typeof result === 'string') {
        await runOptimizationForPrompt(state, result);
    }

    state.rl.prompt();
}

async function handlePlainTextLine(state, trimmedInput) {
    if (trimmedInput === 'exit' || trimmedInput === 'quit') {
        console.log(state.themeManager.color('success')('\n✨ Thanks for using KleoSr PE2-CLI! Goodbye!'));
        state.rl.close();
        return;
    }

    if (trimmedInput === 'help') {
        await handleCommand('/help', buildCommandContext(state));
        state.rl.prompt();
        return;
    }

    if (trimmedInput === 'config') {
        await handleCommand('/settings', buildCommandContext(state));
        state.rl.prompt();
        return;
    }

    if (trimmedInput === 'status') {
        await handleCommand('/config', buildCommandContext(state));
        state.rl.prompt();
        return;
    }

    await runOptimizationForPrompt(state, trimmedInput);
    state.rl.prompt();
}

export async function handleInteractiveUserLine(state, trimmedInput) {
    if (trimmedInput === '') {
        state.rl.prompt();
        return;
    }

    if (trimmedInput.startsWith('/')) {
        const command = trimmedInput.toLowerCase().split(' ')[0];
        await handleSlashCommandLine(state, command);
        return;
    }

    await handlePlainTextLine(state, trimmedInput);
}
