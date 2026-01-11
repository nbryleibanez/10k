use std::env;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_eventbridge::Client as EventBridgeClient;
use lambda_http::{run, service_fn, Body, Error as LambdaError, Request, RequestExt, Response};
use tenk_domain::{GoalId, SessionSource, UserId};
use tenk_dto::{ApiError, LogSessionRequest, PracticeSessionEnvelope};
use tenk_infra::{DynamoRepositories, EventBridgeEmitter};
use tenk_usecases::{LogPracticeCommand, LogPracticeSession};
use tracing::{error, info, Level};

fn extract_user_id(request: &Request) -> Result<UserId, LambdaError> {
    let jwt = request
        .request_context()
        .authorizer()
        .and_then(|auth| auth.jwt);

    if let Some(jwt) = jwt {
        if let Some(sub) = jwt.claims.get("sub") {
            return Ok(UserId(sub.clone()));
        }
    }

    Err("Unauthorized: missing sub claim".into())
}

fn error_response(status: u16, error: ApiError) -> Result<Response<Body>, LambdaError> {
    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&error)?.into())?)
}

async fn handler(
    request: Request,
    repo: DynamoRepositories,
    emitter: EventBridgeEmitter,
) -> Result<Response<Body>, LambdaError> {
    let body = request.body();
    let raw = body.as_ref();
    if raw.is_empty() {
        return error_response(400, ApiError {
            code: "VALIDATION_ERROR".into(),
            message: "missing body".into(),
        });
    }
    let payload: LogSessionRequest = serde_json::from_slice(raw)?;
    let user_id = extract_user_id(&request)?;

    let usecase = LogPracticeSession::new(&repo, &repo);
    let result = usecase
        .execute(LogPracticeCommand {
            goal_id: GoalId(payload.goal_id),
            user_id,
            duration_minutes: payload.duration_minutes,
            started_at: payload.started_at.unwrap_or_else(|| chrono::Utc::now()),
            tags: payload.tags.unwrap_or_default(),
            reflection: payload.reflection,
            quality_rating: payload.quality_rating,
            source: payload.source,
        })
        .await;

    match result {
        Ok(outcome) => {
            if let Err(err) = emitter.publish(&outcome.events).await {
                error!(?err, "failed to publish domain events");
            }
            let body = serde_json::to_string(&PracticeSessionEnvelope { data: outcome.data })?;
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(body.into())?)
        }
        Err(err) => {
            info!(?err, "failed to log session");
            error_response(
                422,
                ApiError {
                    code: "VALIDATION_ERROR".into(),
                    message: err.to_string(),
                },
            )
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let table_name = env::var("TABLE_NAME").unwrap_or_else(|_| "tenk-dev".into());
    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = DynamoClient::new(&shared_config);
    let eventbridge_client = EventBridgeClient::new(&shared_config);
    let repo = DynamoRepositories::new(table_name, client);
    let event_bus = env::var("EVENT_BUS_NAME").unwrap_or_else(|_| "default".into());
    let emitter = EventBridgeEmitter::new(eventbridge_client, event_bus);

    run(service_fn(|event| {
        let repo = repo.clone();
        let emitter = emitter.clone();
        async move { handler(event, repo, emitter).await }
    }))
    .await
}
