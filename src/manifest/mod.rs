use std::{fs, path::Path};

use eyre::{Context, Result};

mod json;

#[derive(Debug, Clone)]
pub struct Manifest {
    pub entry_points: Vec<EntryPoint>,
    pub types: Vec<Type>,
}

impl Manifest {
    pub fn from_json(json: &str) -> Result<Self> {
        json::load_from_json(json)
    }

    pub fn from_json_file(path: impl AsRef<Path>) -> Result<Self> {
        let json = fs::read_to_string(path).wrap_err("Failed to read manifest file.")?;

        json::load_from_json(&json)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Value(ValueType),
    Array(ArrayType),
}

impl Type {
    pub fn rust_name(&self) -> String {
        match self {
            Type::Value(value) => value.rust_name().to_string(),
            Type::Array(array) => array.type_name(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    i8,
    i16,
    i32,
    i64,
    u8,
    u16,
    u32,
    u64,
    f16,
    f32,
    f64,
}

impl ValueType {
    pub fn variants() -> &'static [ValueType] {
        &[
            ValueType::i8,
            ValueType::i16,
            ValueType::i32,
            ValueType::i64,
            ValueType::u8,
            ValueType::u16,
            ValueType::u32,
            ValueType::u64,
            ValueType::f16,
            ValueType::f32,
            ValueType::f64,
        ]
    }

    pub fn rust_name(&self) -> &'static str {
        match self {
            ValueType::i8 => "i8",
            ValueType::i16 => "i16",
            ValueType::i32 => "i32",
            ValueType::i64 => "i64",
            ValueType::u8 => "u8",
            ValueType::u16 => "u16",
            ValueType::u32 => "u32",
            ValueType::u64 => "u64",
            ValueType::f16 => "f32", // special
            ValueType::f32 => "f32",
            ValueType::f64 => "f64",
        }
    }

    pub fn futhark_name(&self) -> &'static str {
        match self {
            ValueType::i8 => "i8",
            ValueType::i16 => "i16",
            ValueType::i32 => "i32",
            ValueType::i64 => "i64",
            ValueType::u8 => "u8",
            ValueType::u16 => "u16",
            ValueType::u32 => "u32",
            ValueType::u64 => "u64",
            ValueType::f16 => "f16",
            ValueType::f32 => "f32",
            ValueType::f64 => "f64",
        }
    }

    pub fn from_manifest(name: &str) -> Result<Self> {
        match name {
            "i8" => Ok(ValueType::i8),
            "i16" => Ok(ValueType::i16),
            "i32" => Ok(ValueType::i32),
            "i64" => Ok(ValueType::i64),
            "u8" => Ok(ValueType::u8),
            "u16" => Ok(ValueType::u16),
            "u32" => Ok(ValueType::u32),
            "u64" => Ok(ValueType::u64),
            "f16" => Ok(ValueType::f16),
            "f32" => Ok(ValueType::f32),
            "f64" => Ok(ValueType::f64),
            _ => Err(eyre::eyre!("Unknown value type {name}.")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ArrayType {
    pub elements: ValueType,
    pub rank: usize,
}

impl ArrayType {
    pub fn struct_name(&self) -> String {
        format!("{}_{}d", self.elements.rust_name(), self.rank).to_ascii_uppercase()
    }

    pub fn type_name(&self) -> String {
        format!("futhark_{}_{}d", self.elements.rust_name(), self.rank)
    }

    pub fn fn_new_name(&self) -> String {
        format!("futhark_new_{}_{}d", self.elements.rust_name(), self.rank)
    }

    pub fn fn_values_name(&self) -> String {
        format!(
            "futhark_values_{}_{}d",
            self.elements.rust_name(),
            self.rank
        )
    }

    pub fn fn_shape_name(&self) -> String {
        format!("futhark_shape_{}_{}d", self.elements.rust_name(), self.rank)
    }

    pub fn fn_free_name(&self) -> String {
        format!("futhark_free_{}_{}d", self.elements.rust_name(), self.rank)
    }
}

#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub name: String,
    pub inputs: Vec<Type>,
    pub outputs: Vec<Type>,
}

impl EntryPoint {
    pub fn fn_name(&self) -> String {
        format!("futhark_entry_{}", self.name)
    }

    pub fn context_fn_name(&self) -> String {
        format!("entry_{}", self.name)
    }
}
