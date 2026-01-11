variable "name" {
  type = string
}

variable "scope" {
  type = string
}

variable "resource_arn" {
  type = string
}

variable "tags" {
  type    = map(string)
  default = {}
}
