<script lang="ts">
  import { createForm } from 'felte';
  import { validator } from '@felte/validator-zod';
  import { z } from 'zod';
  import { tweened } from 'svelte/motion';
  import { cubicOut } from 'svelte/easing';
  import { useSessionMutation } from '$api/sessions';

  const schema = z.object({
    durationMinutes: z.preprocess((val) => Number(val ?? 0), z.number().min(15, 'Minimum 15 minutes')),
    reflection: z.string().max(500).optional(),
    tags: z.string().optional(),
    qualityRating: z.preprocess((val) => Number(val ?? 3), z.number().min(1).max(5))
  });

  const progress = tweened(0, { duration: 400, easing: cubicOut });

  export let goalId: string | null = null;
  let mutation = useSessionMutation(goalId ?? 'goal-demo');
  $: if (goalId) {
    mutation = useSessionMutation(goalId);
  }

  const { form, data, isSubmitting } = createForm<z.infer<typeof schema>>({
    extend: [validator({ schema })],
    onSubmit: async (values) => {
      progress.set(0.8);
      await mutation.mutateAsync({
        goalId,
        durationMinutes: values.durationMinutes,
        tags: values.tags ? values.tags.split(',').map((tag) => tag.trim()) : [],
        reflection: values.reflection,
        qualityRating: values.qualityRating
      });
      progress.set(1);
    },
    onSuccess: () => {
      progress.set(0);
    },
    onError: () => {
      progress.set(0);
    }
  });
</script>

<form use:form class="space-y-4 rounded-2xl border border-brand-6 bg-white p-6 shadow">
  <div>
    <label for="duration" class="text-sm font-medium text-brand-11">Duration (minutes)</label>
    <input
      name="durationMinutes"
      id="duration"
      type="number"
      class="mt-1 w-full rounded-lg border-brand-5 focus:border-accent-8 focus:ring-accent-8"
      placeholder="90"
      value="90"
    />
  </div>
  <div>
    <label for="tags" class="text-sm font-medium text-brand-11">Tags</label>
    <input
      name="tags"
      id="tags"
      type="text"
      class="mt-1 w-full rounded-lg border-brand-5 focus:border-accent-8 focus:ring-accent-8"
      placeholder="scales, improvisation"
    />
  </div>
  <div>
    <label for="reflection" class="text-sm font-medium text-brand-11">Reflection</label>
    <textarea
      name="reflection"
      id="reflection"
      rows="4"
      class="mt-1 w-full rounded-lg border-brand-5 focus:border-accent-8 focus:ring-accent-8"
      placeholder="What went well? What needs work?"
    ></textarea>
  </div>
  <div>
    <label for="quality" class="text-sm font-medium text-brand-11">Quality rating (1-5)</label>
    <input
      name="qualityRating"
      id="quality"
      type="number"
      class="mt-1 w-full rounded-lg border-brand-5 focus:border-accent-8 focus:ring-accent-8"
      placeholder="4"
      value="4"
    />
  </div>
  <button
    type="submit"
    class="w-full rounded-full bg-brand-12 py-3 font-semibold text-white transition hover:bg-brand-11"
    disabled={$isSubmitting}
  >
    {$isSubmitting ? 'Logging…' : 'Log session'}
  </button>
  <div class="h-1 rounded-full bg-brand-4">
    <div class="h-full rounded-full bg-accent-9" style={`width: ${$progress * 100}%`}></div>
  </div>
</form>
