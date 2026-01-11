import type { RequestHandler } from './$types';
import { redirect } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ cookies }) => {
  cookies.delete('tenk_user', { path: '/' });
  throw redirect(303, '/login');
};
