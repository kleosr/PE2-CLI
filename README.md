# PE²-CLI

A rough prompt is hard to hand to someone else. `pe2` sends it to one configured model and writes a PE² prompt back: context, role, task, constraints, and output.

Version 4.0.2. Rust 2021, minimum toolchain 1.85.0.

## What it does

- Scores the raw prompt and picks 1–5 refinement passes from that score, unless you pass `-i`.
- Asks the model for JSON with those five fields, then refines that JSON.
- Writes a markdown file with the prompt, the pass history, and the run metrics.
- Offers a REPL for config, session history, preferences, and stats.

It does not run the prompt it writes. One run talks to one provider, one pass after another.

## Install

From a checkout:

```bash
cargo build --release -p pe2-cli
# binary: target/release/pe2
```

From the git repo (the workspace crate that contains the binary is `pe2-cli`):

```bash
cargo install --git https://github.com/kleosr/PE2-CLI pe2-cli
```

Tag `v*` and [`.github/workflows/publish.yml`](.github/workflows/publish.yml) builds release tarballs for Linux, macOS, and Windows, x64 and arm64.

## Usage

```bash
pe2 "Write a blog post about AI"
pe2 path/to/prompt.txt
pe2 "Explain quantum computing" --provider openai --model gpt-4o -i 5 --max-tokens 512 --temperature 0.5
pe2
pe2 --config
```

If the argument is an existing file, that file is the prompt. If it is not a file, the argument is the prompt text, including a path that is not there.

No prompt, or `--config`, opens the REPL. `--config` does that even when a prompt is also present.

| Flag | Effect |
|------|--------|
| `-p, --provider` | `openai`, `anthropic`, `google`, `openrouter`, `ollama` |
| `-m, --model` | Model id for that provider |
| `--api-key` | Key for this run |
| `-o, --output-file` | Output path |
| `-i, --iterations` | Pass count. Overrides the score. Values below 1 still run one pass |
| `--max-tokens` | Response token cap. Default 1024 |
| `--temperature` | Sampling temperature. Default 0.3 |

Defaults when you have not saved a config: provider `openrouter`, model `openai/gpt-4o-mini`.

REPL commands: `/help`, `/config`, `/session`, `/prefs`, `/stats`, `/clear`, `/exit`. Short aliases are `/h`, `/c`, `/s`, `/p`, `/q`.

## Configuration

Files live under `.kleosr-pe2/` in your home directory.

`config.json` stores provider and model. `/config` writes that file. The API key you type there stays in the process; the saver does not put `api_key` in the file. For a real run the key comes from `--api-key`, that in-memory session, or the env var below. Ollama does not use a key.

| Provider | Variable |
|----------|----------|
| OpenAI | `OPENAI_API_KEY` |
| Anthropic | `ANTHROPIC_API_KEY` |
| Google | `GOOGLE_API_KEY` |
| OpenRouter | `OPENROUTER_API_KEY` |
| Ollama | `OLLAMA_BASE_URL` (default `http://localhost:11434`) |

`preferences.json` holds `track_usage` (default true). `stats.json` holds usage counts. REPL session history is memory only.

Without `-o`, output goes to `./pe2-prompts/pe2-session-<id>.md`. A relative `-o` is resolved from the current directory. A path that contains `..` is rejected.

## How it works

1. Reject the prompt if it is empty, shorter than 10 characters, or longer than 10000, after trimming.
2. Score wording, tech and domain terms, structure, logic words, and a few special characters. The score is capped at 20 and mapped to a difficulty and a pass count from 1 to 5.
3. Call the model with a JSON template, then refine. Each HTTP call times out after 30 seconds.
4. If the model text is not JSON, or JSON wrapped in other text cannot be parsed, write a fallback structure instead of failing the file.
5. If a later pass fails, keep the last good prompt, note the error, and still write the file.

```json
{
  "context": "…",
  "role": "…",
  "task": "…",
  "constraints": "…",
  "output": "…"
}
```

## Layout

```text
crates/pe2-cli         clap entry: REPL or one shot
crates/pe2-core        analysis, pipeline, templates, config, stats
crates/pe2-providers   one HTTP client for the five providers
crates/pe2-tui         banner, spinner, REPL
```

Call direction is `pe2-cli` to the others, `pe2-tui` to core and providers, `pe2-providers` to core. The module map for agents is [`AGENTS.md`](AGENTS.md).

## Verification

CI on pull requests and pushes to `main` runs:

```bash
cargo test --workspace
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Toolchain file: [`.rust-toolchain.toml`](.rust-toolchain.toml).

## Limitations

- The written prompt is only as good as the model response. A non-JSON reply still produces a file, via the fallback structure.
- Passes are sequential and are not retried. A failed later pass stops the loop. The file written at the end is the last prompt that parsed.
- Prompt length is 10–10000 characters. Output paths may not contain `..`.
- Ollama is a local HTTP server. The other providers need a key in the environment, the flag, or the REPL session.
- This repo does not publish an npm package. `package.json` is metadata only.

## License

ISC, as declared in `Cargo.toml`.
