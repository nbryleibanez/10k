use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tenk_domain::{Goal, GoalId, PracticeSession, UserId};
use tenk_usecases::{GoalRepository, SessionRepository, UseCaseError};
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct InMemoryGoalRepository {
    inner: Arc<RwLock<HashMap<String, Goal>>>,
}

#[async_trait]
impl GoalRepository for InMemoryGoalRepository {
    async fn find_by_id(&self, id: &GoalId, _user_id: &UserId) -> Result<Option<Goal>, UseCaseError> {
        let store = self.inner.read().await;
        Ok(store.get(&id.0).cloned())
    }

    async fn save(&self, goal: &Goal) -> Result<(), UseCaseError> {
        let mut store = self.inner.write().await;
        store.insert(goal.id.0.clone(), goal.clone());
        Ok(())
    }

    async fn create(&self, goal: &Goal) -> Result<(), UseCaseError> {
        self.save(goal).await
    }
}

#[derive(Clone, Default)]
pub struct InMemorySessionRepository {
    inner: Arc<RwLock<Vec<PracticeSession>>>,
}

#[async_trait]
impl SessionRepository for InMemorySessionRepository {
    async fn save(&self, session: &PracticeSession) -> Result<(), UseCaseError> {
        let mut store = self.inner.write().await;
        store.push(session.clone());
        Ok(())
    }
}
