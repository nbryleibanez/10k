import { createQuery } from '@tanstack/svelte-query';
import type { Goal } from '$api/types';
import { apiRequest } from './client';

export const goalKeys = {
  detail: (id: string) => ['goals', id] as const
};

export function useGoal(goalId: string) {
  return createQuery<Goal>({
    queryKey: goalKeys.detail(goalId),
    queryFn: () => apiRequest({ path: `/api/goals/${goalId}` }),
    staleTime: 60_000
  });
}
