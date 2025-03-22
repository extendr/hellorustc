/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.3.3 */

unsafe extern "C" { # [doc = " ../../main/sort.c :"] pub fn R_isort (arg1 : * mut :: std :: os :: raw :: c_int , arg2 : :: std :: os :: raw :: c_int) ; pub fn R_rsort (arg1 : * mut f64 , arg2 : :: std :: os :: raw :: c_int) ; pub fn R_csort (arg1 : * mut Rcomplex , arg2 : :: std :: os :: raw :: c_int) ; pub fn rsort_with_index (arg1 : * mut f64 , arg2 : * mut :: std :: os :: raw :: c_int , arg3 : :: std :: os :: raw :: c_int) ; pub fn Rf_revsort (arg1 : * mut f64 , arg2 : * mut :: std :: os :: raw :: c_int , arg3 : :: std :: os :: raw :: c_int) ; pub fn Rf_iPsort (arg1 : * mut :: std :: os :: raw :: c_int , arg2 : :: std :: os :: raw :: c_int , arg3 : :: std :: os :: raw :: c_int) ; pub fn Rf_rPsort (arg1 : * mut f64 , arg2 : :: std :: os :: raw :: c_int , arg3 : :: std :: os :: raw :: c_int) ; pub fn Rf_cPsort (arg1 : * mut Rcomplex , arg2 : :: std :: os :: raw :: c_int , arg3 : :: std :: os :: raw :: c_int) ; # [doc = " ../../main/qsort.c : */\n/* dummy renamed to II to avoid problems with g++ on Solaris"] pub fn R_qsort (v : * mut f64 , i : usize , j : usize) ; pub fn R_qsort_I (v : * mut f64 , II : * mut :: std :: os :: raw :: c_int , i : :: std :: os :: raw :: c_int , j : :: std :: os :: raw :: c_int) ; pub fn R_qsort_int (iv : * mut :: std :: os :: raw :: c_int , i : usize , j : usize) ; pub fn R_qsort_int_I (iv : * mut :: std :: os :: raw :: c_int , II : * mut :: std :: os :: raw :: c_int , i : :: std :: os :: raw :: c_int , j : :: std :: os :: raw :: c_int) ; # [doc = " ../../main/util.c  and others :"] pub fn R_ExpandFileName (arg1 : * const :: std :: os :: raw :: c_char) -> * const :: std :: os :: raw :: c_char ; pub fn R_ExpandFileNameUTF8 (arg1 : * const :: std :: os :: raw :: c_char) -> * const :: std :: os :: raw :: c_char ; pub fn Rf_setIVector (arg1 : * mut :: std :: os :: raw :: c_int , arg2 : :: std :: os :: raw :: c_int , arg3 : :: std :: os :: raw :: c_int) ; pub fn Rf_setRVector (arg1 : * mut f64 , arg2 : :: std :: os :: raw :: c_int , arg3 : f64) ; pub fn Rf_StringFalse (arg1 : * const :: std :: os :: raw :: c_char) -> Rboolean ; pub fn Rf_StringTrue (arg1 : * const :: std :: os :: raw :: c_char) -> Rboolean ; pub fn Rf_isBlankString (arg1 : * const :: std :: os :: raw :: c_char) -> Rboolean ; # [doc = " These two are guaranteed to use '.' as the decimal point,\nand to accept \"NA\"."] pub fn R_atof (str_ : * const :: std :: os :: raw :: c_char) -> f64 ; pub fn R_strtod (c : * const :: std :: os :: raw :: c_char , end : * mut * mut :: std :: os :: raw :: c_char) -> f64 ; pub fn R_tmpnam (prefix : * const :: std :: os :: raw :: c_char , tempdir : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ; pub fn R_tmpnam2 (prefix : * const :: std :: os :: raw :: c_char , tempdir : * const :: std :: os :: raw :: c_char , fileext : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ; pub fn R_free_tmpnam (name : * mut :: std :: os :: raw :: c_char) ; pub fn R_CheckUserInterrupt () ; pub fn R_CheckStack () ; pub fn R_CheckStack2 (arg1 : usize) ; # [doc = " ../../appl/interv.c: also in Applic.h"] pub fn findInterval (xt : * mut f64 , n : :: std :: os :: raw :: c_int , x : f64 , rightmost_closed : Rboolean , all_inside : Rboolean , ilo : :: std :: os :: raw :: c_int , mflag : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ; pub fn findInterval2 (xt : * mut f64 , n : :: std :: os :: raw :: c_int , x : f64 , rightmost_closed : Rboolean , all_inside : Rboolean , left_open : Rboolean , ilo : :: std :: os :: raw :: c_int , mflag : * mut :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ; pub fn find_interv_vec (xt : * mut f64 , n : * mut :: std :: os :: raw :: c_int , x : * mut f64 , nx : * mut :: std :: os :: raw :: c_int , rightmost_closed : * mut :: std :: os :: raw :: c_int , all_inside : * mut :: std :: os :: raw :: c_int , indx : * mut :: std :: os :: raw :: c_int) ; # [doc = " ../../appl/maxcol.c: also in Applic.h"] pub fn R_max_col (matrix : * mut f64 , nr : * mut :: std :: os :: raw :: c_int , nc : * mut :: std :: os :: raw :: c_int , maxes : * mut :: std :: os :: raw :: c_int , ties_meth : * mut :: std :: os :: raw :: c_int) ; }