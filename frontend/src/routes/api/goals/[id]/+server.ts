import type { RequestHandler } from '@sveltejs/kit';
import { json } from '@sveltejs/kit';
import { sampleGoal } from '../../data';

export const GET: RequestHandler = async ({ params }) => {
  if (params.id !== sampleGoal.id) {
    return json({ error: 'Not Found' }, { status: 404 });
  }
  return json(sampleGoal);
};
