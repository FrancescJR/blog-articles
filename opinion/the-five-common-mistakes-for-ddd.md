# 5 common mistakes beginners of DDD do

Domain-Driven Design has become quite a spread practice in the small bubble where I have interactions upon. For developers
is like this desired way of doing things and for managers and HR people is just a marketing word, to feel that
they have an advanced company for the former and to attract talent for the latter.

DDD is actually not that new, from the wikipedia:
> The term was coined by Eric Evans in his book of the same title published in 2003.[4]

so it's been there for about 20 years, and if you happen to have gone to informatics/computer science faculty afterwards - I started
right when this book was published - you probably studied many of the patterns and high level concepts this book
explained. I encourage you to read the [book](https://www.amazon.com/gp/product/B00794TAUG/ref=kinw_myk_ro_title),
known as the blue book for its cover -
you might experience like me a trip back to when you were studying "software engineering I" and "II".

It was 10 years later though that Vaughn Vernon came with his [red book](https://www.amazon.com/gp/product/B00BCLEBN8/ref=kinw_myk_ro_title) -also called like that because
of its cover- to make DDD practice to really explode. This book is also a must-read. I must confess
that I fought against slumberness while doing that, but it's OK to try to read it first, and then
come back to it on specific doubts, because you'll remember that there was something written about that,
like I did [here](https://blog.cesc.cool/dtos-plants-and-being-wrong-with-style).

I'd say he's been the one to
push the usage of DDD with ports and adapters architecture (hexagonal architecture) and the next steps to Event
Driven Architecture or CQRS+ES.

Now, the way I came across those practices was even later, the PHP world is high-jacked by all of
those CMS (Wordpress, Drupal, etc) and that's what gives results fast - and the faster way to get a job after college - so
it's been thanks to these guy: https://codely.com/ and this other guys: https://www.youtube.com/c/carlosbuenosvinoszamora
to bring DDD to the PHP world, at least in Spain, and might I dare to say even to other Spanish-speaking countries.
Carlos Buenosvinos co-authored book about DDD in PHP is also a must-read if you work with PHP.
(I came across them thanks to my boss at the time, thanks for that!).

My experience then with DDD was that one of success, even though it was just the beginning, but knowing where
was the appropriate place to write each piece of logic and how to find that easily, and start having less and
less side effects from each function, it was giving results quite fast. But it took me many years of practice
and especially *reading the books* that it all fall into place, and I could understand the whole narrative
including even that of what I was taught at college.

Unfortunately, DDD is not as spread as it should be, and many early adopters have rookie mistakes when
trying to apply them. Most of those mistakes are actually symptoms of the same problem: a miss-understanding or lack
of the mindset. Hopefully, by pointing them out, I can help you adopt the proper mindset, but nothing will
achieve that as nicely as you spending about 20 hours reading the books and some dollars (that your company
can and should pay).

Well let's begin.

## Mistake 1: thinking DDD === Hexagonal Architecture

It's very common. I have a project, I write three folders under my source folder like this:

```
    ├── src
    │   ├── Application
    │   ├── Domain
    │   └── Infrastructure
```
and I call it a day. Hey I am doing DDD!

Well no, you are stating your intention on using hexagonal architecture, which is always a good thing, but that
by itself is not DDD.

Hexagonal architecture, in my opinion, is the only way to go, for every project. DDD works very well with
hexagonal architecture, it's the best way to protect your domain, and the easiest way to realize where
each piece of logic goes: oh that's business! goes to the domain, oh! that is about infrastructure, then
it goes to infra. But, while using hexagonal architecture is a good start and something
that will save you of many headaches in the future, it's not the same as doing Domain Driven-Design.

In other words, DDD is very hard to do without hexagonal architecture, but you can have hexagonal architecture
and not having DDD at all. You can still have a mess in it. Most of the tactical patterns of DDD fall into
the Domain layer, so you need to take extra care of that layer.

You should start with that layer, coding all the business logic, and then show via tests how all of this
logic is being executed. How the domain is being executed. Even better if you start with the test. And unit tests
should suffice for that! You need to make sure the infrastructure doesn't creep all the way to the domain and try "broom inwards" to the inner
layers all what could be considered business logic. So the separation of concerns is clear.

Even if you are applying properly hexagonal architecture, you can still have something that looks more like a
CRUD application than something done via DDD. It's all about the Domain, if your domain is not rich and what you
simply do is just CRUD operations on some half-baked entities that will eventually become meaningless, you might
as well give access to the DB to the end user, and you'll save lots of hours!

## Mistake 2: overly modular directory structure

For some reason, I see this quite often. And I frankly don't get it. It's overly complicated! And it
gives the wrong intent by the programmer. This is what I am referring to:

```
    ├── src
    │   ├── User
    │   │   ├── Application
    │   │   ├── Domain
    │   │   ├── Infrastructure
    │   └── UserRoles
    │   │   ├── Application
    │   │   ├── Domain
    │   │   ├── Infrastructure
    │   └── Role
    │   │   ├── Application
    │   │   ├── Domain
    │   │   ├── Infrastructure    
```
Well, that would make sense in the case you are looking forward to split this code base into different
services, so you are preparing yourself for it. But what is clear out of it, is the complete disregard
of the domain! Again people focus on the technical details and forget that DDD is. all. about. the. domain.

On the example it's obvious that User and Roles are highly related. They should probably belong
to the same subdomain. And while there might be exceptions, a subdomain should be a bounded context, meaning
a service - microservice or application if you want. Would you add a different infrastructure in the same service?

This might even break aggregate rules! One transaction per aggregate. If you just go and split each entity
and make it a "service" you are forgetting that maybe they belong to the same aggregate! You definitely are in
for a lof of pain. Microservices work on the premise on information hiding, if you have an aggregate defined
in different services, you would want to break this principle...

I don't understand why this became such a practice. (I am afraid it's because of some public repos of codely.tv taken
out of context, or people used to the horrible laravel's ORM eloquent). But you don't want to make a service per entity, you want to make a service per subdomain. A service -
or microservice, application- per entity will for sure lead to a CRUD practice, and eventually to an anemic domain and for sure to a distributed
monolith.

All of this belongs to the strategic patterns of DDD. Showing this directory structure implies that each of
those are bounded contexts and that you expect them to work totally independently of each other. Again, we
need to have the Domain in the heart of our mindset, is that really the case or not? It's ok, and you should,
put all the related aggregates in the same bounded context. Maybe you will see that
you can split the bounded context further to ease the cognitive load. But you start from a big group and split
it. You don't start with everything split by default, and then you join it, uf, that will be a lot of pain...

## Mistake 3: CRUD implementations

This is kind the sum of mistake 1 and 2. You can have a very well understandable code in hexagonal architecture,
divided nicely in modules per aggregates but with an anemic model that only does CRUD operations.

When I started working in a new company I was rudely told by a colleague "oh I did that in DDD it should be
understandable". Seeing CRUD operations on an entity doesn't really explain the business logic. As easy as it
can be to maintain it since it's in hexagonal architecture. To explain them you'd better use tests that
execute the domain. Even some acceptance tests (the ones that test acceptance criteria) could be simply unit tests.

CRUD operations is almost a synonym of an anemic domain, which is the opposite of a rich domain. Guess which one
DDD prefers, having the word "Domain" at the core? Yes, rich domain. An anemic domain is easy to degenerate. You
will have meaningless boxes that can either be apples or pears. In that case, you might as well give admin
access to the DB to the end users.

It always pays off to make an effort at avoiding CRUD verbs, you will soon realize that your domain is, indeed,
more complex than just doing CRUD operations, the move to Event Driven Architecture will be smoother also, having
domain events that make actual sense to business, not to databases.

As a side note here, I think that being very dogmatic on RESTful implementations is harmful. REST works very
well with CRUD and oh, with active record ORM and anemic domains, so if you have to make very tricky and complicated things just
to comply with RESTful principle, I say, ditch RESTful,
at least temporarily, you might realize later how to solve it. The dogma of a rich domain should trump the
dogma of a RESTful API. I have seen codebases degenerated into un-understandable messes just in an effort to
keep RESTful.

## Mistake 4: concealed logic at the infrastructure layer

Here is where the hexagonal architecture will pay off. It will be very easy to identify that. The problem is that is
difficult to keep strong and to not succumb to the temptation. The alternative is also a lot of work! But well, you know, it should be!
and business people should be aware of it, that features have a price and high quality code has too.

So what is this logic? The most common case would be this one, have you ever seen this:
@Gedmo\Mapping\Annotation\Timestampable ? You magically add "createdAt" and "updatedAt" fields to the entity,
and you forget about it. As I said, it's easy, and the alternative? do it yourself. But you should. Does business
need to know when this entity was createdAt or updatedAt? (also, those are CRUD verbs, not a good path) Does
the domain need to work with that? If yes, put it where it belongs, in the domain! The entity should have
attributes/value objects that indicate this information.

Yes it's a lot of work, but worth it! And if the domain doesn't care, then it's OK to leave it? It's extra information!
Well no, it's not OK. Main issue is that it might fail when you retrieve or save in the DB for reasons that
are external to the domain.

All the foreign key checks on the Database? well, I'd recommend to get rid of them, the infrastructure layer should
be free of business logic, if aggregates have relationships, state that clearly in the Domain code, the infrastructure
won't have other choice than to follow. Map the aggregates to the DB as simple as possible. You will see how
easy it will be if you ever want to change to NoSQL if you have all the business logic on the domain!
As soon as you allow logic on the DB, you will have the domain split in different layers, breaking the hexagonal architecture.

Other examples are making an entity sluggable or even translatable! it might not look like it, but try to bring
everything to the domain layer, so infrastructure/application layer are thin and dumb.

That means also that the latest technology and cool stuff, well, it doesn't belong to the domain, so you can make use
of it, but on the layer it belongs. You will thank me when you need to upgrade the version of that latest technology.

## Mistake 5: leaving domain for the last.

Well, I hope you can see that this is much of the same, all above (maybe except #4) are symptoms of this one.
On Domain-Driven Design guess what comes first? Infrastructure? if that were the case don't you think it would
have been named infrastructure-driven design? So yeah, Domain comes first.

What does that mean? That you need to speak a lot with business, or the product owner needs to speak a lot with
business, or have things very very clear, so you can start modeling the domain first. And you need to speak a lot,
communicate a lot, ask questions, maybe even show the code, explain the tests, are we speaking the same language
and on the same page? All of that comes first, the rest follows.

And that comes first, is also what will probably evolve the most, will change the most. Imagine if you have it all
in a single place and not all spread everywhere how easy it will be to maintain it?

When designing the domain is when DDD truly shines, it might not be easy, and as said, it will change, either
because needs change or because we did such a terrible job at the beginning that we need to change it! Making
proper use of Value Objects, entities, aggregates, applying design patterns here, it all makes sense, and
it's pretty.

If you don't care about the domain, you're making sure you're having a bad design out of it. I was also told
"no, in this project there's no domain". Oh, that's impossible, there's no attempt at designing it, which means
a terrible design, since there's no such thing as "no-design". As the red book says, that means it's a bad design.

## Final words

DDD is all about the domain. A rich domain. I hope that by pointing out this common mistakes, you get a general
sense on the proper angle to approach problem-solving.

I would like to finish saying that DDD and Hexagonal architecture are great, but they are not silver bullets. There's
a more important thing than DDD when coding, something conveyed on Clean Code by George.R.R. Martin, I mean, Robert C. Martin
commonly known as Uncle Bob. (I just had to google them, I had the idea in my mind they were physically similar, but not
at all). The idea is that when coding you should strive for making yourself be understood.

You should strive always for simplicity (and control, I hate the no-code thing, if you like that,
just go and become a "website builder" from Drupal, and leave the actual work to the "smart people"). Don't be
afraid of hard work. The code with more "WTF" per second I've seen, it's been always something written by somebody
that thought what was written was very smart. Oh, it was indeed, but, if you have a message to convey, would you encrypt it, or would
you make sure that it is as easy to understand as possible?

We are doing complicated things, let's not make them more complicated than they need to be...(except when you
try to be funny, it's forbidden by Uncle Bob on Clean Code, but a pun in a specific place can make for a very
expensive joke!)