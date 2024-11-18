/* automatically generated by rust-bindgen 0.70.1 */

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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of R_allocator"][::std::mem::size_of::<R_allocator>() - 32usize];
    ["Alignment of R_allocator"][::std::mem::align_of::<R_allocator>() - 8usize];
    ["Offset of field: R_allocator::mem_alloc"]
        [::std::mem::offset_of!(R_allocator, mem_alloc) - 0usize];
    ["Offset of field: R_allocator::mem_free"]
        [::std::mem::offset_of!(R_allocator, mem_free) - 8usize];
    ["Offset of field: R_allocator::res"][::std::mem::offset_of!(R_allocator, res) - 16usize];
    ["Offset of field: R_allocator::data"][::std::mem::offset_of!(R_allocator, data) - 24usize];
};
