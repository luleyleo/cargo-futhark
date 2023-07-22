use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::manifest::{EntryPoint, Manifest, Type};

pub fn context(manifest: &Manifest) -> TokenStream {
    let entry_fns = manifest.entry_points.iter().map(context_entry_fn);

    quote::quote! {
        pub struct Context<B: Backend> {
            config: Config<B>,
            pub(crate) inner: *mut types::futhark_context,
        }

        unsafe impl<B: Backend> Send for Context<B> {}
        unsafe impl<B: Backend> Sync for Context<B> {}

        impl<B: Backend> Context<B> {
            pub fn new(config: Config<B>) -> Self {
                let inner = unsafe { B::futhark_context_new(config.inner) };
                assert!(!inner.is_null());
                Context { config, inner }
            }

            pub fn config(&self) -> &Config<B> {
                &self.config
            }

            pub fn sync(&self) -> bool {
                unsafe { B::futhark_context_sync(self.inner) == 0 }
            }

            #(#entry_fns)*
        }

        impl<B: Backend> Default for Context<B> {
            fn default() -> Self {
                Self::new(Config::default())
            }
        }

        impl<B: Backend> Drop for Context<B> {
            fn drop(&mut self) {
                unsafe {
                    B::futhark_context_free(self.inner);
                }
            }
        }
    }
}

fn context_entry_fn(ep: &EntryPoint) -> TokenStream {
    let name = format_ident!("{}", ep.fn_name());
    let context_name = format_ident!("{}", ep.context_fn_name());

    let in_params = ep.inputs.iter().enumerate().map(|(i, input)| {
        let name = format_ident!("in_{}", i);

        match input {
            Type::Value(value) => {
                let typ = format_ident!("{}", value.rust_name());

                quote!(#name: #typ)
            }
            Type::Array(array) => {
                let typ = format_ident!("{}", array.struct_name());

                quote!(#name: &#typ<B>)
            }
        }
    });

    let out_params = ep.outputs.iter().map(|input| match input {
        Type::Value(value) => {
            let typ = format_ident!("{}", value.rust_name());

            quote!(#typ)
        }
        Type::Array(array) => {
            let typ = format_ident!("{}", array.struct_name());

            quote!(#typ<B>)
        }
    });

    let let_out_vars = ep.outputs.iter().enumerate().map(|(i, input)| {
        let name = format_ident!("out_{}", i);

        match input {
            Type::Value(value) => {
                let typ = format_ident!("{}", value.rust_name());

                quote!(let mut #name: #typ = Default::default();)
            }
            Type::Array(array) => {
                let typ = format_ident!("{}", array.struct_name());

                quote! {
                    let mut #name = #typ {
                        context: self,
                        inner: std::ptr::null_mut(),
                    };
                }
            }
        }
    });

    let out_vars_params = ep
        .outputs
        .iter()
        .enumerate()
        .map(|(i, typ)| (format_ident!("out_{}", i), typ))
        .map(|(ident, typ)| match typ {
            Type::Value(_) => quote!(#ident),
            Type::Array(_) => quote!(#ident.inner),
        });

    let in_vars_params = ep
        .inputs
        .iter()
        .enumerate()
        .map(|(i, typ)| (format_ident!("in_{}", i), typ))
        .map(|(ident, typ)| match typ {
            Type::Value(_) => quote!(#ident),
            Type::Array(_) => quote!(#ident.inner),
        });

    let out_vars = ep
        .outputs
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("out_{}", i));

    quote! {
        #[allow(unused_parens)]
        pub fn #context_name(&self, #(#in_params),*) -> Result<(#(#out_params),*), i64> {
            #(#let_out_vars)*

            let status = unsafe {
                B::#name(self.inner, #(&mut #out_vars_params),*, #(#in_vars_params),*)
            };

            match status {
                0 => Ok((#(#out_vars),*)),
                err => Err(err.into()),
            }
        }
    }
}
