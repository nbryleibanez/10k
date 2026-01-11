use lambda_http::{run, service_fn, Body, Request, Response};
use lambda_runtime::Error;
use tenk_dto::HealthCheckResponse;
use tracing::Level;

async fn handler(_event: Request) -> Result<Response<Body>, Error> {
    let body = serde_json::to_string(&HealthCheckResponse { status: "ok" })?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(body.into())?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    run(service_fn(handler)).await
}
