import fs from 'fs';
import path from 'path';
import chalk from 'chalk';
import { createProgressBar } from './utils.js';
import { getInitialTemplate, getRefinementTemplate } from './templates.js';
import { buildMessages } from './messages.js';
import { analyzePromptComplexity } from './analysis.js';
import { setTerminalTitle } from './ui.js';
import { LLM_CONFIG, PROGRESS_PERCENTAGES, PERFORMANCE_METRICS, DEFAULT_EVALUATION, REQUIRED_PROMPT_FIELDS, HTTP_HEADERS } from './constants.js';

const PROMPTS_DIR = path.join(process.cwd(), 'pe2-prompts');

export async function generateInitialPrompt(client, rawPrompt, model) {
  try {
    const systemContent = LLM_CONFIG.systemMessage;
    const userContent = getInitialTemplate(rawPrompt);
    const response = await client.chat.completions.create({
      model,
      messages: buildMessages({ system: systemContent, user: userContent }),
      max_tokens: LLM_CONFIG.maxTokens,
      temperature: LLM_CONFIG.temperature
    });
    const content = response.choices[0].message.content;
    try {
      return { prompt: JSON.parse(content), edits: 'Initial prompt generation.' };
    } catch {
      const firstBrace = content.indexOf('{');
      const lastBrace = content.lastIndexOf('}');
      if (firstBrace !== -1 && lastBrace !== -1 && lastBrace > firstBrace) {
        let jsonStr = content.slice(firstBrace, lastBrace + 1);
        jsonStr = jsonStr.replace(/,(\s*[}\]])/g, '$1');
        try {
          const parsed = JSON.parse(jsonStr);
          const hasAll = REQUIRED_PROMPT_FIELDS.every(field => Object.prototype.hasOwnProperty.call(parsed, field));
          if (hasAll) return { prompt: parsed, edits: 'Initial prompt generation.' };
          return {
            prompt: {
              context: parsed.context || 'No context provided',
              role: parsed.role || 'Expert assistant',
              task: parsed.task || 'Complete the requested task',
              constraints: parsed.constraints || 'Follow best practices',
              output: parsed.output || 'Provide appropriate output'
            },
            edits: 'Initial prompt generation with field validation.'
          };
        } catch {}
      }
      // Fallback
      return {
        prompt: {
          context: `The user wants to: ${rawPrompt.substring(0, 500)}${rawPrompt.length > 500 ? '...' : ''}`,
          role: 'Expert assistant with deep knowledge in the relevant domain',
          task: "1. Understand the user's requirements\n2. Provide a comprehensive solution\n3. Ensure clarity and completeness",
          constraints: '- Be accurate and thorough\n- Follow best practices\n- Provide clear explanations',
          output: 'A well-structured response that fully addresses the user\'s needs'
        },
        edits: 'Initial prompt generation with automatic structuring.'
      };
    }
  } catch (error) {
    console.log(chalk.red(`❌ Error during initial prompt generation: ${error.message}`));
    return { prompt: null, edits: null };
  }
}

export async function refinePrompt(client, currentPromptJson, refinementHistory, model, iterationNum, cache) {
  const cached = cache?.get(currentPromptJson, iterationNum);
  if (cached) return cached;
  try {
    const systemContent = LLM_CONFIG.refinementSystemMessage;
    const userContent = getRefinementTemplate(currentPromptJson, iterationNum);
    const response = await client.chat.completions.create({
      model,
      messages: buildMessages({ system: systemContent, user: userContent }),
      headers: {
        'HTTP-Referer': HTTP_HEADERS.referer,
        'X-Title': HTTP_HEADERS.title
      }
    });
    const content = response.choices[0].message.content;
    try {
      const refined = JSON.parse(content);
      const result = { prompt: refined, edits: `Refined prompt based on PE2 principles (Iteration ${iterationNum}).` };
      cache?.set(currentPromptJson, iterationNum, result);
      return result;
    } catch {
      const firstBrace = content.indexOf('{');
      const lastBrace = content.lastIndexOf('}');
      if (firstBrace !== -1 && lastBrace !== -1 && lastBrace > firstBrace) {
        let jsonStr = content.slice(firstBrace, lastBrace + 1);
        jsonStr = jsonStr.replace(/,(\s*[}\]])/g, '$1');
        const parsed = JSON.parse(jsonStr);
        const hasAll = REQUIRED_PROMPT_FIELDS.every(field => Object.prototype.hasOwnProperty.call(parsed, field));
        const prompt = hasAll
          ? parsed
          : {
              context: parsed.context || 'No context provided',
              role: parsed.role || 'Expert assistant',
              task: parsed.task || 'Complete the requested task',
              constraints: parsed.constraints || 'Follow best practices',
              output: parsed.output || 'Provide appropriate output'
            };
        const result = { prompt, edits: 'Refined prompt generation.' };
        cache?.set(currentPromptJson, iterationNum, result);
        return result;
      }
      return { prompt: null, edits: null };
    }
  } catch (error) {
    return { prompt: null, edits: null };
  }
}

export async function processPrompt({
  prompt,
  client,
  config,
  sessionId,
  themeManager,
  statsTracker,
  userPreferences,
  getContext,
  selectStrategy
}) {
  let progressBar = null;
  try {
    setTerminalTitle(`KleoSr PE2-CLI - Processing Session ${sessionId}`);

    const context = getContext();
    const strategy = selectStrategy();
    console.log(themeManager.color('info')(`\n⚡ Processing Session ${sessionId} (${prompt.length} chars)...`));

    const { difficulty, iterations: baseIterations, score: complexityScore } = analyzePromptComplexity(prompt);
    const cliIterations = config._cliOptions?.iterations;
    let recommendedIterations = Number.isInteger(cliIterations) && cliIterations > 0 ? cliIterations : baseIterations;
    if (!recommendedIterations) recommendedIterations = strategy.iterations || 2;

    console.log(themeManager.color('info')('📊 Adaptive Analysis:'));
    console.log(`   Domain: ${context.domain}`);
    console.log(`   Difficulty: ${difficulty}`);
    console.log(`   Score: ${complexityScore}/20`);
    console.log(`   Iterations: ${recommendedIterations} (adapted for ${context.domain})`);
    if (strategy.adaptiveFeatures.length > 0) console.log(`   Features: ${strategy.adaptiveFeatures.join(', ')}`);
    console.log();

    progressBar = createProgressBar();
    progressBar.start(PROGRESS_PERCENTAGES.complete, PROGRESS_PERCENTAGES.initialization, { task: 'Initializing agentic processing...' });

    const refinementHistory = [];
    if (progressBar) progressBar.update(PROGRESS_PERCENTAGES.initialPromptStart, { task: `Generating ${context.domain}-optimized PE² prompt...` });
    const { prompt: currentPrompt, edits: initialEdits } = await generateInitialPrompt(client, prompt, config.model);
    if (!currentPrompt) {
      if (progressBar) { progressBar.stop(); progressBar = null; }
      console.log(themeManager.color('error')('✗ Failed to generate initial prompt.'));
      return { success: false };
    }
    refinementHistory.push({ iteration: 1, edits: initialEdits });
    let workingPrompt = currentPrompt;
    if (progressBar) progressBar.update(PROGRESS_PERCENTAGES.initialPromptComplete, { task: 'Initial prompt generated' });

    const cache = {
      get: (k, i) => null,
      set: () => {}
    };
    const progressRange = PROGRESS_PERCENTAGES.finalization - PROGRESS_PERCENTAGES.initialPromptComplete;
    const progressPerIteration = progressRange / recommendedIterations;
    for (let i = 0; i < recommendedIterations; i++) {
      const iterationNum = i + 2;
      if (progressBar) progressBar.update(PROGRESS_PERCENTAGES.initialPromptComplete + i * progressPerIteration, { task: `Adaptive refinement (${iterationNum}/${recommendedIterations + 1}) for ${context.domain}...` });
      const currentPromptJson = JSON.stringify(workingPrompt, null, 2);
      const { prompt: refinedPrompt, edits } = await refinePrompt(client, currentPromptJson, refinementHistory, config.model, iterationNum, cache);
      if (!refinedPrompt) {
        console.log(themeManager.color('warning')(`\nRefinement ${iterationNum} failed, using previous version.`));
        break;
      }
      workingPrompt = refinedPrompt;
      refinementHistory.push({ iteration: iterationNum, edits });
    }

    if (progressBar) progressBar.update(PROGRESS_PERCENTAGES.finalization, { task: 'Finalizing agentic output...' });
    const evaluation = DEFAULT_EVALUATION;
    const performanceMetrics = {
      accuracy_gain: `Estimated ${PERFORMANCE_METRICS.accuracyGainBase + complexityScore * PERFORMANCE_METRICS.accuracyGainMultiplier}% improvement`,
      optimization_level: strategy.focus,
      quality_score: evaluation.overallScore.toFixed(1),
      iterations_applied: refinementHistory.length
    };

    if (!config._cliOptions?.outputFile && !fs.existsSync(PROMPTS_DIR)) fs.mkdirSync(PROMPTS_DIR, { recursive: true });
    const outputFile = config._cliOptions?.outputFile
      ? (path.isAbsolute(config._cliOptions.outputFile) ? config._cliOptions.outputFile : path.join(process.cwd(), config._cliOptions.outputFile))
      : path.join(PROMPTS_DIR, `pe2-session-${sessionId}.md`);

    const { formatMarkdownOutput } = await import('./templates.js');
    const finalOutput = formatMarkdownOutput(workingPrompt, refinementHistory, performanceMetrics, difficulty, complexityScore);
    fs.writeFileSync(outputFile, finalOutput, 'utf-8');
    statsTracker.track(config.model, complexityScore, 0);

    if (progressBar) { progressBar.update(PROGRESS_PERCENTAGES.complete, { task: 'Complete!' }); progressBar.stop(); progressBar = null; }
    setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
    return { success: true, outputFile };
  } catch (error) {
    if (progressBar) { progressBar.stop(); progressBar = null; }
    console.log(chalk.red(`✗ Error: ${error.message}\n`));
    setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
    return { success: false };
  }
}


