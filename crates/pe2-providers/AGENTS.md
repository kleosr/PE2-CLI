# pe2-providers — LLM Adapters

**Parent:** `AGENTS.md`
**Scope:** `crates/pe2-providers/`

**9 modules, ~560 LOC, 5 adapters** — runtime-switchable provider implementations.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Trait | `src/client.rs` (114L) | `LlmClient` trait + `ProviderConfig`, `ProviderKind` |
| Factory | `src/factory.rs` (16L) | `ProviderKind` → boxed adapter |
| OpenAI | `src/openai.rs` (74L) | OpenAI adapter |
| Anthropic | `src/anthropic.rs` (91L) | Claude adapter |
| Google | `src/google.rs` (85L) | Gemini adapter |
| Ollama | `src/ollama.rs` (68L) | Local Ollama adapter |
| OpenRouter | `src/openrouter.rs` (70L) | OpenRouter adapter |
| Headers | `src/headers.rs` (34L) | Bearer/OpenRouter header builders |
| Integration tests | `tests/integration.rs` (187L) | Shared test for all adapters |

## Provider Trait

`LlmClient::chat()` in `client.rs`. Factory maps `ProviderKind` → `OpenAIClient` / `AnthropicClient` / etc.

## Conventions (delta)

- All adapters return `ProviderResponse` struct
- API keys from env vars (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, etc.)
- Each adapter: `reqwest` POST → status check → provider-specific JSON field extraction

## Anti-Patterns

- **Duplicated HTTP flow** — same send/parse/error pattern in all 5 adapters
- **Misleading errors** — `json().await` failure mapped to `CliError::Json` (not network)
- **No per-adapter unit tests** — one integration file only
- **Ollama mixes HTTP + local process concerns**

## Tests

```bash
cargo test -p pe2-providers
```
