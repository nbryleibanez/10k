variable "project" {
  type = string
}

variable "environment" {
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

variable "domain_prefix" {
  type        = string
  description = "Prefix for Cognito hosted UI domain"
}

variable "tags" {
  type    = map(string)
  default = {}
}
