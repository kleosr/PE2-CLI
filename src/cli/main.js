#!/usr/bin/env node
import { Command } from 'commander';
import { SessionManager } from '../utils/session.js';
import { ThemeManager } from '../utils/theme.js';
import { StatsTracker } from '../utils/stats.js';
import { UserPreferences } from '../utils/preferences.js';
import { setupGlobalErrorHandlers } from '../errorHandler.js';
import { displayBanner, setTerminalTitle } from '../ui.js';
import { CLI_SEMVER, PE2_CODE_GENERATION, cliVersionWithPrefix } from '../versionInfo.js';
import { loadConfig } from '../config.js';
import { promptForConfig } from '../configPrompt.js';
import { interactiveMode } from './interactive.js';

const themeManager = new ThemeManager();
const sessionManager = new SessionManager();
const statsTracker = new StatsTracker();
const userPreferences = new UserPreferences();

const savedTheme = userPreferences.get('theme');
if (savedTheme === 'light' || savedTheme === 'dark') {
    themeManager.setTheme(savedTheme);
}

setupGlobalErrorHandlers(themeManager);

const HELP_TEXT = `
Examples:
  npx @kleosr/pe2-cli                              # Start interactive mode
  npx @kleosr/pe2-cli --config                     # Configure API key
  npx @kleosr/pe2-cli "Write a Python function"    # Process text
  npx @kleosr/pe2-cli input.txt                    # Process file
  npx @kleosr/pe2-cli input.txt --model openai/gpt-4o  # Override model
  npx @kleosr/pe2-cli "Complex prompt" --iterations 3

Configuration:
  First run will prompt for API key and model.
  Settings saved to ~/.kleosr-pe2/config.json

Input Methods:
  1. Direct text: npx @kleosr/pe2-cli "Your prompt"
  2. File path: npx @kleosr/pe2-cli input.txt
  3. Interactive: npx @kleosr/pe2-cli (no args)
  4. Force text: --text flag
  5. Force file: --file flag
            `;

const CLI_OPTIONS = [
    ['--model <model>', 'Override model'],
    ['--iterations <number>', 'Refinement rounds', parseInt],
    ['--output-file <file>', 'Output markdown path'],
    ['--auto-difficulty', 'Show complexity analysis'],
    ['-i, --interactive', 'Start interactive mode'],
    ['--config', 'Configure API key and model'],
    ['--text', 'Force input as text'],
    ['--file', 'Force input as file path'],
    ['--provider <provider>', 'Override provider']
];

function applyCliOptions(program) {
    for (const opt of CLI_OPTIONS) {
        program.option(opt[0], opt[1], opt[2]);
    }
}

function configureProgram(program) {
    program
        .name('pe2-cli')
        .description('Convert raw prompts to PE2-optimized prompts.')
        .version(CLI_SEMVER)
        .argument('[input]', 'Text prompt or file path (optional)')
        .showHelpAfterError('(add --help for details)')
        .configureOutput({
            outputError: (str, write) => write(themeManager.color('error')(str))
        })
        .configureHelp({ afterAll: HELP_TEXT });
    applyCliOptions(program);
}

async function runConfigMode() {
    setTerminalTitle('KleoSr PE2-CLI - Configuration');
    displayBanner({ themeManager, userPreferences, config: loadConfig(), interactive: false });
    const versionInfo = `${cliVersionWithPrefix()} • Code ${PE2_CODE_GENERATION}`;
    const timestamp = new Date().toLocaleString();
    console.log(themeManager.color('success')(`Configuration Mode | ${versionInfo} | ${timestamp}`));
    console.log(themeManager.color('primary')('='.repeat(78)));
    await promptForConfig(themeManager);
}

async function runInteractiveMode(input, options) {
    await interactiveMode({
        initialInput: input,
        cliOptions: options,
        themeManager,
        sessionManager,
        statsTracker,
        userPreferences
    });
}

function handleMainError(error) {
    if (error.code === 'commander.helpDisplayed' || error.code === 'commander.version') {
        return;
    }
    console.error(themeManager.color('error')(error.message));
    if (error.stack && process.env.DEBUG) {
        console.error(error.stack);
    }
    process.exit(error.exitCode || 1);
}

async function main() {
    setTerminalTitle('KleoSr PE2-CLI');
    const program = new Command();
    configureProgram(program);

    try {
        await program.parseAsync(process.argv);
        const options = program.opts();
        const input = program.args[0];

        if (options.config) {
            await runConfigMode();
            return;
        }

        await runInteractiveMode(input, options);
    } catch (error) {
        handleMainError(error);
    }
}

main();
