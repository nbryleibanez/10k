import { browser } from '$app/environment';
import { writable, type Writable } from 'svelte/store';

export type TimerState = {
  running: boolean;
  startedAt: number | null;
  elapsedSeconds: number;
  goalId: string | null;
};

const STORAGE_KEY = 'tenk-timer-state';
const defaultState: TimerState = { running: false, startedAt: null, elapsedSeconds: 0, goalId: null };

function loadInitialState(): TimerState {
  if (!browser) return defaultState;
  try {
    const parsed = JSON.parse(localStorage.getItem(STORAGE_KEY) || 'null');
    if (!parsed) return defaultState;
    return {
      ...defaultState,
      ...parsed
    };
  } catch {
    return defaultState;
  }
}

function persist(store: Writable<TimerState>) {
  if (!browser) return;
  store.subscribe((state) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
  });
}

function setupTicker(store: Writable<TimerState>) {
  if (!browser) return;
  const interval = setInterval(() => {
    store.update((state) => {
      if (!state.running || !state.startedAt) return state;
      const elapsed = Math.floor((Date.now() - state.startedAt) / 1000);
      return { ...state, elapsedSeconds: elapsed };
    });
  }, 1000);
  return () => clearInterval(interval);
}

function createTimerStore() {
  const initial = loadInitialState();
  const store = writable<TimerState>(initial);
  const stopTicker = setupTicker(store);
  persist(store);

  return {
    subscribe: store.subscribe,
    start(goalId?: string) {
      const started = Date.now();
      store.set({ running: true, startedAt: started, elapsedSeconds: 0, goalId: goalId ?? null });
    },
    stop() {
      store.update((state) => ({
        ...state,
        running: false,
        startedAt: null,
        elapsedSeconds: state.elapsedSeconds
      }));
    },
    reset() {
      store.set(defaultState);
    },
    destroy() {
      stopTicker?.();
    }
  };
}

export const timerStore = createTimerStore();
