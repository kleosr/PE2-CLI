# pe2-providers — LLM Adapters

**9 modules, ~560 LOC, 5 adapters**

Runtime-switchable provider implementations using a trait-based adapter pattern.

## Structure

```
src/
├── client.rs       (114L) — Provider trait definition + HTTP client setup
├── factory.rs      (16L)  — Runtime selection from config string
├── openai.rs       (74L)  — OpenAI adapter
├── anthropic.rs    (91L)  — Anthropic/Claude adapter
├── google.rs       (85L)  — Google Gemini adapter
├── ollama.rs       (68L)  — Local Ollama adapter
├── openrouter.rs   (70L)  — OpenRouter adapter
├── headers.rs      (34L)  — Request header helpers
└── lib.rs          (8L)   — Crate root, re-exports
```

## Provider Trait

Defined in `client.rs`. Each adapter implements the same async trait with `send_prompt()` method. Factory in `factory.rs` maps provider string → boxed adapter.

## Conventions

- All adapters normalize response to `ProviderResponse` struct
- API keys from env vars (OPENAI_API_KEY, ANTHROPIC_API_KEY, etc.)
- reqwest with rustls-tls for HTTPS

## Anti-Patterns

- **Thin trait = duplication** — most logic (HTTP call, JSON parsing, error mapping) repeated across all 5 adapters
- **No unit tests per adapter** — only one integration test file for all
- **Ollama adapter mixes HTTP + local process concerns**
