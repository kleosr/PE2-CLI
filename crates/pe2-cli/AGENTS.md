# pe2-cli — Binary Entry

**4 modules, ~224 LOC**

The CLI binary. Minimal dispatch layer: parse args, route to interactive or single-prompt mode.

## Where To Look

| Module | File | Role |
|--------|------|-------|
| Main | `src/main.rs` (108L) | Entry point, tokio runtime, mode dispatch |
| Args | `src/args.rs` (48L) | Clap derive structs for all CLI flags |
| Commands | `src/commands.rs` (66L) | Command routing (interactive vs single-shot) |
| Lib | `src/lib.rs` (2L) | Crate root, re-exports |

## Flow

`main.rs` → clap parse → `commands.rs` dispatch → either interactive mode (pe2-tui) or single-shot pipeline (pe2-core).

## Anti-Patterns

- **No tests** — binary entry point completely untested
- **main.rs does too much** — 108 lines mixing clap setup, runtime init, config loading, and mode dispatch
