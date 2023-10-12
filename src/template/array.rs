use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::manifest::ArrayType;

pub fn template(typ: &ArrayType) -> TokenStream {
    let rank = typ.rank;
    let struct_name = typ.struct_ident();
    let type_name = typ.type_ident();
    let fn_new_name = typ.fn_new_ident();
    let fn_shape_name = typ.fn_shape_ident();
    let fn_values_name = typ.fn_values_ident();
    let fn_free_name = typ.fn_free_ident();
    let elem_typ_name = typ.elements_type.ident();

    let summary_doc = format!(
        "Array of type `{}` and rank `{}`.",
        typ.elements_type.name(),
        typ.rank
    );

    let dim_params = (0..rank)
        .map(|i| format_ident!("dim_{i}"))
        .collect::<Vec<_>>();

    quote! {
        #[doc = #summary_doc]
        ///
        /// # Immutability
        /// Futhark arrays can not be mutated once they've been created.
        #[allow(non_camel_case_types)]
        pub struct #struct_name <'c, B: Backend> {
            context: &'c Context<B>,
            pub(crate) inner: *mut types::#type_name,
        }

        impl<'c, B: Backend> #struct_name <'c, B> {
            /// Create a new Futhark array from a flat buffer.
            ///
            ///  Multi-dimensional arrays are expect row-major form.
            #[allow(clippy::identity_op)]
            pub fn new(context: &'c Context<B>, data: &[#elem_typ_name], #(#dim_params: usize),*) -> Self {
                assert_eq!(#(#dim_params *)* 1, data.len());

                let inner = unsafe {
                    B::#fn_new_name(
                        context.inner,
                        data.as_ptr(),
                        #(#dim_params.try_into().unwrap()),*
                    )
                };

                assert!(!inner.is_null());
                #struct_name { context, inner }
            }

            /// Returns the arrays shape
            ///
            /// The length of `array.shape()` is its rank.
            pub fn shape(&self) -> &[usize] {
                unsafe {
                    let shape = B::#fn_shape_name(self.context.inner, self.inner);
                    std::slice::from_raw_parts(shape as *const usize, #rank)
                }
            }

            /// Read the arrays values to a buffer.
            ///
            /// The `out` buffer will have the length of the `shape`s product.
            ///  Multi-dimensional arrays are written in row-major form.
            ///
            /// # Important
            /// Before calling this, you most likely want to call [`Context::sync`] first.
            /// See the documentation of [`Context::sync`] for more details.
            pub fn values(&self, out: &mut Vec<#elem_typ_name>) {
                let s = self.shape();
                let len = s.iter().product::<usize>();

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
