/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.6.0 */

#[doc = "The signature of the C routine that a callback must implement.\nexpr - the expression for the top-level task that was evaluated.\nvalue - the result of the top-level task, i.e. evaluating expr.\nsucceeded - a logical value indicating whether the task completed properly.\nvisible - a logical value indicating whether the result was printed to the R ``console''/stdout.\ndata - user-level data passed to the registration routine."]
pub type R_ToplevelCallback = ::std::option::Option<
    unsafe extern "C" fn(
        expr: SEXP,
        value: SEXP,
        succeeded: Rboolean,
        visible: Rboolean,
        arg1: *mut ::std::os::raw::c_void,
    ) -> Rboolean,
>;
#[doc = "Linked list element for storing the top-level task callbacks."]
pub type R_ToplevelCallbackEl = _ToplevelCallback;
#[doc = "The following definitions are for callbacks to R functions and\nmethods related to user-level tables.  This was implemented in a\nseparate package formerly available from Omegahat and these\ndeclarations allow the package to interface to the internal R code.\n\nSee https://developer.r-project.org/RObjectTables.pdf."]
pub type R_ObjectTable = _R_ObjectTable;
#[doc = " Do we actually need the exists() since it is never called but R\nuses get to see if the symbol is bound to anything?"]
pub type Rdb_exists = ::std::option::Option<
    unsafe extern "C" fn(
        name: *const ::std::os::raw::c_char,
        canCache: *mut Rboolean,
        arg1: *mut R_ObjectTable,
    ) -> Rboolean,
>;
pub type Rdb_get = ::std::option::Option<
    unsafe extern "C" fn(
        name: *const ::std::os::raw::c_char,
        canCache: *mut Rboolean,
        arg1: *mut R_ObjectTable,
    ) -> SEXP,
>;
pub type Rdb_remove = ::std::option::Option<
    unsafe extern "C" fn(
        name: *const ::std::os::raw::c_char,
        arg1: *mut R_ObjectTable,
    ) -> ::std::os::raw::c_int,
>;
pub type Rdb_assign = ::std::option::Option<
    unsafe extern "C" fn(
        name: *const ::std::os::raw::c_char,
        value: SEXP,
        arg1: *mut R_ObjectTable,
    ) -> SEXP,
>;
pub type Rdb_objects =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut R_ObjectTable) -> SEXP>;
pub type Rdb_canCache = ::std::option::Option<
    unsafe extern "C" fn(name: *const ::std::os::raw::c_char, arg1: *mut R_ObjectTable) -> Rboolean,
>;
pub type Rdb_onDetach = ::std::option::Option<unsafe extern "C" fn(arg1: *mut R_ObjectTable)>;
pub type Rdb_onAttach = ::std::option::Option<unsafe extern "C" fn(arg1: *mut R_ObjectTable)>;
#[doc = "Linked list element for storing the top-level task callbacks."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ToplevelCallback {
    #[doc = " the C routine to call."]
    pub cb: R_ToplevelCallback,
    #[doc = " the user-level data to pass to the call to cb()"]
    pub data: *mut ::std::os::raw::c_void,
    #[doc = " Called when the callback is removed."]
    pub finalizer: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>,
    #[doc = " a name by which to identify this element."]
    pub name: *mut ::std::os::raw::c_char,
    #[doc = " the next element in the linked list."]
    pub next: *mut R_ToplevelCallbackEl,
}
#[repr(C)]
pub struct _R_ObjectTable {
    pub type_: ::std::os::raw::c_int,
    pub cachedNames: *mut *mut ::std::os::raw::c_char,
    pub active: Rboolean,
    pub exists: Rdb_exists,
    pub get: Rdb_get,
    pub remove: Rdb_remove,
    pub assign: Rdb_assign,
    pub objects: Rdb_objects,
    pub canCache: Rdb_canCache,
    pub onDetach: Rdb_onDetach,
    pub onAttach: Rdb_onAttach,
    pub privateData: *mut ::std::os::raw::c_void,
}
unsafe extern "C" {
    pub fn Rf_removeTaskCallbackByIndex(id: ::std::os::raw::c_int) -> Rboolean;
    pub fn Rf_removeTaskCallbackByName(name: *const ::std::os::raw::c_char) -> Rboolean;
    pub fn R_removeTaskCallback(which: SEXP) -> SEXP;
    pub fn Rf_addTaskCallback(
        cb: R_ToplevelCallback,
        data: *mut ::std::os::raw::c_void,
        finalizer: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        name: *const ::std::os::raw::c_char,
        pos: *mut ::std::os::raw::c_int,
    ) -> *mut R_ToplevelCallbackEl;
}
