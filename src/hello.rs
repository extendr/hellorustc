#[non_exhaustive]
#[repr(transparent)]
#[derive(Debug)]
pub struct SEXPREC(std::ffi::c_void);

// TODO: What should this be? libc::FILE?
pub enum FILE {}

include!("bindings/R_ext/Boolean.rs");
include!("bindings/R_ext/Complex.rs");

include!("bindings/R_ext/Rdynload.rs");
include!("bindings/Rinternals.rs");

#[no_mangle]
pub unsafe extern "C" fn hellorustc() -> SEXP {
    Rf_mkString(c"hello rustc".as_ptr())
}
