# CQRS + ES in Rust (Part 2)

It's not 100% finished and there are many things to polish. Especially taking into 
account my beginner level of Rust, but let me start explaining a little bit how does it all work.

Lets remember first the requirements from the previous blog post:

>Logging in = Post with an email + password to the service. The service will verify
the password, return an unauthorized response or an accepted with a generated JWT.
>In order to be able to do that, there is another needed functionality, that one
of registering users.

Lets focus first on the register part of the application. Lets begin!

## Cargo

Cargo is a must to work with Rust. Even me a beginner of rust can tell you that.
Check the [oficial documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html)
on how to install Cargo. You definitely need that.

## The project directory structure

We will create an explicit hexagonal architecture folder structure.

You can use cargo to create the project: `cargo new your-name-of-project`

Inside src we will create three folders: domain, application and infrastructure.

```angular2html
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── application
│   ├── domain
│   ├── infrastructure
│   └── main.rs

```

we will focus only on the domain for most of the time. Why? Because we are doing DDD!

## Include files in the project

First thing first, we want `cargo check` to see the stuff that we put inside the domain.
Disclaimer again for my ignorance of rust, but I haven't found a way I really like to include files, I am
sticking to a way that just works but I find it very cumbersome. It would be nice
that somebody can tell me a better way, but so far this is how we're going to do it:

On the Cargo.toml, we will tell it that we are going to use both main and lib:

```toml
#cargo.toml
[lib]
path = "src/lib.rs"

[[bin]]
path = "src/main.rs"
name = "crappy-user"
```
(Now that you are looking at that file, feel free to change the information under "package").

Then, create a lib.rs file where we will add all the "modules" that our app will have (namely, the
3 directories we just created before.)

We will start only with the domain first.

so lets begin, create a file next to main.rs, called lib.rs with these contents:

```rust
//src/lib.rs
pub mod domain;
```

On the domain directory we will create a file 'mod.rs' that will have all the includes
of our "domain" module:

```rust
//src/domain/mod.rs
mod user;
pub(crate) mod user_tests;
pub use user::*;
```

I am going one step ahead here. I am adding already the tests, but on a different file.

Reading the rust book, it says:
>The convention is to create a module named tests in each file to contain the 
> test functions and to annotate the module with cfg(test) [for unit tests]

well, I already know that the code for the tests is going to be almost twice as long as the user
aggregate, that's why I already put it in a different file.

Ok you may create the files that we said rust we are adding, so you have something like this:

```angular2html
src/
├── application
├── domain
│   ├── mod.rs
│   ├── user.rs
│   └── user_tests.rs
├── infrastructure
├── lib.rs
└── main.rs
```

## Lets do some TDD

Since we created both files, lets begin with the test one.

```rust
#[cfg(test)]
pub(crate) mod tests {
    #[test]
    fn useless_test_to_check_project_dir_structure() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

We have the platform ready! Go on the root of the project
where you have the Cargo.toml file and execute: `cargo test`

You should see an outpout like this:

[picture!]

### Meaningful tests

Lets go back to the requirements. We said we would focus on the register here.
Lets write a test for that:

```rust
//src/domain/user_tests.rs
#[test]
fn register_user() {
    //we are deciding here that the user is going to have only those 3 pieces of information:
    let id = Uuid::new_v4();
    let email = "francesc.travesa@mymail.com".to_string();
    let password = "mySecretPassword".to_string();
    
    // we would like to "create" a user by registering:
    let user = User::register(id, email, password);
    
    // then we have some claims:
    // but we won't write that yet, there are architectural bits that I want to 
    // deeply analyze first with you here.
}
```

This looks painfully OOP. Yes it is, I have no idea of FP, I am about to read a couple
of books, but so far, doing DDD, all the tactical patterns of DDD are basically based on OOP.
There will be places where we will use more functional stuff, Rust forces them! but lets not get carried away with fDDD and keep at least some thing under control (under my control, of course, using rust, which I'm a beginner of, and implementing CQRS+ES all at once is challenge enough, please don't make me do fDDD on top of it).

All right, our test doesn't assert any claim, but just with that, we already can start solving those failing tests. Failing tests? Well yes, the code we have here doesn't compile. So let's begin by making it compile first and then let's add the assertions.

If you execute now `cargo check` it will compile though, how is this possible? Because it ignores the test part. But if you do `cargo test` then you will see some expected error messages:

### Uuid


