terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~>6.0"
    }
  }

  backend "s3" {
    bucket = "maeda6uiui-terraform-sandbox"
    region = "ap-northeast-1"
    key    = "misskey-weather-bot-2.tfstate"
  }

  required_version = "~>1.9"
}

provider "aws" {
  region = "ap-northeast-1"

  default_tags {
    tags = {
      Service   = local.service
      Env       = local.env
      ManagedBy = local.managed_by
    }
  }
}
