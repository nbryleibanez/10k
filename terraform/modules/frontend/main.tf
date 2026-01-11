module "s3" {
  source        = "./s3"
  bucket_name   = var.bucket_name
  force_destroy = var.force_destroy
  tags          = var.tags
}

module "cloudfront" {
  source             = "./cloudfront"
  project            = var.project
  environment        = var.environment
  domain_name        = var.domain_name
  certificate_arn    = var.certificate_arn
  bucket_domain_name = module.s3.bucket_domain_name
  bucket_id          = module.s3.bucket_id
  tags               = var.tags
}

data "aws_iam_policy_document" "allow_cloudfront" {
  statement {
    actions   = ["s3:GetObject"]
    resources = ["${module.s3.bucket_arn}/*"]

    principals {
      type        = "Service"
      identifiers = ["cloudfront.amazonaws.com"]
    }

    condition {
      test     = "StringEquals"
      variable = "AWS:SourceArn"
      values   = [module.cloudfront.distribution_arn]
    }
  }
}

resource "aws_s3_bucket_policy" "allow_cloudfront" {
  bucket = module.s3.bucket_id
  policy = data.aws_iam_policy_document.allow_cloudfront.json
}
