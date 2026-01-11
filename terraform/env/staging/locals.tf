locals {
  project     = "tenk"
  environment = "staging"
  aws_region  = "us-east-1"

  tags = {
    Project     = local.project
    Environment = local.environment
  }

  cognito_domain_prefix = "tenk-staging-auth"
  allowed_callback_urls = ["https://staging.tenk.example/auth/callback"]
  allowed_logout_urls   = ["https://staging.tenk.example/logout"]
  cors_allowed_origins  = ["https://staging.tenk.example"]
  sns_topic_arn         = ""

  lambda_artifacts_bucket = "tenk-staging-artifacts"
  frontend_bucket         = "tenk-staging-frontend"
  frontend_domain         = "staging.tenk.example"
  certificate_arn         = "arn:aws:acm:us-east-1:123456789012:certificate/staging-placeholder"
  event_bus_name          = "tenk-staging-events"

  lambda_functions = {
    log_session = {
      description = "Log deliberate practice sessions"
      handler     = "bootstrap"
      runtime     = "provided.al2"
      memory_size = 384
      timeout     = 20
      s3_bucket   = local.lambda_artifacts_bucket
      s3_key      = "lambda/log-session.zip"
      environment = {}
    }
    health = {
      description = "Health check"
      handler     = "bootstrap"
      runtime     = "provided.al2"
      memory_size = 256
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
