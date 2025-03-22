/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.4.3 */

pub type R_allocator_t = R_allocator;
pub type custom_alloc_t = ::std::option::Option<
    unsafe extern "C" fn(allocator: *mut R_allocator_t, arg1: usize) -> *mut ::std::os::raw::c_void,
>;
pub type custom_free_t = ::std::option::Option<
    unsafe extern "C" fn(allocator: *mut R_allocator_t, arg1: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_allocator {
    #[doc = " malloc equivalent"]
    pub mem_alloc: custom_alloc_t,
    #[doc = " free equivalent"]
    pub mem_free: custom_free_t,
    #[doc = " reserved (maybe for copy) - must be NULL"]
    pub res: *mut ::std::os::raw::c_void,
    #[doc = " custom data for the allocator implementation"]
    pub data: *mut ::std::os::raw::c_void,
}
