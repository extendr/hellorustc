/* automatically generated by rust-bindgen 0.70.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.81.0 (eeb90cda1 2024-09-04) */
/* R version: 4.4.1 */

extern "C" {
    #[cfg(feature = "r_4_4_1")]
    #[cfg(target_family = "unix")]
    pub fn Riconv_open(
        tocode: *const ::std::os::raw::c_char,
        fromcode: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
    #[cfg(feature = "r_4_4_1")]
    #[cfg(target_family = "unix")]
    pub fn Riconv(
        cd: *mut ::std::os::raw::c_void,
        inbuf: *mut *const ::std::os::raw::c_char,
        inbytesleft: *mut usize,
        outbuf: *mut *mut ::std::os::raw::c_char,
        outbytesleft: *mut usize,
    ) -> usize;
    #[cfg(feature = "r_4_4_1")]
    #[cfg(target_family = "unix")]
    pub fn Riconv_close(cd: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
