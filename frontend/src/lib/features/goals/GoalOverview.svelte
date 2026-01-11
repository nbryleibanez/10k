<script lang="ts">
  import ProgressRing from '$components/ProgressRing.svelte';
  import type { Goal } from '$api/types';
  import { goalProgress } from '$api/types';

  export let goal: Goal | null = null;

  $: stats = goal ? goalProgress(goal) : null;
</script>

{#if goal}
  <div class="grid gap-8 rounded-3xl border border-brand-6 bg-white/70 p-8 shadow">
    <div class="flex flex-col gap-2">
      <p class="text-sm uppercase tracking-wide text-brand-9">Primary Goal</p>
      <h2 class="text-3xl font-semibold text-brand-12">{goal.title}</h2>
    </div>
    <div class="flex flex-wrap items-center gap-10">
      <ProgressRing
        progress={stats?.progress ?? 0}
        hours={stats?.completedHours ?? 0}
        label="Total"
      />
      <div class="space-y-3 text-brand-11">
        <p><span class="font-semibold">Target:</span> {stats?.targetHours.toLocaleString()} hrs</p>
        <p><span class="font-semibold">Next milestone:</span> {stats?.nextMilestoneHours} hrs</p>
        <p>
          <span class="font-semibold">Remaining:</span>
          {(stats ? stats.targetHours - stats.completedHours : 0).toFixed(1)} hrs
        </p>
      </div>
    </div>
  </div>
{:else}
  <div class="rounded-3xl border border-dashed border-brand-7 p-8 text-center text-brand-9">
    You have not configured a goal yet. Create one to start tracking toward 10,000 hours.
  </div>
{/if}
