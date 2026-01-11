variable "project" {
  type = string
}

variable "environment" {
  type = string
}

variable "domain_name" {
  type = string
}

variable "certificate_arn" {
  type = string
}

variable "bucket_domain_name" {
  type = string
}

variable "bucket_id" {
  type = string
}

variable "default_ttl" {
  type    = number
  default = 3600
}

variable "tags" {
  type    = map(string)
  default = {}
}
