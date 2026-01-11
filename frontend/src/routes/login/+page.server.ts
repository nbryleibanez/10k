import type { Actions } from './$types';
import { redirect } from '@sveltejs/kit';

export const actions: Actions = {
  default: async ({ request, cookies }) => {
    const data = await request.formData();
    const email = data.get('email');
    if (!email || typeof email !== 'string') {
      return { error: 'Email is required' };
    }

    cookies.set('tenk_user', email, {
      httpOnly: true,
      path: '/',
      maxAge: 60 * 60 * 24 * 7,
      sameSite: 'lax'
    });

    throw redirect(303, '/dashboard');
  }
};
