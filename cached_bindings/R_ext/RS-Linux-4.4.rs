/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: x86_64-pc-linux-gnu */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.4.3 */

unsafe extern "C" {
    #[doc = " S Like Memory Management"]
    pub fn R_chk_calloc(arg1: usize, arg2: usize) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_realloc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn R_chk_free(arg1: *mut ::std::os::raw::c_void);
}
