/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.3.3 */

pub type R_NativePrimitiveArgType = :: std :: os :: raw :: c_uint ; # [doc = "These are very similar to those in Rdynpriv.h,\nbut we maintain them separately to give us more freedom to do\nsome computations on the internal versions that are derived from\nthese definitions."] pub type R_FortranMethodDef = R_CMethodDef ; pub type R_ExternalMethodDef = R_CallMethodDef ; pub type DllInfo = _DllInfo ; pub type R_RegisteredNativeSymbol = Rf_RegisteredNativeSymbol ; # [doc = "These are very similar to those in Rdynpriv.h,\nbut we maintain them separately to give us more freedom to do\nsome computations on the internal versions that are derived from\nthese definitions."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct R_CMethodDef { pub name : * const :: std :: os :: raw :: c_char , pub fun : DL_FUNC , pub numArgs : :: std :: os :: raw :: c_int , pub types : * mut R_NativePrimitiveArgType , } # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct R_CallMethodDef { pub name : * const :: std :: os :: raw :: c_char , pub fun : DL_FUNC , pub numArgs : :: std :: os :: raw :: c_int , } # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct _DllInfo { _unused : [u8 ; 0] , } # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct Rf_RegisteredNativeSymbol { _unused : [u8 ; 0] , } pub const SINGLESXP : u32 = 302 ; # [repr (i32)] # [non_exhaustive] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum NativeSymbolType { R_ANY_SYM = 0 , R_C_SYM = 1 , R_CALL_SYM = 2 , R_FORTRAN_SYM = 3 , R_EXTERNAL_SYM = 4 , } unsafe extern "C" { pub fn R_registerRoutines (info : * mut DllInfo , croutines : * const R_CMethodDef , callRoutines : * const R_CallMethodDef , fortranRoutines : * const R_FortranMethodDef , externalRoutines : * const R_ExternalMethodDef) -> :: std :: os :: raw :: c_int ; pub fn R_useDynamicSymbols (info : * mut DllInfo , value : Rboolean) -> Rboolean ; pub fn R_forceSymbols (info : * mut DllInfo , value : Rboolean) -> Rboolean ; pub fn R_getDllInfo (name : * const :: std :: os :: raw :: c_char) -> * mut DllInfo ; # [doc = " To be used by applications embedding R to register their symbols\nthat are not related to any dynamic module"] pub fn R_getEmbeddingDllInfo () -> * mut DllInfo ; pub fn R_FindSymbol (arg1 : * const :: std :: os :: raw :: c_char , arg2 : * const :: std :: os :: raw :: c_char , symbol : * mut R_RegisteredNativeSymbol) -> DL_FUNC ; # [doc = " Interface for exporting and importing functions from one package\nfor use from C code in a package.  The registration part probably\nought to be integrated with the other registrations.  The naming of\nthese routines may be less than ideal."] pub fn R_RegisterCCallable (package : * const :: std :: os :: raw :: c_char , name : * const :: std :: os :: raw :: c_char , fptr : DL_FUNC) ; pub fn R_GetCCallable (package : * const :: std :: os :: raw :: c_char , name : * const :: std :: os :: raw :: c_char) -> DL_FUNC ; }