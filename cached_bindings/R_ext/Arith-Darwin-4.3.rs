/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.3.3 */

unsafe extern "C" {
    #[doc = " IEEE NaN"]
    pub static mut R_NaN: f64;
    #[doc = " IEEE Inf"]
    pub static mut R_PosInf: f64;
    #[doc = " IEEE -Inf"]
    pub static mut R_NegInf: f64;
    #[doc = " NA_REAL: IEEE"]
    pub static mut R_NaReal: f64;
    #[doc = " NA_INTEGER:= INT_MIN currently"]
    pub static mut R_NaInt: ::std::os::raw::c_int;
    #[doc = " NA_STRING is a SEXP, so defined in Rinternals.h"]
    pub fn R_IsNA(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_IsNaN(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_finite(arg1: f64) -> ::std::os::raw::c_int;
}
