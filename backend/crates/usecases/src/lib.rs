use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tenk_domain::{
    Circle, CircleId, DomainEvent, Goal, GoalId, LeaderboardEntry, LogSessionInput, Milestone,
    PracticeSession, SessionId, SessionSource, TimerState, UserId,
};

#[derive(Debug, Deserialize)]
pub struct LogPracticeCommand {
    pub goal_id: GoalId,
    pub user_id: UserId,
    pub duration_minutes: u32,
    pub started_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub reflection: Option<String>,
    pub quality_rating: Option<u8>,
    pub source: SessionSource,
}

#[derive(Debug, Serialize)]
pub struct PracticeSessionResponse {
    pub session_id: SessionId,
    pub goal: Goal,
}

#[derive(Debug)]
pub struct UseCaseOutcome<T> {
    pub data: T,
    pub events: Vec<DomainEvent>,
}

impl<T> UseCaseOutcome<T> {
    pub fn new(data: T, events: Vec<DomainEvent>) -> Self {
        Self { data, events }
    }
}

#[async_trait]
pub trait SessionRepository {
    async fn save(&self, session: &PracticeSession) -> Result<(), UseCaseError>;
}

#[async_trait]
pub trait GoalRepository {
    async fn find_by_id(&self, id: &GoalId, user_id: &UserId) -> Result<Option<Goal>, UseCaseError>;
    async fn save(&self, goal: &Goal) -> Result<(), UseCaseError>;
    async fn create(&self, goal: &Goal) -> Result<(), UseCaseError>;
}

#[async_trait]
pub trait TimerStateRepository {
    async fn save(&self, state: &TimerState) -> Result<(), UseCaseError>;
    async fn find(&self, user_id: &UserId, goal_id: &GoalId) -> Result<Option<TimerState>, UseCaseError>;
}

#[async_trait]
pub trait CircleRepository {
    async fn find(&self, id: &CircleId) -> Result<Option<Circle>, UseCaseError>;
    async fn save(&self, circle: &Circle) -> Result<(), UseCaseError>;
}

#[async_trait]
pub trait LeaderboardRepository {
    async fn save_entries(&self, entries: &[LeaderboardEntry]) -> Result<(), UseCaseError>;
    async fn fetch_entries(&self, circle_id: Option<&CircleId>) -> Result<Vec<LeaderboardEntry>, UseCaseError>;
}

#[async_trait]
pub trait StreakRepository {
    async fn record_activity(&self, user_id: &UserId, date: DateTime<Utc>) -> Result<u32, UseCaseError>;
}

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("domain error: {0}")]
    Domain(#[from] tenk_domain::DomainError),
    #[error("goal not found")]
    GoalNotFound,
    #[error("persistence error: {0}")]
    Persistence(String),
    #[error("circle not found")]
    CircleNotFound,
}

pub struct LogPracticeSession<'a, G, S>
where
    G: GoalRepository + Send + Sync,
    S: SessionRepository + Send + Sync,
{
    goals: &'a G,
    sessions: &'a S,
}

impl<'a, G, S> LogPracticeSession<'a, G, S>
where
    G: GoalRepository + Send + Sync,
    S: SessionRepository + Send + Sync,
{
    pub fn new(goals: &'a G, sessions: &'a S) -> Self {
        Self { goals, sessions }
    }

    pub async fn execute(&self, command: LogPracticeCommand) -> Result<UseCaseOutcome<PracticeSessionResponse>, UseCaseError> {
        let mut goal = self
            .goals
            .find_by_id(&command.goal_id, &command.user_id)
            .await?
            .ok_or(UseCaseError::GoalNotFound)?;

        let (session, mut events) = goal.log_session(LogSessionInput {
            goal_id: command.goal_id.clone(),
            user_id: command.user_id.clone(),
            duration_minutes: command.duration_minutes,
            started_at: command.started_at,
            tags: command.tags.clone(),
            reflection: command.reflection.clone(),
            quality_rating: command.quality_rating,
            source: command.source.clone(),
        })?;

        self.sessions.save(&session).await?;
        self.goals.save(&goal).await?;

        if let Some(event) = maybe_streak_event(&command) {
            events.push(event);
        }

        Ok(UseCaseOutcome::new(
            PracticeSessionResponse {
                session_id: session.id,
                goal,
            },
            events,
        ))
    }
}

fn maybe_streak_event(command: &LogPracticeCommand) -> Option<DomainEvent> {
    let midnight = Utc::now().date_naive().and_hms_opt(0, 0, 0)?;
    if command.started_at >= midnight.into() {
        Some(DomainEvent::StreakExtended {
            user_id: command.user_id.clone(),
            days: 1,
        })
    } else {
        None
    }
}

pub struct CreateGoal<'a, G: GoalRepository> {
    repo: &'a G,
}

impl<'a, G: GoalRepository> CreateGoal<'a, G> {
    pub fn new(repo: &'a G) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, mut goal: Goal) -> Result<Goal, UseCaseError> {
        goal.created_at = Utc::now();
        goal.updated_at = goal.created_at;
        self.repo.create(&goal).await?;
        Ok(goal)
    }
}

pub struct TimerUseCase<'a, R: TimerStateRepository> {
    repo: &'a R,
}

impl<'a, R: TimerStateRepository> TimerUseCase<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub async fn start(&self, user_id: UserId, goal_id: GoalId) -> Result<TimerState, UseCaseError> {
        let state = TimerState {
            user_id,
            goal_id,
            started_at: Utc::now(),
            running: true,
        };
        self.repo.save(&state).await?;
        Ok(state)
    }

    pub async fn stop(&self, user_id: &UserId, goal_id: &GoalId) -> Result<Option<TimerState>, UseCaseError> {
        if let Some(mut state) = self.repo.find(user_id, goal_id).await? {
            state.running = false;
            self.repo.save(&state).await?;
            return Ok(Some(state));
        }
        Ok(None)
    }
}

pub struct CircleMembership<'a, C: CircleRepository> {
    repo: &'a C,
}

impl<'a, C: CircleRepository> CircleMembership<'a, C> {
    pub fn new(repo: &'a C) -> Self {
        Self { repo }
    }

    pub async fn join(&self, circle_id: CircleId, user_id: UserId) -> Result<Circle, UseCaseError> {
        let mut circle = self.repo.find(&circle_id).await?.ok_or(UseCaseError::CircleNotFound)?;
        circle.member_ids.insert(user_id);
        self.repo.save(&circle).await?;
        Ok(circle)
    }

    pub async fn leave(&self, circle_id: CircleId, user_id: &UserId) -> Result<Circle, UseCaseError> {
        let mut circle = self.repo.find(&circle_id).await?.ok_or(UseCaseError::CircleNotFound)?;
        circle.member_ids.remove(user_id);
        self.repo.save(&circle).await?;
        Ok(circle)
    }
}

pub struct LeaderboardService<'a, L: LeaderboardRepository> {
    repo: &'a L,
}

impl<'a, L: LeaderboardRepository> LeaderboardService<'a, L> {
    pub fn new(repo: &'a L) -> Self {
        Self { repo }
    }

    pub async fn calculate(
        &self,
        circle_id: Option<CircleId>,
        sessions: &[PracticeSession],
    ) -> Result<Vec<LeaderboardEntry>, UseCaseError> {
        let mut totals = std::collections::HashMap::<UserId, u32>::new();
        for session in sessions {
            *totals.entry(session.user_id.clone()).or_default() += session.duration_minutes;
        }
        let mut entries: Vec<_> = totals
            .into_iter()
            .map(|(user_id, minutes)| LeaderboardEntry {
                user_id,
                display_name: "Practitioner".into(),
                total_minutes: minutes,
                streak_days: 0,
            })
            .collect();
        entries.sort_by(|a, b| b.total_minutes.cmp(&a.total_minutes));
        self.repo.save_entries(&entries).await?;
        let filtered = if let Some(circle_id) = circle_id {
            let _ = circle_id;
            entries.clone()
        } else {
            entries.clone()
        };
        Ok(filtered)
    }

    pub async fn fetch(&self, circle_id: Option<&CircleId>) -> Result<Vec<LeaderboardEntry>, UseCaseError> {
        self.repo.fetch_entries(circle_id).await
    }
}

pub struct StreakService<'a, S: StreakRepository> {
    repo: &'a S,
}

impl<'a, S: StreakRepository> StreakService<'a, S> {
    pub fn new(repo: &'a S) -> Self {
        Self { repo }
    }

    pub async fn record(&self, user_id: UserId) -> Result<u32, UseCaseError> {
        let today = Utc::now();
        self.repo.record_activity(&user_id, today).await
    }

    pub fn check_streak(&self, last_active: DateTime<Utc>) -> u32 {
        let duration = Utc::now() - last_active;
        (duration.num_days() + 1) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct InMemoryGoals(Arc<Mutex<Option<Goal>>>);

    #[async_trait]
    impl GoalRepository for InMemoryGoals {
        async fn find_by_id(&self, _id: &GoalId, _user_id: &UserId) -> Result<Option<Goal>, UseCaseError> {
            Ok(self.0.lock().await.clone())
        }

        async fn save(&self, goal: &Goal) -> Result<(), UseCaseError> {
            *self.0.lock().await = Some(goal.clone());
            Ok(())
        }

        async fn create(&self, goal: &Goal) -> Result<(), UseCaseError> {
            *self.0.lock().await = Some(goal.clone());
            Ok(())
        }
    }

    struct InMemorySessions;

    #[async_trait]
    impl SessionRepository for InMemorySessions {
        async fn save(&self, _session: &PracticeSession) -> Result<(), UseCaseError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn updates_goal_with_new_session() {
        let goal = Goal {
            id: GoalId::default(),
            user_id: UserId::default(),
            title: "Piano".into(),
            target_hours: 10000,
            completed_minutes: 0,
            milestones: vec![],
            achievements: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let goals = InMemoryGoals(Arc::new(Mutex::new(Some(goal.clone()))));
        let sessions = InMemorySessions;
        let usecase = LogPracticeSession::new(&goals, &sessions);

        let outcome = usecase
            .execute(LogPracticeCommand {
                goal_id: goal.id.clone(),
                user_id: goal.user_id.clone(),
                duration_minutes: 90,
                started_at: Utc::now(),
                tags: vec!["scales".into()],
                reflection: Some("dialed in".into()),
                quality_rating: Some(4),
                source: SessionSource::Manual,
            })
            .await
            .unwrap();

        assert_eq!(outcome.data.goal.completed_minutes, 90);
        assert!(outcome.data.session_id.0.len() > 0);
        assert!(!outcome.events.is_empty());
    }

    #[test]
    fn streak_check_counts_days() {
        let repo = ();
        struct Dummy;
        #[async_trait]
        impl StreakRepository for Dummy {
            async fn record_activity(&self, _user_id: &UserId, _date: DateTime<Utc>) -> Result<u32, UseCaseError> {
                Ok(1)
            }
        }
        let service = StreakService::new(&Dummy);
        let days = service.check_streak(Utc::now() - Duration::days(3));
        assert_eq!(days, 4);
    }
}
