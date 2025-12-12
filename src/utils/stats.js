import fs from 'fs';
import path from 'path';
import os from 'os';

export class StatsTracker {
    constructor() {
        this.statsDir = path.join(os.homedir(), '.kleosr-pe2');
        this.statsFile = path.join(this.statsDir, 'stats.json');
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
        if (!fs.existsSync(this.statsDir)) {
            fs.mkdirSync(this.statsDir, { recursive: true });
        }
        fs.writeFileSync(this.statsFile, JSON.stringify(this.stats, null, 2));
    }

    track(model, complexity, tokens = 0) {
        this.stats.totalPrompts++;
        this.stats.totalTokens += tokens;
        this.stats.averageComplexity = 
            (this.stats.averageComplexity * (this.stats.totalPrompts - 1) + complexity) / 
            this.stats.totalPrompts;
        
        this.stats.modelUsage[model] = (this.stats.modelUsage[model] ?? 0) + 1;
        
        const today = new Date().toISOString().split('T')[0];
        this.stats.dailyUsage[today] = (this.stats.dailyUsage[today] ?? 0) + 1;
        
        this.save();
    }
}
