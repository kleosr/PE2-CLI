import { HTTP_HEADERS } from '../constants.js';

export function buildOpenRouterStyleHeaders(extraHeaders = {}) {
  return {
    'HTTP-Referer': HTTP_HEADERS.referer,
    'X-Title': HTTP_HEADERS.title,
    ...extraHeaders
  };
}
