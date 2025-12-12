import Anthropic from '@anthropic-ai/sdk';

export function createAnthropicClient(apiKey) {
    if (!apiKey?.trim()) {
        throw new Error('Anthropic API key is required');
    }

    const client = new Anthropic({ apiKey });

    return {
        chat: {
            completions: {
                create: async (options) => {
                    const messages = [];
                    const systemParts = [];

                    for (const message of options.messages ?? []) {
                        if (message.role === 'system') {
                            systemParts.push(message.content);
                            continue;
                        }
                        messages.push(message);
                    }

                    const response = await client.messages.create({
                        model: options.model,
                        messages,
                        system: systemParts.length ? systemParts.join('\n\n') : undefined,
                        max_tokens: options.max_tokens || 2048,
                        temperature: options.temperature,
                        stream: options.stream
                    });

                    const content = (response.content ?? [])
                        .filter(block => block.type === 'text')
                        .map(block => block.text)
                        .join('');

                    return {
                        choices: [{
                            message: {
                                content,
                                role: 'assistant'
                            },
                            finish_reason: response.stop_reason || 'stop'
                        }],
                        usage: response.usage,
                        model: response.model
                    };
                }
            }
        }
    };
} 