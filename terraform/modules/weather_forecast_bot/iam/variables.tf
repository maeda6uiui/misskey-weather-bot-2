variable "service" {
  type = string
}

variable "env" {
  type = string
}

variable "aws" {
  type = object({
    region     = string
    account_id = string
  })
}

variable "cloudwatch_log_group_arn" {
  type = string
}

variable "ssm_parameter_arns" {
  type = list(string)
}
