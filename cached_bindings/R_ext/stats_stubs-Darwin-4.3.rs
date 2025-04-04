/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.3.3 */

unsafe extern "C" {
    pub fn S_Rf_divset(
        alg: ::std::os::raw::c_int,
        iv: *mut ::std::os::raw::c_int,
        liv: ::std::os::raw::c_int,
        lv: ::std::os::raw::c_int,
        v: *mut f64,
    );
    pub fn S_nlminb_iterate(
        b: *mut f64,
        d: *mut f64,
        fx: f64,
        g: *mut f64,
        h: *mut f64,
        iv: *mut ::std::os::raw::c_int,
        liv: ::std::os::raw::c_int,
        lv: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        v: *mut f64,
        x: *mut f64,
    );
    pub fn S_nlsb_iterate(
        b: *mut f64,
        d: *mut f64,
        dr: *mut f64,
        iv: *mut ::std::os::raw::c_int,
        liv: ::std::os::raw::c_int,
        lv: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nd: ::std::os::raw::c_int,
        p: ::std::os::raw::c_int,
        r: *mut f64,
        rd: *mut f64,
        v: *mut f64,
        x: *mut f64,
    );
    pub fn S_rcont2(
        nrow: ::std::os::raw::c_int,
        ncol: ::std::os::raw::c_int,
        nrowt: *const ::std::os::raw::c_int,
        ncolt: *const ::std::os::raw::c_int,
        ntotal: ::std::os::raw::c_int,
        fact: *const f64,
        jwork: *mut ::std::os::raw::c_int,
        matrix: *mut ::std::os::raw::c_int,
    );
}
