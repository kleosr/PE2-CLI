import { GoogleGenerativeAI } from '@google/generative-ai';
import { assertNonEmptyString } from '../assertNonEmptyString.js';

const MESSAGE_ROLE_LABELS = {
  system: 'System',
  user: 'User',
  assistant: 'Assistant'
};

export function createGoogleClient(apiKey) {
  assertNonEmptyString(apiKey, 'Google API key is required');

  const genAI = new GoogleGenerativeAI(apiKey);

  return {
    chat: {
      completions: {
        create: async (options) => {
          const model = genAI.getGenerativeModel({ model: options.model });
          const prompt = (options.messages ?? [])
            .map((m) => `${MESSAGE_ROLE_LABELS[m.role] ?? 'User'}: ${m.content}`)
            .join('\n');

          const generation = await model.generateContent(prompt);

          return {
            choices: [{
              message: {
                content: generation.response.text(),
                role: 'assistant'
              },
              finish_reason: 'stop'
            }],
            usage: generation.response.usageMetadata || {},
            model: options.model
          };
        }
      }
    }
  };
}
