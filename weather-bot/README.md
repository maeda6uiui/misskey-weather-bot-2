# weather-bot

Main code to post weather forecast on Misskey

## Prerequisites

### Misskey

You need an account for the bot on a Misskey server.
Generate an access token for the account.
It requires "Compose or delete notes" permission.

### WeatherAPI.com

Create an account on [WeatherAPI.com](https://www.weatherapi.com/) and acquire an API key.

## Test locally

Set the access token for Misskey and the API key for Weather API to environment variables.

```
export MISSKEY_ACCESS_TOKEN=your_access_token
export WEATHER_API_ACCESS_TOKEN=your_access_token
```

The code is built to function on AWS Lambda by default.
You have to disable the default feature and specify `local` feature explicitly when you test it locally.

There is a convenient script to run the code locally.
Modify `--misskey-server-url` option in run_locally.sh and then run the script.

```bash
#!/bin/bash

RUST_LOG=info cargo run --no-default-features --features local -- \
    --weather-api-query Tokyo \
    --weather-api-days 1 \
    --misskey-server-url https://misskey-dabansky.com
```

## Deploy to Lambda

Using the deployment workflow is the easiest way to deploy the code to Lambda.
Set the ARN of the IAM role to a GitHub secret (`AWS_DEPLOYMENT_ROLE_ARN_PROD`) of your GitHub repo.
It can be obtained when you run `terraform apply` (check README.md of the terraform directory for further information).
Once you set the secret, you can trigger `./.github/workflows/deploy-lambda-prod.yml` manually or by pushing your changes to main.

If you don't want to use GitHub Actions for some reason, then check the commands executed in `./.github/workflows/deploy-lambda.yml`.
It basically builds a Docker image, pushes it to an ECR repo, and updates the Lambda function with `aws lambda update-function-code`.

---

One thing you have to do before the bot functions properly, is to set access tokens to Parameter Store.
You have to set the following parameters:

- weather-api-access-token
- misskey-access-token

As the name suggests, they are the access token for Weather API and Misskey respectively.
The parameters are created by Terraform with a dummy value in it, so update the value from AWS Management Console accordingly.
