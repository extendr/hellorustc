/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: x86_64-pc-linux-gnu */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.3.3 */

unsafe extern "C" {
    pub fn Riconv_open(
        tocode: *const ::std::os::raw::c_char,
        fromcode: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
    pub fn Riconv(
        cd: *mut ::std::os::raw::c_void,
        inbuf: *mut *const ::std::os::raw::c_char,
        inbytesleft: *mut usize,
        outbuf: *mut *mut ::std::os::raw::c_char,
        outbytesleft: *mut usize,
    ) -> usize;
    pub fn Riconv_close(cd: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
