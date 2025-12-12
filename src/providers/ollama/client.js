export function createOllamaClient(baseURL = 'http://localhost:11434') {
  const base = baseURL.replace(/\/$/, '');

  async function create({ model, messages, max_tokens = 2048, stream = false }) {
    const response = await fetch(`${base}/api/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        model,
        messages,
        stream,
        options: { num_predict: max_tokens }
      })
    });

    if (!response.ok) {
      throw new Error(`Ollama request failed: ${response.status} ${response.statusText}`);
    }

    const chatResponse = await response.json();
    const content = chatResponse?.message?.content ?? '';

    return {
      choices: [
        {
          message: { content }
        }
      ]
    };
  }

  return {
    chat: {
      completions: {
        create
      }
    }
  };
} 