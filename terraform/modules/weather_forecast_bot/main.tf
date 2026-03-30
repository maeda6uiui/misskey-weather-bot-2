module "ssm" {
  source = "./ssm"
}

module "ecr" {
  source = "./ecr"

  aws = var.aws
}

module "cloudwatch" {
  source = "./cloudwatch"

  service = var.service
  env     = var.env
}

module "iam" {
  source = "./iam"

  service = var.service
  env     = var.env
  aws     = var.aws

  cloudwatch_log_group_arn = module.cloudwatch.log_group.main.arn
  ssm_parameter_arns = [
    for _, v in module.ssm.parameter : v.arn
  ]
}

module "lambda" {
  source = "./lambda"

  service = var.service
  env     = var.env

  lambda_config = var.lambda_config

  repository_url  = module.ecr.main.repository_url
  lambda_role_arn = module.iam.role.lambda.arn
}

module "eventbridge" {
  source = "./eventbridge"

  service = var.service
  env     = var.env

  lambda_arn          = module.lambda.main.arn
  lambda_name         = module.lambda.main.function_name
  schedule_expression = var.schedule_expression
}
