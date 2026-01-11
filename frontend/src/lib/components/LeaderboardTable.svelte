<script lang="ts">
  import type { LeaderboardEntry } from '$api/types';

  export let entries: LeaderboardEntry[] = [];
</script>

<table class="min-w-full divide-y divide-brand-6">
  <thead class="bg-brand-3 text-left text-xs uppercase tracking-wide text-brand-10">
    <tr>
      <th class="px-4 py-3">Rank</th>
      <th class="px-4 py-3">Practitioner</th>
      <th class="px-4 py-3">Total Hours</th>
      <th class="px-4 py-3">Streak</th>
    </tr>
  </thead>
  <tbody class="divide-y divide-brand-4 bg-white/80 text-sm">
    {#each entries as entry, index}
      <tr>
        <td class="px-4 py-3 font-semibold text-brand-11">#{index + 1}</td>
        <td class="px-4 py-3">{entry.display_name}</td>
        <td class="px-4 py-3 tabular-nums">{(entry.total_minutes / 60).toFixed(1)}</td>
        <td class="px-4 py-3">{entry.streak_days} days</td>
      </tr>
    {:else}
      <tr>
        <td colspan="4" class="px-4 py-6 text-center text-brand-9">No entries yet.</td>
      </tr>
    {/each}
  </tbody>
</table>
