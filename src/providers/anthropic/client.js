import Anthropic from '@anthropic-ai/sdk';

export function createAnthropicClient(apiKey) {
    if (!apiKey?.trim()) {
        throw new Error('Anthropic API key is required');
    }

    const client = new Anthropic({ apiKey });

    return {
        chat: {
            completions: {
                create: (params) => client.messages.create({
                    model: params.model,
                    messages: params.messages.filter(m => m.role !== 'system'),
                    system: params.messages.filter(m => m.role === 'system').map(m => m.content).join('\n\n') || undefined,
                    max_tokens: params.max_tokens || 2048,
                    temperature: params.temperature,
                    stream: params.stream
                }).then(response => ({
                    choices: [{
                        message: {
                            content: response.content.filter(block => block.type === 'text').map(block => block.text).join(''),
                            role: 'assistant'
                        },
                        finish_reason: response.stop_reason || 'stop'
                    }],
                    usage: response.usage,
                    model: response.model
                }))
            }
        }
    };
} 