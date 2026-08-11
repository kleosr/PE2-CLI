# pe2-tui — Terminal UI

**Parent:** `AGENTS.md`
**Scope:** `crates/pe2-tui/`

Banner, display, `prompt_flow`, interactive REPL. Generation via `pe2_providers::runner::run_pipeline`.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Prompt flow | `src/prompt_flow.rs` | Spinner + `run_pipeline` |
| Interactive | `src/interactive/mod.rs` | REPL dispatch; takes `PipelineRunOptions` from CLI |
| Slash commands | `src/interactive/slash_commands.rs` | `/config` editor, session/prefs/stats views |
| Prompt input | `src/interactive/prompt.rs` | One-shot generation inside REPL; persists session+stats |
| Display | `src/display.rs` | Themed output / metrics |
| Theme | `src/theme.rs` | `PE2_THEME` fn-palette |
| Banner | `src/banner.rs` | ASCII banner, `TAGLINE` |

## Notes

- Unknown `/…` input uses `validate_and_suggest_command`
- Session API keys are in-memory only (`config.json` does not serialize `api_key`)

## Tests

```bash
cargo test -p pe2-tui
```
