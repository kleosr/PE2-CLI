# PE2-CLI Knowledge Base

**Generated:** 2026-05-17
**Stack:** Node.js ESM CLI (no TypeScript)

## Overview

CLI tool converting raw prompts into PE2-optimized prompts via configurable LLM providers (OpenAI, Anthropic, Google, OpenRouter, Ollama).

## Structure

```
./
├── src/           # Application source (14 entries, 5 subdirs)
├── tests/         # Test files (6 *.test.js, node --test runner)
├── scripts/       # CI helper scripts
├── .github/       # CI workflow (publish-only on v* tag)
└── .sisyphus/     # Agent orchestration state
```

## Where To Look

| Task | Location | Notes |
|------|----------|-------|
| CLI entry point | `src/cli/main.js` | bin entry, commander setup |
| Config loading | `src/config.js` | JSON file at ~/.kleosr-pe2/config.json |
| Prompt processing | `src/engine.js` | Core LLM pipeline |
| Provider clients | `src/providers/{name}/client.js` | 5 providers, adapter pattern |
| Interactive loop | `src/cli/interactive.js` | readline-based REPL |
| Slash commands | `src/cli/commandHandlers/` | 4 command handlers + registry |
| Utilities | `src/utils/` | 7 modules (session, stats, preferences, etc.) |

## Conventions

- **ESM only** — all imports include `.js` extension
- **2-space indent, no semicolons, single quotes, trailing commas**
- **camelCase** for identifiers, **PascalCase** for classes, **UPPER_SNAKE_CASE** for constants
- **async/await** only — no `.then()` chains
- **Atomic JSON writes** via `writeJsonFileAtomically()` for all persistent state
- **Provider adapters normalize** to `{ choices: [{ message: { content } }] }`
- Tests use **Node `node:test`** + `node:assert/strict` — no test libraries
- Node **>=18.17.0** required

## Anti-Patterns (This Project)

- **No linter/formatter** — code style is convention-only, no enforcement
- **No `engine.js` tests** — core business logic is untested
- **No PR CI** — tests only run on tag push (`v*`)
- **`src/providers/index.js` couples model data + client factory** — violates SRP
- **`src/cli/interactive.js` is a god module** — 211 lines doing I/O, config, client creation, and pipeline orchestration
- **No `"files"` or `.npmignore`** — everything (including .sisyphus, scripts) gets published
- **Deferred saves** in `preferences.js`/`stats.js` can lose last write on crash

## Commands

```bash
npm start                # node ./src/cli/main.js
npm test                 # node --test ./tests/*.test.js
npx @kleosr/pe2-cli      # Run without global install
```

## Notes

- Config stored at `~/.kleosr-pe2/` (3 files: config.json, preferences.json, stats.json + sessions/)
- Output goes to `./pe2-prompts/` unless `--output-file` is given
- No `"main"` or `"exports"` in package.json — CLI-only, not importable as library
