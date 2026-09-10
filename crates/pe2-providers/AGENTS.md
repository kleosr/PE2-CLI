@AGENTS.md

# pe2-providers — LLM Adapters

**Parent:** `AGENTS.md`
**Scope:** `crates/pe2-providers/`

One `adapters::Client` implements `pe2_core::engine::EngineLlmProvider` for all 5 kinds; `run_pipeline` + `client::create_client` wire it into `Pipeline`.

## Where To Look

| Module | File | Role |
|--------|------|------|
| Kind/config | `src/client.rs` | `ProviderKind`, `ProviderConfig`, `create_client` |
| Runner | `src/runner.rs` | `run_pipeline` (Ollama reads `OLLAMA_BASE_URL`) |
| HTTP | `src/http.rs` | Shared `post_json` / status / `need_key` / `ptr` helpers |
| Adapters | `src/adapters.rs` | `Client`, `headers(kind, key)`, per-kind URL/body/extract table |

## Tests

```bash
cargo test -p pe2-providers
```
