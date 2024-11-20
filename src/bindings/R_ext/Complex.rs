/* automatically generated by rust-bindgen 0.70.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.81.0 (eeb90cda1 2024-09-04) */
/* R version: 4.4.1 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg(feature = "r_4_4_1")]
#[cfg(target_family = "unix")]
pub struct Rcomplex__bindgen_ty_1 {
    pub r: f64,
    pub i: f64,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(feature = "r_4_4_1")]
#[cfg(target_family = "unix")]
pub union Rcomplex {
    pub __bindgen_anon_1: Rcomplex__bindgen_ty_1,
    pub private_data_c: __BindgenComplex<f64>,
}
