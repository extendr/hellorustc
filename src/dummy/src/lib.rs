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

// #include <R.h>
// #include <Rinternals.h>

// // Function to add two numbers
// SEXP add(SEXP x, SEXP y) {
//     // Ensure inputs are numeric vectors of length 1
//     if (!isReal(x) || LENGTH(x) != 1) {
//         error("x must be a numeric value");
//     }
//     if (!isReal(y) || LENGTH(y) != 1) {
//         error("y must be a numeric value");
//     }

//     // Retrieve the numeric values
//     double x_value = REAL(x)[0];
//     double y_value = REAL(y)[0];

//     // Perform the addition
//     SEXP result = PROTECT(allocVector(REALSXP, 1));
//     REAL(result)[0] = x_value + y_value;

//     UNPROTECT(1);
//     return result;
// }

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
