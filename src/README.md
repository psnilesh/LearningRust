## Learning Rust

A repository with silly programs for getting familiar with [Rust](https://www.rust-lang.org/).

### How to

[Refer official testing guide](https://doc.rust-lang.org/book/ch11-02-running-tests.html) for basic usages of `cargo test`. 

**Enable stdout for unit tests**
```
$ cargo run test -- --show-output
```

**Format code using Rustfmt**
```
$ cargo fmt
```

Copy `git/pre-commit` to `.git/hooks` to automatically check codestyle before every commit.