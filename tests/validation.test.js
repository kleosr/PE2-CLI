import { describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { validatePrompt, validateAndSuggestCommand } from '../src/utils/validation.js';
import { PROMPT_LIMITS } from '../src/constants.js';

describe('validatePrompt', () => {
    test('rejects empty input', () => {
        assert.equal(validatePrompt(''), 'Please enter a prompt');
        assert.equal(validatePrompt('   '), 'Please enter a prompt');
    });

    test('rejects prompt shorter than minimum', () => {
        assert.equal(validatePrompt('short'), 'Prompt too short. Please provide more detail.');
    });

    test('accepts prompt at minimum length', () => {
        const ok = 'x'.repeat(PROMPT_LIMITS.minLength);
        assert.equal(validatePrompt(ok), null);
    });

    test('rejects prompt longer than maximum', () => {
        const tooLong = 'x'.repeat(PROMPT_LIMITS.maxLength + 1);
        assert.equal(validatePrompt(tooLong), 'Prompt too long. Consider breaking it down.');
    });
});

describe('validateAndSuggestCommand', () => {
    test('accepts registered slash command', () => {
        const v = validateAndSuggestCommand('/help');
        assert.equal(v.valid, true);
        assert.equal(v.isCommand, true);
        assert.equal(v.command, '/help');
    });

    test('rejects unknown command with suggestions when applicable', () => {
        const v = validateAndSuggestCommand('/setings');
        assert.equal(v.valid, false);
        assert.equal(v.isCommand, true);
        assert.ok(Array.isArray(v.suggestions));
    });
});
