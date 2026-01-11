locals {
  alarm_actions = length(var.sns_topic_arn) > 0 ? [var.sns_topic_arn] : []
}

resource "aws_cloudwatch_metric_alarm" "lambda_errors" {
  for_each            = var.lambda_functions
  alarm_name          = "${var.name_prefix}-${each.key}-errors"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = 1
  metric_name         = "Errors"
  namespace           = "AWS/Lambda"
  period              = 60
  statistic           = "Sum"
  threshold           = 5
  alarm_description   = "Alarms when ${each.key} records errors"
  alarm_actions       = local.alarm_actions

  dimensions = {
    FunctionName = each.value
  }
}
