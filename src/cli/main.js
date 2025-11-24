#!/usr/bin/env node
import { Command } from 'commander';
import readline from 'readline';
import { SessionManager, ThemeManager, StatsTracker, UserPreferences } from '../utils/index.js';
import { setupGlobalErrorHandlers } from '../errorHandler.js';
import { displayBanner, setTerminalTitle } from '../ui.js';
import { loadConfig } from '../config.js';
import { promptForConfig } from '../configPrompt.js';
import { interactiveMode } from './interactive.js';

const themeManager = new ThemeManager();
const sessionManager = new SessionManager();
const statsTracker = new StatsTracker();
const userPreferences = new UserPreferences();

setupGlobalErrorHandlers();

async function main() {
    setTerminalTitle('KleoSr PE2-CLI');
    const program = new Command();
    
    program
        .name('pe2-cli')
        .description('🚀 KleoSr PE2-CLI: Convert raw prompts to PE2-optimized prompts using adaptive intelligence.')
        .version('3.4.5')
        .argument('[input]', 'Text prompt or path to file (optional - if not provided, starts interactive mode)')
        .option('--model <model>', 'OpenRouter model name (overrides config)')
        .option('--iterations <number>', 'Number of PE2 refinement rounds (auto-detected if not specified)', parseInt)
        .option('--output-file <file>', 'Path to the output markdown file')
        .option('--auto-difficulty', 'Show complexity analysis and exit without processing')
        .option('-i, --interactive', 'Start in interactive mode')
        .option('--config', 'Configure API key and model settings')
        .option('--text', 'Force input to be treated as text (not file path)')
        .option('--file', 'Force input to be treated as file path')
        .option('--provider <provider>', 'Override provider for this run (openai|anthropic|google|openrouter|ollama)')
        .showHelpAfterError('(add --help for additional information)')
        .configureOutput({
            outputError: (str, write) => write(themeManager.color('error')(str))
        })
        .configureHelp({
            afterAll: `
Examples:
  npx @kleosr/pe2-cli                              # Start interactive mode (auto-config)
  npx @kleosr/pe2-cli --config                     # Configure API key and model
  npx @kleosr/pe2-cli --interactive                # Start interactive mode
  npx @kleosr/pe2-cli "Write a Python function"    # Process text directly
  npx @kleosr/pe2-cli input.txt                    # Process file with saved config
  npx @kleosr/pe2-cli input.txt --model openai/gpt-4o  # Override model for this run
  npx @kleosr/pe2-cli "Complex prompt" --iterations 3  # Direct text with specific iterations
  npx @kleosr/pe2-cli "Test prompt" --auto-difficulty  # Show complexity analysis for text
  npx @kleosr/pe2-cli "Complex prompt" --provider ollama --model llama3  # Override provider for this run

Configuration:
  First run will prompt for API key and model selection.
  Settings are saved to ~/.kleosr-pe2/config.json
  Use --config to reconfigure anytime.

Input Methods:
  1. Direct text: npx @kleosr/pe2-cli "Your prompt here"
  2. File path: npx @kleosr/pe2-cli input.txt
  3. Interactive: npx @kleosr/pe2-cli (no arguments)
  4. Force text: --text flag treats input as text even if it looks like a file
  5. Force file: --file flag treats input as file path
            `
        });

    try {
        await program.parseAsync(process.argv);
        const options = program.opts();
        const input = program.args[0];

    if (options.config) {
        setTerminalTitle('KleoSr PE2-CLI - Configuration');
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });
        
        displayBanner({ themeManager, userPreferences, config: loadConfig(), interactive: false });
            console.log(themeManager.color('success')(`🔧 Configuration Mode | v3.4.5 | ${new Date().toLocaleString()}`));
        console.log(themeManager.color('primary')('='.repeat(78)));
        
        await promptForConfig(rl);
        rl.close();
        return;
    }

    await interactiveMode(input, options, themeManager, sessionManager, statsTracker, userPreferences);
    } catch (error) {
        if (error.code === 'commander.helpDisplayed' || error.code === 'commander.version') {
            return;
        }
        console.error(themeManager.color('error')(`❌ Error: ${error.message}`));
        if (error.stack && process.env.DEBUG) {
            console.error(error.stack);
        }
        process.exit(error.exitCode || 1);
    }
}

main().catch(error => {
    console.error(themeManager.color('error')(`❌ Unexpected error: ${error.message}`));
    if (error.stack && process.env.DEBUG) {
        console.error(error.stack);
    }
    process.exit(1);
});

