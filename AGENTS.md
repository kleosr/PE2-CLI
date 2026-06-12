# PE2-CLI Knowledge Base

**Generated:** 2026-06-12
**Commit:** c5ff5b8
**Branch:** main
**Stack:** Rust workspace (5 crates, ~3,200 LOC, 36 source files)

## Overview

CLI tool converting raw prompts into PE2-optimized prompts via configurable LLM providers (OpenAI, Anthropic, Google, OpenRouter, Ollama). Rust implementation — single static binary, zero GC, tokio async.

## Structure

```
./
├── Cargo.toml            # Workspace root (version 4.0.2, edition 2021)
├── crates/
│   ├── pe2-core/         # Core engine: config, analysis, pipeline, templates, session, stats
│   ├── pe2-providers/    # 5 LLM provider adapters (adapter pattern)
│   ├── pe2-tui/          # Terminal UI: banner, spinner, interactive REPL (crossterm)
│   ├── pe2-cli/          # CLI binary: clap args, command routing (entry point)
│   └── pe2-bindings/     # napi-rs bridge (optional, Node.js native addon)
├── npm/                  # npm meta-package (@kleosr/pe2-cli)
└── .github/workflows/    # CI: cargo test + matrix build on v* tag
```

**Dependency DAG:** `cli → {core, providers, tui}` · `tui → {core, providers}` · `bindings → {core, providers}` · `providers → core`

## Where To Look

| Task | Location | Notes |
|------|----------|-------|
| CLI entry | `crates/pe2-cli/src/main.rs` | Binary `pe2`; clap parse, mode dispatch, `SingleClientAdapter` |
| CLI args | `crates/pe2-cli/src/args.rs` | Clap derive structs |
| Core engine | `crates/pe2-core/src/engine.rs` | LLM pipeline, refinement loop |
| Config | `crates/pe2-core/src/config.rs` | JSON config at `~/.kleosr-pe2/` |
| Analysis | `crates/pe2-core/src/analysis.rs` | Prompt complexity scoring |
| Providers | `crates/pe2-providers/src/` | 5 adapters + `client.rs` trait |
| Provider factory | `crates/pe2-providers/src/factory.rs` | Runtime provider selection |
| Interactive loop | `crates/pe2-tui/src/interactive.rs` | Readline REPL with `/commands` |
| Display/UI | `crates/pe2-tui/src/display.rs` | Themed output, spinners |
| Node bindings | `crates/pe2-bindings/src/lib.rs` | napi-rs exports for npm package |
| Integration tests | `crates/pe2-core/tests/`, `crates/pe2-providers/tests/` | Cargo test |
| npm package | `npm/package.json` | Meta-package with platform optional deps |

## Code Map

| Symbol | Type | Location | Importers |
|--------|------|----------|-----------|
| `CliError` | enum | `pe2-core/src/errors.rs` | All crates |
| `constants` | module | `pe2-core/src/constants.rs` | core (analysis, config, session…), providers |
| `Message` | struct | `pe2-core/src/messages.rs` | engine, providers, cli, tui, bindings |
| `Pipeline` | struct | `pe2-core/src/engine.rs` | cli, tui, bindings |
| `LlmClient` | trait | `pe2-providers/src/client.rs` | factory, adapters, cli, tui |
| `create_client` | fn | `pe2-providers/src/factory.rs` | cli, tui, bindings |
| `Config` | struct | `pe2-core/src/config.rs` | cli, tui, engine, bindings |
| `write_json_atomic` | fn | `pe2-core/src/write_atomic.rs` | config, stats, preferences |

## Conventions

- **Rust edition 2021**, crate resolver = `"2"`
- **snake_case** functions/variables, **PascalCase** types/traits/enums, **SCREAMING_SNAKE_CASE** constants
- **async/await** with **tokio** (full features)
- **anyhow** at CLI boundary, **thiserror** (`CliError`) in core
- **serde** for all JSON serialization
- **reqwest** with rustls-tls for HTTP
- **Trait-based provider adapter** via `async_trait`
- Tests: std `#[cfg(test)]` + `cargo test` — no external test framework
- **2-space indent**, no trailing whitespace (Rustfmt default, not CI-enforced)
- Persistent JSON via `write_atomic::write_json_atomic` (config, stats, preferences)

## Anti-Patterns (This Project)

- **No rustfmt/clippy CI** — style is convention-only
- **No PR CI** — tests only on tag push (`v*`)
- **`interactive.rs` god module** — I/O, config, client creation, pipeline in one file
- **Provider duplication** — HTTP/JSON/error flow repeated across 5 adapters
- **No `pe2-cli` tests** — binary entry point untested
- **npm `files` config missing** — may include build artifacts
- **Deferred saves** in preferences/stats — 100ms background thread, last write may be lost on crash
- **`constants.rs` megamodule** — regex, model data, HTTP defaults, tier thresholds mixed
- **Session non-atomic writes** — `session.rs` uses `std::fs::write`, not `write_atomic`
- **Misleading error mapping** — reqwest body parse → `CliError::Json` in all providers
- **`commands.rs` dead path** — exported in lib but TUI reimplements `/config`, `/session`, `/prefs`
- **No `.rust-toolchain.toml`** — Rust 1.81+ documented in README only

## Commands

```bash
cargo build                  # Build all crates (incl. bindings)
cargo test                   # Run all tests
cargo run -- --help          # Run CLI
cargo run -- "prompt text"   # One-shot mode
cargo run -- --config        # Config menu
```

## Subtree Context

| Crate | AGENTS.md |
|-------|-----------|
| `crates/pe2-core/` | Engine, state, pipeline |
| `crates/pe2-providers/` | LLM adapters |
| `crates/pe2-cli/` | Binary entry |
| `crates/pe2-tui/` | Terminal UI |

## Cursor / Agents

- Root `AGENTS.md` is default; nearer subtree `AGENTS.md` auto-scopes to that folder.
- No project `.cursor/rules/` — conventions live here.
- Skills: `~/.cursor/skills/` (user) — not vendored in repo.

## Notes

- Config: `~/.kleosr-pe2/` (`config.json`, `preferences.json`, `stats.json`, `sessions/`)
- Output: `./pe2-prompts/` unless `--output-file` given
- npm: `@kleosr/pe2-cli` at `npm/package.json`; Node >=18.17.0; optional platform deps
- Binary name: `pe2` (package `pe2-cli`)
