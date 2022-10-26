# A real DevOps

A friend of mine shared recently with me this article [https://leebriggs.co.uk/blog/2022/06/21/devops-is-a-failure](https://leebriggs.co.uk/blog/2022/06/21/devops-is-a-failure) and I agree definitely with the first part. DevOps is a failure. DevOps is an actual synonym for Ops and has nothing to do with Dev, unless you count the coding part on an Ops tool the Dev part. (And that's nothing new, bash script has been there forever - before I knew how to sum 2+2, so forever).

## My happy Story

In my case, the first time I heard the term DevOps was only 5 years ago or so, and it was used to describe that thing the developer didn't want to do. In that particular case, the reasons were two, the developer felt he lacked some knowledge to do that action and he sensed there was a "beautiful" way to achieve that, and a "DevOps" would most likely know how to do it in the appropriate way, that's what they did in their previous company after all.

I always worked differently, a personal preference, but I always wanted to do the Ops part. After all, that is completely necessary if you want to share your project with the public!

I didn't realize I was actually doing "DevOps" when I was configuring the Apache web server to serve my Symfony web page. We were LAMP developers, Linux, Apache MySql and PHP. The Linux part says implicitly a little bit of SysAdmin knowledge. The DevOps ancestors.

The real Sysadmin at the Japanese research university I was working in would prepare a server for me and I would be the king of that machine, doing and undoing as I pleased, guided often by the actual sysadmins. I remember adopting NginX quite soon since its configuration was way easier to understand than that of Apache, which examples on internet might become quite complicated to follow.

And I always thought of this as part of my responsibilities as a developer. Then fast forward some years, a move back to Barcelona, until I hear that comment from my colleague. I had been the previous years exercising as DevOps without realizing it, again. In an attempt to start having microservices in our startup, I had started some terraform code that would deploy a couple of ec2 instances into two different availability zones. I didn't know Docker, so I would use Ansible to provision those instances with everything needed to run our code. The deployment to production was a script that would ssh to those machines and git checkout the latest master. Aw yeah baby. And all of that without my DevOps degree.

Containerization (say it 10 times in a row) of the our code was a little bit painful, but breaking the monolith to smaller parts made it became possible. And that was when we had the first "real" DevOps. I was really happy about it, since he knew already, and we were yet learning. And that's how I always looked at the "real" DevOps a companion to the developer, like a consultant maybe. We as team, including the developers and the devops would decide together how to deploy to production, or to use SNS+SQS to communicate integration events between services, and many other things. The Devops would execute most of those plans, but I would feel capable of executing them too, be it by extending the Terraform code the DevOps set its base so nicely, or be it by changing the CI/CD, or whatever else you can think about.

## The cruel reality

It is true that that level of confidence was not shared with my fellow colleagues (therefore the comment), and it became clear that I was a weird animal when I moved from that start-up to the "big corporate" world.

I was shocked to realize that there was a full team of DevOps that were not in any development team, only in their DevOps silo! And developers didn't even know where the infrastructure code was! Developers didn't even have the means to understand where the requests to their code were coming from. You had to put lots of effort, ask many questions (and make friends) to realize how the infrastructure that hosted the whole platform worked, and don't you dare change it! That's not the developer responsibility! That's only the devops responsiblity, those people that have not been in any meeting where it's been explained what the code needs to do, and they need to support it and fix it if it goes badly, which was commonly the case? Nothing made sense to me and I was quite pissed at being handcuffed at work, we should be able to at least know how our infra looks like!

That was when I went across the [Project Phoenix book](https://www.amazon.com/gp/product/B078Y98RG8/ref=kinw_myk_ro_title), and it was scary how accurately my work experience was described - and I was a simple developer! -. I realized that many of those practices came from waterfall times. Those times that I missed because I was working in startups and which little knowledge I had came from what I had "studied" at university (yes, we had to learn how to plan properly using the waterfall method...).

I had to keep reading about it. With the [DevOps handbook](https://www.amazon.com/gp/product/B01M9ASFQ3/ref=kinw_myk_ro_title) I realized that the DevOps department at that company was pretty much doing all the bad practices for DevOps. The only DevOps they did was to deal with DevOps tools, which are Ops tools.

## The solution

I am not sure about the SoftOps from Briggsさん above -it's definitely better to what it exists now for sure- and the approach is from the right point of view: we have to work together.

I think part of the damage is seeing DevOps as a job title instead of what it is: a competence. We do this often. We limit ourselves by saying "I am a frontend", or a "backend". But actually all of these are skills, competencies.

As per Wikipedia:

> DevOps is a set of practices that combines software development (Dev) and IT operations (Ops)

You don't need to be DevOps, Ops, Dev, or well, anything, to use that set of practices.

I would like to empower us all to become real DevOps. If you are an operations worker, you should be able to code some business needs too, and do some peer reviews. If you are a developer you should be able to create the infrastructure of a new microservice all by yourself and deploy it to production. Let's blur this line and stop using boxes to limit ourselves.

We could extrapolate that to backend and frontend too, and I would say, on you go! After all, the desired team is such team that its members have T-shaped skills. And that's not me - a nobody- who says it, this is from [Essential Scrum](https://www.amazon.com/gp/product/B008NAKA5O/ref=kinw_myk_ro_title).

The trick here is work together as a team. Each expert can teach the best practices on its own field and all of them can learn from each other. Of course that can't happen if the teams are separated by competencies (it feels stupid now to have the teams like this, right?).

That's half of the solution, the other half is doing some inside work and actually wanting to learn some stuff from the other people in our team. We all should feel capable to reach a certain level of competency on each area. Maybe we feel more comfortable just developing some backend code, but we should at least know how the other parts of the system work. We are smart enough! (well, you can discuss my case here, not too hard, I have feelings) The others should be able to break it down to us so we can understand it. It might become our responsibility. And who knows, we might like it even better than what we currently do. Then we will feel empowered to give actual real solutions.

Let's become real DevOps!