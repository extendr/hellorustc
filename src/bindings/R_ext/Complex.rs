/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.87.0-nightly (be73c1f46 2025-03-21) */
/* R version: 4.6.0 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rcomplex__bindgen_ty_1 {
    pub r: f64,
    pub i: f64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Rcomplex {
    pub __bindgen_anon_1: Rcomplex__bindgen_ty_1,
    pub private_data_c: __BindgenComplex<f64>,
}
