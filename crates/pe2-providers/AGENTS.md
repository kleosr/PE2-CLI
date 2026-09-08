@AGENTS.md

# pe2-providers — LLM Adapters

**Parent:** `AGENTS.md`
**Scope:** `crates/pe2-providers/`

5 adapters implement `pe2_core::engine::EngineLlmProvider` directly; `run_pipeline` + `client::create_client` wire them into `Pipeline`.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Kind/config | `src/client.rs` | `ProviderKind`, `ProviderConfig`, `create_client` |
| Runner | `src/runner.rs` | `run_pipeline` (Ollama reads `OLLAMA_BASE_URL`) |
| HTTP | `src/http.rs` | Shared `post_json` / status helpers |
| Headers | `src/headers.rs` | Bearer / OpenRouter / Google / Anthropic |
| Adapters | `src/{openai,anthropic,google,ollama,openrouter}.rs` | Provider bodies |

## Tests

```bash
cargo test -p pe2-providers
```
