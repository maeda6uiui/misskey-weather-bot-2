resource "aws_lambda_function" "main" {
  function_name = "${var.service}-${var.env}"

  role = var.lambda_role_arn

  #Actual image is deployed by GitHub Actions
  package_type = "Image"
  image_uri    = "${var.repository_url}:temp"

  timeout     = var.lambda_config.timeout
  memory_size = var.lambda_config.memory_size

  environment {
    variables = var.lambda_config.environment_variables
  }

  lifecycle {
    ignore_changes = [
      image_uri
    ]
  }
}
