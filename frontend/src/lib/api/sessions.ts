import { createMutation } from '@tanstack/svelte-query';
import type { PracticeSession } from '$api/types';
import { apiRequest, queryClient } from './client';

export type SessionInput = {
  goalId: string;
  durationMinutes: number;
  tags?: string[];
  reflection?: string;
  qualityRating?: number;
};

export function useSessionMutation(goalId: string) {
  return createMutation({
    mutationFn: (input: SessionInput) =>
      apiRequest<PracticeSession>({
        path: '/api/sessions',
        method: 'POST',
        body: {
          goal_id: goalId,
          duration_minutes: input.durationMinutes,
          tags: input.tags ?? [],
          reflection: input.reflection,
          quality_rating: input.qualityRating ?? 3
        }
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['goals', goalId] });
      queryClient.invalidateQueries({ queryKey: ['sessions', goalId] });
    }
  });
}
