# Self Hosted infrastructure on AWS with Terraform (part2)
## The first Terraform module

This is a continuation from [part1](https://blog.cesc.cool/self-hosted-infrastructure-on-aws-with-terraform-part1).

It's time to write our first module. Modules are Terraform solution to reuse code. It's a
great way to keep your infrastructure as code in understandable pieces that can be
then grouped together. You can develop those pieces independently, have different people
working in parallel, test independently as well all of those pieces, and then you
will use them like using lego pieces to build our whole company infrastructure.

In our specific case, since we decided that we will have two environments, production and staging,
the only way for us to not repeat the same code among those two environment is to use modules.
We are in *neeed* of modules in that case to reuse the code, and it will also force us to
think what bits are environment dependent and what bits are logic that belong to the module.

If you remember from the last post, also because I gave the spoiler, we know that we need two independent modules for our whole
company IaC. We will start with a very easy module just to learn how modules work and on the next
part we will attack directly a more interesting module.

## route53 module

Let's start by giving excuses to this sucky name. You might want to think more on
"domain" terms (even though we are just doing infra here!) and call it something like
"NamingDomainModule" or something that might imply we are not totally chained to AWS.
It's all your choice. Sometimes if you don't want to use some grey matter to name
things then the default name should become what that thing actually is, and in this
case that "thing" is a "module that manages route53 resources", so forward with this
name! (and yes, I am totally chained to AWS, I can write a guide if we are using AWS,
I would need to find a guide if we were to use Google Cloud or others).

This module will be very simple:

```
    │   └── route53
    │       ├── outputs.tf
    │       ├── public.tf
    │       └── variables.tf

``` 
variables.tf is the place were you define what kind of input this module receives
(and for more things, but mainly that). Public.tf is where we are gonna write the resources
and the outputs.tf are well, yes, the outputs that we want this module to produce.

Let's write the resources in out public.tf. Again, the name of this file, is king of random.
If I have to be honest, I am just walking you through some -not that- old code of mine, and
while I can maybe fix some stuff, I think it's cute to leave some flaws as they were written.

public.tf
```terraform
resource "aws_route53_zone" "business" {
  name         = var.domain
  tags         = local.common_tags
}
```

Well, that was rather short. Only a zone that we will call it "business" with the required
attribute name that will come from one of those inputs from variables.tf and tags.

So let's define those variables that we will require the programmer - also us- to give us when using our
module:

variables.tf
```terraform
locals {
  common_tags = tomap({
    "Environment": title(var.environment),
    "Terraform": "True"
  })
}

variable "environment" {
  description = "The name for the environment, usually staging, production or development."
  default     = "staging"
}

variable "domain" {
  description = "The main domain"
}
```
The variables block are self-descriptive. The interesting part are the locals. Locals
are used for values computed from variables mainly. In that case, we are creating a
set of tags and using one of the variables, the environment, to create it.

outputs.tf
```terraform
output "hosted_zone_id" {
  value = aws_route53_zone.business.id
}
```
From the only resource we have created, we get it's id. I am just advancing you that we will
need that.

### About tags

Tags are kind of very important. It's extremely useful to know which resources are managed
by terraform, (all of them, right?!) and tags is a very human readably way to know that
when you are navigating on the AWS web console.

I said in [part1](https://blog.cesc.cool/self-hosted-infrastructure-on-aws-with-terraform-part1) that there might be a way, on the provider.tf, to add default tags to all the
resources created using terraform apply, but I haven't googled that yet, nor try it, so get
used to see many "tags = var/local.tags" in my code.

EDIT:

The way to add default tags is... no, I haven't googled that yet. But saving the space for
when I do...

### About files

To have a file for variables, outputs, and others is some kind of unwritten convention (I think).
I largely stole it from the [book](https://www.oreilly.com/library/view/terraform-up/9781492046899/). It's useful, you don't really want to jam everything in a
single file, but sometimes I have the feeling it might not be the most useful way to break down
the resources.

A file for variables and for outputs makes a lot of sense. But you might see on other modules
somewhere the division for type of resources, and sometimes that makes for some complicated
understanding, especially when resources depend on each other, and they are located in
different files.

## Call our module

Now that we have our first module, let's use it.

Let's begin with the staging environment, we will create some files that will resemble a
module:

```
    └── staging
        ├── datasources.tf
        ├── main.tf
        ├── providers.tf
        └── variables.tf

```
Let's just ignore datasources.tf for now.
If you remember from the last post, on providers.tf we will put some boilerplate code that
we are gonna use terraform:

```terraform
terraform {
  required_version = ">= 1.0.6"
  backend "s3" {
  }
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.0"
    }
  }
}

provider "aws" {
  region = "eu-west-3"
}
```

The file is basically a copy of the "global" one. (We will add something different here
on the next post).

That might "smell" to some people. Isn't there no way to avoid writing the exact same?
actually there is, with TerraGrunt (explained in the [book](https://www.oreilly.com/library/view/terraform-up/9781492046899/)). For our small infra, though,
we will just accept it. We do need though this file, because we are going to have a main.tf
here, because we gonna have a tf.state that will manage just this part of our Infra.

And the important part about it is that we want this tf.state to manage just this and only
this part of our infra, without ever touching other parts of other infra. We don't want a deploy
to staging to touch any resource that belong to production! That tf state should not even
know that there is an infra for prod. And that's what we are doing by separating our IaC
in this file structure and by "duplicating" this providers.tf (we could actually
change it, but the point of staging is to try things before prod, so you want staging as
similar as possible to prod.)

All right, enough excuses, let's go with the main.tf:

The concept goes like this: on our environment, staging/production, we decide we want
to have a domain called X. So let's try to code that intention.

main.tf:
```terraform
module "route53" {
  source                = "../modules/route53"
  environment           = var.environment
  domain                = var.domain
}
```

we add a "resource" which is actually a whole module. Now we pass it what the module input
requires, the environment and the domain. The source has to be there always.

Since we will use this domain and this environment quite often, we made them variables too.
In this case, more than inputs, they are actually that, variables, well, I'd rather call
them "constants" but let's not make it more complicated.

variables.tf
```terraform
variable "environment" {
  description = "The name for the environment, usually staging, production or development."
  default     = "staging"
}

variable "domain" {
  description = "Main DNS Domain"
  default     = "staging.mycompany.com"
}
```
you might realize I cope paste descriptions.

### Call it from production

Let's mimic the same for production:
```
    ├── production
    │   ├── datasources.tf
    │   ├── main.tf
    │   ├── providers.tf
    │   └── variables.tf

```

the providers.tf is the exact same. The main.tf?
main.tf
```terraform
module "route53" {
  source                = "../modules/route53"
  environment           = var.environment
  domain                = var.domain
}
```
well for this small module, it's the exact same code, but the variables of course are different:
variables.tf
```terraform
variable "environment" {
  description = "The name for the environment, usually staging, production or development."
  default     = "production"
}

variable "domain" {
  description = "Main DNS Domain"
  default     = "company.com"
}
```
So we have our prod and infra base IaC in place!

## Trying it out from localhost

As we tried our "global" part, we do a shortcut and use the makefile to be able
to execute our terraform code. This will actually be executed by the pipeline, yet to
be explained.

What I might not have explained properly is that on the makefile, we want to make sure
that we will use the tf.state file that the same pipeline is gonna use.

And that we need to swear on a very cute cat and a dog that we will be responsible when doing
things from our local. We should strive to let the pipeline do the job. We are just
adding this shortcut because our company has only one employee and can do and undo as they
sees fit.

(on a small note, we can avoid dealing with tf state lock logic - which should be easy to add
but I am not explaining it here- if we are sure that all the deploys are done by the pipeline,
single sequential pipelines,
and not by random devs at random, often conflicting, times)

Let's add this new 2 entries in our Makefile:
```makefile
plan-staging:
	cd terraform/staging; export AWS_PROFILE=company-terraform; terraform init \
        -backend-config="bucket=company-infra" \
        -backend-config="key=infrastructure/staging/terraform.tfstate" \
        -backend-config="region=eu-west-3"; \
	terraform plan

plan-production:
	cd terraform/production; export AWS_PROFILE=company-terraform; terraform init \
        -backend-config="bucket=company-infra" \
        -backend-config="key=infrastructure/production/terraform.tfstate" \
        -backend-config="region=eu-west-3"; \
	terraform plan
```
See that we are using two different state files for prod and staging. We are using the
same bucket we created (manually) for the global infra. No need to create another one.

You can now see what would happen if you execute `make plan-staging`.

The output should be something like this:
```text
Terraform will perform the following actions:

  # module.route53.aws_route53_zone.business will be created
  + resource "aws_route53_zone" "business" {
      + arn           = (known after apply)
      + comment       = "Managed by Terraform"
      + force_destroy = false
      + id            = (known after apply)
      + name          = "staging.company.com"
      + name_servers  = (known after apply)
      + tags          = {
          + "Environment" = "Staging"
          + "Terraform"   = "True"
        }
      + tags_all      = {
          + "Environment" = "Staging"
          + "Terraform"   = "True"
        }
      + zone_id       = (known after apply)
    }

Plan: 1 to add, 0 to change, 0 to destroy.
```

Let's just add two more entries in the Makefile:
```makefile
deploy-staging:
	cd terraform/staging; export AWS_PROFILE=company-terraform; terraform init \
        -backend-config="bucket=company-infra" \
        -backend-config="key=infrastructure/staging/terraform.tfstate" \
        -backend-config="region=eu-west-3"; \
	terraform apply
	
deploy-production:
	cd terraform/production; export AWS_PROFILE=company-terraform; terraform init \
        -backend-config="bucket=company-infra" \
        -backend-config="key=infrastructure/production/terraform.tfstate" \
        -backend-config="region=eu-west-3"; \
	terraform apply	
```
Just for this once, you may want to execute and say 'yes' at the prompt. I mean, it's just
a resource, not a lot of harm! (you may also want to add an entry with `terraform destroy` action)

You can go to AWS console, the route53 service, and you will see listed your recently
created hosted zones by terraform. And you didn't even do a single mouse click! (mouse
are for noobs, keyboards for pros!)

All right on the next chapter we will do a module with more consistency. You have the basics
here though to do whatever you want already!