export function createOpenRouterClient({ apiKey, baseURL = 'https://openrouter.ai/api/v1' }) {
    if (!apiKey?.trim()) {
        throw new Error('OpenRouter API key is required');
    }

    return {
        chat: {
            completions: {
                create: async (options) => {
                    const response = await fetch(`${baseURL}/chat/completions`, {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                            'Authorization': `Bearer ${apiKey}`,
                            'HTTP-Referer': 'https://pe2-cli-tool.local',
                            'X-Title': 'KleoSr PE2-CLI Tool'
                        },
                        body: JSON.stringify({
                            model: options.model,
                            messages: options.messages,
                            max_tokens: options.max_tokens || 2048,
                            temperature: options.temperature,
                            stream: options.stream
                        })
                    });

                    if (!response.ok) {
                        throw new Error(`OpenRouter error: ${response.status}`);
                    }

                    const data = await response.json();
                    return data;
                }
            }
        }
    };
} 