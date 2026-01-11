import { describe, expect, it, vi } from 'vitest';
import { apiRequest } from './client';

vi.stubGlobal('fetch', vi.fn());

describe('apiRequest', () => {
  it('throws on non-OK responses', async () => {
    (fetch as unknown as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      ok: false,
      text: () => Promise.resolve('boom')
    });

    await expect(apiRequest({ path: '/test' })).rejects.toThrow('boom');
  });
});
