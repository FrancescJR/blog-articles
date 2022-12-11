On this part we will have a break of ES and focus on value objects, which are one of the most powerful and disregarded tactical patterns of DDD.

Value Objects can be hard to identify and many times they are wrongly modeled as entities. All of this deserves a full blog entry so let me oversimplify them by saying that you may consider a value object an entity attribute.

In our case for the user, we could have all of what seems that the user has: id, email and password to be value objects.

## Value Objects and Rust

In not strong typed languages, value objects are of immense help. It's a safeguard to make sure that the constructing blocks of your domain are always valid and consistent. Now on strong typed languages such as Rust, it might seem like extra overhead and oh if it didn't feel that way when I actually went for them.  And that there are native Rust crates that out of the box can be used as perfectly valid Value Objects such as en email address or passwords, adds to this feeling of doing extra unnecessary work.

Why if it felt so "YAGNI" (You Ain't Gonna Need It), I still went for it? Well, one thing is Yagni, and the other thing is the "Premature Optimization", the P of STUPID. And it's not definitely the latter, and maybe not even the former. Having a designated Value Objects allows to keep with the Single Responsibility Principle of the SOLID principles, and it will definitely allow for easier extension in the future. Normally the answer to these "architectural" questions are given by the tests. Choose whatever option is easier to test. In that case, having a separated struct to represent the value object allows for less test on the User struct, which will have already a huge amount of tests.

The downside is the amount of files (well, also because I decided to have them separated into different files to not have a bible written in a single file) and since you know that I haven't found yet the best way to include files in our project, you need to keep adding them in the `src/domain/mod.rs`...



```
    // all of these as_ref are a little bit fishy, only used for tests
    // and probable what the compiler wanted me to do was to
    // impl AsRef<T> for UserEmail and so on.
    pub fn email_as_ref(&self) -> &str {
        &self.email
    }
```


## Commands

*Warning: controversial opinion*

### CQRS_ES crate

More than opinion, a decision. It comes from the library cqrs-es of Rust, which I based myself highly to implement the whole event sourcing think
