use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::manifest::Manifest;

pub fn template(manifest: &Manifest) -> TokenStream {
    let static_types = ["futhark_context_config", "futhark_context"]
        .into_iter()
        .map(|ident| format_ident!("{}", ident));

    let dynamic_types = manifest.types.iter().filter_map(|typ| match typ {
        crate::manifest::Type::Value(_) => None,
        crate::manifest::Type::Array(array) => Some(array.type_ident()),
    });

    let structs = std::iter::empty()
        .chain(static_types)
        .chain(dynamic_types)
        .map(|ident| struct_template(&ident));

    quote! {
        #[allow(non_camel_case_types)]
        mod types {
            #(#structs)*
        }
    }
}

fn struct_template(name: &Ident) -> TokenStream {
    quote! {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct #name {
            _unused: [u8; 0],
        }
    }
}
