use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    manifest::{ArrayType, EntryPoint, Manifest, Type},
    Target,
};

fn sys_template(target: Target) -> TokenStream {
    let target = target.name();

    quote! {
        #![allow(
            non_upper_case_globals,
            non_camel_case_types,
            non_snake_case,
            improper_ctypes,
            deref_nullptr,
            dead_code,
            clippy::approx_constant,
            clippy::upper_case_acronyms
        )]

        include!(concat!(env!("OUT_DIR"), "/futhark/", #target, "/futhark_lib.rs"));
    }
}

pub fn trait_template(manifest: &Manifest) -> TokenStream {
    let type_fns = manifest.types.iter().map(|typ| match typ {
        Type::Value(_) => quote!(),
        Type::Array(array) => trait_array_template(array),
    });

    let entry_point_fns = manifest.entry_points.iter().map(trait_entry_point_template);

    quote! {
        pub trait Backend {
            unsafe fn futhark_context_config_new() -> *mut types::futhark_context_config;
            unsafe fn futhark_context_config_free(cfg: *mut types::futhark_context_config);

            unsafe fn futhark_context_new(cfg: *mut types::futhark_context_config) -> *mut types::futhark_context;
            unsafe fn futhark_context_free(cfg: *mut types::futhark_context);
            unsafe fn futhark_context_sync(ctx: *mut types::futhark_context) -> ::std::os::raw::c_int;

            #(#type_fns)*

            #(#entry_point_fns)*
        }
    }
}

fn trait_array_template(array: &ArrayType) -> TokenStream {
    let elem_type_name = array.elements_type.ident();
    let type_name = array.type_ident();

    let name_new = array.fn_new_ident();
    let params_new = (0..array.rank).map(|i| {
        let ident = format_ident!("dim_{}", i);

        quote!(#ident: i64)
    });

    let name_shape = array.fn_shape_ident();
    let name_values = array.fn_values_ident();
    let name_free = array.fn_free_ident();

    quote! {
        unsafe fn #name_new(ctx: *mut types::futhark_context, data: *const #elem_type_name, #(#params_new),*) -> *mut types::#type_name;
        unsafe fn #name_shape(ctx: *mut types::futhark_context, array: *mut types::#type_name) -> *const i64;
        unsafe fn #name_values(ctx: *mut types::futhark_context, array: *mut types::#type_name, data: *mut #elem_type_name) -> std::os::raw::c_int;
        unsafe fn #name_free(ctx: *mut types::futhark_context, array: *mut types::#type_name) -> std::os::raw::c_int;
    }
}

fn trait_entry_point_template(ep: &EntryPoint) -> TokenStream {
    let entry_name = ep.futhark_fn_ident();

    let inputs = ep.inputs.iter().enumerate().map(|(i, typ)| {
        let input_name = format_ident!("in_{}", i);

        match typ {
            Type::Value(value) => {
                let type_name = value.ident();
                quote!(#input_name: #type_name)
            }
            Type::Array(array) => {
                let type_name = array.type_ident();
                quote!(#input_name: *const types::#type_name)
            }
        }
    });

    let outputs = ep.outputs.iter().enumerate().map(|(i, typ)| {
        let output_name = format_ident!("out_{}", i);

        match typ {
            Type::Value(value) => {
                let type_name = value.ident();
                quote!(#output_name: *mut #type_name)
            }
            Type::Array(array) => {
                let type_name = array.type_ident();
                quote!(#output_name: *mut *mut types::#type_name)
            }
        }
    });

    quote! {
        unsafe fn #entry_name (ctx: *mut types::futhark_context, #(#outputs),*, #(#inputs),*) -> std::ffi::c_int;
    }
}

pub fn impl_template(manifest: &Manifest, backend: Target) -> TokenStream {
    let backend_struct = format_ident!("{}", backend.struct_name());

    let type_impls = manifest.types.iter().map(|typ| match typ {
        Type::Value(_) => quote!(),
        Type::Array(array) => impl_array_template(array),
    });

    let entry_impls = manifest
        .entry_points
        .iter()
        .map(impl_entry_point_template);

    let sys = sys_template(backend);

    quote! {
        pub struct #backend_struct;

        mod sys {
            #sys
        }

        impl Backend for #backend_struct {
            unsafe fn futhark_context_config_new() -> *mut types::futhark_context_config {
                sys::futhark_context_config_new() as *mut types::futhark_context_config
            }

            unsafe fn futhark_context_config_free(cfg: *mut types::futhark_context_config) {
                sys::futhark_context_config_free(cfg as *mut sys::futhark_context_config);
            }

            unsafe fn futhark_context_new(
                cfg: *mut types::futhark_context_config,
            ) -> *mut types::futhark_context {
                sys::futhark_context_new(cfg as *mut sys::futhark_context_config)
                    as *mut types::futhark_context
            }

            unsafe fn futhark_context_free(cfg: *mut types::futhark_context) {
                sys::futhark_context_free(cfg as *mut sys::futhark_context);
            }

            unsafe fn futhark_context_sync(
                ctx: *mut types::futhark_context,
            ) -> ::std::os::raw::c_int {
                sys::futhark_context_sync(ctx as *mut sys::futhark_context)
            }

            #(#type_impls)*

            #(#entry_impls)*
        }
    }
}

fn impl_entry_point_template(ep: &EntryPoint) -> TokenStream {
    let entry_name = ep.futhark_fn_ident();

    let rust_inputs = ep.inputs.iter().enumerate().map(|(i, typ)| {
        let input_name = format_ident!("in_{}", i);

        match typ {
            Type::Value(value) => {
                let type_name = value.ident();
                quote!(#input_name: #type_name)
            }
            Type::Array(array) => {
                let type_name = array.type_ident();
                quote!(#input_name: *const types::#type_name)
            }
        }
    });

    let rust_outputs = ep.outputs.iter().enumerate().map(|(i, typ)| {
        let output_name = format_ident!("out_{}", i);

        match typ {
            Type::Value(value) => {
                let type_name = value.ident();
                quote!(#output_name: *mut #type_name)
            }
            Type::Array(array) => {
                let type_name = array.type_ident();
                quote!(#output_name: *mut *mut types::#type_name)
            }
        }
    });

    let futhark_inputs = ep.inputs.iter().enumerate().map(|(i, typ)| {
        let input_name = format_ident!("in_{}", i);

        match typ {
            Type::Value(_) => {
                quote!(#input_name)
            }
            Type::Array(array) => {
                let type_name = array.type_ident();
                quote!(#input_name as *const sys::#type_name)
            }
        }
    });

    let futhark_outputs = ep.outputs.iter().enumerate().map(|(i, typ)| {
        let output_name = format_ident!("out_{}", i);

        match typ {
            Type::Value(_) => {
                quote!(#output_name)
            }
            Type::Array(array) => {
                let type_name = array.type_ident();
                quote!(#output_name as *mut *mut sys::#type_name)
            }
        }
    });

    quote! {
        unsafe fn #entry_name (ctx: *mut types::futhark_context, #(#rust_outputs),*, #(#rust_inputs),*) -> std::ffi::c_int {
            sys::#entry_name(
                ctx as *mut sys::futhark_context,
                #(#futhark_outputs),*,
                #(#futhark_inputs),*
            )
        }
    }
}

fn impl_array_template(array: &ArrayType) -> TokenStream {
    let name_type = array.type_ident();
    let name_new = array.fn_new_ident();
    let name_shape = array.fn_shape_ident();
    let name_values = array.fn_values_ident();
    let name_free = array.fn_free_ident();

    let dims_new = (0..array.rank)
        .map(|i| format_ident!("dim_{}", i))
        .collect::<Vec<_>>();

    quote! {
        unsafe fn #name_new(
            ctx: *mut types::futhark_context,
            data: *const f64,
            #(#dims_new: i64),*
        ) -> *mut types::#name_type {
            sys::#name_new(ctx as *mut sys::futhark_context, data, #(#dims_new),*) as *mut types::#name_type
        }

        unsafe fn #name_free(
            ctx: *mut types::futhark_context,
            arr: *mut types::#name_type,
        ) -> std::os::raw::c_int {
            sys::#name_free(
                ctx as *mut sys::futhark_context,
                arr as *mut sys::#name_type,
            )
        }

        unsafe fn #name_values(
            ctx: *mut types::futhark_context,
            arr: *mut types::#name_type,
            data: *mut f64,
        ) -> std::os::raw::c_int {
            sys::#name_values(
                ctx as *mut sys::futhark_context,
                arr as *mut sys::#name_type,
                data,
            )
        }

        unsafe fn #name_shape(
            ctx: *mut types::futhark_context,
            arr: *mut types::#name_type,
        ) -> *const i64 {
            sys::#name_shape(
                ctx as *mut sys::futhark_context,
                arr as *mut sys::#name_type,
            )
        }
    }
}
