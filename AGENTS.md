# AGENTS.md

Repo-owned agent handbook for PE²-CLI (`pe2`). Single source of truth across coding agents.
CLI that turns raw prompts into PE²-structured prompts via LLM providers (OpenAI, Anthropic, Google, OpenRouter, Ollama). Rust 2021 workspace (`4.0.2`), binary `pe2`, tokio + reqwest (rustls).

## Rules

### Toolchain & MSRV
- Rust edition 2021, MSRV 1.85.0 (`.rust-toolchain.toml`, `.github/workflows/ci.yml`).
- PR CI checks: `cargo test --workspace`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`.
- Pinned workspace crates for MSRV (`rules-backlog.md`, `Cargo.toml`): `comfy-table = "=7.1.4"`, lockfile `idna_adapter 1.2.0`.
- Do not add dependencies or syntax incompatible with Rust 1.85.0.

### Codebase Conventions
- Error handling: `anyhow` at CLI/TUI boundary; `thiserror` (`pe2_core::errors::CliError`) in core.
- Persistence: JSON files via `write_atomic` (config, preferences, stats). REPL session history is in-memory only.
- DAG: `cli → {core, providers, tui}` · `tui → {core, providers}` · `providers → core`. Respect crate boundaries; no circular dependencies.
- Pipeline integration: Providers implement `pe2_core::engine::EngineLlmProvider` directly; callers (CLI/TUI) use `pe2_providers::runner::run_pipeline`. Do not wire providers by hand in CLI/TUI.
- Secrets: Never persist API keys in `~/.kleosr-pe2/config.json` (in-memory session or env vars only: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GOOGLE_API_KEY`, `OPENROUTER_API_KEY`, `OLLAMA_BASE_URL`).

### Package Metadata
- Root `package.json` is private metadata (`@kleosr/pe2-cli` 4.0.2). Install via `cargo install` or GitHub release binaries.
- Default prompt output lands in `./pe2-prompts/` (ignored by git) unless `--output-file` is specified.

## Skills

Reusable task recipes go in `.agents/skills`.
None currently exist in this repository. Add recipes to `.agents/skills/<skill-name>/` when reusable task procedures are introduced.

## Workflows

### Build & Test
- Build workspace: `cargo build`
- Run workspace tests: `cargo test --workspace` (or `cargo test`)
- Run crate-specific tests:
  - `cargo test -p pe2-core`
  - `cargo test -p pe2-providers`
  - `cargo test -p pe2-cli`
  - `cargo test -p pe2-tui`
- Linter / Clippy: `cargo clippy --all-targets -- -D warnings`
- Code formatting check: `cargo fmt --check`
- Format code: `cargo fmt`

### Running the CLI
- One-shot generation: `cargo run -- "prompt text"`
- Interactive REPL / configuration: `cargo run --` or `cargo run -- --config`
- Help: `cargo run -- --help`

### Submodule & Crate Navigation
- CLI binary / modes: `crates/pe2-cli/src/main.rs`, `src/args.rs` (adapter: `crates/pe2-cli/AGENTS.md`)
- Core engine / pipeline: `crates/pe2-core/src/engine.rs`, `src/analysis.rs`, `src/config.rs` (adapter: `crates/pe2-core/AGENTS.md`)
- Providers / adapters: `crates/pe2-providers/src/client.rs`, `src/runner.rs` (adapter: `crates/pe2-providers/AGENTS.md`)
- Terminal UI / interactive REPL: `crates/pe2-tui/src/interactive/`, `src/prompt_flow.rs` (adapter: `crates/pe2-tui/AGENTS.md`)
- CI & Release: `.github/workflows/ci.yml`, `.github/workflows/publish.yml`

## Memory

Project memory and context are maintained in versioned markdown files under the repository root:
- `README.md`: Architecture overview, CLI flags, supported providers, environment variables, and usage.
- `rules-backlog.md`: Workspace status, MSRV notes, and dependency pin constraints.
- Crate-level sibling guides: `crates/pe2-*/AGENTS.md` (scoped module maps extending this handbook).
This repository does not use a `docs/` directory or vendor-specific memory systems; all persistent context lives in tracked markdown files.
