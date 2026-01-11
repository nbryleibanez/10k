locals {
  project     = "tenk"
  environment = "prod"
  aws_region  = "us-east-1"

  tags = {
    Project     = local.project
    Environment = local.environment
  }

  cognito_domain_prefix = "tenk-prod-auth"
  allowed_callback_urls = ["https://app.tenk.example/auth/callback"]
  allowed_logout_urls   = ["https://app.tenk.example/logout"]
  cors_allowed_origins  = ["https://app.tenk.example"]
  sns_topic_arn         = ""

  lambda_artifacts_bucket = "tenk-prod-artifacts"
  frontend_bucket         = "tenk-prod-frontend"
  frontend_domain         = "app.tenk.example"
  certificate_arn         = "arn:aws:acm:us-east-1:123456789012:certificate/replace-me"
  event_bus_name          = "tenk-prod-events"

  lambda_functions = {
    log_session = {
      description = "Log deliberate practice sessions"
      handler     = "bootstrap"
      runtime     = "provided.al2"
      memory_size = 512
      timeout     = 30
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
