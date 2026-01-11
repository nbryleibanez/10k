variable "project" {
  type = string
}

variable "environment" {
  type = string
}

variable "routes" {
  description = "Routing definition for HTTP API"
  type = map(object({
    path          = string
    method        = string
    lambda_arn    = string
    authorization = optional(string, "JWT")
  }))
}

variable "cors_allowed_origins" {
  type    = list(string)
  default = []
}

variable "tags" {
  type    = map(string)
  default = {}
}

variable "stage_name" {
  type    = string
  default = "$default"
}

variable "cognito_user_pool_id" {
  type = string
}

variable "cognito_client_id" {
  type = string
}

variable "region" {
  type = string
}
