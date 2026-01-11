import { getCurrentUser, signIn as cognitoSignIn, signOut as cognitoSignOut, fetchAuthSession } from '@aws-amplify/auth';
import { authStore } from '$stores/session';
import { clientEnv } from '$config/env';
import { configureAmplify } from '$config/amplify';

let configured = false;

function ensureAmplifyConfigured() {
  if (!configured) {
    configureAmplify();
    configured = true;
  }
}

export async function initAuth() {
  if (clientEnv.mockAuth) {
    authStore.set({ user: null });
    return;
  }
  ensureAmplifyConfigured();
  try {
    const user = await getCurrentUser();
    const session = await fetchAuthSession();
    authStore.set({
      user: { username: user.username },
      accessToken: session.tokens?.accessToken?.toString(),
      idToken: session.tokens?.idToken?.toString()
    });
  } catch (error) {
    authStore.set({ user: null });
  }
}

export async function signIn(username: string, password: string) {
  ensureAmplifyConfigured();
  return cognitoSignIn({ username, password });
}

export async function signOut() {
  ensureAmplifyConfigured();
  await cognitoSignOut();
  authStore.set({ user: null });
}
