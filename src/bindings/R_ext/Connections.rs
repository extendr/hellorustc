/* automatically generated by rust-bindgen 0.70.1 */

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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Rconn"][::std::mem::size_of::<Rconn>() - 488usize];
    ["Alignment of Rconn"][::std::mem::align_of::<Rconn>() - 8usize];
    ["Offset of field: Rconn::class"][::std::mem::offset_of!(Rconn, class) - 0usize];
    ["Offset of field: Rconn::description"][::std::mem::offset_of!(Rconn, description) - 8usize];
    ["Offset of field: Rconn::enc"][::std::mem::offset_of!(Rconn, enc) - 16usize];
    ["Offset of field: Rconn::mode"][::std::mem::offset_of!(Rconn, mode) - 20usize];
    ["Offset of field: Rconn::text"][::std::mem::offset_of!(Rconn, text) - 28usize];
    ["Offset of field: Rconn::isopen"][::std::mem::offset_of!(Rconn, isopen) - 32usize];
    ["Offset of field: Rconn::incomplete"][::std::mem::offset_of!(Rconn, incomplete) - 36usize];
    ["Offset of field: Rconn::canread"][::std::mem::offset_of!(Rconn, canread) - 40usize];
    ["Offset of field: Rconn::canwrite"][::std::mem::offset_of!(Rconn, canwrite) - 44usize];
    ["Offset of field: Rconn::canseek"][::std::mem::offset_of!(Rconn, canseek) - 48usize];
    ["Offset of field: Rconn::blocking"][::std::mem::offset_of!(Rconn, blocking) - 52usize];
    ["Offset of field: Rconn::isGzcon"][::std::mem::offset_of!(Rconn, isGzcon) - 56usize];
    ["Offset of field: Rconn::open"][::std::mem::offset_of!(Rconn, open) - 64usize];
    ["Offset of field: Rconn::close"][::std::mem::offset_of!(Rconn, close) - 72usize];
    ["Offset of field: Rconn::destroy"][::std::mem::offset_of!(Rconn, destroy) - 80usize];
    ["Offset of field: Rconn::vfprintf"][::std::mem::offset_of!(Rconn, vfprintf) - 88usize];
    ["Offset of field: Rconn::fgetc"][::std::mem::offset_of!(Rconn, fgetc) - 96usize];
    ["Offset of field: Rconn::fgetc_internal"]
        [::std::mem::offset_of!(Rconn, fgetc_internal) - 104usize];
    ["Offset of field: Rconn::seek"][::std::mem::offset_of!(Rconn, seek) - 112usize];
    ["Offset of field: Rconn::truncate"][::std::mem::offset_of!(Rconn, truncate) - 120usize];
    ["Offset of field: Rconn::fflush"][::std::mem::offset_of!(Rconn, fflush) - 128usize];
    ["Offset of field: Rconn::read"][::std::mem::offset_of!(Rconn, read) - 136usize];
    ["Offset of field: Rconn::write"][::std::mem::offset_of!(Rconn, write) - 144usize];
    ["Offset of field: Rconn::nPushBack"][::std::mem::offset_of!(Rconn, nPushBack) - 152usize];
    ["Offset of field: Rconn::posPushBack"][::std::mem::offset_of!(Rconn, posPushBack) - 156usize];
    ["Offset of field: Rconn::PushBack"][::std::mem::offset_of!(Rconn, PushBack) - 160usize];
    ["Offset of field: Rconn::save"][::std::mem::offset_of!(Rconn, save) - 168usize];
    ["Offset of field: Rconn::save2"][::std::mem::offset_of!(Rconn, save2) - 172usize];
    ["Offset of field: Rconn::encname"][::std::mem::offset_of!(Rconn, encname) - 176usize];
    ["Offset of field: Rconn::inconv"][::std::mem::offset_of!(Rconn, inconv) - 280usize];
    ["Offset of field: Rconn::outconv"][::std::mem::offset_of!(Rconn, outconv) - 288usize];
    ["Offset of field: Rconn::iconvbuff"][::std::mem::offset_of!(Rconn, iconvbuff) - 296usize];
    ["Offset of field: Rconn::oconvbuff"][::std::mem::offset_of!(Rconn, oconvbuff) - 321usize];
    ["Offset of field: Rconn::next"][::std::mem::offset_of!(Rconn, next) - 376usize];
    ["Offset of field: Rconn::init_out"][::std::mem::offset_of!(Rconn, init_out) - 384usize];
    ["Offset of field: Rconn::navail"][::std::mem::offset_of!(Rconn, navail) - 410usize];
    ["Offset of field: Rconn::inavail"][::std::mem::offset_of!(Rconn, inavail) - 412usize];
    ["Offset of field: Rconn::EOF_signalled"]
        [::std::mem::offset_of!(Rconn, EOF_signalled) - 416usize];
    ["Offset of field: Rconn::UTF8out"][::std::mem::offset_of!(Rconn, UTF8out) - 420usize];
    ["Offset of field: Rconn::id"][::std::mem::offset_of!(Rconn, id) - 424usize];
    ["Offset of field: Rconn::ex_ptr"][::std::mem::offset_of!(Rconn, ex_ptr) - 432usize];
    ["Offset of field: Rconn::private"][::std::mem::offset_of!(Rconn, private) - 440usize];
    ["Offset of field: Rconn::status"][::std::mem::offset_of!(Rconn, status) - 448usize];
    ["Offset of field: Rconn::buff"][::std::mem::offset_of!(Rconn, buff) - 456usize];
    ["Offset of field: Rconn::buff_len"][::std::mem::offset_of!(Rconn, buff_len) - 464usize];
    ["Offset of field: Rconn::buff_stored_len"]
        [::std::mem::offset_of!(Rconn, buff_stored_len) - 472usize];
    ["Offset of field: Rconn::buff_pos"][::std::mem::offset_of!(Rconn, buff_pos) - 480usize];
};
extern "C" {
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
