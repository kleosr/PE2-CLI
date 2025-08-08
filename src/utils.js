import chalk from 'chalk';
import cliProgress from 'cli-progress';
import clipboardy from 'clipboardy';
import Table from 'cli-table3';
import { highlight } from 'cli-highlight';
import fs from 'fs';
import path from 'path';
import os from 'os';

// Session management
export class SessionManager {
    constructor() {
        this.sessionDir = path.join(os.homedir(), '.kleosr-pe2', 'sessions');
        this.currentSession = {
            id: Date.now(),
            prompts: [],
            timestamp: new Date(),
            model: null,
            provider: null,
            totalTokens: 0
        };
        this.ensureSessionDir();
    }

    ensureSessionDir() {
        if (!fs.existsSync(this.sessionDir)) {
            fs.mkdirSync(this.sessionDir, { recursive: true });
        }
    }

    addPrompt(prompt, result, complexity) {
        this.currentSession.prompts.push({
            timestamp: new Date(),
            prompt,
            result,
            complexity
        });
        this.save();
    }

    save() {
        const filename = `session-${this.currentSession.id}.json`;
        const filepath = path.join(this.sessionDir, filename);
        fs.writeFileSync(filepath, JSON.stringify(this.currentSession, null, 2));
    }

    loadHistory() {
        const files = fs.readdirSync(this.sessionDir);
        return files
            .filter(f => f.endsWith('.json'))
            .map(f => {
                const content = fs.readFileSync(path.join(this.sessionDir, f), 'utf-8');
                return JSON.parse(content);
            })
            .sort((a, b) => b.timestamp - a.timestamp);
    }
}

// Enhanced progress bar utility with better consistency
export function createProgressBar(options = {}) {
    const defaultOptions = {
        format: ' {bar} | {percentage}% | {task}',
        barCompleteChar: '█',
        barIncompleteChar: '▒',
        hideCursor: true,
        clearOnComplete: false, // Keep for consistency
        stopOnComplete: false,  // Manual control
        barsize: Math.min(40, Math.max(20, (process.stdout.columns || 80) - 40)),
        forceRedraw: true,
        linewrap: false,
        fps: 8,
        noTTYOutput: false,
        notTTYSchedule: 2000,
        synchronousUpdate: false,
        ...options
    };
    
    const progressBar = new cliProgress.SingleBar(defaultOptions);
    
    // Enhanced methods for better cleanup
    const originalStop = progressBar.stop.bind(progressBar);
    progressBar.stop = function() {
        try {
            originalStop();
            // Ensure cursor is visible and clean line
            process.stdout.write('\r\x1b[K\x1b[?25h');
        } catch (error) {
            // Ignore cleanup errors
        }
    };
    
    return progressBar;
}

// Enhanced status bar with improved styling and reduced redundancy
export function displayStatusBar(config, options = {}) {
    const width = Math.min(process.stdout.columns || 80, 100); // Max width for better readability
    const showBorder = options.showBorder !== false; // Default to true, can be disabled
    const compact = options.compact || false;
    
    const status = config.apiKey ? '✓ Connected' : '✗ Not Connected';
    const provider = config.provider || 'Not Set';
    const model = config.model || 'Default';
    
    // Truncate long model names for better layout
    const displayModel = model.length > 20 ? model.substring(0, 17) + '...' : model;
    
    const statusText = compact 
        ? ` ${provider} | ${displayModel} | ${status} `
        : ` Provider: ${provider} | Model: ${displayModel} | Status: ${status} `;
    
    const maxContentWidth = width - 4; // Account for borders
    const truncatedText = statusText.length > maxContentWidth 
        ? statusText.substring(0, maxContentWidth - 3) + '...' 
        : statusText;
    
    const padding = Math.max(0, width - truncatedText.length - 2);
    
    if (showBorder) {
        const themeManager = new ThemeManager(); // Create instance for border color
        const borderColor = chalk.hex(themeManager.colors.border);
        
        console.log(borderColor('┌' + '─'.repeat(width - 2) + '┐'));
        console.log(borderColor('│') + truncatedText + ' '.repeat(padding) + borderColor('│'));
        console.log(borderColor('└' + '─'.repeat(width - 2) + '┘'));
    } else {
        // Borderless version for cleaner look
        console.log(chalk.hex('#A0A0A0')(truncatedText));
    }
}

// Command suggestions
export const COMMANDS = {
    '/settings': 'Configure API provider, model, and key',
    '/config': 'View current settings',
    '/showkey': 'Show full API key (secure display)',
    '/preferences': 'View and manage user preferences',
    '/prefs': 'View and manage user preferences (short)',
    '/compact': 'Toggle compact display mode',
    '/model': 'Quick model switch',
    '/clear': 'Clear screen',
    '/history': 'View recent prompts',
    '/export': 'Export session history',
    '/import': 'Import prompts from file',
    '/theme': 'Toggle between light/dark themes',
    '/help': 'Show all commands',
    '/batch': 'Process multiple prompts from file',
    '/copy': 'Copy last result to clipboard',
    '/clearall': 'Clear all saved prompts'
};

// Enhanced auto-complete for commands with fuzzy matching
export function getCommandSuggestions(input, limit = 5) {
    if (!input.startsWith('/')) return [];
    
    const partial = input.toLowerCase().slice(1); // Remove the '/' prefix
    const commands = Object.keys(COMMANDS);
    
    if (partial === '') {
        // Return most commonly used commands first
        const priorityCommands = ['/help', '/settings', '/config', '/model', '/clear'];
        return priorityCommands.slice(0, limit);
    }
    
    // Fuzzy matching with scoring
    const matches = commands
        .map(cmd => {
            const cmdName = cmd.toLowerCase().slice(1);
            let score = 0;
            
            // Exact prefix match gets highest score
            if (cmdName.startsWith(partial)) {
                score = 1000 + (10 - partial.length);
            }
            // Contains match gets medium score
            else if (cmdName.includes(partial)) {
                score = 500 + (10 - partial.length);
            }
            // Character sequence match gets lower score
            else {
                let partialIndex = 0;
                for (let i = 0; i < cmdName.length && partialIndex < partial.length; i++) {
                    if (cmdName[i] === partial[partialIndex]) {
                        partialIndex++;
                        score += 10;
                    }
                }
                // Only include if we matched all characters
                if (partialIndex < partial.length) score = 0;
            }
            
            return { cmd, score };
        })
        .filter(item => item.score > 0)
        .sort((a, b) => b.score - a.score)
        .slice(0, limit)
        .map(item => item.cmd);
    
    return matches;
}

// Enhanced command validation with suggestions
export function validateAndSuggestCommand(input) {
    if (!input.startsWith('/')) {
        return { valid: false, isCommand: false };
    }
    
    const command = input.toLowerCase().split(' ')[0];
    
    if (COMMANDS[command]) {
        return { 
            valid: true, 
            isCommand: true, 
            command,
            description: COMMANDS[command]
        };
    }
    
    // Get suggestions for invalid commands
    const suggestions = getCommandSuggestions(command, 3);
    
    return {
        valid: false,
        isCommand: true,
        command,
        suggestions,
        message: suggestions.length > 0 
            ? `Unknown command "${command}". Did you mean: ${suggestions.join(', ')}?`
            : `Unknown command "${command}". Type /help to see all commands.`
    };
}

// Dynamic keyboard shortcuts handler
export function handleKeyboardShortcuts(key, config) {
    const shortcuts = {
        'ctrl+h': '/help',
        'ctrl+c': '/config', 
        'ctrl+s': '/settings',
        'ctrl+m': '/model',
        'ctrl+t': '/theme',
        'ctrl+l': '/clear',
        'ctrl+q': '/quit',
        'f1': '/help',
        'f2': '/settings',
        'f3': '/config',
        'f4': '/model'
    };
    
    return shortcuts[key] || null;
}

// Validate prompt
export function validatePrompt(prompt) {
    if (!prompt || prompt.trim().length === 0) {
        return 'Please enter a prompt';
    }
    if (prompt.length < 10) {
        return 'Prompt too short. Please provide more detail.';
    }
    if (prompt.length > 10000) {
        return 'Prompt too long. Consider breaking it down.';
    }
    return null;
}

// Format output based on user preference
export function formatOutput(data, format = 'markdown') {
    switch (format) {
        case 'json':
            return JSON.stringify(data, null, 2);
        case 'yaml':
            // Simple YAML formatter
            return Object.entries(data)
                .map(([key, value]) => `${key}: ${typeof value === 'object' ? '\n  ' + JSON.stringify(value, null, 2).replace(/\n/g, '\n  ') : value}`)
                .join('\n');
        case 'plain':
            return Object.entries(data)
                .map(([key, value]) => `${key}: ${value}`)
                .join('\n');
        default:
            return data; // Return as-is for markdown
    }
}

// Enhanced formatting with better responsiveness
export function formatResponse(text, options = {}) {
    const maxWidth = Math.min(options.maxWidth || 100, process.stdout.columns || 80);
    const indent = options.indent || 0;
    const prefix = ' '.repeat(indent);
    
    if (text.length <= maxWidth - indent) {
        return prefix + text;
    }
    
    // Smart word wrapping
    const words = text.split(' ');
    const lines = [];
    let currentLine = '';
    
    for (const word of words) {
        if ((currentLine + word).length <= maxWidth - indent) {
            currentLine += (currentLine ? ' ' : '') + word;
        } else {
            if (currentLine) lines.push(prefix + currentLine);
            currentLine = word;
        }
    }
    
    if (currentLine) lines.push(prefix + currentLine);
    return lines.join('\n');
}

// Copy to clipboard with feedback
export async function copyToClipboard(text) {
    try {
        await clipboardy.write(text);
        console.log(chalk.green('✓ Copied to clipboard! Press Ctrl+V to paste.'));
        return true;
    } catch (error) {
        if (error.code === 'ENOENT') {
            console.log(chalk.red('✗ Clipboard utility not found on your system.'));
            console.log(chalk.yellow('  • Linux: install xclip or xsel  |  macOS: pbcopy/pbpaste are built-in  |  Windows: ensure clip.exe is accessible'));
        } else {
            console.log(chalk.red(`✗ Failed to copy to clipboard: ${error.message}`));
        }
        return false;
    }
}

// Enhanced table display with theme support
export function createTable(headers, rows, options = {}) {
    const themeManager = new ThemeManager();
    const minimal = options.minimal || false;
    const compact = options.compact || false;
    
    const tableConfig = {
        head: headers,
        style: {
            head: [themeManager.currentTheme === 'dark' ? 'cyan' : 'blue'],
            border: minimal ? [] : [themeManager.currentTheme === 'dark' ? 'gray' : 'black'],
            compact: compact
        }
    };
    
    // Remove borders for minimal style
    if (minimal) {
        tableConfig.chars = {
            'top': '', 'top-mid': '', 'top-left': '', 'top-right': '',
            'bottom': '', 'bottom-mid': '', 'bottom-left': '', 'bottom-right': '',
            'left': '', 'left-mid': '', 'mid': '', 'mid-mid': '',
            'right': '', 'right-mid': '', 'middle': '│'
        };
    }
    
    const table = new Table(tableConfig);
    rows.forEach(row => table.push(row));
    return table.toString();
}

// Syntax highlighting for code blocks
export function highlightCode(code, language = 'javascript') {
    try {
        return highlight(code, { language });
    } catch (error) {
        return code; // Return unhighlighted if error
    }
}

// Enhanced theme management with better contrast
export class ThemeManager {
    constructor() {
        this.themes = {
            dark: {
                primary: '#6BB6FF',     // Improved contrast
                secondary: '#FFE066',   // Better visibility
                success: '#5AE6C5',     // Enhanced green
                error: '#FF7B7B',       // Softer red
                warning: '#FFE066',     // Consistent with secondary
                info: '#C5A9EA',        // Better purple contrast
                text: '#FFFFFF',        // Pure white for max contrast
                muted: '#A0A0A0',       // Improved muted text visibility
                border: '#4A4A4A',      // Better border contrast
                background: '#1A1A1A'   // Dark background reference
            },
            light: {
                primary: '#0052CC',     // Darker blue for better contrast
                secondary: '#B8860B',   // Improved golden color
                success: '#00A846',     // Darker green
                error: '#C62828',       // Darker red
                warning: '#B8860B',     // Consistent with secondary
                info: '#6A4C93',        // Better purple for light theme
                text: '#000000',        // Pure black for max contrast
                muted: '#505050',       // Better visibility than gray
                border: '#D0D0D0',      // Light border
                background: '#FFFFFF'   // Light background reference
            }
        };
        this.currentTheme = 'dark';
    }

    setTheme(theme) {
        if (this.themes[theme]) {
            this.currentTheme = theme;
            return true;
        }
        return false;
    }

    get colors() {
        return this.themes[this.currentTheme];
    }

    color(type) {
        return chalk.hex(this.colors[type]);
    }
}

// Statistics tracker
export class StatsTracker {
    constructor() {
        this.statsFile = path.join(os.homedir(), '.kleosr-pe2', 'stats.json');
        this.stats = this.load();
    }

    load() {
        if (fs.existsSync(this.statsFile)) {
            return JSON.parse(fs.readFileSync(this.statsFile, 'utf-8'));
        }
        return {
            totalPrompts: 0,
            totalTokens: 0,
            averageComplexity: 0,
            modelUsage: {},
            dailyUsage: {}
        };
    }

    save() {
        fs.writeFileSync(this.statsFile, JSON.stringify(this.stats, null, 2));
    }

    track(model, complexity, tokens = 0) {
        this.stats.totalPrompts++;
        this.stats.totalTokens += tokens;
        this.stats.averageComplexity = 
            (this.stats.averageComplexity * (this.stats.totalPrompts - 1) + complexity) / 
            this.stats.totalPrompts;
        
        // Track model usage
        this.stats.modelUsage[model] = (this.stats.modelUsage[model] || 0) + 1;
        
        // Track daily usage
        const today = new Date().toISOString().split('T')[0];
        this.stats.dailyUsage[today] = (this.stats.dailyUsage[today] || 0) + 1;
        
        this.save();
    }

    getStats() {
        return this.stats;
    }

    displayStats() {
        const table = new Table({
            head: ['Metric', 'Value'],
            colWidths: [30, 30]
        });

        table.push(
            ['Total Prompts', this.stats.totalPrompts],
            ['Total Tokens', this.stats.totalTokens],
            ['Average Complexity', this.stats.averageComplexity.toFixed(2)],
            ['Most Used Model', Object.entries(this.stats.modelUsage)
                .sort(([,a], [,b]) => b - a)[0]?.[0] || 'N/A']
        );

        return table.toString();
    }
}

// User preferences and persistence management
export class UserPreferences {
    constructor() {
        this.prefsDir = path.join(os.homedir(), '.kleosr-pe2');
        this.prefsFile = path.join(this.prefsDir, 'preferences.json');
        this.preferences = this.load();
    }

    load() {
        if (fs.existsSync(this.prefsFile)) {
            try {
                const data = fs.readFileSync(this.prefsFile, 'utf-8');
                return JSON.parse(data);
            } catch (error) {
                console.warn('Could not load preferences, using defaults');
            }
        }
        
        return {
            theme: 'dark',
            compactMode: false,
            showBorders: true,
            autoSave: true,
            maxHistoryItems: 50,
            defaultProvider: 'openrouter',
            lastUsedCommands: [],
            favoriteModels: [],
            uiPreferences: {
                terminalWidth: 'auto',
                progressBarStyle: 'standard',
                colorScheme: 'default'
            }
        };
    }

    save() {
        try {
            if (!fs.existsSync(this.prefsDir)) {
                fs.mkdirSync(this.prefsDir, { recursive: true });
            }
            fs.writeFileSync(this.prefsFile, JSON.stringify(this.preferences, null, 2));
        } catch (error) {
            console.warn('Could not save preferences:', error.message);
        }
    }

    get(key, defaultValue = null) {
        return this.preferences[key] ?? defaultValue;
    }

    set(key, value) {
        this.preferences[key] = value;
        this.save();
    }

    // Track command usage for smart suggestions
    trackCommand(command) {
        const lastUsed = this.preferences.lastUsedCommands || [];
        const filtered = lastUsed.filter(cmd => cmd !== command);
        this.preferences.lastUsedCommands = [command, ...filtered].slice(0, 10);
        this.save();
    }

    // Add model to favorites
    addFavoriteModel(model) {
        const favorites = this.preferences.favoriteModels || [];
        if (!favorites.includes(model)) {
            this.preferences.favoriteModels = [model, ...favorites].slice(0, 5);
            this.save();
        }
    }

    // Get personalized command suggestions
    getPersonalizedSuggestions(limit = 5) {
        const lastUsed = this.preferences.lastUsedCommands || [];
        const allCommands = Object.keys(COMMANDS);
        
        // Prioritize recently used commands
        const suggestions = [...new Set([...lastUsed, ...allCommands])].slice(0, limit);
        return suggestions;
    }

    // UI preference helpers
    shouldUseCompactMode() {
        const terminalWidth = process.stdout.columns || 80;
        return this.preferences.compactMode || terminalWidth < 70;
    }

    shouldShowBorders() {
        return this.preferences.showBorders && !this.shouldUseCompactMode();
    }
}

// Export all utilities
export default {
    SessionManager,
    createProgressBar,
    displayStatusBar,
    COMMANDS,
    getCommandSuggestions,
    validatePrompt,
    formatOutput,
    copyToClipboard,
    createTable,
    highlightCode,
    ThemeManager,
    StatsTracker,
    UserPreferences,
    validateAndSuggestCommand,
    handleKeyboardShortcuts,
    formatResponse
}; 