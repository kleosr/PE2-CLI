import fs from 'fs';
import path from 'path';
import { createProgressBar } from './utils/display.js';
import { formatMarkdownOutput, getInitialTemplate, getRefinementTemplate } from './templates.js';
import { buildMessages } from './messages.js';
import { setTerminalTitle } from './ui.js';
import { LLM_CONFIG, PROGRESS_PERCENTAGES, PERFORMANCE_METRICS, DEFAULT_EVALUATION, REQUIRED_PROMPT_FIELDS, HTTP_HEADERS } from './constants.js';
import { PE2_LOCAL_PROMPTS_DIR } from './paths.js';

const FALLBACK_PROMPT = {
  context: 'No context provided',
  role: 'Expert assistant',
  task: 'Complete the requested task',
  constraints: 'Follow best practices',
  output: 'Provide appropriate output'
};

const DEFAULT_PROMPT = {
  context: 'General purpose task',
  role: 'Expert assistant with deep knowledge in the relevant domain',
  task: "1. Understand the user's requirements\n2. Provide a comprehensive solution\n3. Ensure clarity and completeness",
  constraints: '- Be accurate and thorough\n- Follow best practices\n- Provide clear explanations',
  output: 'A well-structured response that fully addresses the user\'s needs'
};

function parseJsonOrThrow(content) {
  return JSON.parse(content);
}

function sliceFirstJsonObject(content) {
  const firstBrace = content.indexOf('{');
  const lastBrace = content.lastIndexOf('}');
  if (firstBrace === -1 || lastBrace === -1 || lastBrace <= firstBrace) {
    return null;
  }
  return content.slice(firstBrace, lastBrace + 1);
}

function tryParseJsonWithTrailingCommaRepair(jsonSlice) {
  const repaired = jsonSlice.replace(/,(\s*[}\]])/g, '$1');
  try {
    return JSON.parse(repaired);
  } catch {
    return null;
  }
}

function buildPromptFromParsedWithFallbacks(parsed) {
  const hasAll = REQUIRED_PROMPT_FIELDS.every(field => Object.prototype.hasOwnProperty.call(parsed, field));
  if (hasAll) {
    return { prompt: parsed, edits: 'Prompt generation with field validation.' };
  }
  return {
    prompt: {
      context: parsed.context || FALLBACK_PROMPT.context,
      role: parsed.role || FALLBACK_PROMPT.role,
      task: parsed.task || FALLBACK_PROMPT.task,
      constraints: parsed.constraints || FALLBACK_PROMPT.constraints,
      output: parsed.output || FALLBACK_PROMPT.output
    },
    edits: 'Prompt generation with field validation.'
  };
}

function buildStructuredPromptFromRawText(rawPrompt) {
  return {
    prompt: {
      ...DEFAULT_PROMPT,
      context: `The user wants to: ${rawPrompt.substring(0, 500)}${rawPrompt.length > 500 ? '...' : ''}`
    },
    edits: 'Prompt generation with automatic structuring.'
  };
}

function parsePromptResponse(content, rawPrompt) {
  try {
    return { prompt: parseJsonOrThrow(content), edits: 'Prompt generation successful.' };
  } catch {
    const jsonSlice = sliceFirstJsonObject(content);
    if (jsonSlice) {
      const parsed = tryParseJsonWithTrailingCommaRepair(jsonSlice);
      if (parsed) {
        return buildPromptFromParsedWithFallbacks(parsed);
      }
    }
    return buildStructuredPromptFromRawText(rawPrompt);
  }
}

export async function generateInitialPrompt(client, rawPrompt, model) {
  const systemContent = LLM_CONFIG.systemMessage;
  const userContent = getInitialTemplate(rawPrompt);
  const response = await client.chat.completions.create({
    model,
    messages: buildMessages({ system: systemContent, user: userContent }),
    max_tokens: LLM_CONFIG.maxTokens,
    temperature: LLM_CONFIG.temperature
  });
  const content = response.choices[0].message.content;
  return parsePromptResponse(content, rawPrompt);
}

export async function refinePrompt(client, currentPromptJson, model, iterationNum) {
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
    const result = parsePromptResponse(content, currentPromptJson);
    return result.prompt ? { prompt: result.prompt, edits: `Refined prompt based on PE2 principles (Iteration ${iterationNum}).` } : { prompt: null, edits: null };
  } catch {
    return { prompt: null, edits: null };
  }
}

export async function processPrompt({
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
}) {
  let progressBar = null;
  try {
    setTerminalTitle(`KleoSr PE2-CLI - Processing Session ${sessionId}`);

    console.log(themeManager.color('info')(`\n⚡ Processing Session ${sessionId} (${prompt.length} chars)...`));

    progressBar = createProgressBar();
    progressBar.start(PROGRESS_PERCENTAGES.complete, PROGRESS_PERCENTAGES.initialization, { task: 'Initializing agentic processing...' });

    const refinementHistory = [];
    progressBar.update(PROGRESS_PERCENTAGES.initialPromptStart, { task: `Generating ${context.domain}-optimized PE² prompt...` });
    const { prompt: currentPrompt, edits: initialEdits } = await generateInitialPrompt(client, prompt, config.model);
    if (!currentPrompt) {
      progressBar.stop();
      progressBar = null;
      return { success: false };
    }
    refinementHistory.push({ iteration: 1, edits: initialEdits });
    let workingPrompt = currentPrompt;
    progressBar.update(PROGRESS_PERCENTAGES.initialPromptComplete, { task: 'Initial prompt generated' });

    const progressRange = PROGRESS_PERCENTAGES.finalization - PROGRESS_PERCENTAGES.initialPromptComplete;
    const progressPerIteration = progressRange / iterations;
    for (let i = 0; i < iterations; i++) {
      const iterationNum = i + 2;
      progressBar.update(PROGRESS_PERCENTAGES.initialPromptComplete + i * progressPerIteration, { task: `Adaptive refinement (${iterationNum}/${iterations + 1}) for ${context.domain}...` });
      const currentPromptJson = JSON.stringify(workingPrompt, null, 2);
      const { prompt: refinedPrompt, edits } = await refinePrompt(client, currentPromptJson, config.model, iterationNum);
      if (!refinedPrompt) {
        console.log(themeManager.color('warning')(`\nRefinement ${iterationNum} failed, using previous version.`));
        break;
      }
      workingPrompt = refinedPrompt;
      refinementHistory.push({ iteration: iterationNum, edits });
    }

    progressBar.update(PROGRESS_PERCENTAGES.finalization, { task: 'Finalizing agentic output...' });
    const evaluation = DEFAULT_EVALUATION;
    const performanceMetrics = {
      accuracy_gain: `Estimated ${PERFORMANCE_METRICS.accuracyGainBase + complexityScore * PERFORMANCE_METRICS.accuracyGainMultiplier}% improvement`,
      optimization_level: strategy.focus,
      quality_score: evaluation.overallScore.toFixed(1),
      iterations_applied: refinementHistory.length
    };

    if (!config._cliOptions?.outputFile && !fs.existsSync(PE2_LOCAL_PROMPTS_DIR)) {
      fs.mkdirSync(PE2_LOCAL_PROMPTS_DIR, { recursive: true });
    }
    const outputFile = config._cliOptions?.outputFile
      ? (path.isAbsolute(config._cliOptions.outputFile) ? config._cliOptions.outputFile : path.join(process.cwd(), config._cliOptions.outputFile))
      : path.join(PE2_LOCAL_PROMPTS_DIR, `pe2-session-${sessionId}.md`);

    const finalOutput = formatMarkdownOutput(workingPrompt, refinementHistory, performanceMetrics, difficulty, complexityScore);
    fs.writeFileSync(outputFile, finalOutput, 'utf-8');
    statsTracker.track(config.model, complexityScore, 0);

    progressBar.update(PROGRESS_PERCENTAGES.complete, { task: 'Complete!' });
    progressBar.stop();
    progressBar = null;
    setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
    return { success: true, outputFile, refinementHistory };
  } catch (error) {
    if (progressBar) { progressBar.stop(); progressBar = null; }
    setTerminalTitle('KleoSr PE²-CLI - Interactive Mode');
    throw error;
  }
}
