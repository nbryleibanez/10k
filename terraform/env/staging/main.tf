provider "aws" {
  region = local.aws_region
}

module "backend" {
  source                = "../../modules/backend"
  project               = local.project
  environment           = local.environment
  aws_region            = local.aws_region
  event_bus_name        = local.event_bus_name
  lambda_functions      = local.lambda_functions
  api_routes            = local.api_routes
  domain_prefix         = local.cognito_domain_prefix
  allowed_callback_urls = local.allowed_callback_urls
  allowed_logout_urls   = local.allowed_logout_urls
  cors_allowed_origins  = local.cors_allowed_origins
  tags                  = local.tags
  sns_topic_arn         = local.sns_topic_arn
}

module "frontend" {
  source          = "../../modules/frontend"
  project         = local.project
  environment     = local.environment
  domain_name     = local.frontend_domain
  certificate_arn = local.certificate_arn
  bucket_name     = local.frontend_bucket
  tags            = local.tags
}

module "api_waf" {
  source       = "../../modules/waf"
  name         = "${local.project}-${local.environment}-api-waf"
  scope        = "REGIONAL"
  resource_arn = module.backend.api_stage_arn
  tags         = local.tags
}

module "cdn_waf" {
  source       = "../../modules/waf"
  name         = "${local.project}-${local.environment}-cdn-waf"
  scope        = "CLOUDFRONT"
  resource_arn = module.frontend.distribution_arn
  tags         = local.tags
}
