#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("custom_bindings/R.rs");

// TODO: What should this be? libc::FILE?
pub enum FILE {}

include!("custom_bindings/R_ext/Boolean.rs");
include!("custom_bindings/custom_Boolean.rs");

include!("custom_bindings/custom_Altrep.rs");
include!("custom_bindings/R_ext/Altrep.rs");
include!("custom_bindings/R_ext/Applic.rs");
include!("custom_bindings/R_ext/Arith.rs");
include!("custom_bindings/R_ext/BLAS.rs");
include!("custom_bindings/R_ext/Lapack.rs");
include!("custom_bindings/R_ext/Linpack.rs");

include!("custom_bindings/R_ext/Callbacks.rs");

include!("custom_bindings/R_ext/Complex.rs");

// include!("bindings/R_ext/Connections.rs"); // FIXME: missing va_list!?
include!("custom_bindings/R_ext/Constants.rs");
include!("custom_bindings/R_ext/Error.rs");

// include!("bindings/R_ext/eventloop.rs"); // FIXME: missing libc::fd_set??
#[cfg(unix)]
include!("custom_bindings/R_ext/GetX11Image.rs");

include!("custom_bindings/R_ext/GraphicsDevice.rs");
include!("custom_bindings/R_ext/GraphicsEngine.rs");

include!("custom_bindings/R_ext/Itermacros.rs");
include!("custom_bindings/R_ext/libextern.rs");
include!("custom_bindings/R_ext/MathThreads.rs");
include!("custom_bindings/R_ext/Memory.rs");
include!("custom_bindings/R_ext/Parse.rs");
include!("custom_bindings/R_ext/Print.rs"); // missing va_list
include!("custom_bindings/R_ext/PrtUtil.rs");
#[cfg(unix)]
include!("custom_bindings/R_ext/QuartzDevice.rs");
include!("custom_bindings/R_ext/Rallocators.rs");
include!("custom_bindings/R_ext/Random.rs");

include!("custom_bindings/R_ext/Rdynload.rs");
include!("custom_bindings/custom_Rdynload.rs");

include!("custom_bindings/R_ext/Riconv.rs");
include!("custom_bindings/R_ext/RS.rs");
include!("custom_bindings/R_ext/RStartup.rs");
include!("custom_bindings/R_ext/stats_package.rs");
// include!("bindings/R_ext/stats_stubs.rs"); // FIXME: redefined in stats_package!
include!("custom_bindings/R_ext/Utils.rs"); // FIXME: redefined in Applic!
include!("custom_bindings/R_ext/Visibility.rs");

include!("custom_bindings/Rinternals.rs");
include!("custom_bindings/custom_Rinternals.rs");

// #[cfg(target_os = "macos")]
// include!("bindings/libintl.rs");
include!("custom_bindings/Rconfig.rs");
include!("custom_bindings/Rdefines.rs");
include!("custom_bindings/Rembedded.rs"); // defined in R internals!

// include!("bindings/Rinterface.rs"); //FIXME: conflicts with Rembedded
include!("custom_bindings/Rmath.rs"); // defined elements in R.rs
include!("custom_bindings/Rversion.rs"); // defined elements in R.rs

// note: not present in 4.3.3, windows-latest
#[cfg(windows)]
include!("custom_bindings/iconv.rs");

// FIXME: there is an issue with wchar_t binding that ought to be debugged on windows.
// #[cfg(windows)]
// include!("bindings/ga.rs");

#[cfg(windows)]
include!("bindings/graphapp.rs");
