locals {
  table_name = "${var.project}-${var.environment}-core"
  common_env = {
    RUST_LOG       = "info"
    ENVIRONMENT    = var.environment
    EVENT_BUS_NAME = var.event_bus_name
  }
}

module "dynamodb" {
  source     = "./dynamodb"
  table_name = local.table_name
  tags       = var.tags
}

module "cognito" {
  source                = "./cognito"
  project               = var.project
  environment           = var.environment
  domain_prefix         = var.domain_prefix
  allowed_callback_urls = var.allowed_callback_urls
  allowed_logout_urls   = var.allowed_logout_urls
  tags                  = var.tags
}

module "lambda" {
  source               = "./lambda"
  project              = var.project
  environment          = var.environment
  functions            = var.lambda_functions
  dynamodb_table_name  = module.dynamodb.table_name
  dynamodb_table_arn   = module.dynamodb.table_arn
  cognito_user_pool_id = module.cognito.user_pool_id
  common_environment   = local.common_env
  tags                 = var.tags
}

module "alarms" {
  source           = "../observability/alarms"
  name_prefix      = "${var.project}-${var.environment}"
  lambda_functions = module.lambda.function_names
  sns_topic_arn    = var.sns_topic_arn
}

locals {
  route_definitions = {
    for key, route in var.api_routes : key => {
      path          = route.path
      method        = route.method
      lambda_arn    = module.lambda.function_arns[route.lambda_key]
      authorization = lookup(route, "authorization", "JWT")
    }
  }
}

module "apigateway" {
  source               = "./apigateway"
  project              = var.project
  environment          = var.environment
  routes               = local.route_definitions
  cors_allowed_origins = var.cors_allowed_origins
  tags                 = var.tags
  cognito_user_pool_id = module.cognito.user_pool_id
  cognito_client_id    = module.cognito.user_pool_client_id
  region               = var.aws_region
}

output "api_endpoint" {
  value = module.apigateway.api_endpoint
}

output "table_name" {
  value = module.dynamodb.table_name
}

output "cognito_user_pool_id" {
  value = module.cognito.user_pool_id
}

output "cognito_user_pool_client_id" {
  value = module.cognito.user_pool_client_id
}

output "lambda_function_names" {
  value = module.lambda.function_names
}
