#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate bindings;

pub use bindings::*;
pub use bindings::{
    DllInfo, R_CallMethodDef, R_registerRoutines, R_useDynamicSymbols, Rboolean, Rf_mkString, SEXP,
};

extern "C" {
    fn ultimate_answer() -> SEXP;
}

#[no_mangle]
pub unsafe extern "C" fn hellorustc() -> SEXP {
    Rf_mkString(c"hello rustc".as_ptr())
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn R_init_hellorustc(dll: *mut DllInfo) {
    use std::ptr;
    let call_entries = [
        R_CallMethodDef {
            name: c"hellorustc".as_ptr(),
            fun: Some(std::mem::transmute::<
                unsafe extern "C" fn() -> SEXP,
                unsafe extern "C" fn(...) -> SEXP,
            >(hellorustc)),
            numArgs: 0,
        },
        R_CallMethodDef {
            name: c"ultimate_answer".as_ptr(),
            fun: Some(std::mem::transmute::<
                unsafe extern "C" fn() -> SEXP,
                unsafe extern "C" fn(...) -> SEXP,
            >(ultimate_answer)),
            numArgs: 0,
        },
        R_CallMethodDef {
            name: ptr::null(),
            fun: None,
            numArgs: 0,
        },
    ];

    R_registerRoutines(
        dll,
        ptr::null(),
        call_entries.as_ptr(),
        ptr::null(),
        ptr::null(),
    );
    R_useDynamicSymbols(dll, Rboolean::FALSE);
    /* Register routines,
    allocate resources. */
}
