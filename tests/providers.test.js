import { describe, test, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert/strict';
import { HTTP_HEADERS } from '../src/constants.js';
import { assertNonEmptyString } from '../src/providers/assertNonEmptyString.js';
import { buildOpenRouterStyleHeaders } from '../src/providers/openRouterHeaders.js';
import { createOllamaClient } from '../src/providers/ollama/client.js';
import { createOpenRouterClient } from '../src/providers/openrouter/client.js';
import { getProviderClient } from '../src/providers/index.js';

describe('assertNonEmptyString', () => {
  test('throws for empty, whitespace-only, null, undefined', () => {
    assert.throws(() => assertNonEmptyString('', 'e'), /e/);
    assert.throws(() => assertNonEmptyString('   ', 'e'), /e/);
    assert.throws(() => assertNonEmptyString(null, 'e'), /e/);
    assert.throws(() => assertNonEmptyString(undefined, 'e'), /e/);
  });

  test('allows non-empty trimmed value', () => {
    assert.doesNotThrow(() => assertNonEmptyString(' abc ', 'e'));
  });
});

describe('buildOpenRouterStyleHeaders', () => {
  test('includes referer and title from constants', () => {
    const headers = buildOpenRouterStyleHeaders();
    assert.equal(headers['HTTP-Referer'], HTTP_HEADERS.referer);
    assert.equal(headers['X-Title'], HTTP_HEADERS.title);
  });

  test('merges extra headers without dropping defaults', () => {
    const headers = buildOpenRouterStyleHeaders({ 'X-Custom': 'v' });
    assert.equal(headers['HTTP-Referer'], HTTP_HEADERS.referer);
    assert.equal(headers['X-Custom'], 'v');
  });
});

describe('getProviderClient', () => {
  test('throws for unsupported provider', () => {
    assert.throws(() => getProviderClient('unknown-provider', 'k'), /Unsupported provider/);
  });

  test('returns a client with chat.completions.create for each supported id', () => {
    const providers = ['openai', 'openrouter', 'anthropic', 'google', 'ollama'];
    const keys = {
      openai: 'sk-test',
      openrouter: 'sk-or',
      anthropic: 'sk-ant',
      google: 'AIza',
      ollama: 'http://127.0.0.1:11434'
    };
    for (const p of providers) {
      const client = getProviderClient(p, keys[p]);
      assert.equal(typeof client.chat?.completions?.create, 'function', p);
    }
  });
});

describe('createOpenRouterClient with mocked fetch', () => {
  let originalFetch;

  beforeEach(() => {
    originalFetch = globalThis.fetch;
  });

  afterEach(() => {
    globalThis.fetch = originalFetch;
  });

  test('throws when api key missing', () => {
    assert.throws(() => createOpenRouterClient({ apiKey: '' }), /OpenRouter API key is required/);
  });

  test('POSTs JSON body and returns parsed JSON on success', async () => {
    const completion = { choices: [{ message: { content: 'ok' } }] };
    globalThis.fetch = async (url, init) => {
      assert.match(String(url), /\/chat\/completions$/);
      assert.equal(init.method, 'POST');
      assert.ok(init.headers.Authorization?.startsWith('Bearer '));
      const body = JSON.parse(init.body);
      assert.equal(body.model, 'm1');
      assert.deepEqual(body.messages, [{ role: 'user', content: 'hi' }]);
      return {
        ok: true,
        json: async () => completion
      };
    };

    const client = createOpenRouterClient({ apiKey: 'secret' });
    const result = await client.chat.completions.create({
      model: 'm1',
      messages: [{ role: 'user', content: 'hi' }]
    });
    assert.deepEqual(result, completion);
  });

  test('preserves max_tokens when zero', async () => {
    let parsedBody;
    globalThis.fetch = async (_url, init) => {
      parsedBody = JSON.parse(init.body);
      return { ok: true, json: async () => ({}) };
    };

    const client = createOpenRouterClient({ apiKey: 'k' });
    await client.chat.completions.create({
      model: 'm',
      messages: [],
      max_tokens: 0
    });
    assert.equal(parsedBody.max_tokens, 0);
  });

  test('throws when response not ok', async () => {
    globalThis.fetch = async () => ({
      ok: false,
      status: 502,
      json: async () => ({})
    });

    const client = createOpenRouterClient({ apiKey: 'k' });
    await assert.rejects(
      () => client.chat.completions.create({ model: 'm', messages: [] }),
      /OpenRouter error: 502/
    );
  });
});

describe('createOllamaClient with mocked fetch', () => {
  let originalFetch;

  beforeEach(() => {
    originalFetch = globalThis.fetch;
  });

  afterEach(() => {
    globalThis.fetch = originalFetch;
  });

  test('throws when base URL missing', () => {
    assert.throws(() => createOllamaClient(''), /Ollama base URL is required/);
  });

  test('POSTs chat payload and maps message content', async () => {
    globalThis.fetch = async (url, init) => {
      assert.equal(url, 'http://localhost:11434/api/chat');
      const body = JSON.parse(init.body);
      assert.equal(body.model, 'llama');
      assert.equal(body.stream, false);
      assert.equal(body.options.num_predict, 512);
      return {
        ok: true,
        status: 200,
        statusText: 'OK',
        json: async () => ({ message: { content: 'reply' } })
      };
    };

    const client = createOllamaClient('http://localhost:11434');
    const out = await client.chat.completions.create({
      model: 'llama',
      messages: [{ role: 'user', content: 'x' }],
      max_tokens: 512
    });
    assert.equal(out.choices[0].message.content, 'reply');
  });

  test('throws when response not ok', async () => {
    globalThis.fetch = async () => ({
      ok: false,
      status: 500,
      statusText: 'Internal'
    });

    const client = createOllamaClient('http://127.0.0.1:11434');
    await assert.rejects(
      () => client.chat.completions.create({ model: 'm', messages: [] }),
      /Ollama request failed: 500 Internal/
    );
  });
});
