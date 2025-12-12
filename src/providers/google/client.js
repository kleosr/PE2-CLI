import { GoogleGenerativeAI } from '@google/generative-ai';

export function createGoogleClient(apiKey) {
    if (!apiKey?.trim()) {
        throw new Error('Google API key is required');
    }

    const genAI = new GoogleGenerativeAI(apiKey);

    const roleLabels = {
        system: 'System',
        user: 'User',
        assistant: 'Assistant'
    };

    return {
        chat: {
            completions: {
                create: async (options) => {
                    const model = genAI.getGenerativeModel({ model: options.model });
                    const prompt = (options.messages ?? [])
                        .map(m => `${roleLabels[m.role] ?? 'User'}: ${m.content}`)
                        .join('\n');

                    const generation = await model.generateContent(prompt);

                    return {
                        choices: [{
                            message: {
                                content: generation.response.text(),
                                role: 'assistant'
                            },
                            finish_reason: 'stop'
                        }],
                        usage: generation.response.usageMetadata || {},
                        model: options.model
                    };
                }
            }
        }
    };
} 