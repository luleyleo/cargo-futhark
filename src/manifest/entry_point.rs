use proc_macro2::Ident;
use quote::format_ident;

use crate::manifest::Type;

#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub name: String,
    pub inputs: Vec<Type>,
    pub outputs: Vec<Type>,
}

impl EntryPoint {
    pub fn futhark_fn_ident(&self) -> Ident {
        format_ident!("futhark_entry_{}", self.name)
    }

    pub fn context_fn_ident(&self) -> Ident {
        format_ident!("entry_{}", self.name)
    }
}
