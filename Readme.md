# Cargo-Futhark integration

A library and cargo subcommand to integrate Futhark into Rust projects.

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