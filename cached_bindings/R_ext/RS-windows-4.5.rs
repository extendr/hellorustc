/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.5.0 */

unsafe extern "C" {
    #[doc = " not of themselves API"]
    pub fn R_chk_calloc(arg1: usize, arg2: usize) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_realloc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_free(arg1: *mut ::std::os::raw::c_void);
    pub fn R_chk_memcpy(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
        arg3: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_memset(
        arg1: *mut ::std::os::raw::c_void,
        arg2: ::std::os::raw::c_int,
        arg3: usize,
    ) -> *mut ::std::os::raw::c_void;
}
