variable "project" {
  type = string
}

variable "environment" {
  type = string
}

variable "functions" {
  description = "Map of lambda functions to create"
  type = map(object({
    description                    = string
    handler                        = string
    runtime                        = string
    memory_size                    = number
    timeout                        = number
    s3_bucket                      = string
    s3_key                         = string
    architecture                   = optional(string, "arm64")
    environment                    = optional(map(string), {})
    layers                         = optional(list(string), [])
    reserved_concurrent_executions = optional(number)
  }))
}

variable "dynamodb_table_name" {
  type = string
}

variable "dynamodb_table_arn" {
  type = string
}

variable "cognito_user_pool_id" {
  type = string
}

variable "tags" {
  type    = map(string)
  default = {}
}

variable "log_retention_days" {
  type    = number
  default = 14
}

variable "common_environment" {
  type    = map(string)
  default = {}
}
