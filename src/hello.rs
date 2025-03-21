#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[non_exhaustive]
#[repr(transparent)]
#[derive(Debug)]
pub struct SEXPREC(std::ffi::c_void);

// TODO: What should this be? libc::FILE?
pub enum FILE {}

include!("bindings/R_ext/Boolean.rs");
include!("bindings/R_ext/Complex.rs");

include!("bindings/R_ext/Rdynload.rs");
include!("custom_bindings/custom_Rdynload.rs");
include!("bindings/Rinternals.rs");

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
            fun: Some(std::mem::transmute::<unsafe extern "C" fn () -> SEXP, unsafe extern "C" fn(...) -> SEXP>(hellorustc)),
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
