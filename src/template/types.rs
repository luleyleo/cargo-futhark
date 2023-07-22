use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::manifest::Manifest;

pub fn types(manifest: &Manifest) -> TokenStream {
    let names = ["futhark_context_config", "futhark_context"]
        .into_iter()
        .map(String::from)
        .chain(manifest.types.iter().filter_map(|typ| match typ {
            crate::manifest::Type::Value(_) => None,
            crate::manifest::Type::Array(array) => Some(array.type_name()),
        }));

    let structs = names.map(|name| type_struct(&name));

    quote! {
        #[allow(non_camel_case_types)]
        mod types {
            #(#structs)*
        }
    }
}

fn type_struct(name: &str) -> TokenStream {
    let name = format_ident!("{}", name);

    quote! {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct #name {
            _unused: [u8; 0],
        }
    }
}
