<script lang="ts">
  import LeaderboardTable from '$components/LeaderboardTable.svelte';
  import type { LeaderboardEntry } from '$api/types';

  export let entries: LeaderboardEntry[] = [];
</script>

<section class="rounded-3xl border border-brand-6 bg-white/90 p-6 shadow">
  <div class="mb-4 flex items-center justify-between">
    <div>
      <p class="text-sm uppercase tracking-wide text-brand-9">Weekly Leaderboard</p>
      <h3 class="text-2xl font-semibold text-brand-12">Deep Work Pace</h3>
    </div>
    <button class="rounded-full border border-brand-8 px-4 py-2 text-sm font-medium text-brand-11">
      Join circle
    </button>
  </div>
  <LeaderboardTable {entries} />
</section>
