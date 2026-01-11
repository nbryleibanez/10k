<script lang="ts">
  import SessionForm from '$features/sessions/SessionForm.svelte';
  import type { PracticeSession } from '$api/types';
  import { createQuery } from '@tanstack/svelte-query';
  import { apiRequest } from '$api/client';

  const sessionsQuery = createQuery<PracticeSession[]>({
    queryKey: ['sessions', 'goal-demo'],
    queryFn: () => apiRequest({ path: '/api/sessions' })
  });

  $: previousSessions = $sessionsQuery.data ?? [];
</script>

<div class="mx-auto max-w-4xl px-6 py-12">
  <h1 class="text-4xl font-semibold text-brand-12">Sessions</h1>
  <p class="text-brand-10">Log new practice, review reflections, and tag focus areas.</p>
  <div class="mt-8 grid gap-8 lg:grid-cols-[2fr_1fr]">
    <SessionForm goalId="goal-demo" />
    <div class="space-y-4 rounded-2xl border border-brand-6 bg-white/80 p-6">
      <h2 class="text-xl font-semibold text-brand-12">Recent sessions</h2>
      <ul class="space-y-3 text-sm text-brand-11">
        {#each previousSessions as session}
          <li class="rounded-lg border border-brand-5 p-3">
            <p class="font-semibold">{session.duration_minutes} min • {session.tags.join(', ')}</p>
            <p class="text-brand-9">{session.reflection}</p>
          </li>
        {/each}
      </ul>
    </div>
  </div>
</div>
