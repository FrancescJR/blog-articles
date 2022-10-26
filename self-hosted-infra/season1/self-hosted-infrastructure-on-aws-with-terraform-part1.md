# Self hosted infrastructure on AWS with Terraform (part1)
## Complete guide to have a self hosted static website hosted in S3 + Cloudfront with Terraform in the context of a small company/self employee.


I am going to explain how to create the whole infrastructure and continuous deployment for simple projects that you might have.

This article is for people who want to have control of the infrastructure and understand what's going under the hood instead of leaving this part to platforms such as github webpages, this very blog, or other simple solutions. If you are part of a small company, you can also use this very guide to start the beginning of your infrastructure as code for your company.

We will start on the free tier, that means that the only thing we can actually deploy there is a static website. That can be perfect for you if you only have a static web site! :D Wait for more complex examples such as deploying lambdas (which, hey, they will for free too), or ECS and EKS in following posts.

I would like to reclaim for all of you the term DevOps. Right now this term refers only to Ops people. But for all of you Devs that do things, let's learn how to deploy our own projects to production, so we honor the actual term DevOps and become one. Developers that do operations.

By the way, I've never held the title of DevOps, but that never stopped me of deploying my projects to productions, nor doing the infrastructure work necessary to bring the development work to production in the companies I worked in. (With the notable exception of big companies that were too immobilized by rules, which hey, isn't one of the things the [DevOps handbook](https://www.amazon.com/gp/product/B01M9ASFQ3/ref=kinw_myk_ro_title) argues against?. Or creating the pipelines necessary to deploy the latest changes to production.

Here we will see how to deploy an static web site developed in React to S3 + Cloudfront using Terraform and Github actions. The guide here is basically what I am using to host and deploy the changes in my personal webpage: [https://cesc.cool](https://cesc.cool) (the domain is "cool" but I am not really cool- it's just to fool you)

Ok without further a due let's start. Let's Terraform the shit out of our infrastructure. Let's create the code that will generate the infrastructure of our small sized company.

## Scaffolding the infra code.

You will need the following:
- an account in AWS (and a little bit of knowledge about it)
- a Github account
- a ready to deploy web page
- Terraform installed

Now, I am going to be frank, I am going to explain the way I normally do it, or did it, so I know it worked. That way is not necessarily the way that it should have been done. It's just the only way I can guarantee is gonna work. Call me lazy, I might not want to try out the proper way just to write a blog post - but I might help in the comments-. When I go through this parts, I will let yo know.

We will follow the folder structure explained in [Terraform up & running](https://www.oreilly.com/library/view/terraform-up/9781492046899/). That folder structure is a good thing. Book, good. Not follow book, not good. Book explain many good things. Me learn a lot from book. You too should read book.

So, create a brand new project, for the sake of naming things, so I can refer back to them, we will call that project company-infra, and it will have this top level directory structure:

```
├── .github #we will use github actions to deploy our infra.
├── Makefile  #not really a fan of that but useful
├── README.md 
└── terraform
    ├── global
    ├── modules
    ├── production
    └── staging
```
Ok, the book doesn't state that code should go inside a dir called terraform, but that's how it happened in my case. The "proper" way is to get rid of that level that it's already implicit as the whole project is infra as code with terraform.

### The modules

What are gonna be our modules? Our project so far, is the simplest project: a static web site developed in React. We said we want to host that using S3 and Cloudfront since it's fast and free. So let's have a module that does just that. Let's call it "cloudfront-public-site".

Maybe it's early to ask this question, but are absolutely all the resources dependent of this module? For example, if we reuse this module to deploy not one site, but two sites!, will this module need everything we need?

As it turns out, - and we will see later- this module needs some input from other sources. It needs to know the hosted_zone_id  (you might not know at this stage, but I am just advancing you the news) of the route53zone.

Actually, we realize that for production, our site will have this domain: site.company.com, and for staging, we want this domain: site.staging.company.com. The dealing of this domain is maybe small, but reason enough to have another module.

It's decided, we will have two modules, so let's create just the directory structure on modules like this:

```
    ├── modules
    │   ├── cloudfront-public-site
    │   │   ├── ...
    │   └── route53
    │       ├──...
```

But now let's switch focus and step back:

### The global part

The "global" part is where the shared resources are. I want you to reflect what "shared" might mean with this directory structure. The right conclusion to get here is that shared resources refers to those resourced that do not vary by environment. That for both staging and production we will use the same.

At this stage I just want an AWS user/role that has enough permissions to deploy my awesome projects. That resource doesn't necessarily need to be shared. You might want to use different users/roles per environment - actually a good idea-. But we are such a small company -probably a single person company- that we don't want to make our own path too complicated for ourselves.

So let's create some files under global:

```
├── global
│   ├── main.tf
│   ├── outputs.tf
│   ├── providers.tf
│   └── variables.tf
```
Having a providers here we are telling that we will have a tfstate saving the state of out infra independent of the rest of things that are in different places of our file structure.

The providers.tf:
```
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
I am not sure I need to explain this one much right? The only thing you might change for the better is the "required_version", use a fixed version (`required_version = "1.0.61"`) , and never move. It's the safest. What you have to think when you do that? that all the machines that are going to execute need installed that exact version of terraform. That's all. (At the moment of writing latest Terraform is 1.2.8, I am still using 1.0.6 since, again, it's the only way I can guarantee the whole thing is gonna work).

Another thing is that I should use a module here instead of plain resources, but hey, this way you learn how to do it both ways!

So the variables for this global part are the following, variables.tf
```
locals {
  //Put all common tags here
  common_tags = tomap({
    "Terraform" = "True"
  })
}
```
which is to say no variables and I am setting a "common" tag that I will reuse for many resouces. (Btw, you might want to google that a little, maybe you can add that at the provider level - you can do that using cdktf*, so probably the same could be done for terraform).

And finally the resources, main.tf:
```
resource "aws_iam_group" "deployers" {
  name = "deployers"
}

resource "aws_iam_user" "github-deploy" {
  name = "github-deploy"
  tags = local.tags
}

resource "aws_iam_access_key" "github-deployer-key" {
  user = aws_iam_user.github-deploy.name
}

resource "aws_iam_user_group_membership" "deployers-membership" {
  user = aws_iam_user.github-deploy.name

  groups = [
    aws_iam_group.deployers.name,
  ]
}
```
This is to create a user, a group, the membership relationship and a key.

## Trying with localhost

We already wrote some terraform code, and we should add tests to it. But we did such a terrible job that our code is not even modular, it has no examples or such. This guide is not for testing terraform code. For that I recommend highly the book again.

But what we can do is execute it directly without testing! How exciting is that huh? I mean, if we take a look at the code, what's the worst thing that can happen? that we create a wrong user, so I don't think the stakes are that high. (FYI users are free in AWS).

Here's where the Makefile comes into play (it's a little bit annoying to deal with them... if you have trouble, copy paste the empty spaces - which should be a tab.). What do we have in our Makefile:

Makefile:
```
plan-global:
	cd terraform/global; export AWS_PROFILE=company-terraform; terraform init \
	    -backend-config="bucket=company-infra" \
        -backend-config="key=infrastructure/global/terraform.tfstate" \
        -backend-config="region=eu-west-3"; \
	terraform plan
```

Before going all crazy and execute `make plan-global` let's take a look.

This is a personal preference, but I really like to deal with AWS credentials with the `~/.aws/credentials` file, that should have this entry:
~./aws/credentials:
```
[company-terraform]
aws_access_key_id = AKIAAAAAAAAAAAAAAAAKKO
aws_secret_access_key = yPijsdfwefsdsqwerwefsdfwefouDfm
#if you put here the region, you might want to remove it from the makefile. But I haven't tried this way, so I can't guarantee it's gonna work.
```
with your own key and secret of course. (Also, make sure you don't use your root user, create another user, give console access - which is the same as having those credentials).

What else do we see in the Makefile? The place where we save the state is a bucket called "company-infra". You can go and create it in AWS s3. You may want to be meta and try to use terraform for that, but I don't want to spend the time to find out if that's possible - I'd browse the book- just do one thing manually, you won't die for that. Being meta is actually bad. Too much complexity. We want explicit things. Explicit things can be changed.

All right, once we have the ~/.aws/credentials file (hey, you know ~ means your home directory right? you can do `cd ~` and then `pwd` and you will know where that is), and the bucket, go ahead go on the root of the project and execute `make global-plan`. You should see how terraform would like to create a user and a group. Well done!

All right. Next step is to create those modules. On part 2.  
