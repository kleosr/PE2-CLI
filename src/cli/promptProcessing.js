import fs from 'fs';
import { displayAdaptiveAnalysis, setTerminalTitle } from '../ui.js';
import { processPrompt as runEngine } from '../engine.js';
import { analyzePromptComplexity } from '../analysis.js';
import { validatePrompt } from '../utils/validation.js';
import { handleError, ValidationError } from '../errorHandler.js';
import { DEFAULT_CONTEXT, DEFAULT_STRATEGY, DIFFICULTY_INDICATORS, PERFORMANCE_METRICS } from '../constants.js';

function displayProcessingHeader(themeManager, sessionId, promptLength) {
    console.log();
    console.log(themeManager.color('info')('╔' + '═'.repeat(58) + '╗'));
    const headerText = `Processing Session ${sessionId} (${promptLength} chars)`;
    const padding = Math.max(0, 58 - headerText.length);
    console.log(themeManager.color('info')(headerText + ' '.repeat(padding)));
    console.log(themeManager.color('info')('╚' + '═'.repeat(58) + '╝'));
    console.log();
}

function resolveIterationCount(cliOptions, baseIterations, strategy) {
    const cliIterations = cliOptions?.iterations;
    if (Number.isInteger(cliIterations) && cliIterations > 0) return cliIterations;
    return baseIterations || strategy.iterations || 2;
}

function displayResultsSummary(displayOptions) {
    const { themeManager, domain, difficulty, complexityScore, refinementCount, strategy, outputFile } = displayOptions;
    console.log(themeManager.color('info')('  Results Summary:'));
    console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
    console.log(`     Domain: ${themeManager.color('primary')(domain)}`);
    const indicator = DIFFICULTY_INDICATORS[difficulty];
    console.log(`     Complexity: ${indicator} ${themeManager.color('info')(difficulty)}`);
    console.log(`     Iterations: ${themeManager.color('primary')(refinementCount)}`);
    console.log(`     Score: ${themeManager.color('success')(`${complexityScore}/${PERFORMANCE_METRICS.complexityScoreMax}`)}`);
    console.log(`     Strategy: ${themeManager.color('primary')(strategy.focus)} optimization`);
    console.log(`     Output: ${themeManager.color('muted')(outputFile)}`);
    console.log(themeManager.color('muted')('  ──────────────────────────────────────────────'));
    console.log();
    console.log(themeManager.color('muted')('  Tip: Use /copy to copy the result to clipboard'));
    console.log();
}

async function executePipeline(pipelineOptions) {
    const { prompt, client, config, domainSettings, sessionId, themeManager, statsTracker } = pipelineOptions;
    const { difficulty, iterations: baseIterations, score: complexityScore } = analyzePromptComplexity(prompt);
    const iterations = resolveIterationCount(config._cliOptions, baseIterations, domainSettings.strategy);

    displayAdaptiveAnalysis({
        themeManager,
        context: domainSettings.context,
        strategy: domainSettings.strategy,
        difficulty,
        complexityScore,
        recommendedIterations: iterations,
        maxScore: PERFORMANCE_METRICS.complexityScoreMax
    });

    const result = await runEngine({
        prompt, client, config,
        context: domainSettings.context,
        strategy: domainSettings.strategy,
        difficulty, complexityScore,
        iterations, sessionId,
        themeManager, statsTracker
    });
    return { result, difficulty, complexityScore };
}

function displaySuccessBanner(themeManager) {
    console.log();
    console.log(themeManager.color('success')('╔' + '═'.repeat(58) + '╗'));
    console.log(themeManager.color('success')('║  PE²-optimized prompt saved successfully' + ' '.repeat(16) + '║'));
    console.log(themeManager.color('success')('╚' + '═'.repeat(58) + '╝'));
    console.log();
}

export async function runPromptOptimizationPipeline(options) {
    const { prompt, client, config, sessionId, themeManager, statsTracker, previousOptimizedMarkdown = null } = options;
    let lastResult = previousOptimizedMarkdown;
    try {
        setTerminalTitle(`KleoSr PE2-CLI - Processing Session ${sessionId}`);

        const validationError = validatePrompt(prompt);
        if (validationError) throw new ValidationError(validationError);

        const domainSettings = {
            context: { ...DEFAULT_CONTEXT },
            strategy: { ...DEFAULT_STRATEGY }
        };

        displayProcessingHeader(themeManager, sessionId, prompt.length);

        const { result, difficulty, complexityScore } = await executePipeline({
            prompt, client, config, domainSettings, sessionId, themeManager, statsTracker
        });

        if (!result.success) return { lastResult };

        lastResult = fs.readFileSync(result.outputFile, 'utf-8');

        displaySuccessBanner(themeManager);

        const refinementCount = result.refinementHistory?.length ?? 0;
        displayResultsSummary({
            themeManager,
            domain: domainSettings.context.domain,
            difficulty,
            complexityScore,
            refinementCount,
            strategy: domainSettings.strategy,
            outputFile: result.outputFile
        });

        setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
    } catch (error) {
        const exitCode = handleError(error, themeManager);
        setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
        if (exitCode !== 0 && !(error instanceof ValidationError)) throw error;
    }
    return { lastResult };
}
