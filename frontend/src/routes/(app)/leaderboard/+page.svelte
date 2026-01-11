<script lang="ts">
  import LeaderboardPanel from '$features/leaderboard/LeaderboardPanel.svelte';
  import { useLeaderboard } from '$api/leaderboard';

  const leaderboardQuery = useLeaderboard();
  $: entries = $leaderboardQuery.data ?? [];
</script>

<div class="mx-auto max-w-4xl px-6 py-12">
  <div class="mb-8 flex flex-col gap-2">
    <p class="text-sm uppercase tracking-wide text-brand-9">Community</p>
    <h1 class="text-4xl font-semibold text-brand-12">Leaderboard</h1>
    <p class="text-brand-10">
      Opt into a circle to compare pace, celebrate streaks, and set up practice sprints.
    </p>
  </div>
  <LeaderboardPanel entries={entries} />
</div>
