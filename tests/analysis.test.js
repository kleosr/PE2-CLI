import { describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { analyzePromptComplexity } from '../src/analysis.js';

describe('analyzePromptComplexity', () => {
    test('returns score, difficulty, and iterations for a simple prompt', () => {
        const r = analyzePromptComplexity('this is a simple prompt with enough words');
        assert.equal(typeof r.score, 'number');
        assert.ok(['NOVICE', 'INTERMEDIATE', 'ADVANCED', 'EXPERT', 'MASTER'].includes(r.difficulty));
        assert.ok(Number.isInteger(r.iterations) && r.iterations >= 1);
    });

    test('technical keywords increase score versus plain text of similar length', () => {
        const plain = 'word '.repeat(20).trim();
        const technical = `${plain} python api docker ml algorithm framework database`;
        const plainResult = analyzePromptComplexity(plain);
        const technicalResult = analyzePromptComplexity(technical);
        assert.ok(technicalResult.score >= plainResult.score);
    });
});
