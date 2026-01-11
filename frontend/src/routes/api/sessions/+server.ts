import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { sessions } from '../data';

export const GET: RequestHandler = async () => {
  return json(sessions);
};

export const POST: RequestHandler = async ({ request }) => {
  const body = await request.json();
  const session = {
    id: `session-${Date.now()}`,
    goal_id: body.goal_id,
    user_id: 'user-demo',
    started_at: body.started_at ?? new Date().toISOString(),
    duration_minutes: body.duration_minutes,
    tags: body.tags ?? [],
    reflection: body.reflection,
    quality_rating: body.quality_rating ?? 3
  };
  sessions.unshift(session);
  return json(session, { status: 201 });
};
