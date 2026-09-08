@AGENTS.md

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
| Persistence | `src/write_atomic.rs` | Atomic JSON/text writes + `read_json_or_default` |
| Stats / prefs | `src/{stats,preferences}.rs` | Stats persist; prefs load `track_usage` |
| Session | `src/session.rs` | In-memory `Vec<SessionEntry>` |

## Tests

```bash
cargo test -p pe2-core
```
