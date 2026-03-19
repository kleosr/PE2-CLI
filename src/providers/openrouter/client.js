import { assertNonEmptyString } from '../assertNonEmptyString.js';
import { DEFAULT_CHAT_COMPLETION_MAX_TOKENS } from '../defaults.js';
import { buildOpenRouterStyleHeaders } from '../openRouterHeaders.js';

function normalizedBaseUrl(baseURL) {
  return baseURL.replace(/\/$/, '');
}

function openRouterChatCompletionsUrl(baseURL) {
  return `${normalizedBaseUrl(baseURL)}/chat/completions`;
}

function buildChatCompletionBody(options) {
  return {
    model: options.model,
    messages: options.messages,
    max_tokens: options.max_tokens ?? DEFAULT_CHAT_COMPLETION_MAX_TOKENS,
    temperature: options.temperature,
    stream: options.stream
  };
}

function buildOpenRouterRequestHeaders(apiKey, extraHeaders) {
  return {
    'Content-Type': 'application/json',
    Authorization: `Bearer ${apiKey}`,
    ...buildOpenRouterStyleHeaders(extraHeaders)
  };
}

async function fetchOpenRouterChatCompletion(baseURL, apiKey, options) {
  const response = await fetch(openRouterChatCompletionsUrl(baseURL), {
    method: 'POST',
    headers: buildOpenRouterRequestHeaders(apiKey, options.headers ?? {}),
    body: JSON.stringify(buildChatCompletionBody(options))
  });

  if (!response.ok) {
    throw new Error(`OpenRouter error: ${response.status}`);
  }

  return response.json();
}

export function createOpenRouterClient({ apiKey, baseURL = 'https://openrouter.ai/api/v1' }) {
  assertNonEmptyString(apiKey, 'OpenRouter API key is required');

  return {
    chat: {
      completions: {
        create: (options) => fetchOpenRouterChatCompletion(baseURL, apiKey, options)
      }
    }
  };
}
