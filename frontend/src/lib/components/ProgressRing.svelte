<script lang="ts">
  export let progress = 0; // 0-1
  export let label = 'Progress';
  export let hours = 0;

  const radius = 80;
  const circumference = 2 * Math.PI * radius;
  const offset = circumference - progress * circumference;
</script>

<div class="flex flex-col items-center gap-3">
  <svg width="200" height="200" class="drop-shadow-sm">
    <circle
      class="text-brand-4"
      stroke-width="12"
      stroke="currentColor"
      fill="transparent"
      r={radius}
      cx="100"
      cy="100"
    />
    <circle
      class="text-accent-9"
      stroke-width="12"
      stroke-linecap="round"
      stroke="currentColor"
      fill="transparent"
      r={radius}
      cx="100"
      cy="100"
      style={`stroke-dasharray:${circumference} ${circumference};stroke-dashoffset:${offset};`}
    />
    <text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" class="text-2xl font-semibold">
      {(progress * 100).toFixed(0)}%
    </text>
  </svg>
  <div class="text-center">
    <div class="text-sm uppercase tracking-wide text-brand-9">{label}</div>
    <div class="text-lg font-medium">{hours.toFixed(1)} hrs logged</div>
  </div>
</div>
