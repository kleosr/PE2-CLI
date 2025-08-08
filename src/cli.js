#!/usr/bin/env node
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { Command } from 'commander';
import chalk from 'chalk';
import figlet from 'figlet';
import readline from 'readline';
import inquirer from 'inquirer';
import os from 'os';
import { SessionManager, createProgressBar, displayStatusBar, COMMANDS, validatePrompt, copyToClipboard, createTable, ThemeManager, StatsTracker, UserPreferences, validateAndSuggestCommand } from './utils.js';
import { PROVIDERS, getProviderClient } from './providers/index.js';
import { CONFIG_DIR, CONFIG_FILE, getDefaultConfig, loadConfig, saveConfig, resolveApiKey } from './config.js';
import { analyzePromptComplexity } from './analysis.js';
import { getInitialTemplate, getRefinementTemplate } from './templates.js';
import { buildMessages } from './messages.js';
import { promptForConfig } from './configPrompt.js';
import { displayBanner, displayInteractiveBanner, formatApiKeyDisplay, formatContentPreview, formatProcessingPromptDisplay, displayComplexityAnalysis } from './ui.js';
import { processPrompt as runEngine, generateInitialPrompt, refinePrompt } from './engine.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const sessionManager = new SessionManager();
const themeManager = new ThemeManager();
const statsTracker = new StatsTracker();
const userPreferences = new UserPreferences();

let lastResult = null;
let isProcessingPrompt = false;
const PROMPTS_DIR = path.join(process.cwd(), 'pe2-prompts');

const promptCache = new Map();
const MAX_CACHE_SIZE = 50;
const getCacheKey = (prompt, iterations) => (prompt.substring(0, 100) + '_' + iterations).replace(/[^a-zA-Z0-9_]/g, '').substring(0, 50);
const getCachedResult = (prompt, iterations) => promptCache.get(getCacheKey(prompt, iterations));
function setCachedResult(prompt, iterations, result) {
    const key = getCacheKey(prompt, iterations);
  if (promptCache.size >= MAX_CACHE_SIZE) promptCache.delete(promptCache.keys().next().value);
  promptCache.set(key, { result, timestamp: Date.now(), hits: 0 });
}

// === BEGIN: Optimize by removing redundant stub classes ===
// These stub classes add unnecessary overhead and complexity
// Replace with simple constants and direct function calls

const DEFAULT_CONTEXT = { domain: 'general', history: [], avgComplexity: 0 };
const DEFAULT_STRATEGY = { iterations: 2, focus: 'optimization', adaptiveFeatures: [], template: '' };
const DEFAULT_EVALUATION = { scores: {}, overallScore: 8.0 };

// Simplified helper functions
function getContext() {
    return DEFAULT_CONTEXT;
}

function selectStrategy() {
    return DEFAULT_STRATEGY;
}

async function evaluatePrompt() {
    return DEFAULT_EVALUATION;
}
// === END: Optimize by removing redundant stub classes ===

// UI helpers moved to ui.js

// Provider model lists moved to providers/index.js

function ensureConfigDir() {
    if (!fs.existsSync(CONFIG_DIR)) {
        fs.mkdirSync(CONFIG_DIR, { recursive: true });
    }
}

function loadConfig() {
    ensureConfigDir();
    if (fs.existsSync(CONFIG_FILE)) {
        try {
            const configData = fs.readFileSync(CONFIG_FILE, 'utf-8');
            return JSON.parse(configData);
        } catch (error) {
            console.log(chalk.yellow('Warning: Could not load config file, using defaults.'));
            return {};
        }
    }
    return {};
}

function saveConfig(config) {
    ensureConfigDir();
    try {
        // Ensure sensitive data is stored with user-only permissions (0600)
        fs.writeFileSync(CONFIG_FILE, JSON.stringify(config, null, 2), { encoding: 'utf-8', mode: 0o600 });
        // In case file existed previously with broader permissions
        fs.chmodSync(CONFIG_FILE, 0o600);
        return true;
    } catch (error) {
        console.log(chalk.red(`❌ Error saving config: ${error.message}`));
        return false;
    }
}

function getDefaultConfig() {
    return {
        apiKey: null,
        model: 'openai/gpt-4o-mini',
        provider: 'openrouter'
    };
}

// Banner helpers moved to ui.js

// Difficulty indicators are exposed by ui/templates modules

// Display complexity moved to ui.js

// Client selection is centralized in providers/index.js

function getProviderClient(provider, apiKey) {
    switch (provider) {
        case 'openai':
            return createOpenAIClient(apiKey, PROVIDERS.openai.baseURL);
        case 'openrouter':
            return createOpenRouterClient({ apiKey, baseURL: PROVIDERS.openrouter.baseURL });
        case 'anthropic':
            return createAnthropicClient(apiKey);
        case 'google':
            return createGoogleClient(apiKey);
        case 'ollama':
            // For Ollama the apiKey parameter is actually the base URL; fall back to default if empty
            return createOllamaClient(apiKey || PROVIDERS.ollama.baseURL);
        default:
            throw new Error(`Unsupported provider: ${provider}`);
    }
}

async function promptForConfig(rl) {
    console.log(chalk.hex('#FFD93D')('\n🔧 Configuration Setup'));
    console.log(chalk.hex('#B19CD9')('Let\'s configure your AI provider and API settings.\n'));
    
    try {
        // Provider selection
        const { provider } = await inquirer.prompt([
            {
                type: 'list',
                name: 'provider',
                message: 'Select your AI provider:',
                choices: [
                    { name: `${PROVIDERS.openai.name} - Direct OpenAI API`, value: 'openai' },
                    { name: `${PROVIDERS.anthropic.name} - Direct Anthropic API`, value: 'anthropic' },
                    { name: `${PROVIDERS.google.name} - Direct Google AI API`, value: 'google' },
                    { name: `${PROVIDERS.openrouter.name} - Access multiple providers`, value: 'openrouter' },
                    { name: `${PROVIDERS.ollama.name} - Local Ollama`, value: 'ollama' }
                ],
                default: 'openrouter'
            }
        ]);

        const providerConfig = PROVIDERS[provider];
        
        // === Provider-specific connection details ===
        let apiKey = '';
        if (provider === 'ollama') {
            const { baseURL } = await inquirer.prompt([
                {
                    type: 'input',
                    name: 'baseURL',
                    message: 'Enter your Ollama base URL (press Enter for default):',
                    default: providerConfig.baseURL
                }
            ]);
            apiKey = baseURL.trim();
        } else {
            const resp = await inquirer.prompt([
                {
                    type: 'password',
                    name: 'apiKey',
                    message: `Enter your ${providerConfig.keyLabel}:`,
                    mask: '*',
                    validate: (input) => {
                        if (!input.trim()) {
                            return 'API key is required';
                        }
                        return true;
                    }
                }
            ]);
            apiKey = resp.apiKey.trim();
        }
    
    // Model selection
        const { model } = await inquirer.prompt([
            {
                type: 'list',
                name: 'model',
                message: 'Select a model:',
                choices: [
                    ...providerConfig.models.map(model => ({
                        name: model === providerConfig.defaultModel ? `${model} (recommended)` : model,
                        value: model
                    })),
                    { name: '📝 Enter Custom Model', value: 'custom' }
                ],
                default: providerConfig.defaultModel
            }
        ]);

        let finalModel = model;
        if (model === 'custom') {
            const { customModel } = await inquirer.prompt([
                {
                    type: 'input',
                    name: 'customModel',
                    message: 'Enter custom model name:',
                    validate: (input) => {
                        if (!input.trim()) {
                            return 'Model name is required';
                        }
                        return true;
                    }
                }
            ]);
            finalModel = customModel.trim();
        }

        const config = {
            provider,
            apiKey,
            model: finalModel
        };
    
    if (saveConfig(config)) {
        console.log(chalk.hex('#50E3C2')(`\n✅ Configuration saved!`));
            console.log(chalk.hex('#B19CD9')(`🌐 Provider: ${providerConfig.name}`));
        console.log(chalk.hex('#B19CD9')(`📝 Model: ${config.model}`));
        console.log(chalk.hex('#B19CD9')(`🔑 API Key: ${config.apiKey.substring(0, 8)}...`));
        console.log(chalk.hex('#B19CD9')(`📁 Config saved to: ${CONFIG_FILE}\n`));
            return config;
    } else {
        console.log(chalk.red('❌ Failed to save configuration.'));
        return null;
    }
    } catch (error) {
        if (error.isTtyError) {
            console.log(chalk.red('❌ Interactive prompts are not supported in this environment.'));
            console.log(chalk.yellow('Please run this in a proper terminal.'));
        } else {
            console.log(chalk.red(`❌ Configuration error: ${error.message}`));
        }
        return null;
    }
}

// Message building is in messages.js

// Optimized prompt generation with improved efficiency
async function generateInitialPrompt(client, rawPrompt, model) {
    try {
        const systemContent = 'You are a precise prompt optimizer. Follow the instructions and return JSON only.';
        const userContent = getInitialTemplate(rawPrompt);
        const response = await client.chat.completions.create({
            model: model,
            messages: buildMessages({ system: systemContent, user: userContent }),
            max_tokens: 1024,
            temperature: 0.3,
        });

        const content = response.choices[0].message.content;
        
        // Debug logging
        if (process.env.DEBUG) {
            console.log(chalk.gray('Raw response content:'));
            console.log(chalk.gray(content.substring(0, 500) + '...'));
        }
        
        // Clean up the content to extract valid JSON
        try {
            // Try to parse as-is first
            return { prompt: JSON.parse(content), edits: "Initial prompt generation." };
        } catch (jsonError) {
            // If that fails, try to extract JSON from the content
            // Robust brace extraction: take substring from first '{' to matching last '}'
            const firstBrace = content.indexOf('{');
            const lastBrace = content.lastIndexOf('}');
            if (firstBrace !== -1 && lastBrace !== -1 && lastBrace > firstBrace) {
                try {
                    let jsonStr = content.slice(firstBrace, lastBrace + 1);
                    jsonStr = jsonStr.replace(/,(\s*[}\]])/g, '$1');
                    const parsed = JSON.parse(jsonStr);
                    
                    // Validate that all required fields are present
                    const requiredFields = ['context', 'role', 'task', 'constraints', 'output'];
                    const hasAllFields = requiredFields.every(field => parsed.hasOwnProperty(field));
                    
                    if (hasAllFields) {
                        return { prompt: parsed, edits: "Initial prompt generation." };
                    } else {
                        // If fields are missing, try to construct them
                        const validPrompt = {
                            context: parsed.context || "No context provided",
                            role: parsed.role || "Expert assistant",
                            task: parsed.task || "Complete the requested task",
                            constraints: parsed.constraints || "Follow best practices",
                            output: parsed.output || "Provide appropriate output"
                        };
                        return { prompt: validPrompt, edits: "Initial prompt generation with field validation." };
                    }
                } catch (parseError) {
                    console.log(chalk.yellow(`Warning: JSON extraction failed: ${parseError.message}`));
                }
            }
            
            // If all else fails, try to extract individual fields
            const extractField = (fieldName) => {
                const patterns = [
                    new RegExp(`"${fieldName}"\\s*:\\s*"([^"]*)"`, 'i'),
                    new RegExp(`'${fieldName}'\\s*:\\s*'([^']*)'`, 'i'),
                    new RegExp(`${fieldName}\\s*:\\s*"([^"]*)"`, 'i'),
                    new RegExp(`\\*\\*${fieldName}\\*\\*:?\\s*([^\\n\\*]+)`, 'i'),
                    new RegExp(`${fieldName}:?\\s*([^\\n]+)`, 'i')
                ];
                
                for (const pattern of patterns) {
                    const match = content.match(pattern);
                    if (match && match[1]) {
                        return match[1].trim();
                    }
                }
                return null;
            };
            
            // Try to construct prompt from extracted fields
            const context = extractField('context');
            const role = extractField('role');
            const task = extractField('task');
            const constraints = extractField('constraints');
            const output = extractField('output');
            
            if (context || role || task) {
                return {
                    prompt: {
                        context: context || "Context based on: " + rawPrompt.substring(0, 200) + "...",
                        role: role || "Expert assistant specialized in the given domain",
                        task: task || "Complete the task as described in the user's prompt",
                        constraints: constraints || "Ensure accuracy, clarity, and adherence to best practices",
                        output: output || "Deliver a comprehensive and well-structured response"
                    },
                    edits: "Initial prompt generation with field extraction fallback."
                };
            }
            
            // Ultimate fallback - create a basic structure
            return {
                prompt: {
                    context: `The user wants to: ${rawPrompt.substring(0, 500)}${rawPrompt.length > 500 ? '...' : ''}`,
                    role: "Expert assistant with deep knowledge in the relevant domain",
                    task: "1. Understand the user's requirements\n2. Provide a comprehensive solution\n3. Ensure clarity and completeness",
                    constraints: "- Be accurate and thorough\n- Follow best practices\n- Provide clear explanations",
                    output: "A well-structured response that fully addresses the user's needs"
                },
                edits: "Initial prompt generation with automatic structuring."
            };
        }
    } catch (error) {
        console.log(chalk.red(`❌ Error during initial prompt generation: ${error.message}`));
        return { prompt: null, edits: null };
    }
}

// Optimized refinement with caching and improved efficiency
async function refinePrompt(client, currentPromptJson, refinementHistory, model, iterationNum) {
    // Check cache first
    const cachedResult = getCachedResult(currentPromptJson, iterationNum);
    if (cachedResult) {
        cachedResult.hits++;
        return cachedResult.result;
    }
    
    try {
        const systemContent = 'You are a precise prompt optimizer. Return JSON only.';
        const userContent = getRefinementTemplate(currentPromptJson, iterationNum);
        const response = await client.chat.completions.create({
            model: model,
            messages: buildMessages({ system: systemContent, user: userContent }),
            headers: {
                "HTTP-Referer": "https://pe2-cli-tool.local",
                "X-Title": "KleoSr PE2-CLI Tool",
            },
        });

        const content = response.choices[0].message.content;
        
        try {
            // Try to parse as-is first
            const refinedPromptJson = JSON.parse(content);
            const editsSummary = `Refined prompt based on PE2 principles (Iteration ${iterationNum}).`;
            return { prompt: refinedPromptJson, edits: editsSummary };
        } catch (jsonError) {
            // If that fails, try to extract JSON from the content
            // Robust brace extraction: take substring from first '{' to matching last '}'
            const firstBrace = content.indexOf('{');
            const lastBrace = content.lastIndexOf('}');
            
            if (firstBrace !== -1 && lastBrace !== -1 && lastBrace > firstBrace) {
                try {
                    let jsonStr = content.slice(firstBrace, lastBrace + 1);
                    jsonStr = jsonStr.replace(/,(\s*[}\]])/g, '$1');
                    const parsed = JSON.parse(jsonStr);
                    
                    // Validate that all required fields are present
                    const requiredFields = ['context', 'role', 'task', 'constraints', 'output'];
                    const hasAllFields = requiredFields.every(field => parsed.hasOwnProperty(field));
                    
                    if (hasAllFields) {
                        const result = { prompt: parsed, edits: "Refined prompt generation." };
                        // Cache successful results
                        setCachedResult(currentPromptJson, iterationNum, result);
                        return result;
                    } else {
                        // If fields are missing, try to construct them
                        const validPrompt = {
                            context: parsed.context || "No context provided",
                            role: parsed.role || "Expert assistant",
                            task: parsed.task || "Complete the requested task",
                            constraints: parsed.constraints || "Follow best practices",
                            output: parsed.output || "Provide appropriate output"
                        };
                        const result = { prompt: validPrompt, edits: "Refined prompt generation with field validation." };
                        setCachedResult(currentPromptJson, iterationNum, result);
                        return result;
                    }
                } catch (parseError) {
                    if (process.env.DEBUG) {
                        console.log(chalk.yellow(`Warning: JSON extraction failed: ${parseError.message}`));
                    }
                }
            }
            
            // If no valid JSON, return null
            if (process.env.DEBUG) {
                console.log(chalk.yellow(`Warning: Could not parse refinement JSON, keeping current version`));
            }
            return { prompt: null, edits: null };
        }
    } catch (error) {
        if (process.env.DEBUG) {
            console.log(chalk.red(`❌ Error during prompt refinement: ${error.message}`));
        }
        return { prompt: null, edits: null };
    }
}

function formatMarkdownOutput(pe2Prompt, history, metrics, difficulty, complexityScore) {
    const indicator = DIFFICULTY_INDICATORS[difficulty];
    
    const markdown = `
# PE²-Optimized Prompt ${indicator}

**Difficulty Level:** ${difficulty} | **Complexity Score:** ${complexityScore}/20 | **Generated:** ${new Date().toLocaleString()}

## Agentic Analysis
- **Domain Focus:** ${metrics.domain_focus || 'general'}
- **Adaptive Features:** ${metrics.adaptive_features || 'standard'}
- **Overall Quality:** ${metrics.overall_quality || 'N/A'}

### Quality Scores
${metrics.quality_scores ? Object.entries(metrics.quality_scores)
    .map(([key, value]) => `- **${key.charAt(0).toUpperCase() + key.slice(1)}:** ${value.toFixed(1)}/10`)
    .join('\n') : '- Not available'}

## Context
${pe2Prompt.context || 'N/A'}

## Role
${pe2Prompt.role || 'N/A'}

## Task
${pe2Prompt.task || 'N/A'}

## Constraints
${pe2Prompt.constraints || 'N/A'}

## Output
${pe2Prompt.output || 'N/A'}

---

# Refinement History
${history.map(item => `### Iteration ${item.iteration}\n- ${item.edits}\n`).join('\n')}

---

# Performance Metrics
- **Estimated Accuracy Gain**: ${metrics.accuracy_gain || 'N/A'}
- **Complexity Analysis**: ${difficulty} level prompt with ${complexityScore}/20 complexity score
- **Optimization Level**: ${history.length} iterations applied
- **Generated by**: KleoSr PE²-CLI v3.1 (Agentic Edition)

---

*Generated with ❤️ by KleoSr PE²-CLI - Adaptive Intelligence for Prompt Engineering*
`;
    return markdown;
}

// Improved API key display function
function formatApiKeyDisplay(apiKey, showFullKey = false) {
    if (!apiKey) return 'Not set';
    if (showFullKey) return apiKey;
    
    const keyLength = apiKey.length;
    if (keyLength <= 12) {
        return apiKey.substring(0, 4) + '•'.repeat(Math.max(4, keyLength - 8)) + apiKey.substring(keyLength - 4);
    }
    return apiKey.substring(0, 8) + '•'.repeat(8) + apiKey.substring(keyLength - 4);
}

// Improved content preview function
function formatContentPreview(content, maxLength = 200, showFullLength = false) {
    if (!content) return '';
    
    if (content.length <= maxLength || showFullLength) {
        return content;
    }
    
    const truncated = content.substring(0, maxLength);
    const lastSpace = truncated.lastIndexOf(' ');
    const cleanTruncated = lastSpace > maxLength * 0.7 ? truncated.substring(0, lastSpace) : truncated;
    
    return `${cleanTruncated}... [${content.length - cleanTruncated.length} more characters]`;
}

// Improved prompt processing display
function formatProcessingPromptDisplay(prompt, maxLength = 100) {
    if (!prompt) return '';
    
    if (prompt.length <= maxLength) {
        return prompt;
    }
    
    const truncated = prompt.substring(0, maxLength);
    const lastSpace = truncated.lastIndexOf(' ');
    const cleanTruncated = lastSpace > maxLength * 0.7 ? truncated.substring(0, lastSpace) : truncated;
    
    return `${cleanTruncated}... [${prompt.length} chars total]`;
}

async function handleCommand(command, rl, config) {
    switch (command) {
        case '/settings':
            setTerminalTitle('KleoSr PE2-CLI - Settings Configuration');
            config = await promptForConfig(rl);
            setTerminalTitle('KleoSr PE2-CLI - Interactive Mode');
            return config;
            
        case '/config':
            setTerminalTitle('KleoSr PE2-CLI - Current Configuration');
            console.log('\n' + themeManager.color('info')('Current Configuration:'));
            
            const configTerminalWidth = process.stdout.columns || 80;
            const useMinimalConfig = configTerminalWidth < 70;
            
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
            
            // Add contextual tips
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
            // Clear any potential lingering output
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
                // Show model selection
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
                console.log(themeManager.color('info')(`📄 Content preview: ${formatContentPreview(content, 300)}`));
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
            
            const prefsTerminalWidth = process.stdout.columns || 80;
            const useMinimalPrefs = prefsTerminalWidth < 70;
            
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
            
            // Refresh UI to show changes
            clearConsole();
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
                
                // Show preview of prompts
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
            console.log('\n' + themeManager.color('info')('Available Commands:'));
            
            const helpTerminalWidth = process.stdout.columns || 80;
            const useMinimalHelp = helpTerminalWidth < 70;
            
            const helpTable = createTable(
                ['Command', 'Description'],
                Object.entries(COMMANDS).map(([cmd, desc]) => [
                    themeManager.color('primary')(cmd), 
                    useMinimalHelp && desc.length > 30 ? desc.substring(0, 27) + '...' : desc
                ]),
                { minimal: useMinimalHelp, compact: true }
            );
            console.log(helpTable);
            
            console.log(themeManager.color('muted')('\n💡 Quick tips:'));
            console.log(themeManager.color('muted')('  • Type any prompt to start optimization'));
            console.log(themeManager.color('muted')('  • Use /exit or /quit to leave'));
            console.log(themeManager.color('muted')('  • Results are auto-saved to pe2-prompts/'));
            break;
    }
    
    return config;
}

async function interactiveMode(initialInput = null, cliOptions = {}) {
    // Fast path: if input provided with --auto-difficulty, compute and exit without requiring config
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
        console.log(themeManager.color('info')(`📝 Input: ${formatProcessingPromptDisplay(rawPrompt, 100)}`));
        displayComplexityAnalysis(difficulty, recIter, compScore, rawPrompt);
        return;
    }

    displayInteractiveBanner({ themeManager, userPreferences, config: loadConfig() });
    
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
        prompt: chalk.hex('#4A90E2')('> ')
    });

    // Load or create configuration
    let config = { ...getDefaultConfig(), ...loadConfig() };
    // Apply CLI overrides for provider/model
    if (cliOptions.provider) {
        config.provider = cliOptions.provider;
    }
    if (cliOptions.model) {
        config.model = cliOptions.model;
    }
    // Attempt to resolve API key from env if not set
    let resolvedApiKey = resolveApiKey(config.provider, config.apiKey);
    if (!resolvedApiKey && config.provider === 'ollama') {
        resolvedApiKey = PROVIDERS.ollama.baseURL;
    }
    if (resolvedApiKey) {
        config.apiKey = resolvedApiKey;
    }
    
    if (!config.apiKey) {
        // If one-shot and not interactive, do not prompt; fail fast
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

    // Initialize client with error handling
    let client;
    try {
        const apiKeyInitial = resolveApiKey(config.provider, config.apiKey);
        client = config.provider ? getProviderClient(config.provider, apiKeyInitial) : getOpenRouterClient(apiKeyInitial);
    } catch (error) {
        console.log(themeManager.color('error')(`\n✗ Failed to initialize API client: ${error.message}`));
        console.log(themeManager.color('muted')('Please check your configuration with /settings'));
        rl.close();
        return;
    }
    
    let sessionCounter = 1;

    // If a prompt was supplied on the command line, handle it immediately.
    if (initialInput) {
        let rawPrompt = initialInput;
        let inputSource = 'direct text';

        // Detect file vs direct text (reuse earlier logic)
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
            displayComplexityAnalysis(difficulty, recIter, compScore, rawPrompt);

            // One-shot behavior when not explicitly interactive
            if (cliOptions.autoDifficulty) {
                rl.close();
                return;
            }

            if (!cliOptions.interactive) {
                await processPrompt(rawPrompt, client, { ...config, _cliOptions: cliOptions }, sessionCounter++);
                rl.close();
                return;
            }

            // Fall back to interactive if flag is set
            await processPrompt(rawPrompt, client, { ...config, _cliOptions: cliOptions }, sessionCounter++);
        }
    }

    rl.prompt();

    rl.on('line', async (input) => {
        const trimmedInput = input.trim();
        
        // Handle empty input
        if (trimmedInput === '') {
            rl.prompt();
            return;
        }
        
        // Handle commands
        if (trimmedInput.startsWith('/')) {
            const command = trimmedInput.toLowerCase().split(' ')[0];
            
            // Validate command and provide suggestions
            const validation = validateAndSuggestCommand(command);
            
            if (!validation.valid && validation.isCommand) {
                console.log(themeManager.color('error')(`✗ ${validation.message}`));
                rl.prompt();
                return;
            }
            
            // Track command usage for personalized suggestions
            if (validation.valid) {
                userPreferences.trackCommand(command);
            }
            
            // Special handling for exit
            if (command === '/exit' || command === '/quit') {
                console.log(themeManager.color('success')('\n✨ Thanks for using KleoSr PE2-CLI! Goodbye!'));
                rl.close();
                return;
            }
            
            // Clear any potential progress bar remnants
            process.stdout.write('\r\x1b[K');
            
            // Handle other commands
            const result = await handleCommand(command, rl, config);
            
            if (result && typeof result === 'object') {
                // Configuration was updated
                config = { ...config, ...result };
                if (result.provider || result.apiKey) {
                    const newApiKey = resolveApiKey(config.provider, config.apiKey);
                    const newClient = config.provider ? getProviderClient(config.provider, newApiKey) : getOpenRouterClient(newApiKey);
                    client = newClient;
                }

                // Refresh UI banner with updated status line
                displayInteractiveBanner({ themeManager, userPreferences, config: loadConfig() });
            }
            
            // Handle special returns (imported content or batch)
            if (typeof result === 'string') {
                await processPrompt(result, client, config, sessionCounter++);
            } else if (result && result.batch) {
                for (const prompt of result.batch) {
                    console.log(`\n${themeManager.color('info')('Processing prompt:')} ${formatProcessingPromptDisplay(prompt, 80)}`);
                    await processPrompt(prompt, client, config, sessionCounter++);
                }
            }
            
            rl.prompt();
            return;
        }
        
        // Old command handling for backward compatibility
        if (trimmedInput === 'exit' || trimmedInput === 'quit') {
            console.log(themeManager.color('success')('\n✨ Thanks for using KleoSr PE2-CLI! Goodbye!'));
            rl.close();
            return;
        }
        
        if (trimmedInput === 'help') {
            await handleCommand('/help', rl, config);
            rl.prompt();
            return;
        }
        
        if (trimmedInput === 'config') {
            await handleCommand('/settings', rl, config);
            rl.prompt();
            return;
        }
        
        if (trimmedInput === 'status') {
            await handleCommand('/config', rl, config);
            rl.prompt();
            return;
        }
        
        // Process as prompt
        await processPrompt(trimmedInput, client, config, sessionCounter++);
            rl.prompt();
    });

    rl.on('close', () => {
        console.log(chalk.hex('#50E3C2')('\n✨ Session ended. Have a great day!'));
        process.exit(0);
    });
}

async function processPrompt(prompt, client, config, sessionId) {
    let progressBar = null;
    isProcessingPrompt = true;
    
    try {
        setTerminalTitle(`KleoSr PE2-CLI - Processing Session ${sessionId}`);
        
        // Validate prompt
        const validationError = validatePrompt(prompt);
        if (validationError) {
            console.log(themeManager.color('error')(`✗ ${validationError}`));
            setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
            return;
        }

        // Optimized context and strategy retrieval
        const context = getContext();
        const strategy = selectStrategy();
        
        console.log(themeManager.color('info')(`\n⚡ Processing Session ${sessionId} (${prompt.length} chars)...`));
        
        // Analyze prompt complexity with context awareness
        const { difficulty, iterations: baseIterations, score: complexityScore } = analyzePromptComplexity(prompt);
        
        // Determine iterations: CLI override > analysis > strategy fallback
        const cliIterations = config._cliOptions?.iterations;
        let recommendedIterations = Number.isInteger(cliIterations) && cliIterations > 0 ? cliIterations : baseIterations;
        if (!recommendedIterations) recommendedIterations = strategy.iterations || 2;
        
        console.log(themeManager.color('info')('📊 Adaptive Analysis:'));
        console.log(`   Domain: ${context.domain}`);
        console.log(`   Difficulty: ${DIFFICULTY_INDICATORS[difficulty]} ${difficulty}`);
        console.log(`   Score: ${complexityScore}/20`);
        console.log(`   Iterations: ${recommendedIterations} (adapted for ${context.domain})`);
        if (strategy.adaptiveFeatures.length > 0) {
            console.log(`   Features: ${strategy.adaptiveFeatures.join(', ')}`);
        }
        console.log();
        
        // Create progress bar only for actual processing
        progressBar = createProgressBar();
        progressBar.start(100, 0, { task: 'Initializing agentic processing...' });
            
            const refinementHistory = [];
            
        // Initial Prompt Generation with context
        if (progressBar) {
            progressBar.update(30, { task: `Generating ${context.domain}-optimized PE² prompt...` });
        }
        const { prompt: currentPrompt, edits: initialEdits } = await generateInitialPrompt(client, prompt, config.model);
        
            if (!currentPrompt) {
            if (progressBar) {
                progressBar.stop();
                progressBar = null;
            }
            console.log(themeManager.color('error')("✗ Failed to generate initial prompt."));
                return;
            }

            refinementHistory.push({ iteration: 1, edits: initialEdits });
            let workingPrompt = currentPrompt;
        if (progressBar) {
            progressBar.update(50, { task: 'Initial prompt generated' });
        }

        // Adaptive Iterative Refinement
        const progressPerIteration = 40 / recommendedIterations;
            for (let i = 0; i < recommendedIterations; i++) {
                const iterationNum = i + 2;
            if (progressBar) {
                progressBar.update(50 + (i * progressPerIteration), { 
                    task: `Adaptive refinement (${iterationNum}/${recommendedIterations + 1}) for ${context.domain}...` 
                });
            }
            
                const currentPromptJson = JSON.stringify(workingPrompt, null, 2);
                const { prompt: refinedPrompt, edits } = await refinePrompt(
                    client, currentPromptJson, refinementHistory, config.model, iterationNum
                );

                if (!refinedPrompt) {
                console.log(themeManager.color('warning')(`\nRefinement ${iterationNum} failed, using previous version.`));
                    break;
                }

                workingPrompt = refinedPrompt;
                refinementHistory.push({ iteration: iterationNum, edits });
            }

        if (progressBar) {
            progressBar.update(90, { task: 'Finalizing agentic output...' });
        }

        // Optimized evaluation
        const evaluation = await evaluatePrompt();
        
        // Streamlined performance metrics
        const performanceMetrics = {
            accuracy_gain: `Estimated ${20 + (complexityScore * 3)}% improvement`,
            optimization_level: strategy.focus,
            quality_score: evaluation.overallScore.toFixed(1),
            iterations_applied: refinementHistory.length
        };

        // Add to conversation memory
        // conversationMemory.addEntry(prompt, workingPrompt, { // Removed as per stub removal
        //     complexityScore,
        //     difficulty,
        //     domain: context.domain,
        //     iterations: recommendedIterations
        // });

            // Generate output filename in pe2-prompts folder
        if (!config._cliOptions?.outputFile) {
        if (!fs.existsSync(PROMPTS_DIR)) {
            fs.mkdirSync(PROMPTS_DIR, { recursive: true });
        }
        }
        // Determine output file
        let outputFile;
        const fromCli = config._cliOptions && config._cliOptions.outputFile;
        if (fromCli) {
            outputFile = path.isAbsolute(config._cliOptions.outputFile)
                ? config._cliOptions.outputFile
                : path.join(process.cwd(), config._cliOptions.outputFile);
        } else {
        const outputFileName = `pe2-session-${sessionId}.md`;
            outputFile = path.join(PROMPTS_DIR, outputFileName);
        }
            
        // Generate markdown output
        const finalOutput = formatMarkdownOutput(
                workingPrompt, refinementHistory, performanceMetrics, difficulty, complexityScore
            );

        // Save to file
        fs.writeFileSync(outputFile, finalOutput, 'utf-8');
        
        // Save to session
        // sessionManager.addPrompt(prompt, workingPrompt, complexityScore); // Removed as per stub removal
        
        // Track statistics
        const usageTokens = 0; // Could be aggregated from provider responses if exposed
        statsTracker.track(config.model, complexityScore, usageTokens);
        
        // Store last result for copy command
        lastResult = typeof finalOutput === 'string' ? finalOutput : JSON.stringify(workingPrompt, null, 2);
        
        // Complete and stop progress bar
        if (progressBar) {
            progressBar.update(100, { task: 'Complete!' });
            progressBar.stop();
            progressBar = null;
        }
        
        console.log(themeManager.color('success')(`\n✓ PE²-optimized prompt saved to ${outputFile}`));
        console.log(themeManager.color('info')(`📊 ${context.domain} domain | ${difficulty} complexity | ${refinementHistory.length} iterations | ${complexityScore}/20 score`));
        console.log(themeManager.color('info')(`🎯 Strategy: ${strategy.focus} optimization\n`));
        
        // Show suggestion to copy
        console.log(themeManager.color('muted')('Tip: Use /copy to copy the result to clipboard'));
        
        // Reset title
        setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
        isProcessingPrompt = false;
        
    } catch (error) {
        // Ensure progress bar is stopped on error
        if (progressBar) {
            progressBar.stop();
            progressBar = null;
        }
        console.log(themeManager.color('error')(`✗ Error: ${error.message}\n`));
        setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
        isProcessingPrompt = false;
    }
}

async function main() {
    setTerminalTitle('KleoSr PE2-CLI');
    const program = new Command();
    program
        .name('pe2-cli')
        .description('🚀 KleoSr PE2-CLI: Convert raw prompts to PE2-optimized prompts using adaptive intelligence.')
        .version('3.3.7')
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

    program.parse();
    const options = program.opts();
    const input = program.args[0];

    // Check if we should configure
    if (options.config) {
        setTerminalTitle('KleoSr PE2-CLI - Configuration');
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });
        
        displayBanner();
        console.log(chalk.hex('#50E3C2')(`🔧 Configuration Mode | v3.3.7 | ${new Date().toLocaleString()}`));
        console.log(chalk.hex('#4A90E2')('='.repeat(78)));
        
        await promptForConfig(rl);
        rl.close();
        return;
    }

    // Always interactive: banner is rendered inside interactiveMode
    await interactiveMode(input, options);
    return;
}

// Map provider -> env var for automatic key resolution
const PROVIDER_ENV_VARS = {
    openai: 'OPENAI_API_KEY',
    anthropic: 'ANTHROPIC_API_KEY',
    google: 'GOOGLE_API_KEY',
    openrouter: 'OPENROUTER_API_KEY',
    ollama: 'OLLAMA_BASE_URL'
};

function resolveApiKey(provider, configApiKey) {
    if (configApiKey && configApiKey.trim()) return configApiKey;
    if (provider === 'ollama') {
        return process.env[PROVIDER_ENV_VARS.ollama] || null;
    }
    const envVar = PROVIDER_ENV_VARS[provider] || 'OPENROUTER_API_KEY';
    return process.env[envVar] || null;
}

main().catch(error => {
    console.error(chalk.red(`❌ Unexpected error: ${error.message}`));
    process.exit(1);
});