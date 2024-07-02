# Serde Partial Deserialization

## Setup

- Rust `=1.79` : The `rustup` command to install toolchains. ([link][install-rust])

> This crate compiles for `no_std` environments.

## Compile

```sh
$ cargo run --release;
```

## Problem

When running the program, the JSON string representing an unsuccessful response
does not deserialize properly. Thus, one cannot access the error `message`
returned by an API.

In the [lib.rs](./src/lib.rs) file, three different attempts are documented to
solve this problem without success. (line 25)



[install-rust]: https://www.rust-lang.org/tools/install
