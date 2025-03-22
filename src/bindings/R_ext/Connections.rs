/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.87.0-nightly (be73c1f46 2025-03-21) */
/* R version: 4.6.0 */

pub type Rconnection = *mut Rconn;
#[repr(C)]
pub struct Rconn {
    pub class: *mut ::std::os::raw::c_char,
    pub description: *mut ::std::os::raw::c_char,
    #[doc = " the encoding of 'description'"]
    pub enc: ::std::os::raw::c_int,
    pub mode: [::std::os::raw::c_char; 5usize],
    pub text: Rboolean,
    pub isopen: Rboolean,
    pub incomplete: Rboolean,
    pub canread: Rboolean,
    pub canwrite: Rboolean,
    pub canseek: Rboolean,
    pub blocking: Rboolean,
    pub isGzcon: Rboolean,
    pub open: ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn) -> Rboolean>,
    #[doc = " routine closing after auto open"]
    pub close: ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn)>,
    #[doc = " when closing connection"]
    pub destroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn)>,
    pub vfprintf: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Rconn,
            arg2: *const ::std::os::raw::c_char,
            arg3: va_list,
        ) -> ::std::os::raw::c_int,
    >,
    pub fgetc:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn) -> ::std::os::raw::c_int>,
    pub fgetc_internal:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn) -> ::std::os::raw::c_int>,
    pub seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut Rconn,
            arg2: f64,
            arg3: ::std::os::raw::c_int,
            arg4: ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub truncate: ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn)>,
    pub fflush:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut Rconn) -> ::std::os::raw::c_int>,
    pub read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: usize,
            arg3: usize,
            arg4: *mut Rconn,
        ) -> usize,
    >,
    pub write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_void,
            arg2: usize,
            arg3: usize,
            arg4: *mut Rconn,
        ) -> usize,
    >,
    #[doc = " number of lines, position on top line"]
    pub nPushBack: ::std::os::raw::c_int,
    #[doc = " number of lines, position on top line"]
    pub posPushBack: ::std::os::raw::c_int,
    pub PushBack: *mut *mut ::std::os::raw::c_char,
    pub save: ::std::os::raw::c_int,
    pub save2: ::std::os::raw::c_int,
    pub encname: [::std::os::raw::c_char; 101usize],
    #[doc = " will be iconv_t, which is a pointer. NULL if not in use"]
    pub inconv: *mut ::std::os::raw::c_void,
    #[doc = " will be iconv_t, which is a pointer. NULL if not in use"]
    pub outconv: *mut ::std::os::raw::c_void,
    #[doc = " The idea here is that no MBCS char will ever not fit"]
    pub iconvbuff: [::std::os::raw::c_char; 25usize],
    #[doc = " The idea here is that no MBCS char will ever not fit"]
    pub oconvbuff: [::std::os::raw::c_char; 50usize],
    #[doc = " The idea here is that no MBCS char will ever not fit"]
    pub next: *mut ::std::os::raw::c_char,
    #[doc = " The idea here is that no MBCS char will ever not fit"]
    pub init_out: [::std::os::raw::c_char; 25usize],
    pub navail: ::std::os::raw::c_short,
    pub inavail: ::std::os::raw::c_short,
    pub EOF_signalled: Rboolean,
    pub UTF8out: Rboolean,
    pub id: *mut ::std::os::raw::c_void,
    pub ex_ptr: *mut ::std::os::raw::c_void,
    pub private: *mut ::std::os::raw::c_void,
    #[doc = " for pipes etc"]
    pub status: ::std::os::raw::c_int,
    pub buff: *mut ::std::os::raw::c_uchar,
    pub buff_len: usize,
    pub buff_stored_len: usize,
    pub buff_pos: usize,
}
pub const R_CONNECTIONS_VERSION: u32 = 1;
unsafe extern "C" {
    pub fn R_new_custom_connection(
        description: *const ::std::os::raw::c_char,
        mode: *const ::std::os::raw::c_char,
        class_name: *const ::std::os::raw::c_char,
        ptr: *mut Rconnection,
    ) -> SEXP;
    pub fn R_ReadConnection(con: Rconnection, buf: *mut ::std::os::raw::c_void, n: usize) -> usize;
    pub fn R_WriteConnection(con: Rconnection, buf: *mut ::std::os::raw::c_void, n: usize)
        -> usize;
    pub fn R_GetConnection(sConn: SEXP) -> Rconnection;
}
