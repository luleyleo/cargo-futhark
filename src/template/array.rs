use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::manifest::ArrayType;

pub fn template(typ: &ArrayType) -> TokenStream {
    let rank = typ.rank;
    let struct_name = format_ident!("{}", typ.struct_name());
    let type_name = format_ident!("{}", typ.type_name());

    let fn_new_name = format_ident!("{}", typ.fn_new_name());
    let fn_shape_name = format_ident!("{}", typ.fn_shape_name());
    let fn_values_name = format_ident!("{}", typ.fn_values_name());
    let fn_free_name = format_ident!("{}", typ.fn_free_name());

    let dim_param_names = (0..rank)
        .map(|i| format_ident!("dim_{i}"))
        .collect::<Vec<_>>();

    quote! {
        pub struct #struct_name <'c, B: Backend> {
            context: &'c Context<B>,
            pub(crate) inner: *mut types::#type_name,
        }

        impl<'c, B: Backend> #struct_name <'c, B> {
            pub fn new(context: &'c Context<B>, data: &[f64], #(#dim_param_names: usize),*) -> Self {
                assert_eq!(#(#dim_param_names *)* 1, data.len());

                let inner = unsafe {
                    B::#fn_new_name(
                        context.inner,
                        data.as_ptr(),
                        #(#dim_param_names.try_into().unwrap()),*
                    )
                };

                assert!(!inner.is_null());
                #struct_name { context, inner }
            }

            pub fn shape(&self) -> &[usize] {
                unsafe {
                    let shape = B::#fn_shape_name(self.context.inner, self.inner);
                    std::slice::from_raw_parts(shape as *const usize, #rank)
                }
            }

            pub fn values(&self, out: &mut Vec<f64>) {
                let s = self.shape();
                let len = s[0] * s[1];

                out.reserve(len - out.capacity());
                unsafe {
                    B::#fn_values_name(self.context.inner, self.inner, out.as_mut_ptr());
                    out.set_len(len);
                }

                assert!(self.context.sync());
            }
        }

        impl<B: Backend> Drop for #struct_name <'_, B> {
            fn drop(&mut self) {
                unsafe {
                    B::#fn_free_name(self.context.inner, self.inner);
                }
            }
        }
    }
}
