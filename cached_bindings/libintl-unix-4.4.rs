/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.4.3 */

pub const _LIBINTL_H: u32 = 1;
pub const __USE_GNU_GETTEXT: u32 = 1;
pub const LIBINTL_VERSION: u32 = 4352;
unsafe extern "C" {
    pub static mut libintl_version: ::std::os::raw::c_int;
    pub fn libintl_gettext(__msgid: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn libintl_dgettext(
        __domainname: *const ::std::os::raw::c_char,
        __msgid: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn libintl_dcgettext(
        __domainname: *const ::std::os::raw::c_char,
        __msgid: *const ::std::os::raw::c_char,
        __category: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn libintl_ngettext(
        __msgid1: *const ::std::os::raw::c_char,
        __msgid2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
    pub fn libintl_dngettext(
        __domainname: *const ::std::os::raw::c_char,
        __msgid1: *const ::std::os::raw::c_char,
        __msgid2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
    pub fn libintl_dcngettext(
        __domainname: *const ::std::os::raw::c_char,
        __msgid1: *const ::std::os::raw::c_char,
        __msgid2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
        __category: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn libintl_textdomain(
        __domainname: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn libintl_bindtextdomain(
        __domainname: *const ::std::os::raw::c_char,
        __dirname: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn libintl_bind_textdomain_codeset(
        __domainname: *const ::std::os::raw::c_char,
        __codeset: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn libintl_set_relocation_prefix(
        orig_prefix: *const ::std::os::raw::c_char,
        curr_prefix: *const ::std::os::raw::c_char,
    );
}
