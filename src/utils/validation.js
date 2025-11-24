import { PROMPT_LIMITS, SESSION_CONFIG } from '../constants.js';

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

export function getCommandSuggestions(input, limit = SESSION_CONFIG.commandSuggestionsLimit) {
    if (!input.startsWith('/')) return [];
    
    const partial = input.toLowerCase().slice(1);
    const commands = Object.keys(COMMANDS);
    
    if (partial === '') {
        const priorityCommands = ['/help', '/settings', '/config', '/model', '/clear'];
        return priorityCommands.slice(0, SESSION_CONFIG.priorityCommandsLimit);
    }
    
    const matches = commands
        .map(cmd => {
            const cmdName = cmd.toLowerCase().slice(1);
            let score = 0;
            
            if (cmdName.startsWith(partial)) {
                score = 1000 + (10 - partial.length);
            }
            else if (cmdName.includes(partial)) {
                score = 500 + (10 - partial.length);
            }
            else {
                let partialIndex = 0;
                for (let i = 0; i < cmdName.length && partialIndex < partial.length; i++) {
                    if (cmdName[i] === partial[partialIndex]) {
                        partialIndex++;
                        score += 10;
                    }
                }
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

export function validatePrompt(prompt) {
    if (!prompt || prompt.trim().length === 0) {
        return 'Please enter a prompt';
    }
    if (prompt.length < PROMPT_LIMITS.minLength) {
        return 'Prompt too short. Please provide more detail.';
    }
    if (prompt.length > PROMPT_LIMITS.maxLength) {
        return 'Prompt too long. Consider breaking it down.';
    }
    return null;
}

