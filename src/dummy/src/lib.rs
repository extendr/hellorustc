// add.c
mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!("../bindings/R.rs");

    #[non_exhaustive]
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct SEXPREC(std::ffi::c_void);

    // TODO: What should this be? libc::FILE?
    pub enum FILE {}

    include!("../bindings/R_ext/Boolean.rs");
    include!("../bindings/R_ext/Complex.rs");

    include!("../bindings/R_ext/Rdynload.rs");
    include!("../bindings/Rinternals.rs");
}
#[allow(unused_imports)]
use bindings::*;

#[no_mangle]
extern "C" fn ultimate_answer() -> SEXP {
    unsafe { Rf_ScalarInteger(42_i32) }
}

include!("../bindings/R_ext/Error.rs");

#[no_mangle]
unsafe extern "C" fn add(x: SEXP, y: SEXP) -> SEXP {
    use std::ffi::CStr;

    // Ensure inputs are numeric vectors of length 1
    if Rf_isReal(x) == Rboolean::FALSE || Rf_xlength(x) != 1 {
        Rf_error(
            CStr::from_bytes_until_nul(b"x must be a numeric value\nul")
                .unwrap()
                .as_ptr(),
        );
    }
    if Rf_isReal(y) == Rboolean::FALSE || Rf_xlength(y) != 1 {
        Rf_error(
            CStr::from_bytes_until_nul(b"y must be a numeric value\nul")
                .unwrap()
                .as_ptr(),
        );
    }

    // Retrieve the numeric values
    let x_value = REAL(x).read();
    let y_value = REAL(y).read();

    // Perform the addition
    let result = Rf_protect(Rf_allocVector(SEXPTYPE::REALSXP, 1));
    REAL(result).write(x_value + y_value);

    Rf_unprotect(1);
    result
}
