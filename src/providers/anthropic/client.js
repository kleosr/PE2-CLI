import Anthropic from '@anthropic-ai/sdk';

export function createAnthropicClient(apiKey, customOptions = {}) {
    if (!apiKey || typeof apiKey !== 'string' || apiKey.trim().length === 0) {
        throw new Error('Anthropic API key is required and must be a non-empty string');
    }

    const anthropic = new Anthropic({ 
        apiKey,
        ...customOptions
    });

    return {
        chat: {
            completions: {
                async create(opts) {
                    if (!opts.model || typeof opts.model !== 'string') {
                        throw new Error('Anthropic model parameter is required and must be a string');
                    }

                    if (!opts.messages || !Array.isArray(opts.messages) || opts.messages.length === 0) {
                        throw new Error('Anthropic messages parameter is required and must be a non-empty array');
                    }

                    for (const message of opts.messages) {
                        if (!message.role || !['system', 'user', 'assistant'].includes(message.role)) {
                            throw new Error(`Invalid message role: ${message.role}. Must be 'system', 'user', or 'assistant'`);
                        }
                        if (!message.content || typeof message.content !== 'string') {
                            throw new Error('Message content is required and must be a string');
                        }
                    }

                    const systemMessages = opts.messages.filter(m => m.role === 'system');
                    const conversationMessages = opts.messages.filter(m => m.role !== 'system');

                    if (conversationMessages.length > 0 && conversationMessages[0].role !== 'user') {
                        throw new Error('Anthropic conversation must start with a user message');
                    }

                    const systemPrompt = systemMessages.length > 0 
                        ? systemMessages.map(m => m.content).join('\n\n')
                        : null;

                    const anthropicParams = {
                        model: opts.model,
                        messages: conversationMessages,
                        max_tokens: opts.max_tokens || 2048,
                        ...(systemPrompt && { system: systemPrompt }),
                        ...(opts.temperature !== undefined && { 
                            temperature: Math.max(0, Math.min(1, opts.temperature)) 
                        }),
                        ...(opts.top_p !== undefined && { 
                            top_p: Math.max(0, Math.min(1, opts.top_p)) 
                        }),
                        ...(opts.stop_sequences && Array.isArray(opts.stop_sequences) && { 
                            stop_sequences: opts.stop_sequences.slice(0, 4) // Max 4 stop sequences
                        }),
                        ...(opts.stream && { stream: opts.stream })
                    };

                    try {
                        const response = await anthropic.messages.create(anthropicParams);
                        
                        let text = '';
                        if (Array.isArray(response.content)) {
                            text = response.content
                                .filter(block => block.type === 'text')
                                .map(block => block.text || '')
                                .join('');
                        } else if (response.content?.text) {
                            text = response.content.text;
                        }

                        return {
                            choices: [{
                                message: { 
                                    content: text,
                                    role: 'assistant'
                                },
                                finish_reason: response.stop_reason || 'stop'
                            }],
                            usage: response.usage || {
                                prompt_tokens: 0,
                                completion_tokens: 0,
                                total_tokens: 0
                            },
                            model: response.model || opts.model
                        };
                    } catch (error) {
                        if (error.status === 401) {
                            throw new Error('Anthropic authentication failed. Please check your API key.');
                        } else if (error.status === 429) {
                            const retryAfter = error.headers?.['retry-after'];
                            const message = retryAfter 
                                ? `Anthropic rate limit exceeded. Retry after ${retryAfter} seconds.`
                                : 'Anthropic rate limit exceeded. Please try again later.';
                            throw new Error(message);
                        } else if (error.status === 400) {
                            throw new Error(`Anthropic request error: ${error.message || 'Invalid request parameters'}`);
                        } else if (error.status === 404) {
                            throw new Error(`Anthropic model not found: ${opts.model}. Please check the model name.`);
                        } else if (error.status === 413) {
                            throw new Error('Anthropic request too large. Please reduce the input size.');
                        } else if (error.status >= 500) {
                            throw new Error('Anthropic server error. Please try again later.');
                        } else if (error.type === 'invalid_request_error') {
                            throw new Error(`Anthropic invalid request: ${error.message}`);
                        } else if (error.type === 'authentication_error') {
                            throw new Error('Anthropic authentication error. Please verify your API key.');
                        } else {
                            throw new Error(`Anthropic API error: ${error.message || 'Unknown error occurred'}`);
                        }
                    }
                }
            }
        }
    };
} 