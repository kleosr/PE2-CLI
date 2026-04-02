import { createOpenAIClient } from './openai/client.js';
import { createAnthropicClient } from './anthropic/client.js';
import { createGoogleClient } from './google/client.js';
import { createOllamaClient } from './ollama/client.js';
import { createOpenRouterClient } from './openrouter/client.js';
import { DEFAULT_OPENROUTER_MODEL, OPENROUTER_MODEL_IDS } from './openrouter/modelIds.js';

export const PROVIDERS = {
  openai: {
    name: 'OpenAI',
    baseURL: 'https://api.openai.com/v1',
    docsUrl: 'https://platform.openai.com/docs/models',
    models: [
      'gpt-5.4',
      'gpt-5.4-mini',
      'gpt-5.4-nano',
      'gpt-5.2'
    ],
    defaultModel: 'gpt-5.4-mini',
    keyLabel: 'OpenAI API Key'
  },
  anthropic: {
    name: 'Anthropic (Claude)',
    baseURL: 'https://api.anthropic.com/v1',
    docsUrl: 'https://docs.anthropic.com/en/docs/about-claude/models/overview',
    models: [
      'claude-opus-4-6',
      'claude-sonnet-4-6',
      'claude-haiku-4-5'
    ],
    defaultModel: 'claude-sonnet-4-6',
    keyLabel: 'Anthropic API Key'
  },
  google: {
    name: 'Google (Gemini)',
    baseURL: 'https://generativelanguage.googleapis.com/v1beta',
    docsUrl: 'https://ai.google.dev/gemini-api/docs/models',
    models: [
      'gemini-3.1-pro-preview',
      'gemini-3-flash-preview',
      'gemini-2.5-pro',
      'gemini-2.5-flash',
      'gemini-2.5-flash-lite'
    ],
    defaultModel: 'gemini-2.5-flash',
    keyLabel: 'Google AI API Key'
  },
  openrouter: {
    name: 'OpenRouter (Multi-Provider)',
    baseURL: 'https://openrouter.ai/api/v1',
    docsUrl: 'https://openrouter.ai/models',
    models: [...OPENROUTER_MODEL_IDS],
    defaultModel: DEFAULT_OPENROUTER_MODEL,
    keyLabel: 'OpenRouter API Key'
  },
  ollama: {
    name: 'Ollama (Local)',
    baseURL: 'http://localhost:11434',
    docsUrl: 'https://github.com/ollama/ollama/blob/main/README.md',
    models: [
      'llama3.3',
      'llama3.2',
      'mistral',
      'mixtral',
      'qwen2.5',
      'deepseek-coder-v2',
      'phi4',
      'codellama',
      'custom'
    ],
    defaultModel: 'llama3.3',
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
