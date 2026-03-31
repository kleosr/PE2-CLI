import { GoogleGenerativeAI } from '@google/generative-ai';
import { assertNonEmptyString } from '../assertNonEmptyString.js';

const MESSAGE_ROLE_LABELS = {
  system: 'System',
  user: 'User',
  assistant: 'Assistant'
};

function buildGooglePrompt(messages) {
  return (messages ?? [])
    .map((msg) => `${MESSAGE_ROLE_LABELS[msg.role] ?? 'User'}: ${msg.content}`)
    .join('\n');
}

function buildGoogleResponse(generation, model) {
  return {
    choices: [{
      message: {
        content: generation.response.text(),
        role: 'assistant'
      },
      finish_reason: 'stop'
    }],
    usage: generation.response.usageMetadata || {},
    model
  };
}

async function createGoogleCompletion(genAI, options) {
  const model = genAI.getGenerativeModel({ model: options.model });
  const prompt = buildGooglePrompt(options.messages);

  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), 30000);

  try {
    const generation = await model.generateContent(prompt, { signal: controller.signal });
    clearTimeout(timeoutId);
    return buildGoogleResponse(generation, options.model);
  } catch (error) {
    clearTimeout(timeoutId);
    if (error.name === 'AbortError') {
      throw new Error('Google API request timed out after 30 seconds');
    }
    throw error;
  }
}

export function createGoogleClient(apiKey) {
  assertNonEmptyString(apiKey, 'Google API key is required');

  const genAI = new GoogleGenerativeAI(apiKey);

  return {
    chat: {
      completions: {
        create: (options) => createGoogleCompletion(genAI, options)
      }
    }
  };
}
