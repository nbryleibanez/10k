use ammonia::Builder as AmmoniaBuilder;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(pub String);

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GoalId(pub String);

impl Default for GoalId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SessionId(pub String);

impl Default for SessionId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MilestoneId(pub String);

impl Default for MilestoneId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CircleId(pub String);

impl Default for CircleId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AchievementId(pub String);

impl Default for AchievementId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub email: bool,
    pub push: bool,
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self { email: true, push: false }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub display_name: String,
    pub email: String,
    pub timezone: String,
    pub notification_preferences: NotificationPreferences,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: MilestoneId,
    pub goal_id: GoalId,
    pub target_hours: u32,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub id: CircleId,
    pub name: String,
    pub description: Option<String>,
    pub member_ids: HashSet<UserId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: AchievementId,
    pub goal_id: GoalId,
    pub user_id: UserId,
    pub title: String,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub user_id: UserId,
    pub entity_type: String,
    pub entity_id: String,
    pub action: AuditAction,
    pub previous_value: Option<serde_json::Value>,
    pub new_value: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HourCount(pub u32);

impl HourCount {
    pub fn new(hours: u32) -> Result<Self, DomainError> {
        if hours > 100_000 {
            return Err(DomainError::InvalidHourCount);
        }
        Ok(Self(hours))
    }

    pub fn from_minutes(minutes: u32) -> Self {
        Self(minutes / 60)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Goal {
    pub id: GoalId,
    pub user_id: UserId,
    pub title: String,
    pub target_hours: u32,
    pub completed_minutes: u32,
    pub milestones: Vec<Milestone>,
    pub achievements: Vec<Achievement>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Goal {
    pub fn progress(&self) -> f32 {
        if self.target_hours == 0 {
            return 0.0;
        }
        self.completed_minutes as f32 / (self.target_hours as f32 * 60.0)
    }

    pub fn log_session(
        &mut self,
        input: LogSessionInput,
    ) -> Result<(PracticeSession, Vec<DomainEvent>), DomainError> {
        if self.is_completed() {
            return Err(DomainError::GoalAlreadyCompleted);
        }

        let session = PracticeSession::new(
            input.goal_id.clone(),
            input.user_id.clone(),
            input.duration_minutes,
            input.started_at,
            input.tags,
            input.reflection,
            input.quality_rating,
            input.source,
        )?;

        self.completed_minutes += session.duration_minutes;
        self.updated_at = Utc::now();

        let mut events = vec![DomainEvent::SessionLogged {
            session_id: session.id.clone(),
            goal_id: session.goal_id.clone(),
            duration_minutes: session.duration_minutes,
        }];

        let milestone_event = self.check_milestones();
        if let Some(milestone) = milestone_event {
            events.push(DomainEvent::MilestoneReached {
                goal_id: self.id.clone(),
                milestone,
            });
        }

        Ok((session, events))
    }

    fn is_completed(&self) -> bool {
        self.completed_minutes >= self.target_hours * 60
    }

    fn check_milestones(&mut self) -> Option<Milestone> {
        for milestone in self.milestones.iter_mut() {
            if milestone.completed_at.is_none()
                && self.completed_minutes >= milestone.target_hours * 60
            {
                milestone.completed_at = Some(Utc::now());
                return Some(milestone.clone());
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogSessionInput {
    pub goal_id: GoalId,
    pub user_id: UserId,
    pub duration_minutes: u32,
    pub started_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub reflection: Option<String>,
    pub quality_rating: Option<u8>,
    pub source: SessionSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionSource {
    Timer,
    Manual,
    Integration,
}

impl Default for SessionSource {
    fn default() -> Self {
        SessionSource::Manual
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeSession {
    pub id: SessionId,
    pub goal_id: GoalId,
    pub user_id: UserId,
    pub started_at: DateTime<Utc>,
    pub duration_minutes: u32,
    pub tags: Vec<String>,
    pub reflection: Option<String>,
    pub quality_rating: Option<u8>,
    pub source: SessionSource,
}

impl PracticeSession {
    pub fn new(
        goal_id: GoalId,
        user_id: UserId,
        duration_minutes: u32,
        started_at: DateTime<Utc>,
        tags: Vec<String>,
        reflection: Option<String>,
        quality_rating: Option<u8>,
        source: SessionSource,
    ) -> Result<Self, DomainError> {
        if duration_minutes == 0 || duration_minutes > (24 * 60) {
            return Err(DomainError::InvalidDuration);
        }

        if started_at > Utc::now() + Duration::minutes(5) {
            return Err(DomainError::FutureSessionNotAllowed);
        }

        if let Some(rating) = quality_rating {
            if rating == 0 || rating > 5 {
                return Err(DomainError::InvalidQualityRating);
            }
        }

        Ok(Self {
            id: SessionId::default(),
            goal_id,
            user_id,
            started_at,
            duration_minutes,
            tags,
            reflection: reflection.map(sanitize_reflection),
            quality_rating,
            source,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub user_id: UserId,
    pub display_name: String,
    pub total_minutes: u32,
    pub streak_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub user_id: UserId,
    pub goal_id: GoalId,
    pub started_at: DateTime<Utc>,
    pub running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    SessionLogged {
        session_id: SessionId,
        goal_id: GoalId,
        duration_minutes: u32,
    },
    MilestoneReached {
        goal_id: GoalId,
        milestone: Milestone,
    },
    StreakExtended {
        user_id: UserId,
        days: u32,
    },
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid duration")]
    InvalidDuration,
    #[error("goal already completed")]
    GoalAlreadyCompleted,
    #[error("goal not found")]
    GoalNotFound,
    #[error("future session not allowed")]
    FutureSessionNotAllowed,
    #[error("invalid quality rating")]
    InvalidQualityRating,
    #[error("invalid hour count")]
    InvalidHourCount,
}

fn sanitize_reflection(input: String) -> String {
    let cleaned = AmmoniaBuilder::default()
        .tags(HashSet::new())
        .clean(&input)
        .to_string();
    cleaned.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_duration() {
        let goal_id = GoalId::default();
        let user_id = UserId::default();
        let result = PracticeSession::new(
            goal_id,
            user_id,
            0,
            Utc::now(),
            vec![],
            None,
            None,
            SessionSource::Manual,
        );
        assert!(matches!(result, Err(DomainError::InvalidDuration)));
    }

    #[test]
    fn sanitizes_reflection_text() {
        let goal_id = GoalId::default();
        let user_id = UserId::default();
        let session = PracticeSession::new(
            goal_id,
            user_id,
            60,
            Utc::now(),
            vec![],
            Some("<script>alert('x')</script>Great session".into()),
            Some(4),
            SessionSource::Manual,
        )
        .unwrap();
        assert_eq!(session.reflection.unwrap(), "alert('x')Great session");
    }
}
