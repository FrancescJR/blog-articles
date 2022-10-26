# Self Hosted infrastructure on AWS with Terraform (part3)

## React static website Cloudfront distribution + S3 Terraform module

Thank you for keeping with me up to part 3. This one we will do something more interesting.

We would like to host our public facing web site in our small company infrastructure. We wanted
to play a little bit with React, so we have already a version ready in our localhost.

For the sake of this guide, and because this is actually how I did it with this exact same page,
I am going to use this project: [https://cesc.cool](https://cesc.cool) as the example. You can check
the code in this [github repository](https://github.com/FrancescJR/cesc-cool)

In this part we will prepare the infra for the site, and on the next part we will set
ready the CI/CD procedure for that project.

## The process

After having had some conversations with some people that actually work as DevOps (in
my point of view those are actually Ops, they have nothing to do with Dev), I would
like to explain my point of view in the whole development of creating the infrastructure and
the CI/CD procedures.

The way I see it, the infrastructure is a whole separate project and needs to happen before we even
start thinking about the CI/CD of the project.

As you could have guessed with the previous parts, I am already separating those "projects". I have
a repository that hosts my projects, it has its own CI/CD - we will go there shortly- but it's totally
independent of the way the project are going to be deployed in there.

Our projects, then, depend on this infrastructure project being in place first. That is basically
the definition of a dependency, you might argue this is bad, but it simplifies so much the
understanding of the whole Operations, that is 10 times worth it.

Of course, when you develop the CI/CD procedure for the specific project, you might realize that
you need to change some stuff from the infrastructure project. No harm in that, we are the owners
of all the projects, so let's do it then, deploy it, and then we will be able to continue with
our project's CI/CD procedure.

On the other hand of this spectrum would be to make everything so dynamic, creating the whole infra
dynamically once running the project's CI/CD procedure. That would quickly become a very complex
project, and while it might be more 'correct', or with less coupling with another project, it will
make it for a very hard understanding of it all. Hard understanding equals technical debt: try to change
that, how long would that take? you might go faster by starting from zero all together.

## Cloudfront public site module

You might need some little knowledge of AWS here. In general terms you want a route53
that points to a cloudfront distribution in AWS. Cloudfront distributions have "origins", the
place where the thing that is served as response of the HTTP request is being served. Of course,
Amazon making it easy to keep using AWS, you can set as origin places in S3.

That's what we're going to do. Create the route53, the cloudfront distribution and the s3 bucket.

We assume that we have developed already a site in react that we will upload to S3 using
a CI/CD procedure.

Let's create finally the terraform module:

```html
    ├── modules
    │   ├── cloudfront-public-site
    │   │   ├── outputs.tf
    │   │   └── variables.tf

```
Let's begin by creating the directory and a couple of standard files of our module. See part 2
about why this separation of resources into files, you may change it as you wish if it makes it
easier for you to follow the code. After all, all the content of all the files are being
"compiled", so there's no "rules" to follow here.

### S3 resource

Let's begin with the most "inner" part. The S3. Create a file called s3.tf in the module:
```
resource "aws_s3_bucket" "default" {
  bucket = var.bucket_name
  tags   = var.tags
}
#we already know we will need those variables later.

resource "aws_s3_bucket_public_access_block" "default" {
  bucket                  = aws_s3_bucket.default.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource aws_s3_bucket_acl "default"{
  bucket = aws_s3_bucket.default.id
  acl    = "private"
}

resource aws_s3_bucket_policy "default" {
  bucket = aws_s3_bucket.default.id
  policy = data.template_file.this.rendered
}

data "template_file" "this" {
  template = file("${path.module}/policy_S3_cloudfront.json.tpl")
  vars = {
    cloudfront_OAI_id = aws_cloudfront_origin_access_identity.default.id
    deployer-arn      = var.deployer-user-arn
    bucket_name       = var.bucket_name
  }
}
```

I am not a fan of naming the resources 'default', actually I would strongly argue against it,
but sometimes my head can't find the proper work in English - nor in any language- let's live
with it.

We create the actual resource S3 bucket, that will have the name that we will set in the
variables. Then we attach the policies.

For the policy, we create a file called "policy_S3_cloudfront.json.tpl" with this content:
```json
{
    "Version": "2012-10-17",
    "Id": "PolicyForCloudFrontPrivateContent",
    "Statement": [
        {
            "Sid": " Grant a CloudFront Origin Identity access to support private content",
            "Effect": "Allow",
            "Principal": {
                "AWS": "arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity ${cloudfront_OAI_id}"
            },
            "Action": "s3:GetObject",
            "Resource": "arn:aws:s3:::${bucket_name}/*"
        },
        {
            "Sid": "Grant access to deployer user",
            "Effect": "Allow",
            "Principal": {
                "AWS": "${deployer-arn}"
            },
            "Action": "s3:*",
            "Resource": [
                "arn:aws:s3:::${bucket_name}",
                "arn:aws:s3:::${bucket_name}/*"
            ]
        }
    ]
}
```
I hope this policy is self explanatory. We allow access to two users:
1. the Cloudfront resource, so when users access our web page, cloudfront will be able to access s3
2. The "github deployer". We created this user in part 1, on the "global infra". This will be the Iam used by the project CI/CD to update the contents on S3

let's review what we pass as variables to this template:
```
data "template_file" "this" {
  template = file("${path.module}/policy_S3_cloudfront.json.tpl")
  vars = {
    cloudfront_OAI_id = aws_cloudfront_origin_access_identity.default.id
    deployer-arn      = var.deployer-user-arn
    bucket_name       = var.bucket_name
  }
}
```
for the cloudfront user, we will create later a resource to indicate the identity that
cloudfront is going to use (aws_cloudfront_origin_access_identity).

The "deployer-arn" is the github user, and it will be passed as variable.

With all of this we know we need to add some variables in our module:

variables.tf
```
variable "bucket_name" {
  description = "S3 bucket Name"
  type        = string
}

variable "deployer-user-arn" {
  description = "The ARN of the AWS user that will deploy to the s3 bucket"
}

variable "tags" {
  description = "A mapping of tags to assign to all resources"
  type        = map(string)
  default     = {}
}
```

#### Insignificant comment about tags
As opposed as [Part2](https://blog.cesc.cool/self-hosted-infrastructure-on-aws-with-terraform-part2#heading-about-tags)

Here we go one step ahead on the tags management. It will be easier to declare all the tags
in the "staging/production", parts, and on each module, let's just say tags will come given.

This is different to how we did it in route53 module. So you can try to make it consistent.

### Cloudfront

Now that we have S3 resources ready, let's see the cloudfront. [Official documentation](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/cloudfront_distribution).

it's just 2 resources but quite big. Create a file called cloudfront.tf:
```
resource "aws_cloudfront_origin_access_identity" "default" {
  comment = "OAI for Public Site ${var.url} ${var.environment}"
}

resource "aws_cloudfront_distribution" "default" {
  # S3 Origin - B2C
  origin {
    domain_name = aws_s3_bucket.default.bucket_domain_name
    origin_id   = local.s3_origin_id

    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.default.cloudfront_access_identity_path
    }
  }

  # S3 Origin - Public (you need an origin for each cache behaviour, well cache behaviour must match
  # an origin on the target_origin_id defined in an origin block
  origin {
    domain_name = aws_s3_bucket.default.bucket_domain_name
    origin_id   = "${local.s3_origin_id}/public"
    origin_path = "/public"

    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.default.cloudfront_access_identity_path
    }
  }

  enabled             = true
  is_ipv6_enabled     = true
  comment             = "CloudFront for Company site ${var.url}"
  default_root_object = "index.html"
  price_class         = var.cf_price_class
  web_acl_id          = var.web_acl_id
  tags                = var.tags
  aliases             =  [var.url]
  
  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    acm_certificate_arn      = var.cf_certificate_arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2018"
  }

  logging_config {
    include_cookies = false
    bucket          = "company-logs.s3.amazonaws.com" 
    prefix          = "${var.environment}/b2c-cloudfront/"
  }
  
  custom_error_response {
    error_caching_min_ttl = 10
    error_code            = 403
    response_code         = 403
    response_page_path    = "/index.html"
  }
  custom_error_response {
    error_caching_min_ttl = 300
    error_code            = 404
    response_code         = 200
    response_page_path    = "/index.html"
  }
  ### BEHAVIORS ###
  default_cache_behavior {
    target_origin_id       = local.s3_origin_id
    viewer_protocol_policy = "redirect-to-https"
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    #cache policy already existing in AWS managed by AWS.
    #https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-managed-cache-policies.html
    cache_policy_id        = "658327ea-f89d-4fab-a63d-7e88639e58f6"
  }

  # Cache behavior for public content lke assets
  dynamic ordered_cache_behavior {
    for_each = var.cf_behaviors_public
    content {
      path_pattern           = ordered_cache_behavior.value
      target_origin_id       = "${local.s3_origin_id}/public"
      viewer_protocol_policy = "https-only"
      allowed_methods        = ["GET", "HEAD"]
      cached_methods         = ["GET", "HEAD"]

      forwarded_values {
        query_string = false

        cookies {
          forward = "none"
        }
      }

      min_ttl     = 86400 #24hs
      default_ttl = 86400
      max_ttl     = 31536000 #1y
      compress    = true
    }
  }

  ordered_cache_behavior {
    allowed_methods = ["GET", "HEAD"]
    cached_methods = ["GET", "HEAD"]
    path_pattern = "assets/*"
    target_origin_id = "${local.s3_origin_id}/public"
    viewer_protocol_policy = "https-only"
    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
    compress    = true
  }

  ordered_cache_behavior {
    allowed_methods = ["GET", "HEAD"]
    cached_methods = ["GET", "HEAD"]
    path_pattern = "avatar.ico"
    target_origin_id = "${local.s3_origin_id}/public"
    viewer_protocol_policy = "https-only"
    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
    compress    = true
  }
}
```
All right, there are many things here, and some things that could be done better. Let's go one by one:

#### OAI Identity

```
resource "aws_cloudfront_origin_access_identity" "default" {
  comment = "OAI for Public Site ${var.url} ${var.environment}"
}
```
The above is the resource used by the template for the access policy in s3. This is going to be
used also below:

We define the resource:
```terraform
resource "aws_cloudfront_distribution" "default" {
```
and we begin by adding the origins.

#### Cloudfront origins

```
  origin {
    domain_name = aws_s3_bucket.default.bucket_domain_name
    origin_id   = local.s3_origin_id

    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.default.cloudfront_access_identity_path
    }
  }

  origin {
    domain_name = aws_s3_bucket.default.bucket_domain_name
    origin_id   = "${local.s3_origin_id}/public"
    origin_path = "/public"

    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.default.cloudfront_access_identity_path
    }
  }
```
We will have two origins, one "regular" and one for assets that are going tom be cached.

If you worked with React, you know that when you "npm prod build" or something similar, I can't
remember exactly. It will create the "public" assets and place all of them on root-project-dir/assets.
That's what the second origin is for.

As you can see in the comments, we will need then later to define at least one behaviour per origin.

#### Cloudfront settings

We continue with some settings:
```
  enabled             = true
  is_ipv6_enabled     = true
  comment             = "CloudFront for Company site ${var.url}"
  default_root_object = "index.html"
  price_class         = var.cf_price_class
  web_acl_id          = var.web_acl_id
  tags                = var.tags
  aliases             =  [var.url]
```
The default root object should be the main file that you set in React, normally index.html.

The other settings are not much important. Actually, let's take a look at the values I've
put in teh variables, because I am using just defaults. This is in case I want to change that
in the future, and being "STUPID" (the P, premature optimization) - in case you doubt
it's not a good thing being STUPID, I just can bear with it handsomely. Feel free to change it to
hardcored values, it will make your life easier.

If not, add those variables on variables.tf
```
variable "cf_price_class" {
  description = "(Optional) - The price class for this distribution. One of PriceClass_All, PriceClass_200, PriceClass_100"
  type        = string
  default     = "PriceClass_100"
}
#TODO Maybe to convert to hardcoded value instead of variables
variable "web_acl_id" {
  description = "(Optional) - A unique identifier that specifies the AWS WAF web ACL, if any, to associate with this distribution."
  type        = string
  default     = ""
}
```
the final one "Aliases" is quite important. It must match the url of the site.
Add then this variable in variables.tf:
```
variable "url" {
  description = "The public site url"
}
```

#### Cloudfront restrictions
```
  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }
```
This is pretty much a default setting. Check the documentation. In my case I did not have to
touch anything here.

#### Cloudfront Certificate

```
  viewer_certificate {
    acm_certificate_arn      = var.cf_certificate_arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2018"
  }
```
You need to tell which certificate are you going to use for this cloudfront distribution.

The proper way would be to create a certificate using terraform. Now this certificate,
in my case, the one that will have a CN of cesc.cool for example, doesn't really belong
to that module. The very same certificate will be needed for all the other sites
or other things. This certificate belongs to the "global" infrastructure. Therefore, we
pass it as a variable, the ARN. You know what to do:

variables.tf
```
variable "cf_certificate_arn" {
  description = "The ARN of the AWS Certificate Manager certificate that you wish to use with this distribution. The ACM certificate must be in EU_WEST_3."
  type        = string
}
```

#### Logging config

```
  logging_config {
    include_cookies = false
    bucket          = "company-logs.s3.amazonaws.com" 
    prefix          = "${var.environment}/b2c-cloudfront/"
  }
```
You may notice my laziness here. This bucket is another resource that belongs to the global
IaC. I should pass this name as variable. I am just lazy and hardcoding this value. AWS will
actually create this bucket if it doesn't exist.

#### Custom error responses

```
  custom_error_response {
    error_caching_min_ttl = 10
    error_code            = 403
    response_code         = 403
    response_page_path    = "/index.html"
  }
  custom_error_response {
    error_caching_min_ttl = 300
    error_code            = 404
    response_code         = 200
    response_page_path    = "/index.html"
  }
```
Those are important. You need to add those custom error responses if you use anything like
react routing. You can make the experiment with my page, I am using react routing, and
I declared this page: "https://cesc.cool/books" now, there is no resource in S3 called
books, no directory, no file, I just want to serve index.html, React routing will do its magic
to serve the content I want to show. But the HTTP response should return the index.html,
which is NOT located in S3://bucket/books/

I tried several ways, but at the end the only thing that worked for me was to add this custom
error responses (or add copies of index.html files all around in S3). I don't like the change
of error code -> response code, but I guess it's a minor evil.

I am not sure if right now there is a better way to solve it. I'd say that eventually, you would
get out of static web page using cloudfront + s3 and jump to a node.js server that will serve
the proper responses. Right now we are just uploading a very simple React web page.

We will see later how to host more complicated bits of infrastructure.

#### The behaviours
```
  ### BEHAVIORS ###
  default_cache_behavior {
    target_origin_id       = local.s3_origin_id
    viewer_protocol_policy = "redirect-to-https"
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]

    #cache policy already existing in AWS managed by AWS.
    #https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-managed-cache-policies.html
    cache_policy_id        = "658327ea-f89d-4fab-a63d-7e88639e58f6"
  }
```
This is the main behaviour, it needs to point to an origin, that is set in the "local.s3_origin_id",
I am reusing an existing cache policy id, so I don't need to set everything. We tell it to redirect
from http to https. Also we only allow GET. It's a static web page, we won't even submit a webform
so no POSTS or other HTTP methods.

Then the next block for behaviours:
```
  # Cache behavior for public content lke assets
  dynamic ordered_cache_behavior {
    for_each = var.cf_behaviors_public
    content {
      path_pattern           = ordered_cache_behavior.value
      target_origin_id       = "${local.s3_origin_id}/public"
      viewer_protocol_policy = "https-only"
      allowed_methods        = ["GET", "HEAD"]
      cached_methods         = ["GET", "HEAD"]

      forwarded_values {
        query_string = false

        cookies {
          forward = "none"
        }
      }

      min_ttl     = 86400 #24hs
      default_ttl = 86400
      max_ttl     = 31536000 #1y
      compress    = true
    }
  }
```
We will strongly cache everything that React puts on the public directory.

You see this has "dynamic"? It means that one of this block will be created
`for_each = var.cf_behaviors_public`. The blocks will be identical, it will only change
the `path_pattern           = ordered_cache_behavior.value`.

Let's set this variable, variables.tf:
```
variable "cf_behaviors_public" {
  description = "CloudFront Behaviors for public content, like assets, fonts, images, which doesn't need extra lambda functions"
  type        = list(string)
  default     = ["static/*", "fonts/*", "favicon*", "humans.txt", "manifest.json", "robots.txt", "apple-app-site-association", "apple-touch-icon*"]
}
```
This is kind of overkill.. honestly, for my webpage, I am actually not using this behaviours.
But if you have a complex React website, this will help you greatly.

Then the actualy behaviours I use, that are slightly different than the block above
that's why I am not adding them on the "dynamic" list. The last two behaviour
blocks for publis/assets/* and for the favicon.ico.

```
  ordered_cache_behavior {
    allowed_methods = ["GET", "HEAD"]
    cached_methods = ["GET", "HEAD"]
    path_pattern = "assets/*"
    target_origin_id = "${local.s3_origin_id}/public"
    viewer_protocol_policy = "https-only"
    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
    compress    = true
  }

  ordered_cache_behavior {
    allowed_methods = ["GET", "HEAD"]
    cached_methods = ["GET", "HEAD"]
    path_pattern = "avatar.ico"
    target_origin_id = "${local.s3_origin_id}/public"
    viewer_protocol_policy = "https-only"
    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }
    compress    = true
  }
```

Note that except the first behaviour, all of the rest point to the "public" origin.

### S3 record

We are finally done. We just need a record to point to this cloudfront distribution.

Create a route53.tf file.

route53.tf
```
resource "aws_route53_record" "default" {
  zone_id = var.hosted_zone_id
  name    = var.url
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.default.domain_name
    zone_id                = var.cloudfront_default_zone_id
    evaluate_target_health = false
  }
}
```
This needs the last two variable that we need to set.
variables.tf
```
variable "hosted_zone_id" {
  description = "The hosted zone id to add the A record to point to the cloudfront distribution"
}

variable "cloudfront_default_zone_id" {
  description = "This is always the hosted zone ID when you create an alias record that routes traffic to a CloudFront distribution."
  default     = "Z2FDTNDATAQYW2"
}
```
The hosted zone id, that will come from the route53 module and the cloudfront zone id, which
is always the same.

Now that I am reviewing it, I actually don't like this `cloudfront_default_zone_id` as a variable. I'd prefer to have this hardcoded.
This value should be always the same. But, as you know, I accept myself with all my flaws too. I hope you do too. You can be better
than me by setting this as a hardcoded value in the same resource.

## Calling the module

All right that has been a slightly more complex module. We have something that looks like
this:

```html
    ├── modules
    │   ├── cloudfront-public-site
    │   │   ├── cloudfront.tf
    │   │   ├── outputs.tf
    │   │   ├── policy_S3_cloudfront.json.tpl
    │   │   ├── route53.tf
    │   │   ├── s3.tf
    │   │   └── variables.tf

```

Let's call it from staging and production

staging/main.tf + production/main.tf:
```
module "route53" {
  source                = "../modules/route53"
  environment           = var.environment
  domain                = var.domain
}

# Instantiating the new module:
module "company-website" {
  source                              = "../modules/cloudfront-public-site"
  environment                         = var.environment
  hosted_zone_id                      = module.route53.hosted_zone_id
  cf_certificate_arn                  = data.aws_acm_certificate.company.arn
  url                                 = "${var.environment}.cesc-blog.cool"
  bucket_name                         = "blog-guide-${var.environment}"
  tags                                = local.common_tags
  deployer-user-arn                   = data.aws_iam_user.github-deployer.arn
}
```

the module above we had it from Part 2. We just added the second one with the necessary
variables. Note the grammar to tell a dependency to another module in here: `  hosted_zone_id                      = module.route53.hosted_zone_id`

If you copied this part, you will notice that you don't really have yet
none of those datasources.

Create a file in staging, called datasources.tf:
```
data "aws_acm_certificate" "cesc-cool" {
  domain   = "company.cool"
  statuses = ["ISSUED"]
  provider = aws.aws-us-east
}

data "aws_iam_user" "github-deployer" {
  user_name = "github-deploy"
}
```
For the latter, remember is the user created in the global infra. As I said in part1, I think
there is a way to even point to a tstate file, but the approach I'm taking is to just
get what I assume is already there as a data resource. After all, it must be there, right?
We will see later when we deploy that for real using a github pipeline.

For the former, the certificate, I am again sinning of lazyness. The certificate could
be created by an independent module, or the global part - because as I have said, the certificate does NOT belong
to this module, but it's more on the global side. But just for this once, I had it already created it.

Now, there's a very improtant detail: CERTIFICATES USED BY CLOUDFRONT DISTRIBUTIONS MUST
BE CREATED ON THE US-EAST ZONE. So create that certificate in that zone!

Do you see this detail on the data resource `provider = aws.aws-us-east`? The way
to tell terraform's aws provider where to create the resources is by specifying it in
the same provider. If we need resources from other zones, we will need to instantiate different
providers. Do that on providers.tf:
```
#Needed because the certificates to be used for cloudfront must be on us-east-1 region.
provider "aws" {
  alias   = "aws-us-east"
  region  = "us-east-1"
}
```
That's it, now we can actually do terraform apply and deploy.

From part 2, you already have the makefile
entry for "plan-staging" and "plan-production".

If you execute those, you should see something like this at the bottom:
```html
Plan: 7 to add, 0 to change, 0 to destroy.
```

This has been quite a long post, so I will finish here. We will create the pipeline (both for the
infra and for the React project) on the next blog post.

Cheers!
