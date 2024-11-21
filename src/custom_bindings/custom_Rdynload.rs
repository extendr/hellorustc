/// The definition in the C-header `Rdynload.h` of this is
///
/// ```c
/// typedef void * (*DL_FUNC)(void);
/// ```
///
/// which implies that it is a function pointer to a function that takes no
/// arguments and return `void*`. This goes against the expectation of
/// `.Call` functions, which are `fn(SEXP, ...) -> SEXP`, which means
/// 0 or more `SEXP`s as arguments, and must return `SEXP`.
///
/// The `DL_FUNC` return type has been redefined to `SEXP`, as `SEXP` is equivalent
/// to `*c_void`, due to the opaque definition of `SEXPREC` in these bindings.
pub type DL_FUNC = ::std::option::Option<unsafe extern "C" fn(...) -> SEXP>;
