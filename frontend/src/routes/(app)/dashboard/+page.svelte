<script lang="ts">
  import GoalOverview from '$features/goals/GoalOverview.svelte';
  import SessionForm from '$features/sessions/SessionForm.svelte';
  import LeaderboardPanel from '$features/leaderboard/LeaderboardPanel.svelte';
  import Timer from '$components/Timer.svelte';
  import { useGoal } from '$api/goals';
  import { useLeaderboard } from '$api/leaderboard';
  import { createQuery } from '@tanstack/svelte-query';
  import { apiRequest } from '$api/client';
  import type { PracticeSession } from '$api/types';

  const goalQuery = useGoal('goal-demo');
  const leaderboardQuery = useLeaderboard();
  const sessionsQuery = createQuery<PracticeSession[]>({
    queryKey: ['sessions', 'goal-demo'],
    queryFn: () => apiRequest({ path: '/api/sessions' })
  });

  $: goal = $goalQuery.data ?? null;
  $: leaderboard = $leaderboardQuery.data ?? [];
  $: recentSessions = $sessionsQuery.data ?? [];
</script>

<div class="mx-auto flex max-w-6xl flex-col gap-8 px-6 py-12">
  <div class="flex flex-col gap-3">
    <p class="text-sm uppercase tracking-wide text-brand-9">Dashboard</p>
    <h1 class="text-4xl font-semibold text-brand-12">Welcome back!</h1>
  </div>
  <div class="grid gap-8 lg:grid-cols-[2fr_1fr]">
    <GoalOverview {goal} />
    <Timer goalId={goal?.id ?? null} />
  </div>
  <div class="grid gap-8 lg:grid-cols-2">
    <SessionForm goalId={goal?.id ?? ''} />
    <LeaderboardPanel entries={leaderboard} />
  </div>
  <div class="rounded-2xl border border-brand-6 bg-white p-6 shadow">
    <h2 class="text-xl font-semibold text-brand-12">Recent sessions</h2>
    <ul class="mt-4 space-y-3 text-sm text-brand-11">
      {#each recentSessions.slice(0, 5) as session}
        <li class="rounded-lg border border-brand-5 p-3">
          <p class="font-semibold">{session.duration_minutes} min • {session.tags.join(', ')}</p>
          {#if session.reflection}
            <p class="text-brand-9">{session.reflection}</p>
          {/if}
        </li>
      {:else}
        <li class="rounded-lg border border-dashed border-brand-5 p-3 text-brand-9">No sessions yet.</li>
      {/each}
    </ul>
  </div>
</div>
