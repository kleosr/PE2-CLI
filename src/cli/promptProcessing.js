import fs from 'fs';
import { setTerminalTitle } from '../ui.js';
import { processPrompt as runEngine } from '../engine.js';
import { analyzePromptComplexity } from '../analysis.js';
import { displayComplexityAnalysis } from '../ui.js';
import { validatePrompt } from '../utils/index.js';
import { handleError, ValidationError } from '../errorHandler.js';
import { PROMPT_LIMITS, DEFAULT_CONTEXT, DEFAULT_STRATEGY, PERFORMANCE_METRICS } from '../constants.js';

export function getContext() {
    return DEFAULT_CONTEXT;
}

export function selectStrategy() {
    return DEFAULT_STRATEGY;
}

export async function processPrompt(prompt, client, config, sessionId, themeManager, statsTracker, userPreferences, lastResult) {
    let progressBar = null;
    let isProcessingPrompt = true;
    
    try {
        setTerminalTitle(`KleoSr PE2-CLI - Processing Session ${sessionId}`);
        
        const validationError = validatePrompt(prompt);
        if (validationError) {
            throw new ValidationError(validationError);
        }

        const context = getContext();
        const strategy = selectStrategy();
        
        console.log();
        console.log(themeManager.color('info')('╔' + '═'.repeat(58) + '╗'));
        console.log(themeManager.color('info')(`║  ⚡ Processing Session ${sessionId} (${prompt.length} chars)` + ' '.repeat(Math.max(0, 58 - 40 - sessionId.toString().length)) + '║'));
        console.log(themeManager.color('info')('╚' + '═'.repeat(58) + '╝'));
        console.log();
        
        const { difficulty, iterations: baseIterations, score: complexityScore } = analyzePromptComplexity(prompt);
        
        const cliIterations = config._cliOptions?.iterations;
        let recommendedIterations = Number.isInteger(cliIterations) && cliIterations > 0 ? cliIterations : baseIterations;
        if (!recommendedIterations) recommendedIterations = strategy.iterations || 2;
        
        const { DIFFICULTY_INDICATORS } = await import('../ui.js');
        const complexityScoreMax = PERFORMANCE_METRICS.complexityScoreMax;
        const scoreBar = '█'.repeat(Math.floor((complexityScore / complexityScoreMax) * 20)) + '░'.repeat(20 - Math.floor((complexityScore / complexityScoreMax) * 20));
        const scoreColor = complexityScore <= 4 ? 'success' : complexityScore <= 8 ? 'warning' : complexityScore <= 12 ? 'secondary' : complexityScore <= 16 ? 'error' : 'error';
        
        console.log(themeManager.color('info')('  📊 Adaptive Analysis:'));
        console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
        console.log(`     ${themeManager.color('text')('Domain')}: ${themeManager.color('primary')(context.domain)}`);
        console.log(`     ${themeManager.color('text')('Difficulty')}: ${DIFFICULTY_INDICATORS[difficulty]} ${themeManager.color('info')(difficulty)}`);
        console.log(`     ${themeManager.color('text')('Score')}: ${themeManager.color(scoreColor)(`${complexityScore}/${complexityScoreMax}`)}`);
        console.log(themeManager.color('muted')(`     ${scoreBar}`));
        console.log(`     ${themeManager.color('text')('Iterations')}: ${themeManager.color('primary')(recommendedIterations)} ${themeManager.color('muted')(`(adapted for ${context.domain})`)}`);
        if (strategy.adaptiveFeatures.length > 0) {
            console.log(`     ${themeManager.color('text')('Features')}: ${themeManager.color('info')(strategy.adaptiveFeatures.join(', '))}`);
        }
        console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
        console.log();
        
        const result = await runEngine({
            prompt,
            client,
            config,
            sessionId,
            themeManager,
            statsTracker,
            userPreferences,
            getContext,
            selectStrategy
        });
        
        if (!result.success) {
            return;
        }
        
        const { outputFile: resultOutputFile } = result;
        const resultContent = fs.readFileSync(resultOutputFile, 'utf-8');
        lastResult = resultContent;
        
        console.log();
        console.log(themeManager.color('success')('╔' + '═'.repeat(58) + '╗'));
        console.log(themeManager.color('success')('║  ✓ PE²-optimized prompt saved successfully' + ' '.repeat(18) + '║'));
        console.log(themeManager.color('success')('╚' + '═'.repeat(58) + '╝'));
        console.log();
        
        const refinementCount = result.refinementHistory?.length || 0;
        const { DIFFICULTY_INDICATORS: DIFF_IND } = await import('../ui.js');
        
        console.log(themeManager.color('info')('  📊 Results Summary:'));
        console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
        console.log(`     ${themeManager.color('text')('Domain')}: ${themeManager.color('primary')(context.domain)}`);
        console.log(`     ${themeManager.color('text')('Complexity')}: ${DIFF_IND[difficulty]} ${themeManager.color('info')(difficulty)}`);
        console.log(`     ${themeManager.color('text')('Iterations')}: ${themeManager.color('primary')(refinementCount)}`);
        console.log(`     ${themeManager.color('text')('Score')}: ${themeManager.color('success')(`${complexityScore}/${complexityScoreMax}`)}`);
        console.log(`     ${themeManager.color('text')('Strategy')}: ${themeManager.color('primary')(strategy.focus)} optimization`);
        console.log(`     ${themeManager.color('text')('Output')}: ${themeManager.color('muted')(resultOutputFile)}`);
        console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
        console.log();
        console.log(themeManager.color('muted')('  💡 Tip: Use /copy to copy the result to clipboard'));
        console.log();
        
        setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
        isProcessingPrompt = false;
        
    } catch (error) {
        if (progressBar) {
            progressBar.stop();
            progressBar = null;
        }
        const exitCode = handleError(error, themeManager);
        setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
        isProcessingPrompt = false;
        if (exitCode !== 0 && !(error instanceof ValidationError)) {
            throw error;
        }
    }
    
    return { lastResult, isProcessingPrompt };
}

