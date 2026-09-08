<div align="center">
  <img src="https://img.shields.io/badge/rust-1.85+-orange?logo=rust&style=flat-square" />
  <img src="https://img.shields.io/github/v/tag/kleosr/PE2-CLI?style=flat-square&color=blue" />
  <img src="https://img.shields.io/badge/build-passing-brightgreen?style=flat-square" />
  <img src="https://img.shields.io/badge/license-ISC-blue?style=flat-square" />
  <img src="https://img.shields.io/badge/built%20with-Cursor-6c47ff?style=flat-square" />
</div>

<br />

<div align="center">
  <h1>PE²-CLI</h1>
  <p><strong>Structured Prompt Generation — Rust Native</strong></p>
  <p>Drop in a rough prompt. Get back a PE²-structured prompt you can ship.<br />Single binary, no GC, fast startup, async all the way through.</p>
</div>

<br />

---

## Install

```bash
# from source (Rust 1.85+)
cargo install --git https://github.com/kleosr/PE2-CLI

# or grab a release binary
# https://github.com/kleosr/PE2-CLI/releases
```

## Usage

```bash
# interactive REPL
pe2

# one-shot
pe2 "Write a blog post about AI"

# from a file
pe2 path/to/prompt.txt

# override provider, model, and refinement
pe2 "Explain quantum computing" --provider openai --model gpt-4o -i 5 --max-tokens 512 --temperature 0.5

# same REPL as no-args mode
pe2 --config
```

No arguments opens the interactive REPL. Set provider and API key with `/config` in the REPL, or edit `~/.kleosr-pe2/config.json` directly.

Single-shot `-i`, `--max-tokens`, and `--temperature` flags apply directly to the pipeline. Omit them and complexity analysis picks the refinement count.

## Configuration

Config lives at `~/.kleosr-pe2/config.json`. Edit it by hand or use `/config` in the REPL.

| Flag | What it does |
|------|-------------|
| `-p, --provider` | `openai`, `anthropic`, `google`, `openrouter`, `ollama` |
| `-m, --model` | Model ID for the provider |
| `--api-key` | API key (or use the env var below) |
| `-o, --output-file` | Output path |
| `-i, --iterations` | Refinement passes (overrides auto-detection) |
| `--max-tokens` | Max response tokens (default: 1024) |
| `--temperature` | Sampling temperature (default: 0.3) |

### Environment variables

| Provider | Variable |
|----------|----------|
| OpenAI | `OPENAI_API_KEY` |
| Anthropic | `ANTHROPIC_API_KEY` |
| Google | `GOOGLE_API_KEY` |
| OpenRouter | `OPENROUTER_API_KEY` |
| Ollama (local) | `OLLAMA_BASE_URL` |

## How it works

1. **Complexity analysis** scores the prompt on tech terms, domain, structure, logic, and special characters, then maps to a difficulty tier.
2. **Initial LLM call** uses a structured JSON template.
3. **Refinement loop** runs 1–5 passes depending on complexity (or your `-i` override).
4. **Output** lands in `./pe2-prompts/` as markdown with history and metrics.

```json
{
  "context": "…",
  "role": "…",
  "task": "…",
  "constraints": "…",
  "output": "…"
}
```

## Architecture

```
crates/
├── pe2-core/       — config, analysis, engine/pipeline, templates, persistence
├── pe2-providers/  — OpenAI, Anthropic, Google, Ollama, OpenRouter adapters
├── pe2-tui/        — banner, spinner, themed output, interactive REPL
└── pe2-cli/        — clap entry, single-shot and interactive dispatch
```

No circular deps. Single binary.

## Development

```bash
git clone https://github.com/kleosr/PE2-CLI.git
cd PE2-CLI
cargo build
cargo test
cargo run -- --help
```

Integration tests sit under `crates/*/tests/`.

## CI/CD

Tag a release with `v*` and GitHub Actions will:

1. Run `cargo test`
2. Build for linux/darwin/windows (x64 and arm64)
3. Upload release tarballs

## Why Rust?

The first version was Node.js. It worked, but startup sat around 300ms, GC paused during refinement, and concurrency meant callbacks and promises everywhere.

Rust gave a static binary, roughly 2ms startup, no GC, and tokio async. Five provider calls in parallel is routine.

Built with [Cursor](https://cursor.com).

## License

ISC
