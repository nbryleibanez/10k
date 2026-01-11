use async_trait::async_trait;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use aws_sdk_eventbridge::types::PutEventsRequestEntry;
use aws_sdk_eventbridge::Client as EventBridgeClient;
use tenk_domain::{DomainEvent, Goal, GoalId, PracticeSession, SessionId, UserId};
use tenk_usecases::{GoalRepository, SessionRepository, UseCaseError};
use thiserror::Error;
use tracing::instrument;

#[derive(Clone)]
pub struct DynamoRepositories {
    pub table_name: String,
    pub client: Client,
}

impl DynamoRepositories {
    pub fn new(table_name: impl Into<String>, client: Client) -> Self {
        Self {
            table_name: table_name.into(),
            client,
        }
    }

    fn goal_pk(user_id: &UserId) -> String {
        format!("USER#{}", user_id.0)
    }

    fn goal_sk(goal_id: &GoalId) -> String {
        format!("GOAL#{}", goal_id.0)
    }

    fn session_pk(goal_id: &GoalId) -> String {
        format!("GOAL#{}", goal_id.0)
    }

    fn session_sk(session_id: &SessionId) -> String {
        format!("SESSION#{}", session_id.0)
    }

    fn leaderboard_sk(goal: &Goal) -> String {
        format!("HOURS#{}", goal.completed_minutes)
    }
}

#[async_trait]
impl GoalRepository for DynamoRepositories {
    #[instrument(skip(self))]
    async fn find_by_id(&self, goal_id: &GoalId, user_id: &UserId) -> Result<Option<Goal>, UseCaseError> {
        let response = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(Self::goal_pk(user_id)))
            .key("sk", AttributeValue::S(Self::goal_sk(goal_id)))
            .send()
            .await
            .map_err(|err| UseCaseError::Persistence(err.to_string()))?;

        if let Some(item) = response.item {
            let payload = item.get("payload").and_then(|p| p.as_s().ok());
            if let Some(json) = payload {
                let goal: Goal =
                    serde_json::from_str(json).map_err(|err| UseCaseError::Persistence(err.to_string()))?;
                return Ok(Some(goal));
            }
        }

        Ok(None)
    }

    #[instrument(skip(self, goal))]
    async fn save(&self, goal: &Goal) -> Result<(), UseCaseError> {
        self.put_goal(goal).await
    }

    #[instrument(skip(self, goal))]
    async fn create(&self, goal: &Goal) -> Result<(), UseCaseError> {
        self.put_goal(goal).await
    }
}

impl DynamoRepositories {
    async fn put_goal(&self, goal: &Goal) -> Result<(), UseCaseError> {
        let payload =
            serde_json::to_string(goal).map_err(|err| UseCaseError::Persistence(err.to_string()))?;

        self.client
            .put_item()
            .table_name(&self.table_name)
            .item("pk", AttributeValue::S(Self::goal_pk(&goal.user_id)))
            .item("sk", AttributeValue::S(Self::goal_sk(&goal.id)))
            .item("payload", AttributeValue::S(payload))
            .item("gsi1pk", AttributeValue::S(format!("LEADERBOARD#{}", goal.user_id.0)))
            .item("gsi1sk", AttributeValue::S(Self::leaderboard_sk(goal)))
            .send()
            .await
            .map_err(|err| UseCaseError::Persistence(err.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl SessionRepository for DynamoRepositories {
    #[instrument(skip(self, session))]
    async fn save(&self, session: &PracticeSession) -> Result<(), UseCaseError> {
        let payload =
            serde_json::to_string(session).map_err(|err| UseCaseError::Persistence(err.to_string()))?;

        self.client
            .put_item()
            .table_name(&self.table_name)
            .item("pk", AttributeValue::S(Self::session_pk(&session.goal_id)))
            .item("sk", AttributeValue::S(Self::session_sk(&session.id)))
            .item("session_id", AttributeValue::S(session.id.0.clone()))
            .item("payload", AttributeValue::S(payload))
            .item("gsi1pk", AttributeValue::S(format!("SESSION#{}", session.user_id.0)))
            .item("gsi1sk", AttributeValue::S(format!("TS#{}", session.started_at.timestamp())))
            .send()
            .await
            .map_err(|err| UseCaseError::Persistence(err.to_string()))?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct EventBridgeEmitter {
    client: EventBridgeClient,
    bus_name: String,
}

impl EventBridgeEmitter {
    pub fn new(client: EventBridgeClient, bus_name: impl Into<String>) -> Self {
        Self {
            client,
            bus_name: bus_name.into(),
        }
    }

    pub async fn publish(&self, events: &[DomainEvent]) -> Result<(), EventEmitterError> {
        if events.is_empty() {
            return Ok(());
        }

        let entries: Vec<_> = events
            .iter()
            .map(|event| PutEventsRequestEntry::builder()
                .detail_type("domain_event")
                .detail(
                    serde_json::to_string(event)
                        .map_err(|err| EventEmitterError::Serialization(err.to_string()))?,
                )
                .event_bus_name(&self.bus_name)
                .source("tenk.backend")
                .build()
            )
            .collect::<Result<_, _>>()?;

        self.client
            .put_events()
            .set_entries(Some(entries))
            .send()
            .await
            .map_err(|err| EventEmitterError::Aws(err.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum EventEmitterError {
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("aws error: {0}")]
    Aws(String),
}
