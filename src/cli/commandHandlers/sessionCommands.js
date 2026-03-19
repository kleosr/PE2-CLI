import fs from 'fs';
import path from 'path';
import inquirer from 'inquirer';
import { formatContentPreview, formatProcessingPromptDisplay } from '../../ui.js';
import { copyToClipboard } from '../../utils/display.js';
import { PE2_LOCAL_PROMPTS_DIR } from '../../paths.js';

export async function handleHistory(ctx) {
    const { themeManager, sessionManager } = ctx;
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
}

export async function handleExport(ctx) {
    const { themeManager, sessionManager } = ctx;
    const exportPath = path.join(process.cwd(), `pe2-export-${Date.now()}.json`);
    fs.writeFileSync(exportPath, JSON.stringify(sessionManager.currentSession, null, 2));
    console.log(themeManager.color('success')(`✓ Session exported to: ${exportPath}`));
}

export async function handleImport(ctx) {
    const { themeManager } = ctx;
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
        const previewLength = 300;
        console.log(themeManager.color('info')(`📄 Content preview: ${formatContentPreview(content, { maxLength: previewLength })}`));
        return content;
    } catch (error) {
        console.log(themeManager.color('error')(`✗ Import failed: ${error.message}`));
    }
}

export async function handleBatch(ctx) {
    const { themeManager } = ctx;
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
}

export async function handleCopy(ctx) {
    const { themeManager, lastResult } = ctx;
    if (lastResult) {
        await copyToClipboard(lastResult);
    } else {
        console.log(themeManager.color('warning')('No result to copy.'));
    }
}

export async function handleClearall(ctx) {
    const { themeManager } = ctx;
    if (fs.existsSync(PE2_LOCAL_PROMPTS_DIR)) {
        fs.readdirSync(PE2_LOCAL_PROMPTS_DIR).forEach(f => {
            fs.unlinkSync(path.join(PE2_LOCAL_PROMPTS_DIR, f));
        });
        console.log(themeManager.color('success')('✓ All saved prompts cleared.'));
    } else {
        console.log(themeManager.color('warning')('No prompts folder to clear.'));
    }
}
