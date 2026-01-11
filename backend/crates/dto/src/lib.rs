use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tenk_domain::{Goal, SessionSource};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LogSessionRequest {
    pub goal_id: String,
    pub duration_minutes: u32,
    pub started_at: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
    pub reflection: Option<String>,
    pub quality_rating: Option<u8>,
    #[serde(default)]
    pub source: SessionSource,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PracticeSessionResponse {
    pub session_id: String,
    pub goal: Goal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PracticeSessionEnvelope {
    pub data: PracticeSessionResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthCheckResponse {
    pub status: &'static str,
}

pub mod paths {
    use super::{ApiError, ApiResponse, LogSessionRequest, PracticeSessionResponse};

    #[utoipa::path(
        post,
        path = "/sessions",
        request_body = LogSessionRequest,
        responses(
            (status = 200, description = "Session logged", body = PracticeSessionEnvelope),
            (status = 400, description = "Invalid input", body = ApiError),
            (status = 401, description = "Unauthorized", body = ApiError)
        ),
        security(
            ("cognito_jwt" = [])
        )
    )]
    pub async fn log_session() {}
}
