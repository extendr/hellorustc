
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!("bindings/R.rs");

// TODO: What should this be? libc::FILE?
pub enum FILE {}

include!("bindings/R_ext/Boolean.rs");
include!("custom_bindings/custom_Boolean.rs");

include!("custom_bindings/custom_Altrep.rs");
include!("bindings/R_ext/Altrep.rs");
include!("bindings/R_ext/Applic.rs");
include!("bindings/R_ext/Arith.rs");
include!("bindings/R_ext/BLAS.rs");
include!("bindings/R_ext/Lapack.rs");
include!("bindings/R_ext/Linpack.rs");

include!("bindings/R_ext/Callbacks.rs");

include!("bindings/R_ext/Complex.rs");

// include!("bindings/R_ext/Connections.rs"); // FIXME: missing va_list!?
include!("bindings/R_ext/Constants.rs");
include!("bindings/R_ext/Error.rs");

// include!("bindings/R_ext/eventloop.rs"); // FIXME: missing libc::fd_set??
#[cfg(unix)]
include!("bindings/R_ext/GetX11Image.rs");

include!("bindings/R_ext/GraphicsDevice.rs");
include!("bindings/R_ext/GraphicsEngine.rs");

include!("bindings/R_ext/Itermacros.rs");
include!("bindings/R_ext/libextern.rs");
include!("bindings/R_ext/MathThreads.rs");
include!("bindings/R_ext/Memory.rs");
include!("bindings/R_ext/Parse.rs");
// include!("bindings/R_ext/Print.rs"); // missing va_list
include!("bindings/R_ext/PrtUtil.rs");
#[cfg(unix)]
include!("bindings/R_ext/QuartzDevice.rs");
// include!("bindings/R_ext/Rallocators.rs"); // FIXME: defined in Rinternals
include!("bindings/R_ext/Random.rs");

include!("bindings/R_ext/Rdynload.rs");
include!("custom_bindings/custom_Rdynload.rs");

include!("bindings/R_ext/Riconv.rs");
include!("bindings/R_ext/RS.rs");
include!("bindings/R_ext/RStartup.rs");
include!("bindings/R_ext/stats_package.rs");
// include!("bindings/R_ext/stats_stubs.rs"); // FIXME: redefined in stats_package!
// include!("bindings/R_ext/Utils.rs"); // FIXME: redefined in Applic!
include!("bindings/R_ext/Visibility.rs");

include!("bindings/Rinternals.rs");
include!("custom_bindings/custom_Rinternals.rs");

#[cfg(target_os = "macos")]
include!("bindings/libintl.rs");
include!("bindings/Rconfig.rs");
include!("bindings/Rdefines.rs");
// include!("bindings/Rembedded.rs"); // defined in R internals!
// include!("bindings/Rinterface.rs");
// include!("bindings/Rmath.rs"); // defined elements in R.rs
include!("bindings/Rversion.rs"); // defined elements in R.rs


