# Domain Driven Design via Derive (D4)

Deriving a persistence entity for your domain model

## Description

Keeping the domain model and persistence models cleanly decoupled usually involves a bit of copy and paste. For simple
usecases this ends up with almost identical domain and persistence models. Let's see if we can simplify these simple
usecases with a Derive macro.

## Getting Started

* [Install Rust and Cargo via Rustup](https://doc.rust-lang.org/cargo/getting-started/installation.html) Stable rust
  toolchain is fine

### Executing program

To see the result of the Entity macro

```
cargo +nightly expand
```

## Crates used

* [uuid](https://crates.io/crates/uuid) -> Generate and parse UUIDs
* [chrono](https://crates.io/crates/chrono) -> Date & Time library for rust
* [syn crate](https://crates.io/crates/syn) -> Parser for Rust source code
* [quote crate](https://crates.io/crates/quote) -> For turning Rust syntax tree data structures into tokens of source
* [proc-macro2 crate](https://crates.io/crates/proc-macro2) -> A wrapper around the rust's procedural macro API
* [proc-macro-error crate](https://crates.io/crates/proc-macro-error) -> Better error reporting for proc-macros
* [anyhow](https://crates.io/crates/anyhow) -> Easy idiomatic error handling in Rust applications
* [trybuild](https://crates.io/crates/trybuild) -> Test harness for testing proc macros
* [test-case](https://crates.io/crates/test-case) -> Parameterized tests in rust

## Acknowledgments

* [DomPizzie for the Readme template](https://gist.github.com/DomPizzie/7a5ff55ffa9081f2de27c315f5018afc)

## Further reading

* [Official Rust Book, Macros chapter](https://doc.rust-lang.org/book/ch19-06-macros.html)
* [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)
* [David Tolnay's proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
