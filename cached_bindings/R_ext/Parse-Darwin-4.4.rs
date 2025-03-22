/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.4.3 */

#[repr(u32)]
#[non_exhaustive]
#[doc = " PARSE_NULL will not be returned by R_ParseVector"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ParseStatus {
    PARSE_NULL = 0,
    PARSE_OK = 1,
    PARSE_INCOMPLETE = 2,
    PARSE_ERROR = 3,
    PARSE_EOF = 4,
}
unsafe extern "C" {
    pub fn R_ParseVector(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ParseStatus,
        arg4: SEXP,
    ) -> SEXP;
}
