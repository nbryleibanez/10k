use tenk_dto::{paths, ApiError, LogSessionRequest, PracticeSessionEnvelope, PracticeSessionResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(paths::log_session),
    components(
        schemas(LogSessionRequest, PracticeSessionResponse, PracticeSessionEnvelope, ApiError)
    ),
    tags(
        (name = "sessions", description = "Practice sessions API")
    )
)]
struct ApiDoc;

fn main() {
    let doc = ApiDoc::openapi();
    println!("{}", doc.to_json().unwrap());
}
