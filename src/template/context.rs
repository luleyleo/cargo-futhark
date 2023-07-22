use proc_macro2::TokenStream;
use quote::quote;

use crate::manifest::{EntryPoint, Manifest};

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

fn context_entry_fn(_ep: &EntryPoint) -> TokenStream {
    quote!()
}
