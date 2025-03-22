/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.3.3 */

pub type libiconv_t = *mut ::std::os::raw::c_void;
unsafe extern "C" {
    pub fn libiconv_open(
        tocode: *const ::std::os::raw::c_char,
        fromcode: *const ::std::os::raw::c_char,
    ) -> libiconv_t;
    pub fn libiconv(
        cd: libiconv_t,
        inbuf: *mut *const ::std::os::raw::c_char,
        inbytesleft: *mut usize,
        outbuf: *mut *mut ::std::os::raw::c_char,
        outbytesleft: *mut usize,
    ) -> usize;
    pub fn libiconv_close(cd: libiconv_t) -> ::std::os::raw::c_int;
    pub fn libiconvlist(
        do_one: ::std::option::Option<
            unsafe extern "C" fn(
                namescount: ::std::os::raw::c_uint,
                names: *const *const ::std::os::raw::c_char,
                data: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        daXta: *mut ::std::os::raw::c_void,
    );
}
