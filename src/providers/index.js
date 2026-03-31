import { createOpenAIClient } from './openai/client.js';
import { createAnthropicClient } from './anthropic/client.js';
import { createGoogleClient } from './google/client.js';
import { createOllamaClient } from './ollama/client.js';
import { createOpenRouterClient } from './openrouter/client.js';

export const PROVIDERS = {
  openai: {
    name: 'OpenAI',
    baseURL: 'https://api.openai.com/v1',
    models: [
      'gpt-4o', 'gpt-4o-mini', 'gpt-4-turbo', 'gpt-4', 'gpt-3.5-turbo',
    ],
    defaultModel: 'gpt-4o-mini',
    keyLabel: 'OpenAI API Key'
  },
  anthropic: {
    name: 'Anthropic (Claude)',
    baseURL: 'https://api.anthropic.com/v1',
    models: [
      'claude-3-5-sonnet-20241022', 'claude-3-5-haiku-20241022', 'claude-3-opus-20240229',
    ],
    defaultModel: 'claude-3-5-sonnet-20241022',
    keyLabel: 'Anthropic API Key'
  },
  google: {
    name: 'Google (Gemini)',
    baseURL: 'https://generativelanguage.googleapis.com/v1beta',
    models: [
      'gemini-1.5-pro-latest', 'gemini-1.5-flash-latest', 'gemini-1.0-pro'
    ],
    defaultModel: 'gemini-1.5-flash-latest',
    keyLabel: 'Google AI API Key'
  },
  openrouter: {
    name: 'OpenRouter (Multi-Provider)',
    baseURL: 'https://openrouter.ai/api/v1',
    models: [
      'openai/gpt-4o', 'openai/gpt-4o-mini', 'openai/gpt-4-turbo', 'openai/gpt-4', 'openai/gpt-3.5-turbo',
      'anthropic/claude-3-5-sonnet', 'anthropic/claude-3-opus', 'google/gemini-1.5-pro-latest'
    ],
    defaultModel: 'openai/gpt-4o-mini',
    keyLabel: 'OpenRouter API Key'
  },
  ollama: {
    name: 'Ollama (Local)',
    baseURL: 'http://localhost:11434',
    models: ['llama3.2', 'llama3.1', 'mistral', 'mixtral', 'codellama', 'deepseek-coder', 'custom'],
    defaultModel: 'llama3.2',
    keyLabel: 'Ollama Base URL'
  }
};

const PROVIDER_FACTORIES = {
  openai: (key) => createOpenAIClient(key, PROVIDERS.openai.baseURL),
  openrouter: (key) => createOpenRouterClient({ apiKey: key, baseURL: PROVIDERS.openrouter.baseURL }),
  anthropic: (key) => createAnthropicClient(key),
  google: (key) => createGoogleClient(key),
  ollama: (baseURL) => createOllamaClient(baseURL || PROVIDERS.ollama.baseURL)
};

export function getProviderClient(provider, apiKey) {
  const createClient = PROVIDER_FACTORIES[provider];
  if (!createClient) throw new Error(`Unsupported provider: ${provider}`);
  return createClient(apiKey);
}
