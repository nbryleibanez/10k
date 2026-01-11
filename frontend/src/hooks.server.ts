import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
  const userId = event.cookies.get('tenk_user');
  event.locals.user = userId
    ? {
        id: userId
      }
    : null;
  return resolve(event);
};
