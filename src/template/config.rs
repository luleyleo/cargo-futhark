use proc_macro2::TokenStream;

pub fn template() -> TokenStream {
    quote::quote! {
        /// Configuration for the [`Context`] struct.
        ///
        /// Any changes to the configuration must be made before calling [`Context::new`].
        ///
        /// The type parameter `B` specifies which backend to use.
        /// You can find the list of all compiled backends in the [`backends`] module.
        pub struct Config<B: Backend> {
            _phantom: PhantomData<B>,
            pub(crate) inner: *mut types::futhark_context_config,
        }

        impl<B: Backend> Config<B> {
            /// Create a new configuration with default settings.
            ///
            /// The default values are:
            /// - `debugging = false`
            /// - `profiling = false`
            /// - `logging = false`
            /// - `tuning_params = None`
            /// - `cache_file = None`
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
