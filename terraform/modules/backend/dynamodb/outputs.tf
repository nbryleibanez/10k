output "table_name" {
  value       = aws_dynamodb_table.this.name
  description = "Primary table name"
}

output "table_arn" {
  value       = aws_dynamodb_table.this.arn
  description = "Table ARN"
}

output "stream_arn" {
  value       = aws_dynamodb_table.this.stream_arn
  description = "Stream ARN"
}
