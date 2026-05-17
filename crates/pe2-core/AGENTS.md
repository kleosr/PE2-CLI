# pe2-core — Engine & State

**14 modules, ~1,400 LOC**

The brain of PE2-CLI. Owns config, analysis, pipeline orchestration, templates, session persistence, and usage stats.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Engine | `src/engine.rs` (254L) | LLM pipeline: complexity → call → refine → output loop |
| Analysis | `src/analysis.rs` (139L) | Scores prompts on 5 factors → maps to difficulty tiers |
| Config | `src/config.rs` (81L) | JSON config at ~/.kleosr-pe2/config.json |
| Templates | `src/templates.rs` (88L) | PE2 system prompts passed to LLM |
| Session | `src/session.rs` (119L) | Persistent session state with atomic writes |
| Stats | `src/stats.rs` (109L) | Usage metrics, also atomic |
| Preferences | `src/preferences.rs` (99L) | User preferences persistence |
| Validation | `src/validation.rs` (108L) | Input/output validation logic |
| Integration tests | `tests/integration.rs` (455L) | End-to-end pipeline tests |

## Key Flow

`engine.rs` orchestrates: analysis → provider call → parse → refine loop → output. Complexity tier determines refinement count (1–5 passes).

## Conventions

- All persistent state uses atomic JSON writes (`write_atomic.rs`)
- Errors via `thiserror` (crate-level error enum in `errors.rs`)
- Constants in `constants.rs` (model defaults, tier thresholds)

## Anti-Patterns

- **No unit tests on engine.rs** — only integration tests cover the pipeline
- **Deferred saves** in preferences/stats can lose last write on crash
- **messages.rs duplicates provider response parsing** — share with pe2-providers?
