// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  namespace App {
    interface Locals {
      user: {
        id: string;
      } | null;
    }
    interface PageState {
      environment: import('$config/env').ClientEnv;
    }
    // interface Platform {}
  }
}

export {}; 
