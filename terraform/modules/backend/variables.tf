variable "project" {
  type = string
}

variable "environment" {
  type = string
}

variable "lambda_functions" {
  description = "Configuration for backend lambda functions"
  type = map(object({
    description = string
    handler     = string
    runtime     = string
    memory_size = number
    timeout     = number
    s3_bucket   = string
    s3_key      = string
    environment = optional(map(string), {})
    layers      = optional(list(string), [])
  }))
}

variable "api_routes" {
  description = "Route definitions mapping to lambda keys"
  type = map(object({
    path          = string
    method        = string
    lambda_key    = string
    authorization = optional(string, "JWT")
  }))
}

variable "domain_prefix" {
  type = string
}

variable "allowed_callback_urls" {
  type    = list(string)
  default = []
}

variable "allowed_logout_urls" {
  type    = list(string)
  default = []
}

variable "cors_allowed_origins" {
  type    = list(string)
  default = []
}

variable "tags" {
  type    = map(string)
  default = {}
}

variable "aws_region" {
  type = string
}

variable "event_bus_name" {
  type    = string
  default = "default"
}

variable "sns_topic_arn" {
  type    = string
  default = ""
}
