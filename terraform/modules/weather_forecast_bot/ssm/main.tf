resource "aws_ssm_parameter" "weather_api_access_token" {
  name = "weather-api-access-token"
  type = "SecureString"

  #You have to set the actual value via AWS management console.
  value = "dummy"

  lifecycle {
    ignore_changes = [
      value
    ]
  }
}

resource "aws_ssm_parameter" "misskey_access_token" {
  name = "misskey-access-token"
  type = "SecureString"

  #You have to set the actual value via AWS management console.
  value = "dummy"

  lifecycle {
    ignore_changes = [
      value
    ]
  }
}
