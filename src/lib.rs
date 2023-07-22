mod manifest;
mod template;

mod target;
pub use target::Target;

mod generator;
pub use generator::Generator;

pub use eyre::Result;
