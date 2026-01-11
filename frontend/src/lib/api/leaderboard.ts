import { createQuery } from '@tanstack/svelte-query';
import type { LeaderboardEntry } from '$api/types';
import { apiRequest } from './client';

export const leaderboardKeys = {
  weekly: ['leaderboard', 'weekly'] as const
};

export function useLeaderboard() {
  return createQuery<LeaderboardEntry[]>({
    queryKey: leaderboardKeys.weekly,
    queryFn: () => apiRequest({ path: '/api/leaderboard' }),
    staleTime: 30_000
  });
}
