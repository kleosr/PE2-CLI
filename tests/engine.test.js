import { describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { refinePrompt, generateInitialPrompt } from '../src/engine.js';

describe('refinePrompt', () => {
  test('throws when the client request fails', async () => {
    const client = {
      chat: {
        completions: {
          create: async () => {
            throw new Error('network failure');
          }
        }
      }
    };
    await assert.rejects(
      () =>
        refinePrompt({
          client,
          currentPromptJson: '{}',
          model: 'test-model',
          iterationNum: 2
        }),
      /network failure/
    );
  });

  test('throws when model returns empty content', async () => {
    const client = {
      chat: {
        completions: {
          create: async () => ({
            choices: [{ message: { content: '   ' } }]
          })
        }
      }
    };
    await assert.rejects(
      () =>
        refinePrompt({
          client,
          currentPromptJson: '{}',
          model: 'test-model',
          iterationNum: 2
        }),
      /empty content/
    );
  });
});

describe('generateInitialPrompt', () => {
  test('throws when model returns empty content', async () => {
    const client = {
      chat: {
        completions: {
          create: async () => ({
            choices: [{ message: { content: null } }]
          })
        }
      }
    };
    await assert.rejects(
      () => generateInitialPrompt(client, 'x'.repeat(20), 'test-model'),
      /empty content/
    );
  });
});
