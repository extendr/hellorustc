extern crate bindings;
// pub use bindings::*;
use bindings::{
    Rboolean, Rf_ScalarInteger, Rf_allocVector, Rf_error, Rf_isReal, Rf_protect, Rf_unprotect,
    Rf_xlength, REAL, REAL0, SEXP, SEXPTYPE,
};

// `REAL0` is non-api. But this does not cause r-cmd-check failure`
#[allow(dead_code)]
unsafe extern "C" fn add2(x: SEXP) {
    REAL0(x);
}

#[no_mangle]
unsafe extern "C" fn add(x: SEXP, y: SEXP) -> SEXP {
    // Ensure inputs are numeric vectors of length 1
    if Rf_isReal(x) == Rboolean::FALSE || Rf_xlength(x) != 1 {
        Rf_error((c"x must be a numeric value").as_ptr());
    }
    if Rf_isReal(y) == Rboolean::FALSE || Rf_xlength(y) != 1 {
        Rf_error((c"y must be a numeric value").as_ptr());
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

#[no_mangle]
pub unsafe extern "C" fn ultimate_answer() -> SEXP {
    Rf_ScalarInteger(4242_i32)
}

// #[lang = "eh_personality"]
// extern "C" fn rust_eh_personality() {}

// #[allow(non_snake_case)]
// pub unsafe extern "C" fn R_init_hellorustc(dll: *mut DllInfo) {
//     use std::ptr;
//     let call_entries = [
//         R_CallMethodDef {
//             name: c"ultimate_answer".as_ptr(),
//             // fun: Some(std::mem::transmute(ultimate_answer)),
//             // fun: Some(ultimate_answer as  unsafe extern "C" fn() -> *mut std::ffi::c_void),
//             fun: Some(std::mem::transmute(&ultimate_answer)),
//             numArgs: 0,
//         },
//         R_CallMethodDef {
//             name: ptr::null(),
//             fun: None,
//             numArgs: 0,
//         },
//     ];

//     R_registerRoutines(
//         dll,
//         ptr::null(),
//         call_entries.as_ptr(),
//         ptr::null(),
//         ptr::null(),
//     );
//     R_useDynamicSymbols(dll, Rboolean::FALSE);
//     /* Register routines,
//     allocate resources. */
// }
