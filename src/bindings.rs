
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("bindings/R.rs");

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