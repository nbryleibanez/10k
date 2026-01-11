import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { leaderboard } from '../data';

export const GET: RequestHandler = async () => {
  return json(leaderboard);
};
