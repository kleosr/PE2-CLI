export const DEFAULT_OPENROUTER_MODEL = 'openai/gpt-5.4-mini';

export const OPENROUTER_MODEL_IDS = [
  'openai/gpt-5.4',
  'openai/gpt-5.4-mini',
  'openai/gpt-5.4-nano',
  'anthropic/claude-sonnet-4-6',
  'anthropic/claude-opus-4-6',
  'google/gemini-2.5-flash',
  'google/gemini-2.5-pro',
  'google/gemini-3.1-pro-preview'
];

export function resolveOpenRouterModelId(model) {
  const trimmed = model == null ? '' : String(model).trim();
  if (trimmed === '') return DEFAULT_OPENROUTER_MODEL;
  return trimmed;
}
