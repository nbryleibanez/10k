<script lang="ts">
  import '../app.css';
  import { QueryClientProvider } from '@tanstack/svelte-query';
  import { queryClient } from '$api/client';
  import { onMount } from 'svelte';
  import { clientEnv } from '$config/env';
  import { browser } from '$app/environment';
  import { initAuth } from '$lib/auth/cognito';

  onMount(async () => {
    await initAuth();
    if (browser && clientEnv.sentryDsn) {
      const { init } = await import('@sentry/sveltekit');
      init({ dsn: clientEnv.sentryDsn, tracesSampleRate: 0.1 });
    }
  });
</script>

<QueryClientProvider client={queryClient}>
  <slot />
</QueryClientProvider>
