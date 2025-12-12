import OpenAI from 'openai';
import { HTTP_HEADERS } from '../../constants.js';

export function createOpenAIClient(apiKey, baseURL = 'https://api.openai.com/v1') {
    if (!apiKey?.trim()) {
        throw new Error('OpenAI API key is required');
    }

    return new OpenAI({
        apiKey,
        baseURL,
        defaultHeaders: {
            'HTTP-Referer': HTTP_HEADERS.referer,
            'X-Title': HTTP_HEADERS.title
        }
    });
} 