resource "aws_cloudwatch_log_group" "main" {
  name = "/aws/lambda/${var.service}-${var.env}"
}
