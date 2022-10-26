# DTOs, plants and being wrong with style.

When I wrote my slogan, I just meant to be funny, but it turns out, it's also kind of true. Here's the slogan I put for Hashnode: "I hate to be wrong, so I read all the books. I still manage to be wrong. But with style".

And this is a short post about DTOs.

DTOs stand for Data Transfer Objects, and I will continue by not capitalizing all the words. It's an object that has no logic inside and it often represents the view of an aggregate, but could be

I have been very used to never ever return domain aggregates directly from the application layer. All my application services/query handlers/ use cases (however you want to call them, depending on how close or far away are you from CQRS, we'll talk about that some other day) that returned something, would look similar in this fashion:

```
class GetPlantHandler {
  public function __construct(private PlantRepositoryInterface $plantRepository)
  {}

  public function __invoke(GetSinglePlantCommand $command): PlantDTO
  {
    //...
  }
```

My main motive is to protect my precious domain from predatory external entities that want to learn too much about it, so I would return just some plain representation, with plain types (not value objects). That would also help me keep my domain clean, without polluting it with concerns that are purely representation concerns of the domain.

And then I came accross an example that would return directly the aggregate, something like that:


```
class GetPlantHandler {
  public function __construct(private PlantRepositoryInterface $plantRepository)
  {}

  public function __invoke(GetSinglePlantCommand $command): Plant
  {
    //... (yeah only the signature changed, just imagine the rest here)
  }
```

And I was like, no, come on, you *need* to convert always to a DTO, right? Are you letting the infrastructure layer to deal directly with domain aggregates? Well there is nothing wrong with that actually, infra can see the domain too... (this is the evolution of my character, in this case a real life character who happens to be me, having some room for doubt).

But I would swear it was Vaughn Vernon that wrote in [Implementing Driven Design](https://www.amazon.com/gp/product/B00BCLEBN8/ref=kinw_myk_ro_title) that not using a DTO for the application layer is an anti-pattern right? So I went to double check the book:

> A popular way to tackle the problem of rendering multiple Aggregate instances to a single view is to use Data Transfer Objects [Fowler, Pof EAA] or DTOs. The DTO is designed to hold the entire number of attributes that need to be displayed in a view. The Application Service will use Repositories to read the encessary Aggregate instances and then delegate to a DTO Assembler [Fowler, P of EAA] to map the attributes of the DTO. The DTO thus carries the full complement of information that needs to be rendered. The user interface component accesses each individual DTO attribute and renders it onto the view.

On this quote there are already two things that make me shake my view a little bit. The first is that the DTO is used to solve the problem of rendering *multiple* aggregates, and in my example I am just returning a single aggregate. The second is the use of the "DTO Assembler". In my case it's always been something like that:

```
class PlantDTO
{
    private function __construct(
        public readonly string $name,
        public readonly string $species
    ) {
    }

    public function fromPlant(Plant $plant): self
    {
        return new self(
            $plant->name()->value(),
            $plant->species()->value()
        );
    }
}
```

So my fancy DTO assembler is nothing as fancy as such as a named constructor (if you are thinking about the [Law of Demeter](https://en.wikipedia.org/wiki/Law_of_Demeter ) here, wait for another opinionated blog post about it). I honestly quite like it this way, it's the simplest way. At some points I might use something that so far I would have called "Presenter" but now I see it was a "DTO assembler".

But let's continue reading:

> Interestingly, the DTO pattern was originally designed to deal with a remote presentation tier that consumes the DTO instances. The DTO is built on the business tier, serialized, sent over the wire, and deserialized on the presentation tier. If your presentation tier is not remote, this pattern many times leads to accidental complexity in the application's design, as in YAGNI ("You Ain't Gonna Need It"). This includes the disadvantage of requiring the creation of classes that sometimes closely resemble the shape of domain objects but are not quite the same.

Ouch, this touched too close home. Indeed, the presentation tier is not remote. Here the presentation is not the frontend, would be the JSON returned by our application when a requests comes. That JSON is the presentation. And no, it's not remote. And yes, the DTO resembles closely a domain object. I found myself so many times creating DTOs that were basically domain aggregates without the logic in it, that this paragraph really hurted me. Also because I think that the single and most hurtful thing in the developer world is accidental complexity - often the result of developers thinking that they are super smart.

But it's not all lost. My main motivations to separate Domain concerns from infrastructure concerns are still valid, and so the book continues:

> There is an approach that provides a possible improvement when DTOs are unnecessary. This ones gathers multiple whole Aggregate instances for view rendition into a single Domain Payload Object [Vernon, DPO]. DPO has motivations similar to DTO but takes advantage of the single virtual machine application architecture.

Vernon様 referencing himself, of course, and well, this actually sounds more like I am doing. Could it be that I am actually using this pattern, DPO, instead of a DTO?

> [DPOs are] designed to containt references to whole Aggregate instances, not individual attributes. Cluster of Aggregate instances can be transferred between logical tiers or layers by a simple Payload container object. The Application Service uses Repositories to retrieve the necessary Aggregate instances and then instantiates the DPO to hold references to each.

Well, this is getting more complex and nuanced than I though DTO would be. And that I remember of. (I would fall asleep sometimes while reading this book, so that might have been one of those times).

In the example, I am not really using an aggregate reference, but I rather create a new immutable object that has in itself the view of such aggregate. Also, while I do use them for collections of aggregates sometimes, in the example, as I said before, I am doing it for a single aggregate. What it gets closer here is that I do use this to transfer information between layers (-domain-, application, infrastructure).

The book continues with the downside of having this reference to the aggregate and then how to lazy load the necessary information. This sounds quite different to what I am doing. I have already solved the lazy loading by instantiating the immutable object in the first place.

Of course, now I am more confused than when I started. It happens to the best families. Could it be that I am totally out of context? Well, a little bit. And not that much too, I could have been right all along. Let's continue.

> If your application provides a REST-based resource as discussed in REST(4), these will need to produce state representation of domain objects for clients. It is very important to create representations that are based on use case, not on Aggregate instances.

Here! Here! Me! Me! That's what I was doing all along! I was out of context, because I am always in a REST API context by default. And it doesn't need to be the case all the time. So what I am doing is a "state representation of an aggregate instance". SRAI? I think I can keep calling it a DTO.

The final quote then brings us further in the world of CQRS, and dealing with View Data. Which I will show some projects about that soon enough too.

> However, it maybe more accurate to think of a set of RESTful resources as a separate model in their own right - a View Model or Presentation Model [Fowler, PM]. Resist the temptation to produce representations that are a one to one reflection of your domain model Aggregate states, possibly with links to navigate deeper state.

But this is for CQRS, here we are one step before that, when we are not so cool yet.

## Conclusion

So was I wrong or was I right?

Well, unfortunately, I think I was -75%- wrong. While we have to keep this separation of concerns between presenting something and the domain itself, that doesn't mean we *have* to write all of DTOs all over the place (in the application layer).

Where I came across the place they didn't use it DTOs on the application layer, a JSON object was build using the tools the framework provided. The translation to a presentation happened all the same, but in the infrastructure layer.

Less lines to write, less code to maintain, less accidental complexity. The only downside I can say is that this logic lives in infrastructure, and the common mindset is to try to bring things more and more to the inner layers. So you might argue that this specific use case/ command handler/ application service (whatever you want to call it) response's is that exact one. So you explicitly specify that on there. Still in infrastructure you will need to make a transformation to the final Http Response.

There's always room to wiggle. The choice, as always, is yours. Just remember, we can always be wrong, but it doesn't matter as long as we recognize it. That's how you do it with style.