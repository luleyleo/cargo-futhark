use std::marker::PhantomData;
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
mod types {
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct futhark_context_config {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct futhark_context {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct futhark_f64_2d {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct futhark_f64_1d {
        _unused: [u8; 0],
    }
}
pub mod backends {
    use super::types;
    pub trait Backend {
        unsafe fn futhark_context_config_new() -> *mut types::context_config;
        unsafe fn futhark_context_config_free(cfg: *mut types::context_config);
        unsafe fn futhark_context_new(cfg: *mut types::context_config) -> *mut types::context;
        unsafe fn futhark_context_free(cfg: *mut types::context);
        unsafe fn futhark_context_sync(ctx: *mut types::context) -> ::std::os::raw::c_int;
        unsafe fn futhark_new_f64_2d(
            ctx: *mut types::context,
            data: *const f64,
            dim_0: usize,
            dim_1: usize,
        ) -> *mut types::futhark_f64_2d;
        unsafe fn futhark_shape_f64_2d(
            ctx: *mut types::context,
            array: *mut types::futhark_f64_2d,
        ) -> *const i64;
        unsafe fn futhark_values_f64_2d(
            ctx: *mut types::context,
            array: *mut types::futhark_f64_2d,
            data: *mut f64,
        );
        unsafe fn futhark_free_f64_2d(ctx: *mut types::context, array: *mut types::futhark_f64_2d);
        unsafe fn futhark_new_f64_1d(
            ctx: *mut types::context,
            data: *const f64,
            dim_0: usize,
        ) -> *mut types::futhark_f64_1d;
        unsafe fn futhark_shape_f64_1d(
            ctx: *mut types::context,
            array: *mut types::futhark_f64_1d,
        ) -> *const i64;
        unsafe fn futhark_values_f64_1d(
            ctx: *mut types::context,
            array: *mut types::futhark_f64_1d,
            data: *mut f64,
        );
        unsafe fn futhark_free_f64_1d(ctx: *mut types::context, array: *mut types::futhark_f64_1d);
        unsafe fn futhark_entry_attractive_sector(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_bent_cigar(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_bueche_rastrigin(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
        );
        unsafe fn futhark_entry_different_powers(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_discus(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_ellipsoidal(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
        );
        unsafe fn futhark_entry_ellipsoidal_rotated(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_gallagher(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_2d,
            in_2: *const types::futhark_f64_1d,
            in_3: *const types::futhark_f64_2d,
            in_4: f64,
            in_5: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_griewank_rosenbrock(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: f64,
            in_2: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_katsuura(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
            in_4: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_linear_slope(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
        );
        unsafe fn futhark_entry_lunacek(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_rastrigin(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
        );
        unsafe fn futhark_entry_rastrigin_rotated(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
            in_4: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_rosenbrock(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
        );
        unsafe fn futhark_entry_rosenbrock_rotated(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: f64,
            in_2: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_schaffers_f7(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
            in_4: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_schaffers_f7_ill(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
            in_4: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_schwefel(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
        );
        unsafe fn futhark_entry_sharp_ridge(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_sphere(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
        );
        unsafe fn futhark_entry_step_ellipsoidal(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
            in_4: *const types::futhark_f64_2d,
        );
        unsafe fn futhark_entry_weierstrass(
            ctx: *mut types::context,
            out_0: *mut *mut types::futhark_f64_1d,
            in_0: *const types::futhark_f64_2d,
            in_1: *const types::futhark_f64_1d,
            in_2: f64,
            in_3: *const types::futhark_f64_2d,
            in_4: *const types::futhark_f64_2d,
        );
    }
    mod c {
        pub struct C;
        mod sys {
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
            include!(concat!(
                env!("OUT_DIR"),
                "/futhark/",
                "c",
                "/futhark_lib.rs"
            ));
        }
        impl C {
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
            unsafe fn futhark_new_f64_2d(
                ctx: *mut types::futhark_context,
                data: *const f64,
                dim_0: i64,
                dim_1: i64,
            ) -> *mut types::futhark_f64_2d {
                sys::futhark_new_f64_2d(ctx as *mut sys::futhark_context, data, dim_0, dim_1)
                    as *mut types::futhark_f64_2d
            }
            unsafe fn futhark_free_f64_2d(
                ctx: *mut types::futhark_context,
                arr: *mut types::futhark_f64_2d,
            ) -> std::os::raw::c_int {
                sys::futhark_free_f64_2d(
                    ctx as *mut sys::futhark_context,
                    arr as *mut sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_values_f64_2d(
                ctx: *mut types::futhark_context,
                arr: *mut types::futhark_f64_2d,
                data: *mut f64,
            ) -> std::os::raw::c_int {
                sys::futhark_values_f64_2d(
                    ctx as *mut sys::futhark_context,
                    arr as *mut sys::futhark_f64_2d,
                    data,
                )
            }
            unsafe fn futhark_shape_f64_2d(
                ctx: *mut types::futhark_context,
                arr: *mut types::futhark_f64_2d,
            ) -> *const i64 {
                sys::futhark_shape_f64_2d(
                    ctx as *mut sys::futhark_context,
                    arr as *mut sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_new_f64_1d(
                ctx: *mut types::futhark_context,
                data: *const f64,
                dim_0: i64,
            ) -> *mut types::futhark_f64_1d {
                sys::futhark_new_f64_1d(ctx as *mut sys::futhark_context, data, dim_0)
                    as *mut types::futhark_f64_1d
            }
            unsafe fn futhark_free_f64_1d(
                ctx: *mut types::futhark_context,
                arr: *mut types::futhark_f64_1d,
            ) -> std::os::raw::c_int {
                sys::futhark_free_f64_1d(
                    ctx as *mut sys::futhark_context,
                    arr as *mut sys::futhark_f64_1d,
                )
            }
            unsafe fn futhark_values_f64_1d(
                ctx: *mut types::futhark_context,
                arr: *mut types::futhark_f64_1d,
                data: *mut f64,
            ) -> std::os::raw::c_int {
                sys::futhark_values_f64_1d(
                    ctx as *mut sys::futhark_context,
                    arr as *mut sys::futhark_f64_1d,
                    data,
                )
            }
            unsafe fn futhark_shape_f64_1d(
                ctx: *mut types::futhark_context,
                arr: *mut types::futhark_f64_1d,
            ) -> *const i64 {
                sys::futhark_shape_f64_1d(
                    ctx as *mut sys::futhark_context,
                    arr as *mut sys::futhark_f64_1d,
                )
            }
            unsafe fn futhark_entry_attractive_sector(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_attractive_sector(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_bent_cigar(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_bent_cigar(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_bueche_rastrigin(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
            ) {
                sys::futhark_entry_bueche_rastrigin(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                )
            }
            unsafe fn futhark_entry_different_powers(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_different_powers(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_discus(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_discus(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_ellipsoidal(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
            ) {
                sys::futhark_entry_ellipsoidal(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                )
            }
            unsafe fn futhark_entry_ellipsoidal_rotated(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_ellipsoidal_rotated(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_gallagher(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_2d,
                in_2: *const types::futhark_f64_1d,
                in_3: *const types::futhark_f64_2d,
                in_4: f64,
                in_5: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_gallagher(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_2d,
                    in_2 as *const sys::futhark_f64_1d,
                    in_3 as *const sys::futhark_f64_2d,
                    in_4,
                    in_5 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_griewank_rosenbrock(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: f64,
                in_2: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_griewank_rosenbrock(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1,
                    in_2 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_katsuura(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
                in_4: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_katsuura(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                    in_4 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_linear_slope(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
            ) {
                sys::futhark_entry_linear_slope(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                )
            }
            unsafe fn futhark_entry_lunacek(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_lunacek(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_rastrigin(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
            ) {
                sys::futhark_entry_rastrigin(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                )
            }
            unsafe fn futhark_entry_rastrigin_rotated(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
                in_4: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_rastrigin_rotated(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                    in_4 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_rosenbrock(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
            ) {
                sys::futhark_entry_rosenbrock(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                )
            }
            unsafe fn futhark_entry_rosenbrock_rotated(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: f64,
                in_2: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_rosenbrock_rotated(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1,
                    in_2 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_schaffers_f7(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
                in_4: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_schaffers_f7(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                    in_4 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_schaffers_f7_ill(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
                in_4: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_schaffers_f7_ill(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                    in_4 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_schwefel(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
            ) {
                sys::futhark_entry_schwefel(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                )
            }
            unsafe fn futhark_entry_sharp_ridge(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_sharp_ridge(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_sphere(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
            ) {
                sys::futhark_entry_sphere(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                )
            }
            unsafe fn futhark_entry_step_ellipsoidal(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
                in_4: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_step_ellipsoidal(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                    in_4 as *const sys::futhark_f64_2d,
                )
            }
            unsafe fn futhark_entry_weierstrass(
                ctx: *mut types::context,
                out_0: *mut *mut types::futhark_f64_1d,
                in_0: *const types::futhark_f64_2d,
                in_1: *const types::futhark_f64_1d,
                in_2: f64,
                in_3: *const types::futhark_f64_2d,
                in_4: *const types::futhark_f64_2d,
            ) {
                sys::futhark_entry_weierstrass(
                    ctx as *mut sys::futhark_context,
                    out_0 as *mut *mut sys::futhark_f64_1d,
                    in_0 as *const sys::futhark_f64_2d,
                    in_1 as *const sys::futhark_f64_1d,
                    in_2,
                    in_3 as *const sys::futhark_f64_2d,
                    in_4 as *const sys::futhark_f64_2d,
                )
            }
        }
    }
    pub use c::C;
}
use backends::Backend;
pub struct F64_2D<'c, B: Backend> {
    context: &'c Context<B>,
    pub(crate) inner: *mut types::futhark_f64_2d,
}
impl<'c, B: Backend> F64_2D<'c, B> {
    pub fn new(context: &'c Context<B>, data: &[f64], dim_0: usize, dim_1: usize) -> Self {
        assert_eq!(rows * columns, data.len());
        let inner = unsafe {
            B::futhark_free_f64_2d(
                context.inner,
                data.as_ptr(),
                dim_0.try_into().unwrap(),
                dim_1.try_into().unwrap(),
            )
        };
        assert!(!inner.is_null());
        F64_2D { context, inner }
    }
    pub fn shape(&self) -> &[usize] {
        unsafe {
            let shape = B::futhark_shape_f64_2d(self.context.inner, self.inner);
            std::slice::from_raw_parts(shape as *const usize, 2usize)
        }
    }
    pub fn values(&self, out: &mut Vec<f64>) {
        let s = self.shape();
        let len = s[0] * s[1];
        out.reserve(len - out.capacity());
        unsafe {
            B::futhark_values_f64_2d(self.context.inner, self.inner, out.as_mut_ptr());
            out.set_len(len);
        }
        assert!(self.context.sync());
    }
}
impl<B: Backend> Drop for F64_2D<'_, B> {
    fn drop(&mut self) {
        unsafe {
            B::futhark_free_f64_2d(self.context.inner, self.inner);
        }
    }
}
pub struct F64_1D<'c, B: Backend> {
    context: &'c Context<B>,
    pub(crate) inner: *mut types::futhark_f64_1d,
}
impl<'c, B: Backend> F64_1D<'c, B> {
    pub fn new(context: &'c Context<B>, data: &[f64], dim_0: usize) -> Self {
        assert_eq!(rows * columns, data.len());
        let inner = unsafe {
            B::futhark_free_f64_1d(context.inner, data.as_ptr(), dim_0.try_into().unwrap())
        };
        assert!(!inner.is_null());
        F64_1D { context, inner }
    }
    pub fn shape(&self) -> &[usize] {
        unsafe {
            let shape = B::futhark_shape_f64_1d(self.context.inner, self.inner);
            std::slice::from_raw_parts(shape as *const usize, 1usize)
        }
    }
    pub fn values(&self, out: &mut Vec<f64>) {
        let s = self.shape();
        let len = s[0] * s[1];
        out.reserve(len - out.capacity());
        unsafe {
            B::futhark_values_f64_1d(self.context.inner, self.inner, out.as_mut_ptr());
            out.set_len(len);
        }
        assert!(self.context.sync());
    }
}
impl<B: Backend> Drop for F64_1D<'_, B> {
    fn drop(&mut self) {
        unsafe {
            B::futhark_free_f64_1d(self.context.inner, self.inner);
        }
    }
}
