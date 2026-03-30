output "parameter" {
  value = {
    weather_api_access_token = {
      arn = aws_ssm_parameter.weather_api_access_token.arn
    }
    misskey_access_token = {
      arn = aws_ssm_parameter.misskey_access_token.arn
    }
  }
}
