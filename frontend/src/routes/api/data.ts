import type { Goal, LeaderboardEntry, PracticeSession } from '$api/types';

export const sampleGoal: Goal = {
  id: 'goal-demo',
  user_id: 'user-demo',
  title: 'Become a jazz pianist',
  target_hours: 10000,
  completed_minutes: 1264 * 60,
  milestones: [
    {
      id: 'm1',
      goal_id: 'goal-demo',
      target_hours: 1500,
      due_date: null,
      completed_at: null
    }
  ],
  achievements: [],
  created_at: new Date().toISOString(),
  updated_at: new Date().toISOString()
};

export const leaderboard: LeaderboardEntry[] = [
  { user_id: '1', display_name: 'Avery', total_minutes: 2100 * 60, streak_days: 45 },
  { user_id: '2', display_name: 'Reese', total_minutes: 1840 * 60, streak_days: 37 },
  { user_id: '3', display_name: 'Kai', total_minutes: 1622 * 60, streak_days: 19 }
];

export const sessions: PracticeSession[] = [
  {
    id: 's1',
    goal_id: sampleGoal.id,
    user_id: sampleGoal.user_id,
    started_at: new Date().toISOString(),
    duration_minutes: 95,
    tags: ['timing'],
    reflection: 'Locked in the groove.',
    quality_rating: 4
  }
];
