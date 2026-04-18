# terraform

Set up a AWS Lambda function with Terraform

## How to run it

Below is the procedure to spin up the resources for production.
You need an AWS account before moving on to the next step.

### Edit providers.tf

Create an S3 bucket to store the state file, and set the bucket name to `bucket`.
Change the `region` according to the region where the S3 bucket is hosted.

```terraform
backend "s3" {
    bucket = "misskey-weather-bot-tfstate"
    region = "ap-northeast-1"
    key    = "misskey-weather-bot-2.tfstate"
}
```

### Edit info.tf

`managed_by` is used for the tag set to the resources.
It doesn't affect the actual functionality of the resources, but you could change it to your repo URL in case you fork this repo.

```terraform
locals {
  service    = "misskey-weather-bot-2"
  env        = "prod"
  managed_by = "https://github.com/maeda6uiui/misskey-weather-bot-2"
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
