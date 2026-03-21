# PE2-CLI

Command-line tool that takes a rough prompt (text or file), calls a configured LLM, and returns a structured PE2-style prompt. Release line is **4.0.0** (npm semver); the CLI labels the generation as **Code V4** in the banner.

Visit at: https://www.npmjs.com/package/@kleosr/pe2-cli

## Requirements

- Node.js 18.17 or newer

You need an API key (or base URL for Ollama) for whichever provider you pick. The tool does not ship bundled models.

## Install

```bash
npm install -g @kleosr/pe2-cli
```

Or run without a global install:

```bash
npx @kleosr/pe2-cli --help
```

## Usage

No arguments starts interactive mode (and will ask for config on first run if nothing is saved):

```bash
npx @kleosr/pe2-cli
```

Other entry points:

```bash
npx @kleosr/pe2-cli --config
npx @kleosr/pe2-cli "Your prompt as plain text"
npx @kleosr/pe2-cli path/to/prompt.txt
npx @kleosr/pe2-cli "Some text" --iterations 3
npx @kleosr/pe2-cli "Some text" --provider ollama --model llama3
npx @kleosr/pe2-cli "Some text" --auto-difficulty
```

For the full flag list, use:

```bash
npx @kleosr/pe2-cli --help
```

## Configuration

Settings are stored in `~/.kleosr-pe2/config.json` (Unix) or the equivalent under your user profile on Windows. Run `--config` to change provider, model, and API key.

Supported providers in code: `openai`, `anthropic`, `google`, `openrouter`, `ollama`. Default in the project constants targets OpenRouter with a small model id; you can override per run with `--provider` and `--model`.

## Output

Unless you pass `--output-file`, session output can be written under `pe2-prompts/` in the current working directory (see `src/paths.js` and `src/engine.js`).

## Development

```bash
git clone https://github.com/kleosr/PE2-CLI.git
cd PE2-CLI
npm ci
npm test
npm start
```

Tests use Node’s built-in runner: `node --test ./tests/*.test.js`.

## CI (npm publish)

Pushing a tag `v*` runs `.github/workflows/publish.yml`. That job needs a GitHub Actions secret named `NPM_TOKEN` (granular npm token with publish access to this package).

From a machine where you are logged into GitHub: install [GitHub CLI](https://cli.github.com/) (`winget install GitHub.cli`), run `gh auth login -h github.com -s repo` once, then:

```powershell
.\scripts\set-npm-token-for-actions.ps1
```

It only sends the token to GitHub’s API; it does not store it in the repo.

## Package

- npm: [@kleosr/pe2-cli](https://www.npmjs.com/package/@kleosr/pe2-cli)
- Source: [github.com/kleosr/PE2-CLI](https://github.com/kleosr/PE2-CLI)

## License

ISC (see `package.json`).
