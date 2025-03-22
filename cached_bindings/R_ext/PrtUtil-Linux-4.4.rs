/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: x86_64-pc-linux-gnu */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.4.3 */

unsafe extern "C" {
    #[doc = " Computation of printing formats"]
    pub fn Rf_formatLogical(
        arg1: *const ::std::os::raw::c_int,
        arg2: R_xlen_t,
        arg3: *mut ::std::os::raw::c_int,
    );
    pub fn Rf_formatInteger(
        arg1: *const ::std::os::raw::c_int,
        arg2: R_xlen_t,
        arg3: *mut ::std::os::raw::c_int,
    );
    pub fn Rf_formatReal(
        arg1: *const f64,
        arg2: R_xlen_t,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    );
    pub fn Rf_formatComplex(
        arg1: *const Rcomplex,
        arg2: R_xlen_t,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut ::std::os::raw::c_int,
        arg8: *mut ::std::os::raw::c_int,
        arg9: ::std::os::raw::c_int,
    );
    pub fn formatLogicalS(arg1: SEXP, arg2: R_xlen_t, arg3: *mut ::std::os::raw::c_int);
    pub fn formatIntegerS(arg1: SEXP, arg2: R_xlen_t, arg3: *mut ::std::os::raw::c_int);
    pub fn formatRealS(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    );
    pub fn formatComplexS(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut ::std::os::raw::c_int,
        arg8: *mut ::std::os::raw::c_int,
        arg9: ::std::os::raw::c_int,
    );
    #[doc = " Formating of values"]
    pub fn Rf_EncodeLogical(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn Rf_EncodeInteger(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn Rf_EncodeReal0(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    pub fn Rf_EncodeComplex(
        arg1: Rcomplex,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
        arg7: ::std::os::raw::c_int,
        arg8: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    #[doc = " Legacy, misused by packages RGtk2 and qtbase"]
    pub fn Rf_EncodeReal(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    #[doc = " Printing"]
    pub fn Rf_IndexWidth(arg1: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn Rf_VectorIndex(arg1: R_xlen_t, arg2: ::std::os::raw::c_int);
    #[doc = "void printLogicalVector(int *, R_xlen_t, int);"]
    pub fn Rf_printIntegerVector(
        arg1: *const ::std::os::raw::c_int,
        arg2: R_xlen_t,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_printRealVector(arg1: *const f64, arg2: R_xlen_t, arg3: ::std::os::raw::c_int);
    pub fn Rf_printComplexVector(
        arg1: *const Rcomplex,
        arg2: R_xlen_t,
        arg3: ::std::os::raw::c_int,
    );
    pub fn printIntegerVectorS(arg1: SEXP, arg2: R_xlen_t, arg3: ::std::os::raw::c_int);
    pub fn printRealVectorS(arg1: SEXP, arg2: R_xlen_t, arg3: ::std::os::raw::c_int);
    pub fn printComplexVectorS(arg1: SEXP, arg2: R_xlen_t, arg3: ::std::os::raw::c_int);
}
