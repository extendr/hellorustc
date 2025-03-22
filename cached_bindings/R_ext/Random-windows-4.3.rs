/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.3.3 */

pub type Int32 = :: std :: os :: raw :: c_uint ; # [repr (i32)] # [non_exhaustive] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum RNGtype { WICHMANN_HILL = 0 , MARSAGLIA_MULTICARRY = 1 , SUPER_DUPER = 2 , MERSENNE_TWISTER = 3 , KNUTH_TAOCP = 4 , USER_UNIF = 5 , KNUTH_TAOCP2 = 6 , LECUYER_CMRG = 7 , } # [repr (i32)] # [non_exhaustive] # [doc = " Different kinds of \"N(0,1)\" generators :"] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum N01type { BUGGY_KINDERMAN_RAMAGE = 0 , AHRENS_DIETER = 1 , BOX_MULLER = 2 , USER_NORM = 3 , INVERSION = 4 , KINDERMAN_RAMAGE = 5 , } # [repr (i32)] # [non_exhaustive] # [doc = " Different ways to generate discrete uniform samples"] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum Sampletype { ROUNDING = 0 , REJECTION = 1 , } unsafe extern "C" { pub fn R_sample_kind () -> Sampletype ; pub fn GetRNGstate () ; pub fn PutRNGstate () ; pub fn unif_rand () -> f64 ; pub fn R_unif_index (arg1 : f64) -> f64 ; # [doc = " These are also defined in Rmath.h"] pub fn norm_rand () -> f64 ; pub fn exp_rand () -> f64 ; pub fn user_unif_rand () -> * mut f64 ; pub fn user_unif_init (arg1 : Int32) ; pub fn user_unif_nseed () -> * mut :: std :: os :: raw :: c_int ; pub fn user_unif_seedloc () -> * mut :: std :: os :: raw :: c_int ; pub fn user_norm_rand () -> * mut f64 ; }