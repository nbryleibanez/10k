import type { LayoutLoad } from './$types';
import { clientEnv } from '$config/env';

export const load: LayoutLoad = () => {
  return {
    environment: clientEnv
  };
};
