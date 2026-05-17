<div align="center">
  <img src="https://img.shields.io/badge/rust-1.81+-orange?logo=rust&style=flat-square" />
  <img src="https://img.shields.io/github/v/release/kleosr/PE2-CLI?style=flat-square&color=blue" />
  <img src="https://img.shields.io/github/actions/workflow/status/kleosr/PE2-CLI/publish.yml?branch=main&style=flat-square" />
  <img src="https://img.shields.io/github/license/kleosr/PE2-CLI?style=flat-square" />
  <img src="https://img.shields.io/badge/built%20with-Cursor-6c47ff?style=flat-square" />
</div>

<br />

<div align="center">
  <h1>PE²-CLI</h1>
  <p><strong>Structured Prompt Generation — Rust Native</strong></p>
  <p>Give it a rough idea. Get back a production-ready PE²-structured prompt.<br />Zero GC. True async. Sub-millisecond startup.</p>
</div>

<br />

---

## 📦 Install

```bash
# Via npm (meta-package, no postinstall scripts)
npm install -g @kleosr/pe2-cli

# From source (requires Rust 1.81+)
cargo install --git https://github.com/kleosr/PE2-CLI

# Or grab a binary from the releases page
# https://github.com/kleosr/PE2-CLI/releases
```

## 🚀 Usage

```bash
# Interactive mode — just run it
pe2

# One-shot
pe2 "Write a blog post about AI"

# From a file
pe2 path/to/prompt.txt

# With overrides
pe2 "Explain quantum computing" --provider openai --model gpt-4o --iterations 5

# Config menu
pe2 --config
```

No arguments = interactive mode. First run will prompt you to set up your provider and API key.

## ⚙️ Configuration

Settings live in `~/.kleosr-pe2/config.json`. Change them through the interactive menu (`/config`) or edit the file directly.

| Flag | What it does |
|------|-------------|
| `-p, --provider` | LLM provider: `openai`, `anthropic`, `google`, `openrouter`, `ollama` |
| `-m, --model` | Model identifier (check your provider's docs) |
| `--api-key` | Your API key (or set the env var below) |
| `-o, --output-file` | Where to save the result |
| `-i, --iterations` | Refinement pass count (auto-detected by default) |
| `--max-tokens` | Max response tokens (default: 1024) |
| `--temperature` | Sampling temperature (default: 0.3) |

### Environment Variables

| Provider | Variable |
|----------|----------|
| OpenAI | `OPENAI_API_KEY` |
| Anthropic | `ANTHROPIC_API_KEY` |
| Google | `GOOGLE_API_KEY` |
| OpenRouter | `OPENROUTER_API_KEY` |
| Ollama (local) | `OLLAMA_BASE_URL` |

## 🔄 How It Works

1. **Complexity analysis** — scores your prompt on 5 factors (tech, domain, structure, logic, special chars), maps to difficulty tiers
2. **LLM call** — sends it with a structured JSON template
3. **Refinement loop** — auto-detects complexity; simple prompts get 1 pass, technical deep-dives get up to 5
4. **Output** — writes to `./pe2-prompts/` as markdown with full history and metrics

```json
{
  "context": "…",
  "role": "…",
  "task": "…",
  "constraints": "…",
  "output": "…"
}
```

Every prompt is returned as a structured PE² JSON document.

## 🏗️ Architecture

```
crates/
├── pe2-core/       — config, analysis, engine/pipeline, templates
├── pe2-providers/  — 5 adapters: OpenAI, Anthropic, Google, Ollama, OpenRouter
├── pe2-tui/        — banner, spinner, themed display, interactive REPL (crossterm)
├── pe2-cli/        — binary entry: clap args, single-prompt + interactive modes
└── pe2-bindings/   — napi-rs bridge (optional, Node.js native addon)
```

Clean dependency chain — no circular deps. ~3,200 lines of Rust across 36 source files.

## 🛠️ Development

```bash
git clone https://github.com/kleosr/PE2-CLI.git
cd PE2-CLI
cargo build
cargo test
cargo run -- --help
```

Tests live in `crates/*/tests/`. Run with `cargo test`.

## 🤖 CI/CD

Push a `v*` tag and GitHub Actions:
1. Runs `cargo test`
2. Matrix builds across 6 platforms (linux x64/arm64, darwin x64/arm64, windows x64/arm64)
3. Uploads release tarballs

## 🦀 Why Rust?

I wrote the first version in Node.js. It worked, but:
- **~300ms startup** waiting for the runtime to warm up
- **GC pauses** during prompt refinement
- **Callbacks and promises** for concurrency

Rust fixed all of it. Single static binary, ~2ms startup, zero GC, and tokio async everywhere. Five concurrent provider calls? No problem.

Built with [Cursor](https://cursor.com).

## 📄 License

ISC
