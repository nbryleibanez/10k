variable "table_name" {
  type        = string
  description = "Name of the primary single-table"
}

variable "tags" {
  type        = map(string)
  description = "Resource tags"
  default     = {}
}

variable "enable_stream" {
  type        = bool
  description = "Enable DynamoDB streams"
  default     = true
}
