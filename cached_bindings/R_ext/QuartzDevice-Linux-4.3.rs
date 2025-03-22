/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: x86_64-pc-linux-gnu */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.3.3 */

pub type CGContextRef = *mut ::std::os::raw::c_void;
pub type QuartzDesc_t = *mut ::std::os::raw::c_void;
pub type QuartzBackend_t = QuartzBackend_s;
#[doc = " parameters that are passed to functions that create backends"]
pub type QuartzParameters_t = QuartzParameters_s;
pub type QuartzFunctions_t = QuartzFunctons_s;
#[doc = " type of a Quartz contructor"]
pub type quartz_create_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        dd: *mut ::std::os::raw::c_void,
        fn_: *mut QuartzFunctions_t,
        par: *mut QuartzParameters_t,
    ) -> QuartzDesc_t,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QuartzBackend_s {
    #[doc = " structure size"]
    pub size: ::std::os::raw::c_int,
    pub width: f64,
    pub height: f64,
    pub scalex: f64,
    pub scaley: f64,
    pub pointsize: f64,
    pub bg: ::std::os::raw::c_int,
    pub canvas: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub userInfo: *mut ::std::os::raw::c_void,
    #[doc = " Get the context for this device"]
    pub getCGContext: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
        ) -> CGContextRef,
    >,
    pub locatePoint: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
            x: *mut f64,
            y: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub close: ::std::option::Option<
        unsafe extern "C" fn(dev: QuartzDesc_t, userInfo: *mut ::std::os::raw::c_void),
    >,
    pub newPage: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
            flags: ::std::os::raw::c_int,
        ),
    >,
    pub state: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
            state: ::std::os::raw::c_int,
        ),
    >,
    pub par: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
            set: ::std::os::raw::c_int,
            key: *const ::std::os::raw::c_char,
            value: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub sync: ::std::option::Option<
        unsafe extern "C" fn(dev: QuartzDesc_t, userInfo: *mut ::std::os::raw::c_void),
    >,
    pub cap: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
#[doc = " parameters that are passed to functions that create backends"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QuartzParameters_s {
    #[doc = " structure size"]
    pub size: ::std::os::raw::c_int,
    pub type_: *const ::std::os::raw::c_char,
    pub file: *const ::std::os::raw::c_char,
    pub title: *const ::std::os::raw::c_char,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub pointsize: f64,
    pub family: *const ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_int,
    pub connection: ::std::os::raw::c_int,
    pub bg: ::std::os::raw::c_int,
    pub canvas: ::std::os::raw::c_int,
    pub dpi: *mut f64,
    #[doc = " the following parameters can be used to pass custom parameters when desired"]
    pub pard1: f64,
    #[doc = " the following parameters can be used to pass custom parameters when desired"]
    pub pard2: f64,
    pub pari1: ::std::os::raw::c_int,
    pub pari2: ::std::os::raw::c_int,
    pub pars1: *const ::std::os::raw::c_char,
    pub pars2: *const ::std::os::raw::c_char,
    pub parv: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QuartzFunctons_s {
    #[doc = " create a new device"]
    pub Create: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut QuartzBackend_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    #[doc = " returns device number"]
    pub DevNumber:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> ::std::os::raw::c_int>,
    #[doc = " call to close the device"]
    pub Kill: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t)>,
    #[doc = " notifies Q back-end that the implementation has created a new context"]
    pub ResetContext: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t)>,
    #[doc = " get device width (in inches)"]
    pub GetWidth: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    #[doc = " get device height (in inches)"]
    pub GetHeight: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    #[doc = " set device size (in inches)"]
    pub SetSize:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, width: f64, height: f64)>,
    #[doc = " get device width (in pixels)"]
    pub GetScaledWidth: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    #[doc = " get device height (in pixels)"]
    pub GetScaledHeight: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    #[doc = " set device size (in pixels)"]
    pub SetScaledSize:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, width: f64, height: f64)>,
    #[doc = " get x scale factor (px/pt ratio)"]
    pub GetXScale: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    #[doc = " get y scale factor (px/pt ratio)"]
    pub GetYScale: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    #[doc = " sets both scale factors (px/pt ratio)"]
    pub SetScale:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, scalex: f64, scaley: f64)>,
    #[doc = " sets text scale factor"]
    pub SetTextScale: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, scale: f64)>,
    #[doc = " sets text scale factor"]
    pub GetTextScale: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    #[doc = " sets point size"]
    pub SetPointSize: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, ps: f64)>,
    #[doc = " gets point size"]
    pub GetPointSize: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    #[doc = " sets dirty flag"]
    pub GetDirty:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> ::std::os::raw::c_int>,
    #[doc = " gets dirty flag"]
    pub SetDirty: ::std::option::Option<
        unsafe extern "C" fn(desc: QuartzDesc_t, dirty: ::std::os::raw::c_int),
    >,
    #[doc = " replay display list\nNote: it inhibits sync calls during repaint,\nthe caller is responsible for calling sync if needed.\nDirty flag is kept unmodified"]
    pub ReplayDisplayList: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t)>,
    pub GetSnapshot: ::std::option::Option<
        unsafe extern "C" fn(
            desc: QuartzDesc_t,
            last: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    #[doc = " create a (replayable) snapshot of the device contents.\nwhen 'last' is set then the last stored display list is used,\notherwise a new snapshot is created"]
    pub RestoreSnapshot: ::std::option::Option<
        unsafe extern "C" fn(desc: QuartzDesc_t, snapshot: *mut ::std::os::raw::c_void),
    >,
    #[doc = " get anti-alias flag"]
    pub GetAntialias:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> ::std::os::raw::c_int>,
    #[doc = " set anti-alias flag"]
    pub SetAntialias:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, aa: ::std::os::raw::c_int)>,
    #[doc = " get background color"]
    pub GetBackground:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> ::std::os::raw::c_int>,
    #[doc = " activate/select the device"]
    pub Activate: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t)>,
    #[doc = " get/set Quartz-specific parameters. desc can be NULL for global parameters"]
    pub SetParameter: ::std::option::Option<
        unsafe extern "C" fn(
            desc: QuartzDesc_t,
            key: *const ::std::os::raw::c_char,
            value: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetParameter: ::std::option::Option<
        unsafe extern "C" fn(
            desc: QuartzDesc_t,
            key: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
pub const QNPF_REDRAW: u32 = 1;
pub const QDFLAG_DISPLAY_LIST: u32 = 1;
pub const QDFLAG_INTERACTIVE: u32 = 2;
pub const QDFLAG_RASTERIZED: u32 = 4;
pub const QPFLAG_ANTIALIAS: u32 = 256;
pub const QuartzParam_EmbeddingFlags: &[u8; 16] = b"embedding flags\0";
pub const QP_Flags_CFLoop: u32 = 1;
pub const QP_Flags_Cocoa: u32 = 2;
pub const QP_Flags_Front: u32 = 4;
unsafe extern "C" {
    #[doc = " all device implementations have to call this general Quartz device constructor at some point"]
    pub fn QuartzDevice_Create(
        dd: *mut ::std::os::raw::c_void,
        def: *mut QuartzBackend_t,
    ) -> QuartzDesc_t;
    #[doc = " FIXME: no longer used, remove in due course */\n/* from unix/aqua.c - loads grDevices if necessary and returns NULL on failure"]
    pub fn getQuartzFunctions() -> *mut QuartzFunctions_t;
    pub static mut ptr_QuartzBackend: ::std::option::Option<
        unsafe extern "C" fn(
            dd: *mut ::std::os::raw::c_void,
            fn_: *mut QuartzFunctions_t,
            par: *mut QuartzParameters_t,
        ) -> QuartzDesc_t,
    >;
    #[doc = " C version of the Quartz call (experimental)\nreturns 0 on success, error code on failure"]
    pub fn Quartz_C(
        par: *mut QuartzParameters_t,
        q_create: quartz_create_fn_t,
        errorCode: *mut ::std::os::raw::c_int,
    ) -> QuartzDesc_t;
}
