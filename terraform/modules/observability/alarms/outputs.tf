output "alarm_names" {
  value = [for alarm in aws_cloudwatch_metric_alarm.lambda_errors : alarm.alarm_name]
}
