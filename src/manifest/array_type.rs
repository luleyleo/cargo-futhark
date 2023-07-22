use proc_macro2::Ident;
use quote::format_ident;

use crate::manifest::ValueType;

#[derive(Debug, Clone, Copy)]
pub struct ArrayType {
    pub elements: ValueType,
    pub rank: usize,
}

impl ArrayType {
    pub fn struct_ident(&self) -> Ident {
        format_ident!(
            "Array_{}_{}D",
            self.elements.name().to_ascii_uppercase(),
            self.rank
        )
    }

    pub fn type_ident(&self) -> Ident {
        format_ident!("futhark_{}_{}d", self.elements.name(), self.rank)
    }

    pub fn fn_new_ident(&self) -> Ident {
        format_ident!("futhark_new_{}_{}d", self.elements.name(), self.rank)
    }

    pub fn fn_values_ident(&self) -> Ident {
        format_ident!("futhark_values_{}_{}d", self.elements.name(), self.rank)
    }

    pub fn fn_shape_ident(&self) -> Ident {
        format_ident!("futhark_shape_{}_{}d", self.elements.name(), self.rank)
    }

    pub fn fn_free_ident(&self) -> Ident {
        format_ident!("futhark_free_{}_{}d", self.elements.name(), self.rank)
    }
}
