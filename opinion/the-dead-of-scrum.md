# The dead of Scrum

Initially I thought of calling this blog post "Shape Up, a fresh view to software project management". But the devil
of the clickbait was stronger.

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
apparent that it is too difficult to do it right. You have to write the tasks with the right level of concreteness to let them
be groomed afterwards. At this point you need to discover things about that task
that you didn't know before - that would be going up hill on Shape Up-  which by the way is work that is not shown anywhere
in Scrum, just as a bunch of people guessing in the grooming meeting without actually getting the hands dirty on the code
to make sure the approach is possible or not. Once you discover them, then you write more level of concreteness or even
create more tasks - that would be when you are "downhill" on Shape Up. To me is a no-brainer that the Shape Up
relaxed method is better.

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
the "why" and the lessons learned (a lot of learnt from Scrum, of course!) we will find our unique path.
