<script lang="ts">
  import { page } from '$app/stores';
  import { derived } from 'svelte/store';

  const links = [
    { href: '/dashboard', label: 'Dashboard' },
    { href: '/sessions', label: 'Sessions' },
    { href: '/leaderboard', label: 'Leaderboard' }
  ];

  const currentPath = derived(page, ($page) => $page.url.pathname);
</script>

<div class="min-h-screen bg-brand-2">
  <header class="border-b border-brand-5 bg-white/90">
    <nav class="mx-auto flex max-w-6xl items-center justify-between px-6 py-4">
      <a class="text-xl font-semibold text-brand-12" href="/">10k</a>
      <div class="flex gap-4 text-sm font-medium">
        {#each links as link}
          <a
            class="rounded-full px-4 py-2"
            class:font-semibold={$currentPath.startsWith(link.href)}
            href={link.href}
          >
            {link.label}
          </a>
        {/each}
      </div>
      <div class="flex items-center gap-3">
        <span class="text-sm text-brand-11">Signed in</span>
        <a class="rounded-full bg-brand-12 px-4 py-2 text-sm text-white" href="/logout">Logout</a>
      </div>
    </nav>
  </header>
  <slot />
</div>
