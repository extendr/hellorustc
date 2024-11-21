#[non_exhaustive]
#[repr(transparent)]
#[derive(Debug)]
pub struct SEXPREC(std::ffi::c_void);

extern "C" {
    // Return type should match `SEXPTYPE`
    pub fn TYPEOF(x: SEXP) -> SEXPTYPE;
}

mod secret {
    use super::*;
    extern "C" {
        #[link_name = "Rf_isS4"]
        pub fn Rf_isS4_original(arg1: SEXP) -> u32;
    }
}

pub unsafe fn Rf_isS4(arg1: SEXP) -> Rboolean {
    unsafe {
        if secret::Rf_isS4_original(arg1) == 0 {
            Rboolean::FALSE
        } else {
            Rboolean::TRUE
        }
    }
}
