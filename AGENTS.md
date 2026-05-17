# PE2-CLI Knowledge Base

**Generated:** 2026-05-17
**Stack:** Rust workspace (5 crates, ~3,200 LOC, 36 source files)

## Overview

CLI tool converting raw prompts into PE2-optimized prompts via configurable LLM providers (OpenAI, Anthropic, Google, OpenRouter, Ollama). Rust implementation — single static binary, zero GC, tokio async.

## Structure

```
./
├── Cargo.toml            # Workspace root (version 4.0.2, edition 2021)
├── crates/
│   ├── pe2-core/         # Core engine: config, analysis, pipeline, templates, session, stats (14 modules)
│   ├── pe2-providers/    # 5 LLM provider adapters (9 modules, adapter pattern)
│   ├── pe2-tui/          # Terminal UI: banner, spinner, interactive REPL (crossterm)
│   ├── pe2-cli/          # CLI binary: clap args, command routing (entry point)
│   └── pe2-bindings/     # napi-rs bridge (optional, Node.js native addon)
├── npm/                  # npm meta-package (@kleosr/pe2-cli)
├── .github/workflows/    # CI: cargo test + matrix build on v* tag
```

## Where To Look

| Task | Location | Notes |
|------|----------|-------|
| CLI entry | `crates/pe2-cli/src/main.rs` | clap arg parsing, dispatch |
| CLI args | `crates/pe2-cli/src/args.rs` | Clap derive structs |
| Core engine | `crates/pe2-core/src/engine.rs` | LLM pipeline, refinement loop |
| Config | `crates/pe2-core/src/config.rs` | JSON config at ~/.kleosr-pe2/ |
| Analysis | `crates/pe2-core/src/analysis.rs` | Prompt complexity scoring |
| Providers | `crates/pe2-providers/src/` | 5 adapters (openai, anthropic, google, ollama, openrouter) |
| Provider factory | `crates/pe2-providers/src/factory.rs` | Runtime provider selection |
| Interactive loop | `crates/pe2-tui/src/interactive.rs` | Readline REPL with /commands |
| Display/UI | `crates/pe2-tui/src/display.rs` | Themed output, spinners |
| Session mgmt | `crates/pe2-core/src/session.rs` | Persistent session state |
| Stats | `crates/pe2-core/src/stats.rs` | Usage metrics |
| Preferences | `crates/pe2-core/src/preferences.rs` | User preferences |
| Templates | `crates/pe2-core/src/templates.rs` | PE2 prompt templates |
| Integration tests | `crates/pe2-core/tests/`, `crates/pe2-providers/tests/` | Cargo test |
| npm package | `npm/package.json` | Meta-package with platform optional deps |

## Conventions

- **Rust edition 2021**, crate resolver = "2"
- **snake_case** for functions/variables, **PascalCase** for types/traits/enums, **SCREAMING_SNAKE_CASE** for constants
- **async/await** with **tokio** runtime (full features)
- **anyhow/thiserror** for error handling
- **serde** for all JSON serialization
- **reqwest** with rustls-tls for HTTP
- **Trait-based provider adapter** pattern via `async_trait`
- Tests use standard **`#[cfg(test)]`** + **`cargo test`** — no test framework
- **2-space indent**, no trailing whitespace (Rustfmt default)
- All persistent state written via atomic JSON writes

## Anti-Patterns (This Project)

- **No rustfmt/clippy CI enforcement** — style is convention-only
- **No PR CI** — tests only run on tag push (`v*`)
- **interactive.rs is a god module** — 266 lines doing I/O, config, client creation, pipeline
- **Provider adapter trait is thin** — most logic duplicated across 5 adapters
- **No `pe2-cli` tests** — binary entry point untested
- **npm package published without `files` config** — includes build artifacts
- **Deferred saves** in preferences/stats can lose last write on crash
- **constants.rs is a megamodule** — 129 lines mixing regex patterns, model data, HTTP defaults, tier thresholds
- **Session writes skip atomic write** — uses `std::fs::write` directly, inconsistent with rest of persistence
- **Misleading error mapping** — reqwest errors mapped to `CliError::Json` in all providers
- **No `.rust-toolchain.toml`** — Rust version only documented in README, not pinned

## Commands

```bash
cargo build                  # Build all crates
cargo test                   # Run all tests
cargo run -- --help          # Run CLI
cargo run -- "prompt text"   # One-shot mode
cargo run -- --config        # Config menu
```

## Notes

- Config stored at `~/.kleosr-pe2/` (config.json, preferences.json, stats.json + sessions/)
- Output goes to `./pe2-prompts/` unless `--output-file` given
- npm package at `npm/package.json` is meta-package with optional platform deps (v4.0.1)
- Node >=18.17.0 required for npm install (native addon via napi-rs)
- Binary name: `pe2`
