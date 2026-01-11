output "bucket_id" {
  value = module.s3.bucket_id
}

output "distribution_domain" {
  value = module.cloudfront.distribution_domain_name
}

output "distribution_id" {
  value = module.cloudfront.distribution_id
}

output "distribution_arn" {
  value = module.cloudfront.distribution_arn
}
