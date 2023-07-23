# Cargo-Futhark integration

A library and cargo subcommand to conveniently integrate [Futhark](https://futhark-lang.org/) into Rust projects.

## Usage

First, install the command-line tool:
```sh
cargo install cargo-futhark
```

Then create your new app (skip if it already exists):
```sh
cargo new --bin my-app
cd my-app
```

Create the Futhark package:
```sh
cargo futhark new futhark-lib
```

And add it to your `my-app/Cargo.toml`:
```toml
[package]
# package stuff ...

[workspace]
members = ["futhark-lib"]

[dependencies]
futhark-lib = { path = "futhark-lib" }
```

## Related Work

- [crates-io:futhark](https://crates.io/crates/futhark)
    - just a placeholder.
- [create-io:genfut](https://crates.io/crates/genfut)
    - no support for multiple targets
    - no simple CLI generate project
- [crates-io:futhark-bindgen](https://crates.io/crates/futhark-bindgen)
    - no support for multiple targets
    - no simple CLI generate project
    - supports OCaml