# pe2-core — Engine & State

**Parent:** `AGENTS.md`
**Scope:** `crates/pe2-core/`

Config, analysis, `Pipeline` / `EngineLlmProvider` / `Message`, templates, session/stats/prefs.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Engine | `src/engine.rs` | Pipeline, messages, output paths |
| Analysis | `src/analysis.rs` | Complexity → difficulty / iterations |
| Config | `src/config.rs` | Paths, load/save, `mask_api_key` |
| Validation | `src/validation.rs` | Prompt + slash resolve/suggest |
| Constants | `src/constants/` | `limits`, `llm`, `patterns`, `providers` |
| Stats / prefs / session | `src/{stats,preferences,session}.rs` | Persistence via `JsonStore` |

## Tests

```bash
cargo test -p pe2-core
```
