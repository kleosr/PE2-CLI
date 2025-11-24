import chalk from 'chalk';
import { Chalk } from 'chalk';

export class ThemeManager {
    constructor() {
        const colorLevel = this.detectColorLevel();
        this.chalkInstance = new Chalk({ level: colorLevel });
        this.chalkStderr = new Chalk({ level: colorLevel, stream: process.stderr });
        
        this.themes = {
            dark: {
                primary: '#6BB6FF',
                secondary: '#FFE066',
                success: '#5AE6C5',
                error: '#FF7B7B',
                warning: '#FFE066',
                info: '#C5A9EA',
                text: '#FFFFFF',
                muted: '#A0A0A0',
                border: '#4A4A4A',
                background: '#1A1A1A'
            },
            light: {
                primary: '#0052CC',
                secondary: '#B8860B',
                success: '#00A846',
                error: '#C62828',
                warning: '#B8860B',
                info: '#6A4C93',
                text: '#000000',
                muted: '#505050',
                border: '#D0D0D0',
                background: '#FFFFFF'
            }
        };
        this.currentTheme = 'dark';
        this.colorCache = new Map();
    }

    detectColorLevel() {
        if (process.env.FORCE_COLOR) {
            const level = parseInt(process.env.FORCE_COLOR, 10);
            return isNaN(level) ? 1 : Math.min(3, Math.max(0, level));
        }
        return chalk.level;
    }

    setTheme(theme) {
        if (this.themes[theme]) {
            this.currentTheme = theme;
            this.colorCache.clear();
            return true;
        }
        return false;
    }

    get colors() {
        return this.themes[this.currentTheme];
    }

    color(type) {
        const cacheKey = `${this.currentTheme}-${type}`;
        if (this.colorCache.has(cacheKey)) {
            return this.colorCache.get(cacheKey);
        }
        const colorFn = this.chalkInstance.hex(this.colors[type]);
        this.colorCache.set(cacheKey, colorFn);
        return colorFn;
    }

    error(str) {
        return this.chalkStderr.hex(this.colors.error)(str);
    }
}

