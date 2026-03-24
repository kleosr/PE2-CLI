import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync } from 'fs';
import { tmpdir } from 'os';
import path from 'path';
import { writeJsonFileAtomically } from '../src/utils/writeJsonFileAtomically.js';

test('writes valid JSON readable after rename', () => {
    const dir = mkdtempSync(path.join(tmpdir(), 'pe2-atomic-'));
    const filePath = path.join(dir, 'data.json');
    writeJsonFileAtomically(filePath, { n: 42, s: 'ok' });
    const parsed = JSON.parse(readFileSync(filePath, 'utf8'));
    assert.deepEqual(parsed, { n: 42, s: 'ok' });
});
