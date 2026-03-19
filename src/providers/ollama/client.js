import { assertNonEmptyString } from '../assertNonEmptyString.js';
import { DEFAULT_CHAT_COMPLETION_MAX_TOKENS } from '../defaults.js';

function stripTrailingSlash(url) {
  return url.replace(/\/$/, '');
}

async function postOllamaChat(base, { model, messages, max_tokens, stream }) {
  const response = await fetch(`${base}/api/chat`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      model,
      messages,
      stream,
      options: { num_predict: max_tokens }
    })
  });

  if (!response.ok) {
    throw new Error(`Ollama request failed: ${response.status} ${response.statusText}`);
  }

  const chatResponse = await response.json();
  const content = chatResponse?.message?.content ?? '';

  return {
    choices: [
      {
        message: { content }
      }
    ]
  };
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
