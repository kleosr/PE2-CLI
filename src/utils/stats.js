import fs from 'fs';
import path from 'path';
import os from 'os';
import Table from 'cli-table3';

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
        
        this.stats.modelUsage[model] = (this.stats.modelUsage[model] || 0) + 1;
        
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

