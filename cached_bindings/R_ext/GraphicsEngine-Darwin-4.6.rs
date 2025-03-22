/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.6.0 */

pub type pGEcontext = *mut R_GE_gcontext;
pub type GEDevDesc = _GEDevDesc;
pub type GEcallback = ::std::option::Option<
    unsafe extern "C" fn(arg1: GEevent, arg2: *mut GEDevDesc, arg3: SEXP) -> SEXP,
>;
pub type pGEDevDesc = *mut GEDevDesc;
#[doc = "-------------------------------------------------------------------\n\n  COLOUR CODE is concerned with the internals of R colour representation\n\n  From colors.c, used in par.c, grid/src/gpar.c"]
pub type rcolor = ::std::os::raw::c_uint;
#[doc = " A structure containing graphical parameters\n\n This is how graphical parameters are passed from graphics systems\n to the graphics engine AND from the graphics engine to graphics\n devices.\n\n Devices are not *required* to honour graphical parameters\n (e.g., alpha transparency is going to be tough for some)"]
#[repr(C)]
pub struct R_GE_gcontext {
    #[doc = " pen colour (lines, text, borders, ...)"]
    pub col: ::std::os::raw::c_int,
    #[doc = " fill colour (for polygons, circles, rects, ...)"]
    pub fill: ::std::os::raw::c_int,
    #[doc = " Gamma correction"]
    pub gamma: f64,
    #[doc = " Line width (roughly number of pixels)"]
    pub lwd: f64,
    #[doc = " Line type (solid, dashed, dotted, ...)"]
    pub lty: ::std::os::raw::c_int,
    #[doc = " Line end"]
    pub lend: R_GE_lineend,
    #[doc = " line join"]
    pub ljoin: R_GE_linejoin,
    #[doc = " line mitre"]
    pub lmitre: f64,
    #[doc = " Character expansion (font size = fontsize*cex)"]
    pub cex: f64,
    #[doc = " Font size in points"]
    pub ps: f64,
    #[doc = " Line height (multiply by font size)"]
    pub lineheight: f64,
    #[doc = " Font face (plain, italic, bold, ...)"]
    pub fontface: ::std::os::raw::c_int,
    #[doc = " Font family"]
    pub fontfamily: [::std::os::raw::c_char; 201usize],
    #[doc = " Reference to a pattern fill"]
    pub patternFill: SEXP,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GESystemDesc {
    #[doc = " An array of information about each graphics system that\n has registered with the graphics engine.\n This is used to store graphics state for each graphics\n system on each device."]
    pub systemSpecific: *mut ::std::os::raw::c_void,
    #[doc = " An array of function pointers, one per graphics system that\n has registered with the graphics engine.\n\n system_Callback is called when the graphics engine wants\n to give a graphics system the chance to play with its\n device-specific information (stored in systemSpecific)\n There are two parameters:  an \"event\" to tell the graphics\n system why the graphics engine has called this function,\n and the systemSpecific pointer.  The graphics engine\n has to pass the systemSpecific pointer because only\n the graphics engine will know what array index to use."]
    pub callback: GEcallback,
}
#[repr(C)]
pub struct _GEDevDesc {
    #[doc = " Stuff that the devices can see (and modify).\n All detailed in GraphicsDevice.h"]
    pub dev: pDevDesc,
    #[doc = " toggle for display list status"]
    pub displayListOn: Rboolean,
    #[doc = " display list"]
    pub displayList: SEXP,
    #[doc = " A pointer to the end of the display list\nto avoid traversing pairlists"]
    pub DLlastElt: SEXP,
    #[doc = " The last element of the display list\n just prior to when the display list\n was last initialised"]
    pub savedSnapshot: SEXP,
    #[doc = " Has the device received any output?"]
    pub dirty: Rboolean,
    #[doc = " Should a graphics call be stored\n on the display list?\n Set to FALSE by do_recordGraphics,\n do_dotcallgr, and do_Externalgr\n so that nested calls are not\n recorded on the display list"]
    pub recordGraphics: Rboolean,
    #[doc = " Stuff about the device that only graphics systems see.\n The graphics engine has no idea what is in here.\n Used by graphics systems to store system state per device."]
    pub gesd: [*mut GESystemDesc; 24usize],
    #[doc = " per-device setting for 'ask' (use NewFrameConfirm)"]
    pub ask: Rboolean,
    #[doc = " Is a device appending a path ?"]
    pub appending: Rboolean,
}
pub const R_GE_definitions: u32 = 13;
pub const R_GE_deviceClip: u32 = 14;
pub const R_GE_group: u32 = 15;
pub const R_GE_glyphs: u32 = 16;
pub const R_GE_version: u32 = 16;
pub const MAX_GRAPHICS_SYSTEMS: u32 = 24;
pub const LTY_BLANK: i32 = -1;
pub const LTY_SOLID: u32 = 0;
pub const LTY_DASHED: u32 = 68;
pub const LTY_DOTTED: u32 = 49;
pub const LTY_DOTDASH: u32 = 13361;
pub const LTY_LONGDASH: u32 = 55;
pub const LTY_TWODASH: u32 = 9762;
pub const DEG2RAD: f64 = 0.017453292519943295;
pub const R_GE_linearGradientPattern: u32 = 1;
pub const R_GE_radialGradientPattern: u32 = 2;
pub const R_GE_tilingPattern: u32 = 3;
pub const R_GE_patternExtendPad: u32 = 1;
pub const R_GE_patternExtendRepeat: u32 = 2;
pub const R_GE_patternExtendReflect: u32 = 3;
pub const R_GE_patternExtendNone: u32 = 4;
pub const R_GE_compositeClear: u32 = 1;
pub const R_GE_compositeSource: u32 = 2;
pub const R_GE_compositeOver: u32 = 3;
pub const R_GE_compositeIn: u32 = 4;
pub const R_GE_compositeOut: u32 = 5;
pub const R_GE_compositeAtop: u32 = 6;
pub const R_GE_compositeDest: u32 = 7;
pub const R_GE_compositeDestOver: u32 = 8;
pub const R_GE_compositeDestIn: u32 = 9;
pub const R_GE_compositeDestOut: u32 = 10;
pub const R_GE_compositeDestAtop: u32 = 11;
pub const R_GE_compositeXor: u32 = 12;
pub const R_GE_compositeAdd: u32 = 13;
pub const R_GE_compositeSaturate: u32 = 14;
pub const R_GE_compositeMultiply: u32 = 15;
pub const R_GE_compositeScreen: u32 = 16;
pub const R_GE_compositeOverlay: u32 = 17;
pub const R_GE_compositeDarken: u32 = 18;
pub const R_GE_compositeLighten: u32 = 19;
pub const R_GE_compositeColorDodge: u32 = 20;
pub const R_GE_compositeColorBurn: u32 = 21;
pub const R_GE_compositeHardLight: u32 = 22;
pub const R_GE_compositeSoftLight: u32 = 23;
pub const R_GE_compositeDifference: u32 = 24;
pub const R_GE_compositeExclusion: u32 = 25;
pub const R_GE_nonZeroWindingRule: u32 = 1;
pub const R_GE_evenOddRule: u32 = 2;
pub const R_GE_alphaMask: u32 = 1;
pub const R_GE_luminanceMask: u32 = 2;
pub const R_GE_capability_semiTransparency: u32 = 0;
pub const R_GE_capability_transparentBackground: u32 = 1;
pub const R_GE_capability_rasterImage: u32 = 2;
pub const R_GE_capability_capture: u32 = 3;
pub const R_GE_capability_locator: u32 = 4;
pub const R_GE_capability_events: u32 = 5;
pub const R_GE_capability_patterns: u32 = 6;
pub const R_GE_capability_clippingPaths: u32 = 7;
pub const R_GE_capability_masks: u32 = 8;
pub const R_GE_capability_compositing: u32 = 9;
pub const R_GE_capability_transformations: u32 = 10;
pub const R_GE_capability_paths: u32 = 11;
pub const R_GE_capability_glyphs: u32 = 12;
pub const R_GE_text_style_normal: u32 = 1;
pub const R_GE_text_style_italic: u32 = 2;
pub const R_GE_text_style_oblique: u32 = 3;
#[repr(u32)]
#[non_exhaustive]
#[doc = " The graphics engine will only accept locations and dimensions\n in native device coordinates, but it provides the following functions\n for converting between a couple of simple alternative coordinate\n systems and device coordinates:\n    DEVICE = native units of the device\n    NDC = Normalised device coordinates\n    INCHES = inches (!)\n    CM = centimetres (!!)"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GEUnit {
    #[doc = " native device coordinates (rasters)"]
    GE_DEVICE = 0,
    #[doc = " normalised device coordinates x=(0,1), y=(0,1)"]
    GE_NDC = 1,
    GE_INCHES = 2,
    GE_CM = 3,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GEevent {
    #[doc = " In response to this event, the registered graphics system\n should allocate and initialise the systemSpecific structure\n\n Should return R_NilValue on failure so that engine\n can tidy up memory allocation"]
    GE_InitState = 0,
    #[doc = " This event gives the registered system a chance to undo\n anything done in the initialisation."]
    GE_FinaliseState = 1,
    #[doc = " This is sent by the graphics engine prior to initialising\n the display list.  It give the graphics system the chance\n to squirrel away information it will need for redrawing the\n the display list"]
    GE_SaveState = 2,
    #[doc = " This is sent by the graphics engine prior to replaying the\n display list.  It gives the graphics system the chance to\n restore any information it saved on the GE_SaveState event"]
    GE_RestoreState = 6,
    #[doc = " Copy system state information to the current device.\n This is used when copying graphics from one device to another\n so all the graphics system needs to do is to copy across\n the bits required for the display list to draw faithfully\n on the new device."]
    GE_CopyState = 3,
    #[doc = " Create a snapshot of the system state that is sufficient\n for the current \"image\" to be reproduced"]
    GE_SaveSnapshotState = 4,
    #[doc = " Restore the system state that is saved by GE_SaveSnapshotState"]
    GE_RestoreSnapshotState = 5,
    #[doc = " When replaying the display list, the graphics engine\n checks, after each replayed action, that the action\n produced valid output.  This is the graphics system's\n chance to say that the output is crap (in which case the\n graphics engine will abort the display list replay)."]
    GE_CheckPlot = 7,
    #[doc = " The device wants to scale the current pointsize\n (for scaling an image)\n This is not a nice general solution, but a quick fix for\n the Windows device."]
    GE_ScalePS = 8,
}
#[repr(u32)]
#[non_exhaustive]
#[doc = "  Some line end/join constants"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_GE_lineend {
    GE_ROUND_CAP = 1,
    GE_BUTT_CAP = 2,
    GE_SQUARE_CAP = 3,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_GE_linejoin {
    GE_ROUND_JOIN = 1,
    GE_MITRE_JOIN = 2,
    GE_BEVEL_JOIN = 3,
}
unsafe extern "C" {
    pub fn R_GE_getVersion() -> ::std::os::raw::c_int;
    pub fn R_GE_checkVersionOrDie(version: ::std::os::raw::c_int);
    pub fn Rf_desc2GEDesc(dd: pDevDesc) -> pGEDevDesc;
    pub fn GEdeviceNumber(arg1: pGEDevDesc) -> ::std::os::raw::c_int;
    pub fn GEgetDevice(arg1: ::std::os::raw::c_int) -> pGEDevDesc;
    pub fn GEaddDevice(arg1: pGEDevDesc);
    pub fn GEaddDevice2(arg1: pGEDevDesc, arg2: *const ::std::os::raw::c_char);
    pub fn GEaddDevice2f(
        arg1: pGEDevDesc,
        arg2: *const ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    );
    pub fn GEkillDevice(arg1: pGEDevDesc);
    pub fn GEcreateDevDesc(dev: pDevDesc) -> pGEDevDesc;
    pub fn GEdestroyDevDesc(dd: pGEDevDesc);
    pub fn GEsystemState(
        dd: pGEDevDesc,
        index: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
    pub fn GEregisterWithDevice(dd: pGEDevDesc);
    pub fn GEregisterSystem(callback: GEcallback, systemRegisterIndex: *mut ::std::os::raw::c_int);
    pub fn GEunregisterSystem(registerIndex: ::std::os::raw::c_int);
    pub fn GEhandleEvent(event: GEevent, dev: pDevDesc, data: SEXP) -> SEXP;
    pub fn GEfromDeviceX(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceX(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceY(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceY(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceWidth(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceWidth(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceHeight(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceHeight(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn Rf_RGBpar(arg1: SEXP, arg2: ::std::os::raw::c_int) -> rcolor;
    pub fn Rf_RGBpar3(arg1: SEXP, arg2: ::std::os::raw::c_int, arg3: rcolor) -> rcolor;
    pub fn Rf_col2name(col: rcolor) -> *const ::std::os::raw::c_char;
    #[doc = " Convert either a name or a #RRGGBB[AA] string to internal.\nBecause people were using it, it also converts \"1\", \"2\" ...\nto a colour in the palette, and \"0\" to transparent white."]
    pub fn R_GE_str2col(s: *const ::std::os::raw::c_char) -> rcolor;
    pub fn GE_LENDpar(value: SEXP, ind: ::std::os::raw::c_int) -> R_GE_lineend;
    pub fn GE_LENDget(lend: R_GE_lineend) -> SEXP;
    pub fn GE_LJOINpar(value: SEXP, ind: ::std::os::raw::c_int) -> R_GE_linejoin;
    pub fn GE_LJOINget(ljoin: R_GE_linejoin) -> SEXP;
    pub fn GESetClip(x1: f64, y1: f64, x2: f64, y2: f64, dd: pGEDevDesc);
    pub fn GENewPage(gc: pGEcontext, dd: pGEDevDesc);
    pub fn GELine(x1: f64, y1: f64, x2: f64, y2: f64, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEPolyline(
        n: ::std::os::raw::c_int,
        x: *mut f64,
        y: *mut f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEPolygon(
        n: ::std::os::raw::c_int,
        x: *mut f64,
        y: *mut f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEXspline(
        n: ::std::os::raw::c_int,
        x: *mut f64,
        y: *mut f64,
        s: *mut f64,
        open: Rboolean,
        repEnds: Rboolean,
        draw: Rboolean,
        gc: pGEcontext,
        dd: pGEDevDesc,
    ) -> SEXP;
    pub fn GECircle(x: f64, y: f64, radius: f64, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GERect(x0: f64, y0: f64, x1: f64, y1: f64, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEPath(
        x: *mut f64,
        y: *mut f64,
        npoly: ::std::os::raw::c_int,
        nper: *mut ::std::os::raw::c_int,
        winding: Rboolean,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GERaster(
        raster: *mut ::std::os::raw::c_uint,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        angle: f64,
        interpolate: Rboolean,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GECap(dd: pGEDevDesc) -> SEXP;
    pub fn GEText(
        x: f64,
        y: f64,
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        xc: f64,
        yc: f64,
        rot: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEMode(mode: ::std::os::raw::c_int, dd: pGEDevDesc);
    pub fn GESymbol(
        x: f64,
        y: f64,
        pch: ::std::os::raw::c_int,
        size: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEPretty(lo: *mut f64, up: *mut f64, ndiv: *mut ::std::os::raw::c_int);
    pub fn GEMetricInfo(
        c: ::std::os::raw::c_int,
        gc: pGEcontext,
        ascent: *mut f64,
        descent: *mut f64,
        width: *mut f64,
        dd: pGEDevDesc,
    );
    pub fn GEStrWidth(
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        dd: pGEDevDesc,
    ) -> f64;
    pub fn GEStrHeight(
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        dd: pGEDevDesc,
    ) -> f64;
    pub fn GEStrMetric(
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        ascent: *mut f64,
        descent: *mut f64,
        width: *mut f64,
        dd: pGEDevDesc,
    );
    pub fn GEstring_to_pch(pch: SEXP) -> ::std::os::raw::c_int;
    #[doc = "-------------------------------------------------------------------\n\n  LINE TEXTURE CODE is concerned with the internals of R\n  line texture representation."]
    pub fn GE_LTYpar(arg1: SEXP, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_uint;
    pub fn GE_LTYget(arg1: ::std::os::raw::c_uint) -> SEXP;
    #[doc = " Raster operations"]
    pub fn R_GE_rasterScale(
        sraster: *mut ::std::os::raw::c_uint,
        sw: ::std::os::raw::c_int,
        sh: ::std::os::raw::c_int,
        draster: *mut ::std::os::raw::c_uint,
        dw: ::std::os::raw::c_int,
        dh: ::std::os::raw::c_int,
    );
    pub fn R_GE_rasterInterpolate(
        sraster: *mut ::std::os::raw::c_uint,
        sw: ::std::os::raw::c_int,
        sh: ::std::os::raw::c_int,
        draster: *mut ::std::os::raw::c_uint,
        dw: ::std::os::raw::c_int,
        dh: ::std::os::raw::c_int,
    );
    pub fn R_GE_rasterRotatedSize(
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        angle: f64,
        wnew: *mut ::std::os::raw::c_int,
        hnew: *mut ::std::os::raw::c_int,
    );
    pub fn R_GE_rasterRotatedOffset(
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        angle: f64,
        botleft: ::std::os::raw::c_int,
        xoff: *mut f64,
        yoff: *mut f64,
    );
    pub fn R_GE_rasterResizeForRotation(
        sraster: *mut ::std::os::raw::c_uint,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        newRaster: *mut ::std::os::raw::c_uint,
        wnew: ::std::os::raw::c_int,
        hnew: ::std::os::raw::c_int,
        gc: pGEcontext,
    );
    pub fn R_GE_rasterRotate(
        sraster: *mut ::std::os::raw::c_uint,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        angle: f64,
        draster: *mut ::std::os::raw::c_uint,
        gc: pGEcontext,
        perPixelAlpha: Rboolean,
    );
    #[doc = " From plotmath.c"]
    pub fn GEExpressionWidth(expr: SEXP, gc: pGEcontext, dd: pGEDevDesc) -> f64;
    pub fn GEExpressionHeight(expr: SEXP, gc: pGEcontext, dd: pGEDevDesc) -> f64;
    pub fn GEExpressionMetric(
        expr: SEXP,
        gc: pGEcontext,
        ascent: *mut f64,
        descent: *mut f64,
        width: *mut f64,
        dd: pGEDevDesc,
    );
    pub fn GEMathText(
        x: f64,
        y: f64,
        expr: SEXP,
        xc: f64,
        yc: f64,
        rot: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    #[doc = " From plot3d.c : used in package clines"]
    pub fn GEcontourLines(
        x: *mut f64,
        nx: ::std::os::raw::c_int,
        y: *mut f64,
        ny: ::std::os::raw::c_int,
        z: *mut f64,
        levels: *mut f64,
        nl: ::std::os::raw::c_int,
    ) -> SEXP;
    #[doc = " From vfonts.c"]
    pub fn R_GE_VStrWidth(
        s: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        dd: pGEDevDesc,
    ) -> f64;
    pub fn R_GE_VStrHeight(
        s: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        dd: pGEDevDesc,
    ) -> f64;
    pub fn R_GE_VText(
        x: f64,
        y: f64,
        s: *const ::std::os::raw::c_char,
        enc: cetype_t,
        x_justify: f64,
        y_justify: f64,
        rotation: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEcurrentDevice() -> pGEDevDesc;
    pub fn GEdeviceDirty(dd: pGEDevDesc) -> Rboolean;
    pub fn GEdirtyDevice(dd: pGEDevDesc);
    pub fn GEcheckState(dd: pGEDevDesc) -> Rboolean;
    pub fn GErecording(call: SEXP, dd: pGEDevDesc) -> Rboolean;
    pub fn GErecordGraphicOperation(op: SEXP, args: SEXP, dd: pGEDevDesc);
    pub fn GEinitDisplayList(dd: pGEDevDesc);
    pub fn GEplayDisplayList(dd: pGEDevDesc);
    pub fn GEcopyDisplayList(fromDevice: ::std::os::raw::c_int);
    pub fn GEcreateSnapshot(dd: pGEDevDesc) -> SEXP;
    pub fn GEplaySnapshot(snapshot: SEXP, dd: pGEDevDesc);
    pub fn GEonExit();
    pub fn GEnullDevice();
    pub fn Rf_CreateAtVector(
        axp: *mut f64,
        usr: *const f64,
        nint: ::std::os::raw::c_int,
        logflag: Rboolean,
    ) -> SEXP;
    pub fn Rf_GAxisPars(
        min: *mut f64,
        max: *mut f64,
        n: *mut ::std::os::raw::c_int,
        log: Rboolean,
        axis: ::std::os::raw::c_int,
    );
    #[doc = " Patterns - from ../../main/patterns.c"]
    pub fn R_GE_isPattern(x: SEXP) -> Rboolean;
    pub fn R_GE_patternType(pattern: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_linearGradientX1(pattern: SEXP) -> f64;
    pub fn R_GE_linearGradientY1(pattern: SEXP) -> f64;
    pub fn R_GE_linearGradientX2(pattern: SEXP) -> f64;
    pub fn R_GE_linearGradientY2(pattern: SEXP) -> f64;
    pub fn R_GE_linearGradientNumStops(pattern: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_linearGradientStop(pattern: SEXP, i: ::std::os::raw::c_int) -> f64;
    pub fn R_GE_linearGradientColour(pattern: SEXP, i: ::std::os::raw::c_int) -> rcolor;
    pub fn R_GE_linearGradientExtend(pattern: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_radialGradientCX1(pattern: SEXP) -> f64;
    pub fn R_GE_radialGradientCY1(pattern: SEXP) -> f64;
    pub fn R_GE_radialGradientR1(pattern: SEXP) -> f64;
    pub fn R_GE_radialGradientCX2(pattern: SEXP) -> f64;
    pub fn R_GE_radialGradientCY2(pattern: SEXP) -> f64;
    pub fn R_GE_radialGradientR2(pattern: SEXP) -> f64;
    pub fn R_GE_radialGradientNumStops(pattern: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_radialGradientStop(pattern: SEXP, i: ::std::os::raw::c_int) -> f64;
    pub fn R_GE_radialGradientColour(pattern: SEXP, i: ::std::os::raw::c_int) -> rcolor;
    pub fn R_GE_radialGradientExtend(pattern: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_tilingPatternFunction(pattern: SEXP) -> SEXP;
    pub fn R_GE_tilingPatternX(pattern: SEXP) -> f64;
    pub fn R_GE_tilingPatternY(pattern: SEXP) -> f64;
    pub fn R_GE_tilingPatternWidth(pattern: SEXP) -> f64;
    pub fn R_GE_tilingPatternHeight(pattern: SEXP) -> f64;
    pub fn R_GE_tilingPatternExtend(pattern: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_clipPathFillRule(path: SEXP) -> ::std::os::raw::c_int;
    pub fn GEStroke(path: SEXP, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEFill(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEFillStroke(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pGEDevDesc);
    pub fn R_GE_maskType(mask: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_glyphInfoGlyphs(glyphInfo: SEXP) -> SEXP;
    pub fn R_GE_glyphInfoFonts(glyphInfo: SEXP) -> SEXP;
    pub fn R_GE_glyphID(glyphs: SEXP) -> SEXP;
    pub fn R_GE_glyphX(glyphs: SEXP) -> SEXP;
    pub fn R_GE_glyphY(glyphs: SEXP) -> SEXP;
    pub fn R_GE_glyphFont(glyphs: SEXP) -> SEXP;
    pub fn R_GE_glyphSize(glyphs: SEXP) -> SEXP;
    pub fn R_GE_glyphColour(glyphs: SEXP) -> SEXP;
    pub fn R_GE_glyphRotation(glyphs: SEXP) -> SEXP;
    pub fn R_GE_hasGlyphRotation(glyphs: SEXP) -> Rboolean;
    pub fn R_GE_glyphFontFile(glyphFont: SEXP) -> *const ::std::os::raw::c_char;
    pub fn R_GE_glyphFontIndex(glyphFont: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_glyphFontFamily(glyphFont: SEXP) -> *const ::std::os::raw::c_char;
    pub fn R_GE_glyphFontWeight(glyphFont: SEXP) -> f64;
    pub fn R_GE_glyphFontStyle(glyphFont: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_glyphFontPSname(glyphFont: SEXP) -> *const ::std::os::raw::c_char;
    pub fn GEGlyph(
        n: ::std::os::raw::c_int,
        glyphs: *mut ::std::os::raw::c_int,
        x: *mut f64,
        y: *mut f64,
        font: SEXP,
        size: f64,
        colour: ::std::os::raw::c_int,
        rot: f64,
        dd: pGEDevDesc,
    );
}
) -> *const ::std::os::raw::c_char;
    pub fn R_GE_glyphFontWeight(glyphFont: SEXP) -> f64;
    pub fn R_GE_glyphFontStyle(glyphFont: SEXP) -> ::std::os::raw::c_int;
    pub fn R_GE_glyphFontPSname(glyphFont: SEXP) -> *const ::std::os::raw::c_char;
    pub fn GEGlyph(
        n: ::std::os::raw::c_int,
        glyphs: *mut ::std::os::raw::c_int,
        x: *mut f64,
        y: *mut f64,
        font: SEXP,
        size: f64,
        colour: ::std::os::raw::c_int,
        rot: f64,
        dd: pGEDevDesc,
    );
}
