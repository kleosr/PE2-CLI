# pe2-core — Engine & State

**Parent:** `AGENTS.md`
**Scope:** `crates/pe2-core/`

**13 modules, ~1,250 LOC** — config, analysis, pipeline, templates, session, stats.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Engine | `src/engine.rs` (254L) | LLM pipeline: complexity → call → refine → output |
| Analysis | `src/analysis.rs` (139L) | 5-factor scoring → difficulty tiers |
| Config | `src/config.rs` (81L) | JSON at `~/.kleosr-pe2/config.json` |
| Paths | `src/paths.rs` (21L) | Output path resolution |
| Templates | `src/templates.rs` (88L) | PE2 system prompts for LLM |
| Session | `src/session.rs` (119L) | Session persistence (`std::fs::write`, not atomic) |
| Stats | `src/stats.rs` (109L) | Usage metrics |
| Preferences | `src/preferences.rs` (99L) | User preferences |
| Validation | `src/validation.rs` (108L) | Input/output validation |
| Errors | `src/errors.rs` | `CliError` enum — imported by all crates |
| Constants | `src/constants.rs` (112L) | Model defaults, tier thresholds, regex |
| Write atomic | `src/write_atomic.rs` | `write_json_atomic` for config/stats/prefs |
| Integration tests | `tests/integration.rs` (455L) | End-to-end pipeline (JS port lineage) |

## Key Flow

`engine.rs`: analysis → provider call → parse → refine loop (1–5 passes by tier) → output markdown.

## Conventions (delta)

- Flat `src/*.rs` — no nested module dirs
- `EngineLlmProvider` trait abstracts LLM calls from engine to adapters
- Unit tests only in `analysis.rs`, `validation.rs`; pipeline covered by integration tests

## Anti-Patterns

- **No unit tests on `engine.rs`** — integration tests only
- **Deferred saves** in `preferences.rs`/`stats.rs` — background thread, `.ok()` swallows errors
- **`session.rs` skips atomic write** — inconsistent with config/stats/prefs
- **`engine.rs` L206** — direct `std::fs::write` for output markdown
- **`messages.rs` duplicates provider response parsing** — overlap with `pe2-providers`

## Tests

```bash
cargo test -p pe2-core
```
