/* automatically generated by rust-bindgen 0.70.1 */

pub const Rboolean_FALSE: Rboolean = 0;
pub const Rboolean_TRUE: Rboolean = 1;
pub type Rboolean = u32;
extern "C" {
    pub fn Rf_initEmbeddedR(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn Rf_endEmbeddedR(fatal: ::std::os::raw::c_int);
    pub fn Rf_initialize_R(
        ac: ::std::os::raw::c_int,
        av: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn setup_Rmainloop();
    pub fn R_ReplDLLinit();
    pub fn R_ReplDLLdo1() -> ::std::os::raw::c_int;
    pub fn R_setStartTime();
    pub fn R_RunExitFinalizers();
    pub fn CleanEd();
    pub fn Rf_KillAllDevices();
    pub static mut R_DirtyImage: ::std::os::raw::c_int;
    pub fn R_CleanTempDir();
    pub static mut R_TempDir: *mut ::std::os::raw::c_char;
    pub fn R_SaveGlobalEnv();
    pub fn fpu_setup(start: Rboolean);
}
