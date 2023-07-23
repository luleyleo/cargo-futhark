#![warn(missing_docs)]

//! # cargo-futhark
//!
//! A library and cargo subcommand to conveniently integrate [Futhark](https://futhark-lang.org/) into Rust projects.
//!
//! See the [usage section](https://github.com/luleyleo/cargo-futhark#usage) in the readme to get started with the CLI.
//! This documentation will focus on the code generation aspect of this crate.
//!
//! ## Generating Bindings
//!
//! The core of this library is the [`Generator`].
//! Using it, you can automatically generate safe bindings for your Futhark code.
//! Take a look at its documentation to understand how it works.
//!
//! ## Using Generated Bindings
//!
//! Once you've generated the bindings, including them is as easy as this:
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/futhark/futhark_lib.rs"));
//! ```
//!

mod manifest;
mod template;

mod target;
pub use target::Target;

mod generator;
pub use generator::Generator;

pub use eyre::Result;
