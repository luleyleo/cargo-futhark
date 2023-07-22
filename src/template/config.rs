use proc_macro2::TokenStream;

pub fn template() -> TokenStream {
    quote::quote! {
        pub struct Config<B: Backend> {
            _phantom: PhantomData<B>,
            pub(crate) inner: *mut types::futhark_context_config,
        }

        impl<B: Backend> Config<B> {
            pub fn new() -> Self {
                let inner = unsafe { B::futhark_context_config_new() };
                assert!(!inner.is_null());
                Config {
                    _phantom: PhantomData,
                    inner,
                }
            }
        }

        impl<B: Backend> Default for Config<B> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<B: Backend> Drop for Config<B> {
            fn drop(&mut self) {
                unsafe {
                    B::futhark_context_config_free(self.inner);
                }
            }
        }
    }
}
