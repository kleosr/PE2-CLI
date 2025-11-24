import fs from 'fs';
import path from 'path';
import os from 'os';
import { SESSION_CONFIG, DEFAULT_CONFIG, UI_CONFIG } from '../constants.js';
import { COMMANDS } from './validation.js';

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
            maxHistoryItems: SESSION_CONFIG.maxHistoryItems,
            defaultProvider: DEFAULT_CONFIG.provider,
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

    trackCommand(command) {
        const lastUsed = this.preferences.lastUsedCommands || [];
        const filtered = lastUsed.filter(cmd => cmd !== command);
        this.preferences.lastUsedCommands = [command, ...filtered].slice(0, SESSION_CONFIG.maxLastUsedCommands);
        this.save();
    }

    addFavoriteModel(model) {
        const favorites = this.preferences.favoriteModels || [];
        if (!favorites.includes(model)) {
            this.preferences.favoriteModels = [model, ...favorites].slice(0, SESSION_CONFIG.maxFavoriteModels);
            this.save();
        }
    }

    getPersonalizedSuggestions(limit = SESSION_CONFIG.commandSuggestionsLimit) {
        const lastUsed = this.preferences.lastUsedCommands || [];
        const allCommands = Object.keys(COMMANDS);
        const suggestions = [...new Set([...lastUsed, ...allCommands])].slice(0, limit);
        return suggestions;
    }

    shouldUseCompactMode() {
        const terminalWidth = process.stdout.columns || UI_CONFIG.terminalWidth.default;
        return this.preferences.compactMode || terminalWidth < UI_CONFIG.terminalWidth.compactThreshold;
    }

    shouldShowBorders() {
        return this.preferences.showBorders && !this.shouldUseCompactMode();
    }
}

