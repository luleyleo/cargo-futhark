use std::{fs, path::Path};

use eyre::{Context, Result};

mod json;

mod value_type;
pub use value_type::ValueType;

mod array_type;
pub use array_type::ArrayType;

mod entry_point;
pub use entry_point::EntryPoint;

#[derive(Debug, Clone)]
pub struct Manifest {
    pub entry_points: Vec<EntryPoint>,
    pub types: Vec<Type>,
}

impl Manifest {
    pub fn from_json_file(path: impl AsRef<Path>) -> Result<Self> {
        let json = fs::read_to_string(path).wrap_err("Failed to read manifest file.")?;

        json::load(&json)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Value(ValueType),
    Array(ArrayType),
}
#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub type_: Type,
}