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

    pub fn from_manifest(name: &str) -> eyre::Result<Self> {
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
