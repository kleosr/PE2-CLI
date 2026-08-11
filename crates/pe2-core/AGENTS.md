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
| Validation | `src/validation.rs` | Prompt + slash resolve/suggest (`SLASH_TOKENS` table) |
| Constants | `src/constants/` | `limits`, `llm`, `patterns` (LazyLock), `providers` |
| Errors | `src/errors.rs` | `CliError` + exit codes |
| Templates | `src/templates.rs` | Initial/refinement prompt templates, markdown output |
| Persistence | `src/{json_store,write_atomic}.rs` | `JsonStore<T>` over atomic writes |
| Stats / prefs | `src/{stats,preferences}.rs` | Via `JsonStore` |
| Session | `src/session.rs` | `Arc<Mutex<Vec<_>>>` + `write_json_atomic` (not `JsonStore`) |

## Tests

```bash
cargo test -p pe2-core
```
