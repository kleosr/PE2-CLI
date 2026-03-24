import { describe, test, afterEach } from 'node:test';
import assert from 'node:assert/strict';
import { getDefaultConfig, resolveApiKey } from '../src/config.js';

describe('getDefaultConfig', () => {
    test('returns provider and model defaults', () => {
        const c = getDefaultConfig();
        assert.equal(typeof c.provider, 'string');
        assert.equal(typeof c.model, 'string');
        assert.ok(c.provider.length > 0);
    });
});

describe('resolveApiKey', () => {
    afterEach(() => {
        delete process.env.OPENAI_API_KEY;
        delete process.env.OLLAMA_BASE_URL;
    });

    test('prefers configured key when non-empty', () => {
        assert.equal(resolveApiKey('openai', 'sk-from-config'), 'sk-from-config');
    });

    test('uses OpenAI env when config key empty', () => {
        process.env.OPENAI_API_KEY = 'sk-from-env';
        assert.equal(resolveApiKey('openai', ''), 'sk-from-env');
        assert.equal(resolveApiKey('openai', null), 'sk-from-env');
    });

    test('uses Ollama base URL env for ollama provider', () => {
        process.env.OLLAMA_BASE_URL = 'http://127.0.0.1:11434';
        assert.equal(resolveApiKey('ollama', ''), 'http://127.0.0.1:11434');
    });
});
