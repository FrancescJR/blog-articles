# The dead of Scrum

It all started when a friend of mine that works in a cool place - to me that place looks like super duper advanced and that the use very cutting edge technologies - said
that they might follow some practices from [Shape Up](https://basecamp.com/shapeup). So I eagerly sent the pdf version
to my Kindle and devoured the book in about 3 nights. It's nicely written, so it doesn't get dense.

At the beginning of the book it says that this is not Scrum or Agile, but I beg to differ. Many of the values explained
in this book are core values of agile. After all, both methodologies are trying to solve the problem of: 
How to ship robust software to the final user in the fastest possible way.

So dividing the work in vertical slices for complete pieces of functionality, small, and being able to ship them or finally
"complete" them. Or setting a fix scope and giving it the time and respect -without interruptions- it deserves. Both
are examples that you can find in both methodologies.

After all, from [Essential Scrum ](https://www.amazon.com/gp/product/B008NAKA5O/ref=kinw_myk_ro_title) - the book where
most of my knowledge of Scrum comes from - it says:
> Being highly proficient with Scrum and thus being more agile, however, is not the end goal, but rather a means of
> more effectively and economically achieving business goals. [..]
> Just as no one can tell you where your Scrum implementation should end, no one can lead you down a predefined
> path that will guarantee success. Instead, you must learn, inspect, and adapt your way forward base on your organization's
> own unique goals and culture and the ever-changing complex environment in which you must operate.

If you keep doing Scrum, and acting on the issues that came up on the retrospectives, you might eventually
end up on the Shape Up process. That's what I believe it happened at BaseCamp. Of course some core concepts
are fundamentally different, but the same good foundations are still there.

I am saying that because I don't think Shape Up is the silver lining for all the companies to follow, but rather a 
solution that worked extremely well at Base Camp. On here I am totally on the side that "no one can lead you down
a predefined path that will guarantee success".

I also find some critical aspects for shipping good quality software, concepts coming from the "DevOps revolution",
(the one described perfectly here on the [Project Phoenix](https://www.amazon.com/gp/product/B078Y98RG8/ref=kinw_myk_ro_title)),
completely missing. For example, acceptance tests that validate the "pitch" could have been added. And other continuous
integration practices would make even more sense. 

The examples of software being delivered in the book look maybe a little too simplistic, which made me wonder how they
tackle way bigger projects, and how they deal with technical debt. Per the discussion and screenshots, it looks like
it all is based in kind of "monolithic/crud" approaches as opposed as event driven architectures, or rich domains. With
those approaches technical debt will for surely creep and make its way, until the six weeks cycle they defend won't be 
enough unless they don't add a refactor before starting. Or possibly the team they have as "support" (the enabling team
explained at [Building Microservices](https://www.amazon.com/gp/product/B09B5L4NVT/ref=kinw_myk_ro_title)) takes care of that.
Those are more technical aspects that the book didn't need to address though.

But this book gives a fresh and new approach to software development that will be for the liking of many. Its procedures are way simpler, way
less overhead than Scrum, and brings some novel concepts that are, I believe, the proper way to look at things: forget about estimates,
make the proper bet, and commit to it. You bet, you don't estimate. - For that you need to "shape" properly, which is
basically the most ambiguous part of the book, but I accept that, and to hammer the scope, constantly, even at building
phase, to make sure you are in the bounds of your bet (similar to the Scrum sprint where you are not supposed to work
on things that are not on the commitment)-.

Leaving the team breath and face the problem on its own is also the right decision, as opposed as the constant tracking
of Scrum with dailies, task moving and so on. That's how you properly delegate. Maybe there are some processes to "top
down" for my taste - some decisions that should be accorded with more people of the organization, but well, that's just personal taste.

Other brilliant ideas are the "scope mapping" and the "hill chart" to track the project progress. Once it's explained
in the book, you see clearly, that the grooming of Scrum makes no sense. Not that it makes no sense, but it becomes
apparent that it is too difficult to do it right. Even impossible. Let me develop that:
---
## The impossible problem

On Scrum, the work is divided into pieces, user stories, or however you want to call them, that are at 
the beginning at very high level, without much concreteness and very hard to estimate at this point in time.
Normally people would estimate them with those fuzzy ambiguous t-shirt estimations.

Then those tasks go through a process of grooming and got divided into smaller tasks, that should make sense
on their own, and are easier to estimate, so you finally get a complete whole estimation for the big task.

Scrum here misses a very big point that Shape Up got right. You don't know what you don't know yet. That's what
they call going up-hill, when you go do proof of concepts for new stuff, or investigate, and you go dirty in the code
to figure out how to approach the problem. On Scrum all of this work is minimized and assigned fully on the
grooming meeting. And I would say this work is actually the most important part! It's good to put more than one brain
going up-hill, but making it a crew process is probably
wrong, social politics will play a higher input on how to solve a problem, instead of the actual problem-solving skills.

Once you figure out what to do, then is when you go "down hill" on the Shape Up process. On Scrum that would be
when you have those tasks groomed and a clear "todo" list. That work is probably the most unimportant, and one
that when you do it, you often think "why don't we hire an intern to do that?", but the one that Scrum puts the 
focus on. The cool problem solving part has been pretty much ignored, and assumed that everybody knows
what needs to be done even almost from the T-shirt estimation time.

We also lost a lot of time with Scrum poker trying to estimate those small tasks, jumping our way forward
without proper thinking, (boring quite a bunch of people in the process).
And all of this estimation are for what? to realize that the T-shirt estimation was
completely off-track? What's the value, once you are all dirty in the problem, to know that it will take longer
than expected?

All of this is to tell the stakeholders how much a piece of software is worth. And that is the impossible problem.
On Theory of Computation, the main [semi-decidability problem](https://www.geeksforgeeks.org/decidability-semi-decidability-and-undecidability-in-toc/), the problem where you reduce other problems 
to prove that they are actually semi-decidable, is the problem that states something like (it's been a long time that
I coursed that, give me some leeway) "given a turing machine, and an input which is the same turing machine, will
the turing machine halt when processing this input?". How long will it take to complete that piece of software could be reduced to this semi-decidable problem. If we 
were Turing Machines, we might never halt, we don't know what we don't know yet. So this problem has no answer.

Now, that is very inconvenient, mainly because our salaries depend on that. We kind of need to tell the stakeholders
or our clients, how much something is worth. Both Scrum and Shape Up solve that partially
by scoping very strictly the problem, so we are more likely to "halt", (and also we don't waste time on non-core features
or nice to haves). Scrum goes an extra mile by adding a huge overhead with all the grooming process that Shape
Up simply avoids by more strict scoping and changing the concept from estimation to a "bet" (a word that gives
more emphasis on uncertainty that "estimate" does). 

And going that extra mile, was really worth it? The only case I can think of is when you need to bill your clients for
the time spent, but for that, estimations should be avoided and rather the actual time tracking should be used.

Yes, you might want to use the estimates to confirm with the client whether to go ahead with the project or not (doesn't
that sound like a bet). And then having the discussion of "yeah, it actually was way more expensive that we said, sorry!".

The impossible problem.

---

The best thing is that, apparently, you don't need Scrum to ship good quality software to the end users. It's been probably
been a while that everybody knows that Scrum is not the golden panacea for software project management. Dailies make
none to little sense (why is it relevant what I did or what I am going to do? maybe I've been figuring out something,
or done a proof of concept that I will throw to the trash. And if I'm blocked, well, shouldn't I have said something
yesterday when I discovered that I was blocked?). How many "ceremonies" are during the small span of a sprint? And
how many of them are actually useful? Probably only the retrospective. For the groomings, well I find Shape Up approach way better. Also, I always found very disturbing that
the cycle is called "sprint". Have you tried sprinting over and over and over?

Maybe the problem with Scrum was its own success, with its certificates, trainings and all of this money machine. Becoming
Scrum certified surely was expensive enough to convert the trainee to a Scrum Zealots that puts way more
emphasis on the "how" instead of on the "why". The same disease affecting the Scrum certified wannabes that would protect
Scrum rules above all else, ask them on interviews -you can always check them out...- 
or bark them at you when it surely made no sense, forgetting completely the big picture. We all might have done that
at some point or another.

So having a proven methodology that challenges Scrum, it can only be welcomed with delight. It opens the door to
so many possibilities and frees from what it looked like the only way of working in software development. And to my surprise, there
are other tangential initiatives that challenge Scrum like https://unfix.work/. 

All of them sell themselves as non-agile, but I am sure the core values are still there. A colleague of mine 
put it very well: "the Scrum hype is passing away but the good foundations will remain". As long as we remember
the "why" and the lessons learned, we will find our unique path.

Don't forget this lesson from Scrum:
> You must learn, inspect, and adapt your way forward base on your organization's
> own unique goals and culture and the ever-changing complex environment in which you must operate.



