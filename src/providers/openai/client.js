import OpenAI from 'openai';

export function createOpenAIClient(apiKey, baseURL = 'https://api.openai.com/v1') {
    if (!apiKey?.trim()) {
        throw new Error('OpenAI API key is required');
    }

    return new OpenAI({
        apiKey,
        baseURL,
        defaultHeaders: {
            'HTTP-Referer': 'https://pe2-cli-tool.local',
            'X-Title': 'KleoSr PE2-CLI Tool'
        }
    });
} 