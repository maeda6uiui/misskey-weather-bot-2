module "account_info" {
  source = "../../modules/account_info"
}

module "misskey_weather_bot" {
  source = "../../modules/misskey_weather_bot"

  service = local.service
  env     = local.env
  aws     = module.account_info.aws

  lambda_config = {
    timeout     = 15
    memory_size = 128
    environment_variables = {
      WEATHER_API_ACCESS_TOKEN_PATH = "weather-api-access-token"
      MISSKEY_ACCESS_TOKEN_PATH     = "misskey-access-token"
      WEATHER_API_ENDPOINT          = "https://api.weatherapi.com/v1/forecast.json"
      WEATHER_API_QUERY             = "Yuzhno-Sakhalinsk"
      WEATHER_API_DAYS              = "1"
      MISSKEY_SERVER_URL            = "https://misskey-dabansky.com"
    }
  }
  schedule_expression = "cron(0 22 * * ? *)"
}

module "github_actions" {
  source = "../../modules/github_actions"

  service = local.service
  env     = local.env

  misskey_weather_bot = module.misskey_weather_bot

  github_info = {
    username  = "maeda6uiui"
    repo_name = "misskey-weather-bot-2"
  }
}
