import { clientEnv } from '$config/env';
import { QueryClient } from '@tanstack/svelte-query';

export const queryClient = new QueryClient();

type HttpMethod = 'GET' | 'POST' | 'PATCH' | 'PUT' | 'DELETE';

export interface ApiRequestOptions<TBody = unknown> {
  path: string;
  method?: HttpMethod;
  body?: TBody;
  token?: string;
}

const normalizedBase =
  typeof window !== 'undefined' && clientEnv.apiBaseUrl === ''
    ? ''
    : clientEnv.apiBaseUrl?.replace(/\/$/, '') ?? '';

function buildUrl(path: string) {
  if (/^https?:/i.test(path)) {
    return path;
  }
  if (normalizedBase.length === 0) {
    return path;
  }
  return `${normalizedBase}${path}`;
}

export async function apiRequest<TResponse, TBody = unknown>({
  path,
  method = 'GET',
  body,
  token
}: ApiRequestOptions<TBody>): Promise<TResponse> {
  const response = await fetch(buildUrl(path), {
    method,
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { Authorization: `Bearer ${token}` } : {})
    },
    body: body ? JSON.stringify(body) : undefined
  });

  if (!response.ok) {
    const text = await response.text();
    throw new Error(text || 'API request failed');
  }

  return (await response.json()) as TResponse;
}
