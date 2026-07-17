# AGENTS.md

## Overview

CLI that turns raw prompts into PE²-structured prompts via LLM providers (OpenAI, Anthropic, Google, OpenRouter, Ollama). Rust 2021 workspace (`4.0.2`), binary `pe2`, tokio + reqwest (rustls).

## Where to look

| Task | Path |
|------|------|
| Binary / modes | `crates/pe2-cli/src/main.rs`, `args.rs` |
| Pipeline | `crates/pe2-core/src/engine.rs` |
| Config / keys | `crates/pe2-core/src/config.rs` |
| Complexity scoring | `crates/pe2-core/src/analysis.rs` |
| Provider trait / factory | `crates/pe2-providers/src/client.rs`, `factory.rs` |
| Pipeline glue | `crates/pe2-providers/src/runner.rs` |
| Shared HTTP | `crates/pe2-providers/src/http.rs` |
| Interactive REPL | `crates/pe2-tui/src/interactive/` |
| Generate + render | `crates/pe2-tui/src/prompt_flow.rs` |
| Node napi bridge | `crates/pe2-bindings/src/lib.rs` |
| Release CI | `.github/workflows/publish.yml` |
| npm meta | `npm/package.json` |

**DAG:** `cli → {core, providers, tui}` · `tui → {core, providers}` · `bindings → {core, providers}` · `providers → core`

## Commands

| Action | Command |
|--------|---------|
| Build | `cargo build` |
| Test | `cargo test` |
| Help | `cargo run -- --help` |
| One-shot | `cargo run -- "prompt text"` |
| Interactive | `cargo run --` (or `--config`) |

## Conventions

- `anyhow` at CLI boundary; `thiserror` (`CliError`) in core
- Persist JSON via `write_atomic` / `JsonStore` (config, prefs, stats, sessions)
- Providers implement `EngineLlmProvider` directly; callers use `run_pipeline`
- Toolchain: `.rust-toolchain.toml` (1.85.0); PR CI in `.github/workflows/ci.yml`

## Anti-patterns

- Do not wire providers in CLI/TUI by hand — use `pe2_providers::runner::run_pipeline`
- Do not persist API keys in `config.json` (session/env only)

## Notes

- Config: `~/.kleosr-pe2/` (`config.json`, `preferences.json`, `stats.json`, `sessions/`)
- Default output: `./pe2-prompts/` unless `--output-file`
- npm: `@kleosr/pe2-cli` (`npm/`); root `package.json` is private and version-skewed
- Child `AGENTS.md` under `crates/pe2-{core,providers,cli,tui}/` (may be denser than this root map)
