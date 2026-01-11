import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig(() => {
  return {
    plugins: [sveltekit()],
    build: {
      sourcemap: true
    },
    test: {
      environment: 'jsdom',
      include: ['src/**/*.{test,spec}.{ts,js}'],
      coverage: {
        reporter: ['text', 'html']
      }
    }
  };
});
