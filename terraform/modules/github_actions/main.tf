resource "aws_iam_openid_connect_provider" "github_actions" {
  url             = "https://token.actions.githubusercontent.com"
  client_id_list  = ["sts.amazonaws.com"]
}

resource "random_id" "rnd" {
  byte_length = 4
}

resource "aws_iam_role" "github_actions" {
  name = "${var.service}-github-actions-${var.env}-${random_id.rnd.hex}"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = "sts:AssumeRoleWithWebIdentity"
        Principal = {
          Federated = aws_iam_openid_connect_provider.github_actions.arn
        }
        Condition = {
          StringEquals = {
            "token.actions.githubusercontent.com:aud" = "sts.amazonaws.com"
          }
          StringLike = {
            "token.actions.githubusercontent.com:sub" = "repo:${var.github_info.username}/${var.github_info.repo_name}:*"
          }
        }
      }
    ]
  })
}

resource "aws_iam_policy" "allow_github_actions_access_to_ecr" {
  name = "${var.service}-allow-github-actions-access-to-ecr-${var.env}"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "ecr:UploadLayerPart",
          "ecr:PutImage",
          "ecr:InitiateLayerUpload",
          "ecr:CompleteLayerUpload",
          "ecr:BatchGetImage",
          "ecr:BatchCheckLayerAvailability"
        ]
        Resource = var.misskey_weather_bot.ecr.main.arn
      },
      {
        Effect   = "Allow"
        Action   = "ecr:GetAuthorizationToken"
        Resource = "*"
      }
    ]
  })
}

resource "aws_iam_policy" "allow_github_actions_access_to_lambda" {
  name = "${var.service}-allow-github-actions-access-to-lambda-${var.env}"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = "lambda:UpdateFunctionCode"
        Resource = var.misskey_weather_bot.lambda.main.arn
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "allow_github_actions_access_to_ecr" {
  role       = aws_iam_role.github_actions.name
  policy_arn = aws_iam_policy.allow_github_actions_access_to_ecr.arn
}

resource "aws_iam_role_policy_attachment" "allow_github_actions_access_to_lambda" {
  role       = aws_iam_role.github_actions.name
  policy_arn = aws_iam_policy.allow_github_actions_access_to_lambda.arn
}
