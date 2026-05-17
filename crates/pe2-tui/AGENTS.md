# pe2-tui — Terminal UI

**5 modules, ~460 LOC**

All user-facing terminal output: welcome banner, spinners, colored/structured display, and the interactive REPL loop.

## Where To Look

| Module | File | Role |
|--------|------|-------|
| Interactive | `src/interactive.rs` (266L) | REPL loop: readline, /commands, dispatch |
| Display | `src/display.rs` (127L) | Themed output formatting, spinners |
| Banner | `src/banner.rs` (31L) | ASCII art welcome screen |
| Theme | `src/theme.rs` (31L) | Color scheme/theme constants |
| Lib | `src/lib.rs` (4L) | Crate root, re-exports |

## Key Details

- Uses **crossterm** for terminal control (cross-platform)
- Interactive mode supports `/help`, `/config`, `/session`, `/stats`, `/clear`
- Spinner via indicatif during LLM calls

## Anti-Patterns

- **interactive.rs is a god module** — 266 lines handling I/O, config loading, client creation, and pipeline dispatch
- Tightly coupled to pe2-core internals (creates providers, runs engine directly)
- No tests
