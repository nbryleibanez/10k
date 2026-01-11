export type Milestone = {
  id: string;
  goal_id: string;
  target_hours: number;
  due_date?: string;
  completed_at?: string;
};

export type Goal = {
  id: string;
  user_id: string;
  title: string;
  target_hours: number;
  completed_minutes: number;
  milestones: Milestone[];
  achievements: { id: string; title?: string }[];
  created_at: string;
  updated_at: string;
};

export type PracticeSession = {
  id: string;
  goal_id: string;
  user_id: string;
  started_at: string;
  duration_minutes: number;
  tags: string[];
  reflection?: string;
  quality_rating?: number;
};

export type LeaderboardEntry = {
  user_id: string;
  display_name: string;
  total_minutes: number;
  streak_days: number;
};

export function goalProgress(goal: Goal) {
  const completedHours = goal.completed_minutes / 60;
  const target = goal.target_hours;
  const fallback = goal.milestones.length ? goal.milestones[goal.milestones.length - 1] : undefined;
  const nextMilestone = goal.milestones.find((m) => !m.completed_at) ?? fallback;

  return {
    completedHours,
    targetHours: target,
    nextMilestoneHours: nextMilestone?.target_hours ?? target,
    progress: Math.min(1, completedHours / target)
  };
}
