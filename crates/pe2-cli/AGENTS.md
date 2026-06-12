# pe2-cli — Binary Entry

**Parent:** `AGENTS.md`
**Scope:** `crates/pe2-cli/`

**4 modules, ~224 LOC** — minimal dispatch: parse args, route to interactive or single-prompt mode.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Main | `src/main.rs` (108L) | Entry, tokio runtime, `SingleClientAdapter`, mode dispatch |
| Args | `src/args.rs` (48L) | Clap derive structs |
| Commands | `src/commands.rs` (66L) | Config/prefs helpers — exported but unused by TUI |
| Lib | `src/lib.rs` (2L) | Re-exports `args`, `commands` |

## Flow

`main.rs` → clap parse → interactive (`pe2-tui`) or single-shot pipeline inline (`run_single_prompt`, ~65L).

`SingleClientAdapter` (defined in `main.rs`) implements `EngineLlmProvider` — not in core/providers.

## Conventions (delta)

- Lib+bin in one crate; `main.rs` imports `pe2_cli::args` from own lib
- Artifact name **`pe2`** via `[[bin]]`, package name `pe2-cli`
- `anyhow::Context` at boundary; downcasts `CliError` for exit codes

## Anti-Patterns

- **No tests** — binary entry completely untested
- **`main.rs` does too much** — clap, runtime, config, adapter, pipeline, display
- **`commands.rs` dead path** — TUI reimplements `/config`, `/session`, `/prefs` in `interactive.rs`

## Tests

```bash
# No crate tests — verify via integration:
cargo test -p pe2-core
cargo run -- --help
```
