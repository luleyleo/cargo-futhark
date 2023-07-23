use crate::{
    manifest::{Manifest, Type},
    Target,
};
use enumflags2::BitFlags;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod array;
mod backend;
mod config;
mod context;
mod types;

pub fn combined(manifest: &Manifest, targets: BitFlags<Target>) -> TokenStream {
    let config = config::template();
    let context = context::template(manifest);
    let types = types::template(manifest);
    let backend_trait = backend::trait_template(manifest);

    let structs = manifest.types.iter().map(|typ| match typ {
        Type::Value(_) => quote!(),
        Type::Array(array) => array::template(array),
    });

    let backends = targets.iter().map(|target| {
        let target_name = format_ident!("{}", target.name());
        let target_struct_name = format_ident!("{}", target.struct_name());
        let target_impl = backend::impl_template(manifest, target);

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

        /// List of supported backends.
        ///
        /// If your desired backend is not available, you might have to modify your `build.rs`
        /// file and add the desired `Target` to the `Generator` using `Generator::with_target`.
        #[allow(clippy::missing_safety_doc)]
        pub mod backends {
            use super::types;

            #backend_trait

            #(#backends)*
        }
        use backends::Backend;

        #(#structs)*
    }
}
