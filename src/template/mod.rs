use crate::{
    manifest::{Manifest, Type},
    Target,
};
use enumflags2::BitFlags;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod arrays;
pub use arrays::array_struct;

mod backend;
pub use backend::{backend_impl, backend_trait};

mod types;
pub use types::types;

mod config;
pub use config::config;

mod context;
pub use context::context;

pub fn combined(manifest: &Manifest, targets: BitFlags<Target>) -> TokenStream {
    let config = config();
    let context = context(manifest);
    let types = types(manifest);
    let backend_trait = backend_trait(manifest);

    let structs = manifest.types.iter().map(|typ| match typ {
        Type::Value(_) => quote!(),
        Type::Array(array) => array_struct(array),
    });

    let backends = targets.iter().map(|target| {
        let target_name = format_ident!("{}", target.name());
        let target_struct_name = format_ident!("{}", target.struct_name());
        let target_impl = backend_impl(manifest, target);

        quote! {
            mod #target_name {
                use super::Backend;
                use super::super::types;

                #target_impl
            }
            pub use #target_name::#target_struct_name;
        }
    });

    quote! {
        use std::marker::PhantomData;

        #config
        #context

        #types

        pub mod backends {
            use super::types;

            #backend_trait
            #(#backends)*
        }
        use backends::Backend;

        #(#structs)*
    }
}
