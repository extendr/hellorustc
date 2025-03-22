/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.4.3 */

unsafe extern "C" {
    #[doc = " Double Precision LINPACK"]
    pub fn dpbfa_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
    );
    pub fn dpbsl_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
    );
    pub fn dpoco_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut ::std::os::raw::c_int,
    );
    pub fn dpodi_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
        arg5: *mut ::std::os::raw::c_int,
    );
    pub fn dpofa_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    );
    pub fn dposl_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
    );
    pub fn dqrdc_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut f64,
        arg8: *mut ::std::os::raw::c_int,
    );
    pub fn dqrsl_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: *mut f64,
        arg9: *mut f64,
        arg10: *mut f64,
        arg11: *mut f64,
        arg12: *mut ::std::os::raw::c_int,
        arg13: *mut ::std::os::raw::c_int,
    );
    pub fn dsvdc_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut f64,
        arg8: *mut ::std::os::raw::c_int,
        arg9: *mut f64,
        arg10: *mut ::std::os::raw::c_int,
        arg11: *mut f64,
        arg12: *mut ::std::os::raw::c_int,
        arg13: *mut ::std::os::raw::c_int,
    );
    pub fn dtrco_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut ::std::os::raw::c_int,
    );
    pub fn dtrsl_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
    );
    #[doc = " The following routines are listed as they have always been declared\nhere, but they are not currently included in R"]
    pub fn dchdc_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut ::std::os::raw::c_int,
    );
    pub fn dchdd_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut ::std::os::raw::c_int,
        arg8: *mut f64,
        arg9: *mut f64,
        arg10: *mut f64,
        arg11: *mut f64,
        arg12: *mut ::std::os::raw::c_int,
    );
    pub fn dchex_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut f64,
        arg7: *mut ::std::os::raw::c_int,
        arg8: *mut ::std::os::raw::c_int,
        arg9: *mut f64,
        arg10: *mut f64,
        arg11: *mut ::std::os::raw::c_int,
    );
    pub fn dchud_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut ::std::os::raw::c_int,
        arg8: *mut f64,
        arg9: *mut f64,
        arg10: *mut f64,
        arg11: *mut f64,
    );
    pub fn dgbco_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut f64,
        arg8: *mut f64,
    );
    pub fn dgbdi_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut f64,
    );
    pub fn dgbfa_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut ::std::os::raw::c_int,
    );
    pub fn dgbsl_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut f64,
        arg8: *mut ::std::os::raw::c_int,
    );
    pub fn dgeco_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut f64,
    );
    pub fn dgedi_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut ::std::os::raw::c_int,
    );
    pub fn dgefa_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
    );
    pub fn dgesl_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut ::std::os::raw::c_int,
    );
    pub fn dgtsl_(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut f64,
        arg6: *mut ::std::os::raw::c_int,
    );
    pub fn dpbco_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut ::std::os::raw::c_int,
    );
    pub fn dpbdi_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
    );
    pub fn dppco_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut f64,
        arg4: *mut f64,
        arg5: *mut ::std::os::raw::c_int,
    );
    pub fn dppdi_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut f64,
        arg4: *mut ::std::os::raw::c_int,
    );
    pub fn dppfa_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
    );
    pub fn dppsl_(arg1: *mut f64, arg2: *mut ::std::os::raw::c_int, arg3: *mut f64);
    pub fn dptsl_(arg1: *mut ::std::os::raw::c_int, arg2: *mut f64, arg3: *mut f64, arg4: *mut f64);
    pub fn dsico_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut f64,
    );
    pub fn dsidi_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut f64,
        arg8: *mut ::std::os::raw::c_int,
    );
    pub fn dsifa_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_int,
    );
    pub fn dsisl_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
        arg5: *mut f64,
    );
    pub fn dspco_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
        arg5: *mut f64,
    );
    pub fn dspdi_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
        arg5: *mut ::std::os::raw::c_int,
        arg6: *mut f64,
        arg7: *mut ::std::os::raw::c_int,
    );
    pub fn dspfa_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    );
    pub fn dspsl_(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut f64,
    );
}
