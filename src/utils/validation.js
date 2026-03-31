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
        return ['/help', '/settings', '/config', '/model', '/clear'].slice(0, SESSION_CONFIG.priorityCommandsLimit);
    }

    return commands
        .filter(cmd => cmd.toLowerCase().slice(1).startsWith(partial))
        .slice(0, limit);
}

function buildUnknownCommandMessage(command, suggestions) {
    if (suggestions.length > 0) {
        return `Unknown command "${command}". Did you mean: ${suggestions.join(', ')}?`;
    }
    return `Unknown command "${command}". Type /help to see all commands.`;
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
        message: buildUnknownCommandMessage(command, suggestions)
    };
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
