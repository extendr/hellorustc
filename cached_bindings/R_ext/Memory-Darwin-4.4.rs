/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.4.3 */

unsafe extern "C" {
    pub fn vmaxget() -> *mut ::std::os::raw::c_void;
    pub fn vmaxset(arg1: *const ::std::os::raw::c_void);
    pub fn R_gc();
    pub fn R_gc_running() -> ::std::os::raw::c_int;
    pub fn R_alloc(arg1: usize, arg2: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    pub fn R_allocLD(nelem: usize) -> *mut f64;
    pub fn S_alloc(
        arg1: ::std::os::raw::c_long,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn S_realloc(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_long,
        arg3: ::std::os::raw::c_long,
        arg4: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn R_malloc_gc(arg1: usize) -> *mut ::std::os::raw::c_void;
    pub fn R_calloc_gc(arg1: usize, arg2: usize) -> *mut ::std::os::raw::c_void;
    pub fn R_realloc_gc(
        arg1: *mut ::std::os::raw::c_void,
        arg2: usize,
    ) -> *mut ::std::os::raw::c_void;
}
