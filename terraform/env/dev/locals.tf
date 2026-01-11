locals {
  project     = "tenk"
  environment = "dev"
  aws_region  = "us-east-1"

  tags = {
    Project     = local.project
    Environment = local.environment
  }

  cognito_domain_prefix = "tenk-dev-auth"
  allowed_callback_urls = ["http://localhost:5173/auth/callback"]
  allowed_logout_urls   = ["http://localhost:5173/logout"]
  cors_allowed_origins  = ["http://localhost:5173"]
  sns_topic_arn         = ""

  lambda_artifacts_bucket = "tenk-dev-artifacts"
  frontend_bucket         = "tenk-dev-frontend"
  frontend_domain         = "dev.tenk.example"
  certificate_arn         = "arn:aws:acm:us-east-1:123456789012:certificate/dev-placeholder"
  event_bus_name          = "tenk-dev-events"

  lambda_functions = {
    log_session = {
      description = "Log deliberate practice sessions"
      handler     = "bootstrap"
      runtime     = "provided.al2"
      memory_size = 256
      timeout     = 15
      s3_bucket   = local.lambda_artifacts_bucket
      s3_key      = "lambda/log-session.zip"
      environment = { LOG_LEVEL = "debug" }
    }
    health = {
      description = "Health check"
      handler     = "bootstrap"
      runtime     = "provided.al2"
      memory_size = 128
      timeout     = 5
      s3_bucket   = local.lambda_artifacts_bucket
      s3_key      = "lambda/health.zip"
      environment = {}
    }
  }

  api_routes = {
    log_session = {
      path          = "/sessions"
      method        = "POST"
      lambda_key    = "log_session"
      authorization = "JWT"
    }
    health = {
      path          = "/health"
      method        = "GET"
      lambda_key    = "health"
      authorization = "NONE"
    }
  }
}
