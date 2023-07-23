use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::manifest::{EntryPoint, Manifest, Type};

pub fn template(manifest: &Manifest) -> TokenStream {
    let entry_fns = manifest.entry_points.iter().map(entry_fn_template);

    quote::quote! {
        /// Futhark context object.
        ///
        /// This required for anything Futhark related.
        /// It also provides the `entry` functions.
        pub struct Context<B: Backend> {
            config: Config<B>,
            pub(crate) inner: *mut types::futhark_context,
        }

        unsafe impl<B: Backend> Send for Context<B> {}
        unsafe impl<B: Backend> Sync for Context<B> {}

        impl<B: Backend> Context<B> {
            /// Creates a new Futhark context.
            pub fn new(config: Config<B>) -> Self {
                let inner = unsafe { B::futhark_context_new(config.inner) };
                assert!(!inner.is_null());
                Context { config, inner }
            }

            /// Returns the configuration.
            ///
            /// It's read-only because it can not be changed after
            /// [`Context::new`] has been called.
            pub fn config(&self) -> &Config<B> {
                &self.config
            }

            /// Sync execution and memory between CPU and GPU.
            ///
            /// # Important
            /// **Always** call `sync` before accessing the result of any `entry` function.
            ///
            /// Only `C` and `MultiCore` targets have their result immediately available.
            /// For all other targets, you have to call `sync` before using an `entry` functions return value.
            /// This is necessary, because execution of `entry` functions happens asynchronously for most targets.
            ///
            /// Additionally, you have to check **both**
            /// - the [`Result`] of the `entry` function to be [`Ok`]
            /// - and the result of `sync` to be `true`.
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

fn entry_fn_template(ep: &EntryPoint) -> TokenStream {
    let futhark_entry_name = ep.futhark_fn_ident();
    let entry_name = ep.context_fn_ident();

    let rust_input = ep.inputs.iter().enumerate().map(|(i, input)| {
        let name = format_ident!("in_{}", i);

        match input {
            Type::Value(value) => {
                let typ = value.ident();

                quote!(#name: #typ)
            }
            Type::Array(array) => {
                let typ = array.struct_ident();

                quote!(#name: &#typ<B>)
            }
        }
    });

    let rust_output = ep.outputs.iter().map(|input| match input {
        Type::Value(value) => {
            let typ = value.ident();

            quote!(#typ)
        }
        Type::Array(array) => {
            let typ = array.struct_ident();

            quote!(#typ<B>)
        }
    });

    let let_output_vars = ep.outputs.iter().enumerate().map(|(i, input)| {
        let name = format_ident!("out_{}", i);

        match input {
            Type::Value(value) => {
                let typ = value.ident();

                quote!(let mut #name: #typ = Default::default();)
            }
            Type::Array(array) => {
                let typ = array.struct_ident();

                quote! {
                    let mut #name = #typ {
                        context: self,
                        inner: std::ptr::null_mut(),
                    };
                }
            }
        }
    });

    let futhark_output = ep
        .outputs
        .iter()
        .enumerate()
        .map(|(i, typ)| (format_ident!("out_{}", i), typ))
        .map(|(ident, typ)| match typ {
            Type::Value(_) => quote!(#ident),
            Type::Array(_) => quote!(#ident.inner),
        });

    let futhark_input = ep
        .inputs
        .iter()
        .enumerate()
        .map(|(i, typ)| (format_ident!("in_{}", i), typ))
        .map(|(ident, typ)| match typ {
            Type::Value(_) => quote!(#ident),
            Type::Array(_) => quote!(#ident.inner),
        });

    let output_vars = ep
        .outputs
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("out_{}", i));

    let summary_doc = format!("Entry point `{entry_name}`.");

    quote! {
        #[doc = #summary_doc]
        ///
        /// # Important
        /// Execution might happen asynchronously, so you have to call [`Context::sync`]
        /// before using it. See the documentation of [`Context::sync`] for details.
        #[allow(unused_parens, clippy::double_parens)]
        pub fn #entry_name(&self, #(#rust_input),*) -> Result<(#(#rust_output),*), i64> {
            #(#let_output_vars)*

            let status = unsafe {
                B::#futhark_entry_name(self.inner, #(&mut #futhark_output),*, #(#futhark_input),*)
            };

            match status {
                0 => Ok((#(#output_vars),*)),
                err => Err(err.into()),
            }
        }
    }
}
