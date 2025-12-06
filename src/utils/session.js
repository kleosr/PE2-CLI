import fs from 'fs';
import path from 'path';
import os from 'os';

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
        return fs.readdirSync(this.sessionDir)
            .filter(f => f.endsWith('.json'))
            .map(f => JSON.parse(fs.readFileSync(path.join(this.sessionDir, f), 'utf-8')))
            .sort((a, b) => b.timestamp - a.timestamp);
    }
}
