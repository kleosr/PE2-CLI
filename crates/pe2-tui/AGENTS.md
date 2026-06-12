# pe2-tui — Terminal UI

**Parent:** `AGENTS.md`
**Scope:** `crates/pe2-tui/`

**5 modules, ~460 LOC** — banner, spinners, themed display, interactive REPL.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Interactive | `src/interactive.rs` (266L) | REPL: readline, `/commands`, provider+pipeline dispatch |
| Display | `src/display.rs` (127L) | Themed output, spinners, metrics |
| Banner | `src/banner.rs` (31L) | ASCII welcome screen |
| Theme | `src/theme.rs` (31L) | Color scheme constants |
| Lib | `src/lib.rs` (4L) | Re-exports |

## Key Details

- **crossterm** for terminal control; **indicatif** spinner during LLM calls
- `/help`, `/config`, `/session`, `/prefs`, `/stats`, `/clear` slash commands
- Creates provider via `pe2_providers::factory::create_client`, runs `pe2_core::engine::Pipeline` directly

## Conventions (delta)

- Depends on `pe2-providers` directly (skips `pe2-cli`) — presentation layer owns HTTP client creation
- `display.rs` imports `pe2_core::analysis` and `engine` types for formatted output

## Anti-Patterns

- **`interactive.rs` god module** — I/O, config, client creation, pipeline in 266 lines
- **Tight coupling** to `pe2-core` internals and `pe2-providers` factory
- **No tests**

## Tests

```bash
# Manual only — no crate tests:
cargo run --           # interactive mode
cargo run -- "test"    # single-shot via pe2-cli
```
