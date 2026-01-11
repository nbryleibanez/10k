variable "name_prefix" {
  type = string
}

variable "lambda_functions" {
  type = map(string)
}

variable "sns_topic_arn" {
  type    = string
  default = ""
}
