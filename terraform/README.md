# terraform

Set up an AWS Lambda function with Terraform

## How to run it

Below is the procedure to spin up the resources for production.
You need an AWS account before moving on to the next step.

### Edit env/prod/providers.tf

Create an S3 bucket to store the state file, and set the bucket name to `bucket`.
Change the `region` according to the region where the S3 bucket is hosted.

```terraform
backend "s3" {
    bucket = "misskey-weather-bot-tfstate"
    region = "ap-northeast-1"
    key    = "misskey-weather-bot-2.tfstate"
}
```

### Edit env/prod/info.tf

`managed_by` is used for the tag set to the resources.
It doesn't affect the actual functionality of the resources, but you could change it to your repo URL in case you fork this repo.

```terraform
locals {
  service    = "misskey-weather-bot-2"
  env        = "prod"
  managed_by = "https://github.com/maeda6uiui/misskey-weather-bot-2"
}
```

### Edit env/prod/main.tf

#### weather_forecast_bot module

You have to modify the environment variables for the Lambda function to work properly.

```terraform
environment_variables = {
    EMOJI_CSV_FILEPATH            = "/var/runtime/Data/weather_conditions.csv"
    WEATHER_API_ACCESS_TOKEN_PATH = "weather-api-access-token"
    MISSKEY_ACCESS_TOKEN_PATH     = "misskey-access-token"
    WEATHER_API_ENDPOINT          = "https://api.weatherapi.com/v1/forecast.json"
    WEATHER_API_QUERY             = "Yuzhno-Sakhalinsk"
    WEATHER_API_DAYS              = "1"
    MISSKEY_SERVER_URL            = "https://misskey-dabansky.com"
}
```

`WEATHER_API_QUERY` is the query for Weather API.
It can be latitude and longitute, city name, etc.
Check [the official documentation](https://www.weatherapi.com/docs/) to find out what parameters it takes.

`WEATHER_API_DAYS` determines the number of days for weather forecast, and it should be 1.
You could specify more than 1 for this parameter, but the current implementation of the bot only displays 1 day of forecast.

`MISSKEY_SERVER_URL` is the server URL to post weather forecast on.
Set your server URL to this parameter.

---

You may want to change `schedule_expression`.
This parameter controls when the bot runs and posts weather forecast on Misskey.

```terraform
schedule_expression = "cron(0 22 * * ? *)"
```

#### github_actions module

You have to change `username` and `repo_name` of `github_info` to your username and your repo name.

```terraform
module "github_actions" {
  source = "../../modules/github_actions"

  service = local.service
  env     = local.env

  weather_forecast_bot = module.weather_forecast_bot

  github_info = {
    username  = "maeda6uiui"
    repo_name = "misskey-weather-bot-2"
  }
}
```

### Run Terraform

Run `terraform plan` and check the plan result.

```
terraform init
terraform plan -out prod.tfplan
```

Run `terraform apply` if the plan result is shown as expected.

```
terraform apply prod.tfplan
```

The ARN of the IAM role to use on GitHub Actions is output after applying the plan.
Take a note of it if you are going to use GitHub Actions on your repo to deploy the Lambda function.
You can check the outputs with `terraform output` afterward.
