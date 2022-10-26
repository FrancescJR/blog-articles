# Self Hosted infrastructure on AWS with Terraform (part5)
## The React project simple pipeline

Finally the last part of this "season"!

Now that we have a pipeline that build our infrastructure, and we have our infrastructure in place, it is time to use this infrastructure!

We had the module that hosted our React side with Cloudfront and S3. Now it's time
to add the pipeline in our React web page.

This is what I am using for real for my web page, and you can check the code [here](https://github.com/FrancescJR/cesc-cool). It's a simple React based static web page.

## The simplest of the pipelines

I've spent quite a great deal of time on the last blog post excusing myself on the whys and hows I was doing the pipeline that way and the caveats and all of that. Fortunately, for the React website, there's no need for excuses as I am following the lean, DevOps, Agile and all of those principles for real.

Remember that this is a simple React static web page, so it doesn't need any backend. The
project can be hosted on its entirety on S3. That's why the workflow to devevelop this
page is also going to be very easy.

What's the preferred workflow? Trunk based development. (Read [all](https://www.amazon.com/gp/product/B003YMNVC0)
the [books](https://www.amazon.com/gp/product/B01M9ASFQ3/ref=kinw_myk_ro_title) if you don't
believe me.) We won't even bother to create branches or anything, we are the only
ones developing on this project, so let's get rid of all the extra nuisance. We will
just get our copy of the code, change it, commit into main directly and push it.

The pipeline needs then to be triggered at any push on main, and deploy to production.
As simple as that.

.github/workflows/deploy-production.yml
```
name: Deploy PRODUCTION
on:
  push:
    branches:
      - main
jobs:
  deploy-production:
    runs-on: ubuntu-latest
    steps:
      - id: EniltrexAction
        uses: eniltrexAdmin/deploy-static-aws-cf-s3-site-action@v1.0.1
        with:
          aws-access-key-id: ${{ secrets.AWS_DEPLOYER_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_DEPLOYER_KEY_SECRET }}
          aws-region: eu-west-3
          aws-s3-bucket-name: ${{ secrets.AWS_S3_BUCKET_NAME }}
          aws-cf-id: ${{ secrets.AWS_CF_ID }}
```

That's an easy pipeline. I am just using an action I wrote to do precisely that.
It needs for some specific variables, credentials for the 'deployer' and some AWS infrastructure bits, the cloudfront ID and the S3 bucket. You will see later for what are they being used.

For the credentials, you might remember that I created, on the other project, the infrastructure project, in the global part, a user called "deployer", so that's the one. It is kind of a security measure. The user that can deploy my react webpage, can deploy my reactpage only, but not the whole infra of my company. All of this is actually up to you. You may use the same user for everything. After all, it's a small company with only 1 employee!

Let's take a look at the action. The code is [here](https://github.com/eniltrexAdmin/deploy-static-aws-cf-s3-site-action).

Another disclaimer is in place here. I am quite sure I followed the instructions on
how to do an action, but whenever I see other actions they look way more complete and
with tests, etc. Tests are good, but other stuff seems to me overcomplexity, like
making the action to run on all kinds of OS or other stuff. Anyway this is to say
to not take me as a guide for writing your GitHub action, there are probably better
places for that.

All right this is the important part of the action:
```
runs:
  using: "composite"
  steps:
    - name: Check out repository code
      uses: actions/checkout@v2
    - run: echo "💡 The ${{ github.repository }} repository has been cloned to the runner."
      shell: bash
    - name: NPM install
      shell: bash
      run: npm install
    - name: NPM build
      shell: bash
      run: npm run build
    - name: List files in the repository
      shell: bash
      run: |
        ls ${{ github.workspace }}
    - name: Configure AWS credentials
      uses: aws-actions/configure-aws-credentials@v1
      with:
        aws-access-key-id: ${{ inputs.aws-access-key-id }}
        aws-secret-access-key: ${{ inputs.aws-secret-access-key }}
        aws-region: ${{ inputs.aws-region }}
    - name: Copy files to s3 bucket
      shell: bash
      run: |
        aws s3 sync build/ s3://${{ inputs.aws-s3-bucket-name }}/public/
        aws s3 cp build/index.html s3://${{ inputs.aws-s3-bucket-name }}/
    - name: Invalidate cache so CF stops serving old non existent versions
      shell: bash
      run: |
        aws cloudfront create-invalidation --distribution-id ${{ inputs.aws-cf-id }} --paths '/*'
```
Summing it up, it installs npm, builds the project, syncs s3 with the resulting build
and finally invalidates the Cloudfront cache (that's why we need the Cloudfront ID
as variable).

This is an action because I am reusing it for all of my static frontend sites, like this
awesome [minesweeper](https://minesweeper.staging.eniltrex.biz/) 🤣

That all looks simple and easy. And yes it is. No need for extra complexity here. We
work so hard on other place so we could have an easy and fast deploy to production here.

You might notice we are missing tests here too. Indeed a big flaw, that you can
think on how to solve it. Run the tests before executing this action!

## The workflow in action

As a matter of fact, I just edited my own web page, and I did just that on my localhost:
```
git add src/Pages/BookContent.tsx
git push origin main
```
after a couple of minutes, the changes were already in [production](https://cesc.cool)!

If I wanted to check what GitHub did, those are the screenshots:


![image.png](https://cdn.hashnode.com/res/hashnode/image/upload/v1665423847612/exMtH5X7A.png align="left")


![image.png](https://cdn.hashnode.com/res/hashnode/image/upload/v1665423921302/mR7rDGHIJ.png align="left")

## Closing

Despite my mumbling, rambling and writing too much in general, I will always advocate
for simplicity. There are places where I am not following my own advice, but I hope its
clear what should be done instead. In case of doubt, don't make systems more complicated
thinking that's "smart", it never is smart. Complicate them if there's no other alternative
but a little of pondering will often tell you otherwise.

I hope that you feel ready and confident to host your own infrastructure on AWS now. I know that I might have skimmed some parts, don't hesitate to ask for clarifications in the
comments. All of these I've shown here is what I actually use to host all of my projects. I did try and re-executed many things in order to write these articles (most of it is idempotent!) but there is a high chance that I have missed something that I had in the code and I forgot to tell or explain. Again, don't hesitate to ask questions in the comments.

I am developer, but I do enjoy the operations part. I hope to give the Ops power to all
the devs, at least make you loose some fear about that aspect. It's awesome if we have the privilege to ask to an experienced DevOps, but if we can't, that should not stop us to deploy and share our work to the world.

It's time for a break on this series and to focus on CQRS+ES for a while.