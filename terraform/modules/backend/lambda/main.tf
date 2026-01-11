locals {
  name_prefix = "${var.project}-${var.environment}"
}

data "aws_iam_policy_document" "assume" {
  statement {
    effect  = "Allow"
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "this" {
  for_each           = var.functions
  name               = "${local.name_prefix}-${each.key}-lambda"
  assume_role_policy = data.aws_iam_policy_document.assume.json
  tags               = var.tags
}

resource "aws_iam_role_policy" "dynamodb" {
  for_each = var.functions
  name     = "${local.name_prefix}-${each.key}-ddb"
  role     = aws_iam_role.this[each.key].id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = ["dynamodb:PutItem", "dynamodb:GetItem", "dynamodb:Query", "dynamodb:UpdateItem"]
        Resource = [
          var.dynamodb_table_arn,
          "${var.dynamodb_table_arn}/index/*"
        ]
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "basic" {
  for_each   = var.functions
  role       = aws_iam_role.this[each.key].name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_cloudwatch_log_group" "this" {
  for_each          = var.functions
  name              = "/aws/lambda/${local.name_prefix}-${each.key}"
  retention_in_days = var.log_retention_days
  tags              = var.tags
}

resource "aws_lambda_function" "this" {
  for_each = var.functions

  function_name = "${local.name_prefix}-${each.key}"
  description   = each.value.description
  role          = aws_iam_role.this[each.key].arn
  handler       = each.value.handler
  runtime       = each.value.runtime
  s3_bucket     = each.value.s3_bucket
  s3_key        = each.value.s3_key
  memory_size   = each.value.memory_size
  timeout       = each.value.timeout
  architectures = [each.value.architecture]
  layers        = each.value.layers

  reserved_concurrent_executions = lookup(each.value, "reserved_concurrent_executions", null)

  tracing_config {
    mode = "Active"
  }

  environment {
    variables = merge(
      var.common_environment,
      each.value.environment,
      {
        TABLE_NAME                          = var.dynamodb_table_name,
        USER_POOL_ID                        = var.cognito_user_pool_id,
        EVENT_BUS_NAME                      = var.common_environment["EVENT_BUS_NAME"],
        AWS_NODEJS_CONNECTION_REUSE_ENABLED = "1"
      }
    )
  }

  depends_on = [aws_cloudwatch_log_group.this]
  tags       = var.tags
}

output "function_arns" {
  value       = { for key, fn in aws_lambda_function.this : key => fn.arn }
  description = "Lambda ARNs keyed by logical name"
}

output "function_names" {
  value       = { for key, fn in aws_lambda_function.this : key => fn.function_name }
  description = "Lambda names"
}
