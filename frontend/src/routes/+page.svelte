<script lang="ts">
  import GoalOverview from '$features/goals/GoalOverview.svelte';
  import LeaderboardPanel from '$features/leaderboard/LeaderboardPanel.svelte';
  import Timer from '$components/Timer.svelte';
  import { useGoal } from '$api/goals';
  import { useLeaderboard } from '$api/leaderboard';

  const goalQuery = useGoal('goal-demo');
  const leaderboardQuery = useLeaderboard();

  $: goal = $goalQuery.data ?? null;
  $: leaderboard = $leaderboardQuery.data ?? [];
</script>

<section class="relative overflow-hidden bg-gradient-to-b from-brand-1 to-brand-3">
  <div class="mx-auto flex max-w-6xl flex-col gap-12 px-6 py-24">
    <div class="grid items-center gap-10 md:grid-cols-2">
      <div class="space-y-6">
        <p class="inline-flex rounded-full border border-brand-7 px-3 py-1 text-xs uppercase tracking-wide text-brand-10">
          Put in the 10,000 hours
        </p>
        <h1 class="text-5xl font-semibold text-brand-12">
          Track every deliberate hour on your journey to mastery.
        </h1>
        <p class="text-lg text-brand-11">
          10k keeps your goal, streak, and accountability circle in one place with deep-work timers,
          milestone celebrations, and signals that keep you moving for years—not days.
        </p>
        <div class="flex flex-wrap gap-4">
          <a
            class="rounded-full bg-accent-9 px-6 py-3 text-white shadow-lg transition hover:bg-accent-10"
            href="/app"
          >
            Launch app
          </a>
          <button class="rounded-full border border-brand-8 px-6 py-3 text-brand-11">
            Watch demo
          </button>
        </div>
      </div>
      <div class="grid gap-6">
        <GoalOverview {goal} />
        <Timer goalId={goal?.id ?? null} />
      </div>
    </div>
    <LeaderboardPanel entries={leaderboard} />
  </div>
</section>
