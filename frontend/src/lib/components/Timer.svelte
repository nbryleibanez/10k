<script lang="ts">
  import { createEventDispatcher, onDestroy } from 'svelte';
  import { timerStore, type TimerState } from '$stores/timer';

  export let goalId: string | null = null;
  const dispatch = createEventDispatcher<{ start: void; stop: number }>();

  let state: TimerState = { running: false, startedAt: null, elapsedSeconds: 0, goalId: null };
  const unsubscribe = timerStore.subscribe((value) => {
    state = value;
  });

  const toggle = () => {
    if (state.running) {
      timerStore.stop();
      dispatch('stop', state.elapsedSeconds);
      return;
    }
    timerStore.start(goalId ?? undefined);
    dispatch('start');
  };

  onDestroy(() => {
    unsubscribe();
  });

  const formatted = () => {
    const hours = Math.floor(state.elapsedSeconds / 3600)
      .toString()
      .padStart(2, '0');
    const minutes = Math.floor((state.elapsedSeconds % 3600) / 60)
      .toString()
      .padStart(2, '0');
    const seconds = Math.floor(state.elapsedSeconds % 60)
      .toString()
      .padStart(2, '0');
    return `${hours}:${minutes}:${seconds}`;
  };
</script>

<div class="flex flex-col gap-4 rounded-2xl border border-brand-6 bg-white/70 p-6 shadow-sm">
  <div class="text-center text-sm uppercase tracking-wide text-brand-8">Deep Practice Timer</div>
  <div class="text-center text-5xl font-semibold tabular-nums">{formatted()}</div>
  <button
    class="rounded-full bg-accent-9 px-6 py-3 text-white transition hover:bg-accent-10"
    on:click={toggle}
  >
    {state.running ? 'Stop Session' : 'Start Session'}
  </button>
</div>
