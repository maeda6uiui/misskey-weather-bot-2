variable "service" {
  type = string
}

variable "env" {
  type = string
}

variable "lambda_config" {
  type = object({
    timeout               = number
    memory_size           = number
    environment_variables = map(string)
  })
}

variable "lambda_role_arn" {
  type = string
}

variable "repository_url" {
  type = string
}
