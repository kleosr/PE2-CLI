import fs from 'fs';
import { displayAdaptiveAnalysis, setTerminalTitle } from '../ui.js';
import { processPrompt as runEngine } from '../engine.js';
import { analyzePromptComplexity } from '../analysis.js';
import { validatePrompt } from '../utils/validation.js';
import { handleError, ValidationError } from '../errorHandler.js';
import { DEFAULT_CONTEXT, DEFAULT_STRATEGY, DIFFICULTY_INDICATORS, PERFORMANCE_METRICS } from '../constants.js';

export async function runPromptOptimizationPipeline({
    prompt,
    client,
    config,
    sessionId,
    themeManager,
    statsTracker,
    previousOptimizedMarkdown = null
}) {
    let lastResult = previousOptimizedMarkdown;
    try {
        setTerminalTitle(`KleoSr PE2-CLI - Processing Session ${sessionId}`);

        const validationError = validatePrompt(prompt);
        if (validationError) {
            throw new ValidationError(validationError);
        }

        const context = { ...DEFAULT_CONTEXT };
        const strategy = { ...DEFAULT_STRATEGY };

        console.log();
        console.log(themeManager.color('info')('╔' + '═'.repeat(58) + '╗'));
        console.log(themeManager.color('info')(`║  ⚡ Processing Session ${sessionId} (${prompt.length} chars)` + ' '.repeat(Math.max(0, 58 - 40 - sessionId.toString().length)) + '║'));
        console.log(themeManager.color('info')('╚' + '═'.repeat(58) + '╝'));
        console.log();

        const { difficulty, iterations: baseIterations, score: complexityScore } = analyzePromptComplexity(prompt);

        const cliIterations = config._cliOptions?.iterations;
        const iterations = Number.isInteger(cliIterations) && cliIterations > 0
            ? cliIterations
            : (baseIterations || strategy.iterations || 2);

        displayAdaptiveAnalysis({
            themeManager,
            context,
            strategy,
            difficulty,
            complexityScore,
            recommendedIterations: iterations,
            maxScore: PERFORMANCE_METRICS.complexityScoreMax
        });

        const result = await runEngine({
            prompt,
            client,
            config,
            context,
            strategy,
            difficulty,
            complexityScore,
            iterations,
            sessionId,
            themeManager,
            statsTracker
        });

        if (!result.success) {
            return { lastResult };
        }

        const optimizedMarkdown = fs.readFileSync(result.outputFile, 'utf-8');
        lastResult = optimizedMarkdown;

        console.log();
        console.log(themeManager.color('success')('╔' + '═'.repeat(58) + '╗'));
        console.log(themeManager.color('success')('║  ✓ PE²-optimized prompt saved successfully' + ' '.repeat(18) + '║'));
        console.log(themeManager.color('success')('╚' + '═'.repeat(58) + '╝'));
        console.log();

        const refinementCount = result.refinementHistory?.length ?? 0;

        console.log(themeManager.color('info')('  📊 Results Summary:'));
        console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
        console.log(`     ${themeManager.color('text')('Domain')}: ${themeManager.color('primary')(context.domain)}`);
        console.log(`     ${themeManager.color('text')('Complexity')}: ${DIFFICULTY_INDICATORS[difficulty]} ${themeManager.color('info')(difficulty)}`);
        console.log(`     ${themeManager.color('text')('Iterations')}: ${themeManager.color('primary')(refinementCount)}`);
        console.log(`     ${themeManager.color('text')('Score')}: ${themeManager.color('success')(`${complexityScore}/${PERFORMANCE_METRICS.complexityScoreMax}`)}`);
        console.log(`     ${themeManager.color('text')('Strategy')}: ${themeManager.color('primary')(strategy.focus)} optimization`);
        console.log(`     ${themeManager.color('text')('Output')}: ${themeManager.color('muted')(result.outputFile)}`);
        console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
        console.log();
        console.log(themeManager.color('muted')('  💡 Tip: Use /copy to copy the result to clipboard'));
        console.log();

        setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
    } catch (error) {
        const exitCode = handleError(error, themeManager);
        setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
        if (exitCode !== 0 && !(error instanceof ValidationError)) {
            throw error;
        }
    }
    return { lastResult };
}
