/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: x86_64-pc-linux-gnu */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.3.3 */

pub type InputHandlerProc =
    ::std::option::Option<unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void)>;
pub type InputHandler = _InputHandler;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _InputHandler {
    pub activity: ::std::os::raw::c_int,
    pub fileDescriptor: ::std::os::raw::c_int,
    pub handler: InputHandlerProc,
    pub next: *mut _InputHandler,
    #[doc = " Whether we should be listening to this file descriptor or not."]
    pub active: ::std::os::raw::c_int,
    #[doc = " Data that can be passed to the routine as its only argument.\nThis might be a user-level function or closure when we implement\na callback to R mechanism."]
    pub userData: *mut ::std::os::raw::c_void,
}
pub const XActivity: u32 = 1;
pub const StdinActivity: u32 = 2;
unsafe extern "C" {
    pub fn initStdinHandler() -> *mut InputHandler;
    pub fn consoleInputHandler(buf: *mut ::std::os::raw::c_uchar, len: ::std::os::raw::c_int);
    pub fn addInputHandler(
        handlers: *mut InputHandler,
        fd: ::std::os::raw::c_int,
        handler: InputHandlerProc,
        activity: ::std::os::raw::c_int,
    ) -> *mut InputHandler;
    pub fn getInputHandler(
        handlers: *mut InputHandler,
        fd: ::std::os::raw::c_int,
    ) -> *mut InputHandler;
    pub fn removeInputHandler(
        handlers: *mut *mut InputHandler,
        it: *mut InputHandler,
    ) -> ::std::os::raw::c_int;
    pub fn getSelectedHandler(handlers: *mut InputHandler, mask: *mut fd_set) -> *mut InputHandler;
    pub fn R_checkActivity(
        usec: ::std::os::raw::c_int,
        ignore_stdin: ::std::os::raw::c_int,
    ) -> *mut fd_set;
    pub fn R_checkActivityEx(
        usec: ::std::os::raw::c_int,
        ignore_stdin: ::std::os::raw::c_int,
        intr: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> *mut fd_set;
    pub fn R_runHandlers(handlers: *mut InputHandler, mask: *mut fd_set);
    pub fn R_SelectEx(
        n: ::std::os::raw::c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
        intr: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
    pub static mut R_InputHandlers: *mut InputHandler;
    pub static mut R_PolledEvents: ::std::option::Option<unsafe extern "C" fn()>;
    pub static mut R_wait_usec: ::std::os::raw::c_int;
}
