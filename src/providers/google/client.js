import { GoogleGenerativeAI } from '@google/generative-ai';

export function createGoogleClient(apiKey) {
    if (!apiKey?.trim()) {
        throw new Error('Google API key is required');
    }

    const genAI = new GoogleGenerativeAI(apiKey);

    return {
        chat: {
            completions: {
                create: async (opts) => {
                    const model = genAI.getGenerativeModel({ model: opts.model });
                    const result = await model.generateContent(opts.messages[opts.messages.length - 1].content);

                    return {
                        choices: [{
                            message: {
                                content: result.response.text(),
                                role: 'assistant'
                            },
                            finish_reason: 'stop'
                        }],
                        usage: result.response.usageMetadata || {},
                        model: opts.model
                    };
                }
            }
        }
    };
} 