import OpenAI from 'openai';
import { assertNonEmptyString } from '../assertNonEmptyString.js';
import { buildOpenRouterStyleHeaders } from '../openRouterHeaders.js';

export function createOpenAIClient(apiKey, baseURL = 'https://api.openai.com/v1') {
  assertNonEmptyString(apiKey, 'OpenAI API key is required');

  return new OpenAI({
    apiKey,
    baseURL,
    defaultHeaders: buildOpenRouterStyleHeaders()
  });
}
