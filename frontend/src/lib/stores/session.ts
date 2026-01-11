import { readable, writable } from 'svelte/store';

export type AuthUser = {
  username: string;
  email?: string;
} | null;

export type AuthState = {
  user: AuthUser;
  accessToken?: string;
  idToken?: string;
};

const initial: AuthState = { user: null };
export const authStore = writable<AuthState>(initial);

export const isAuthenticated = readable(false, (set) => {
  const unsubscribe = authStore.subscribe((state) => {
    set(Boolean(state.user));
  });
  return unsubscribe;
});
