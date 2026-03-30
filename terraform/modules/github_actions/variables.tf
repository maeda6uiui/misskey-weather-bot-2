variable "service" {
  type = string
}

variable "env" {
  type = string
}

variable "github_info" {
  type = object({
    username  = string
    repo_name = string
  })
}

variable "weather_forecast_bot" {
  type = object({
    ecr = object({
      main = object({
        arn = string
      })
    })
    lambda = object({
      main = object({
        arn = string
      })
    })
  })
}
