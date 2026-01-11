output "api_endpoint" {
  value = module.apigateway.api_endpoint
}

output "api_stage_arn" {
  value = module.apigateway.stage_arn
}

output "api_execution_arn" {
  value = module.apigateway.execution_arn
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
