@AGENTS.md

# pe2-cli — Binary Entry

**Parent:** `AGENTS.md`
**Scope:** `crates/pe2-cli/`

Binary `pe2`: clap parse → interactive REPL or one-shot via `pe2_tui::prompt_flow`.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Main | `src/main.rs` | Mode dispatch, config merge, pipeline options |
| Args | `src/args.rs` | Clap `Args` |
| Lib | `src/lib.rs` | `pub mod args` |
| Smoke | `tests/smoke.rs` | Clap / validation smoke tests |

## Flow

`main` → `--config` / no prompt → `setup_and_run_interactive(options)` · else → `generate_prompt_with_spinner`.

## Tests

```bash
cargo test -p pe2-cli
```
