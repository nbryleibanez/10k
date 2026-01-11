import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';

const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter,
    alias: {
      $components: 'src/lib/components',
      $features: 'src/lib/features',
      $stores: 'src/lib/stores',
      $api: 'src/lib/api',
      $config: 'src/lib/config'
    }
  }
};

export default config;
