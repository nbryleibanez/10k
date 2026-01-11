locals {
  name = "${var.project}-${var.environment}-api"
}

resource "aws_apigatewayv2_api" "this" {
  name          = local.name
  protocol_type = "HTTP"

  cors_configuration {
    allow_origins = length(var.cors_allowed_origins) > 0 ? var.cors_allowed_origins : ["*"]
    allow_methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"]
    allow_headers = ["content-type", "authorization", "x-requested-with"]
  }

  tags = var.tags
}

resource "aws_apigatewayv2_stage" "this" {
  api_id      = aws_apigatewayv2_api.this.id
  name        = var.stage_name
  auto_deploy = true
  tags        = var.tags

  default_route_settings {
    throttling_rate_limit  = 100
    throttling_burst_limit = 50
  }
}

resource "aws_apigatewayv2_authorizer" "cognito" {
  api_id           = aws_apigatewayv2_api.this.id
  authorizer_type  = "JWT"
  identity_sources = ["$request.header.Authorization"]
  name             = "${local.name}-cognito"

  jwt_configuration {
    audience = [var.cognito_client_id]
    issuer   = "https://cognito-idp.${var.region}.amazonaws.com/${var.cognito_user_pool_id}"
  }
}

resource "aws_apigatewayv2_integration" "this" {
  for_each               = var.routes
  api_id                 = aws_apigatewayv2_api.this.id
  integration_type       = "AWS_PROXY"
  integration_uri        = each.value.lambda_arn
  payload_format_version = "2.0"
  integration_method     = "POST"
  timeout_milliseconds   = 29000
}

resource "aws_apigatewayv2_route" "this" {
  for_each           = var.routes
  api_id             = aws_apigatewayv2_api.this.id
  route_key          = "${upper(each.value.method)} ${each.value.path}"
  target             = "integrations/${aws_apigatewayv2_integration.this[each.key].id}"
  authorization_type = lookup(each.value, "authorization", "JWT")
  authorizer_id      = lookup(each.value, "authorization", "JWT") == "JWT" ? aws_apigatewayv2_authorizer.cognito.id : null
}

resource "aws_lambda_permission" "api" {
  for_each      = var.routes
  action        = "lambda:InvokeFunction"
  function_name = each.value.lambda_arn
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.this.execution_arn}/*/*"
}
