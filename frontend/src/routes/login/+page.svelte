<script lang="ts">
  export let form;
</script>

<div class="mx-auto max-w-md px-6 py-16">
  <h1 class="text-3xl font-semibold text-brand-12">Sign in</h1>
  <p class="text-brand-10">Use your email to start a mock session.</p>
  <form method="POST" class="mt-6 space-y-4">
    <div>
      <label for="email" class="text-sm font-medium text-brand-11">Email</label>
      <input
        type="email"
        id="email"
        name="email"
        required
        class="mt-1 w-full rounded-lg border-brand-5 focus:border-accent-8 focus:ring-accent-8"
      />
    </div>
    <button
      type="submit"
      class="w-full rounded-full bg-brand-12 py-3 font-semibold text-white transition hover:bg-brand-11"
    >
      Continue
    </button>
    {#if form?.error}
      <p class="text-sm text-red-600">{form.error}</p>
    {/if}
  </form>
</div>
