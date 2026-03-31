import { assertNonEmptyString } from '../assertNonEmptyString.js';
import { DEFAULT_CHAT_COMPLETION_MAX_TOKENS } from '../defaults.js';

function stripTrailingSlash(url) {
  return url.replace(/\/$/, '');
}

function buildOllamaRequestBody(options) {
  return {
    model: options.model,
    messages: options.messages,
    stream: options.stream,
    options: { num_predict: options.max_tokens }
  };
}

function buildOllamaResponse(chatResponse) {
  return {
    choices: [
      {
        message: { content: chatResponse?.message?.content ?? '' }
      }
    ]
  };
}

function handleOllamaTimeout() {
  throw new Error('Ollama request timed out after 30 seconds');
}

async function postOllamaChat(base, options) {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), 30000);

  try {
    const response = await fetch(`${base}/api/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(buildOllamaRequestBody(options)),
      signal: controller.signal
    });
    clearTimeout(timeoutId);

    if (!response.ok) {
      throw new Error(`Ollama request failed: ${response.status} ${response.statusText}`);
    }

    const chatResponse = await response.json();
    return buildOllamaResponse(chatResponse);
  } catch (error) {
    clearTimeout(timeoutId);
    if (error.name === 'AbortError') handleOllamaTimeout();
    throw error;
  }
}

export function createOllamaClient(baseURL = 'http://localhost:11434') {
  assertNonEmptyString(baseURL, 'Ollama base URL is required');

  const base = stripTrailingSlash(baseURL);

  return {
    chat: {
      completions: {
        create: (options) => postOllamaChat(base, {
          model: options.model,
          messages: options.messages,
          max_tokens: options.max_tokens ?? DEFAULT_CHAT_COMPLETION_MAX_TOKENS,
          stream: options.stream ?? false
        })
      }
    }
  };
}
