import Anthropic from '@anthropic-ai/sdk';
import { assertNonEmptyString } from '../assertNonEmptyString.js';
import { DEFAULT_CHAT_COMPLETION_MAX_TOKENS } from '../defaults.js';

function extractSystemMessages(messages) {
  const systemParts = [];
  const userMessages = [];
  for (const message of messages ?? []) {
    if (message.role === 'system') {
      systemParts.push(message.content);
    } else {
      userMessages.push(message);
    }
  }
  return { systemParts, userMessages };
}

function buildAnthropicResponse(response) {
  const content = (response.content ?? [])
    .filter((block) => block.type === 'text')
    .map((block) => block.text)
    .join('');

  return {
    choices: [{
      message: { content, role: 'assistant' },
      finish_reason: response.stop_reason || 'stop'
    }],
    usage: response.usage,
    model: response.model
  };
}

async function createAnthropicCompletion(client, options) {
  const { systemParts, userMessages } = extractSystemMessages(options.messages);
  const response = await client.messages.create({
    model: options.model,
    messages: userMessages,
    system: systemParts.length ? systemParts.join('\n\n') : undefined,
    max_tokens: options.max_tokens ?? DEFAULT_CHAT_COMPLETION_MAX_TOKENS,
    temperature: options.temperature,
    stream: options.stream
  });
  return buildAnthropicResponse(response);
}

export function createAnthropicClient(apiKey) {
  assertNonEmptyString(apiKey, 'Anthropic API key is required');

  const client = new Anthropic({
    apiKey,
    maxTimeoutMs: 30000,
    defaultHeaders: {
      'anthropic-dangerous-direct-browser-sdk': 'true'
    }
  });

  return {
    chat: {
      completions: {
        create: (options) => createAnthropicCompletion(client, options)
      }
    }
  };
}
