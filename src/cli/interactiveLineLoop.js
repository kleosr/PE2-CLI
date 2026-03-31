import { loadConfig, resolveApiKey } from '../config.js';
import { displayBanner, formatProcessingPromptDisplay } from '../ui.js';
import { validateAndSuggestCommand } from '../utils/validation.js';
import { getProviderClient } from '../providers/index.js';
import { handleCommand } from './commands.js';
import { runPromptOptimizationPipeline } from './promptProcessing.js';

const EXIT_COMMANDS = new Set(['/exit', '/quit', 'exit', 'quit']);
const PLAIN_TEXT_COMMANDS = {
  help: '/help',
  config: '/settings',
  status: '/config'
};

function buildCommandContext(loopState) {
    return {
        config: loopState.config,
        themeManager: loopState.themeManager,
        sessionManager: loopState.sessionManager,
        userPreferences: loopState.userPreferences,
        lastResult: loopState.lastResult
    };
}

function configWithCliOptions(baseConfig, cliOptions) {
    return { ...baseConfig, _cliOptions: cliOptions };
}

async function runOptimizationForPrompt(loopState, promptText) {
    const processResult = await runPromptOptimizationPipeline({
        prompt: promptText,
        client: loopState.client,
        config: configWithCliOptions(loopState.config, loopState.cliOptions),
        sessionId: loopState.sessionCounter++,
        themeManager: loopState.themeManager,
        statsTracker: loopState.statsTracker,
        previousOptimizedMarkdown: loopState.lastResult
    });
    if (processResult) {
        loopState.lastResult = processResult.lastResult;
    }
}

function updateClientFromConfig(loopState, result) {
    loopState.config = { ...loopState.config, ...result };
    if (result.provider || result.apiKey) {
        loopState.client = getProviderClient(
            loopState.config.provider || 'openrouter',
            resolveApiKey(loopState.config.provider, loopState.config.apiKey)
        );
    }
}

function handleBatchResult(loopState, batch) {
    for (const prompt of batch) {
        console.log(`\nProcessing prompt: ${formatProcessingPromptDisplay(prompt, 80)}`);
        runOptimizationForPrompt(loopState, prompt);
    }
}

function handleInvalidCommand(loopState, validation) {
    console.log(loopState.themeManager.color('error')(validation.message));
    loopState.rl.prompt();
}

function handleExitCommand(loopState) {
    console.log(loopState.themeManager.color('success')('\nThanks for using KleoSr PE2-CLI! Goodbye!'));
    loopState.rl.close();
}

function handleCommandResult(loopState, result) {
    if (result?.batch) {
        handleBatchResult(loopState, result.batch);
    } else if (result && typeof result === 'object') {
        updateClientFromConfig(loopState, result);
        displayBanner({
            themeManager: loopState.themeManager,
            userPreferences: loopState.userPreferences,
            config: loadConfig(),
            interactive: true
        });
    } else if (typeof result === 'string') {
        runOptimizationForPrompt(loopState, result);
    }
}

async function handleSlashCommandLine(loopState, command) {
    const validation = validateAndSuggestCommand(command);

    if (!validation.valid && validation.isCommand) {
        handleInvalidCommand(loopState, validation);
        return;
    }

    if (validation.valid) {
        loopState.userPreferences.trackCommand(command);
    }

    if (EXIT_COMMANDS.has(command)) {
        handleExitCommand(loopState);
        return;
    }

    process.stdout.write('\r\x1b[K');
    const result = await handleCommand(command, buildCommandContext(loopState));
    handleCommandResult(loopState, result);
    loopState.rl.prompt();
}

async function handlePlainTextLine(loopState, trimmedInput) {
    if (EXIT_COMMANDS.has(trimmedInput)) {
        console.log(loopState.themeManager.color('success')('\nThanks for using KleoSr PE2-CLI! Goodbye!'));
        loopState.rl.close();
        return;
    }

    const mappedCommand = PLAIN_TEXT_COMMANDS[trimmedInput];
    if (mappedCommand) {
        await handleCommand(mappedCommand, buildCommandContext(loopState));
        loopState.rl.prompt();
        return;
    }

    await runOptimizationForPrompt(loopState, trimmedInput);
    loopState.rl.prompt();
}

export async function handleInteractiveUserLine(loopState, trimmedInput) {
    if (trimmedInput === '') {
        loopState.rl.prompt();
        return;
    }

    if (trimmedInput.startsWith('/')) {
        const command = trimmedInput.toLowerCase().split(' ')[0];
        await handleSlashCommandLine(loopState, command);
        return;
    }

    await handlePlainTextLine(loopState, trimmedInput);
}
