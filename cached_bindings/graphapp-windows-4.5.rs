/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.5.0 */

#[doc = "  Types."]
pub type GAbyte = ::std::os::raw::c_uchar;
pub type objptr = *mut objinfo;
#[doc = " end of R modification"]
pub type rgb = ::std::os::raw::c_ulong;
pub type font = objptr;
pub type cursor = objptr;
pub type drawing = objptr;
pub type bitmap = drawing;
pub type window = drawing;
pub type control = drawing;
pub type label = control;
pub type button = control;
pub type checkbox = control;
pub type radiobutton = control;
pub type radiogroup = control;
pub type field = control;
pub type textbox = control;
pub type scrollbar = control;
pub type listbox = control;
pub type progressbar = control;
pub type menubar = control;
pub type menu = control;
pub type menuitem = control;
pub type drawstate = *mut drawstruct;
pub type image = *mut imagedata;
#[doc = "  Call-backs."]
pub type voidfn = ::std::option::Option<unsafe extern "C" fn()>;
pub type timerfn = ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>;
pub type actionfn = ::std::option::Option<unsafe extern "C" fn(c: control)>;
pub type drawfn = ::std::option::Option<unsafe extern "C" fn(c: control, r: rect)>;
pub type mousefn = ::std::option::Option<
    unsafe extern "C" fn(c: control, buttons: ::std::os::raw::c_int, xy: point),
>;
pub type intfn =
    ::std::option::Option<unsafe extern "C" fn(c: control, argument: ::std::os::raw::c_int)>;
pub type keyfn =
    ::std::option::Option<unsafe extern "C" fn(c: control, key: ::std::os::raw::c_int)>;
pub type menufn = ::std::option::Option<unsafe extern "C" fn(m: menuitem)>;
pub type scrollfn =
    ::std::option::Option<unsafe extern "C" fn(s: scrollbar, position: ::std::os::raw::c_int)>;
pub type dropfn =
    ::std::option::Option<unsafe extern "C" fn(c: control, data: *mut ::std::os::raw::c_char)>;
pub type imfn =
    ::std::option::Option<unsafe extern "C" fn(c: control, f: *mut font, xy: *mut point)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objinfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct point {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rect {
    #[doc = " top-left point inside rect"]
    pub x: ::std::os::raw::c_int,
    #[doc = " top-left point inside rect"]
    pub y: ::std::os::raw::c_int,
    #[doc = " width and height of rect"]
    pub width: ::std::os::raw::c_int,
    #[doc = " width and height of rect"]
    pub height: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drawstruct {
    pub dest: drawing,
    pub hue: rgb,
    pub mode: ::std::os::raw::c_int,
    pub p: point,
    pub linewidth: ::std::os::raw::c_int,
    pub fnt: font,
    pub crsr: cursor,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct imagedata {
    pub depth: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub cmapsize: ::std::os::raw::c_int,
    pub cmap: *mut rgb,
    pub pixels: *mut GAbyte,
}
pub const _GRAPHAPP_H: u32 = 240;
pub const Pi: f64 = 3.14159265359;
pub const NoButton: u32 = 0;
pub const LeftButton: u32 = 1;
pub const MiddleButton: u32 = 2;
pub const RightButton: u32 = 4;
pub const BELL: u32 = 7;
pub const BKSP: u32 = 8;
pub const VTAB: u32 = 11;
pub const FF: u32 = 12;
pub const ESC: u32 = 27;
pub const INS: u32 = 8257;
pub const DEL: u32 = 8998;
pub const HOME: u32 = 8632;
pub const END: u32 = 8600;
pub const PGUP: u32 = 8670;
pub const PGDN: u32 = 8671;
pub const ENTER: u32 = 8996;
pub const LEFT: u32 = 8592;
pub const UP: u32 = 8593;
pub const RIGHT: u32 = 8594;
pub const DOWN: u32 = 8595;
pub const F1: u32 = 10092;
pub const F2: u32 = 10093;
pub const F3: u32 = 10094;
pub const F4: u32 = 10095;
pub const F5: u32 = 10096;
pub const F6: u32 = 10097;
pub const F7: u32 = 10098;
pub const F8: u32 = 10099;
pub const F9: u32 = 10100;
pub const F10: u32 = 10101;
pub const gaRed: u32 = 16711680;
pub const gaGreen: u32 = 65280;
pub const gaBlue: u32 = 255;
pub const Transparent: u32 = 4294967295;
pub const Black: u32 = 0;
pub const White: u32 = 16777215;
pub const Yellow: u32 = 16776960;
pub const Magenta: u32 = 16711935;
pub const Cyan: u32 = 65535;
pub const Grey: u32 = 8421504;
pub const Gray: u32 = 8421504;
pub const LightGrey: u32 = 12632256;
pub const LightGray: u32 = 12632256;
pub const DarkGrey: u32 = 4210752;
pub const DarkGray: u32 = 4210752;
pub const DarkBlue: u32 = 128;
pub const DarkGreen: u32 = 32768;
pub const DarkRed: u32 = 9109504;
pub const LightBlue: u32 = 8438015;
pub const LightGreen: u32 = 8454016;
pub const LightRed: u32 = 16761087;
pub const Pink: u32 = 16756655;
pub const Brown: u32 = 6303744;
pub const Orange: u32 = 16744448;
pub const Purple: u32 = 12583167;
pub const Lime: u32 = 8453888;
pub const Zeros: u32 = 0;
pub const DnorS: u32 = 1;
pub const DandnotS: u32 = 2;
pub const notS: u32 = 3;
pub const notDandS: u32 = 4;
pub const notD: u32 = 5;
pub const DxorS: u32 = 6;
pub const DnandS: u32 = 7;
pub const DandS: u32 = 8;
pub const DxnorS: u32 = 9;
pub const GA_D: u32 = 10;
pub const DornotS: u32 = 11;
pub const GA_S: u32 = 12;
pub const notDorS: u32 = 13;
pub const DorS: u32 = 14;
pub const Ones: u32 = 15;
pub const Plain: u32 = 0;
pub const Bold: u32 = 1;
pub const Italic: u32 = 2;
pub const BoldItalic: u32 = 3;
pub const SansSerif: u32 = 4;
pub const FixedWidth: u32 = 8;
pub const Wide: u32 = 16;
pub const Narrow: u32 = 32;
pub const AlignTop: u32 = 0;
pub const AlignBottom: u32 = 256;
pub const VJustify: u32 = 512;
pub const VCenter: u32 = 1024;
pub const VCentre: u32 = 1024;
pub const AlignLeft: u32 = 0;
pub const AlignRight: u32 = 4096;
pub const Justify: u32 = 8192;
pub const Center: u32 = 16384;
pub const Centre: u32 = 16384;
pub const AlignCenter: u32 = 16384;
pub const AlignCentre: u32 = 16384;
pub const Underline: u32 = 2048;
pub const AltKey: u32 = 1;
pub const CmdKey: u32 = 1;
pub const CtrlKey: u32 = 2;
pub const OptionKey: u32 = 2;
pub const ShiftKey: u32 = 4;
pub const SimpleWindow: u32 = 0;
pub const Menubar: u32 = 16;
pub const Titlebar: u32 = 32;
pub const Closebox: u32 = 64;
pub const Resize: u32 = 128;
pub const Maximize: u32 = 256;
pub const Minimize: u32 = 512;
pub const HScrollbar: u32 = 1024;
pub const VScrollbar: u32 = 2048;
pub const CanvasSize: u32 = 4194304;
pub const Modal: u32 = 4096;
pub const Floating: u32 = 8192;
pub const Centered: u32 = 16384;
pub const Centred: u32 = 16384;
pub const Workspace: u32 = 65536;
pub const Document: u32 = 131072;
pub const ChildWindow: u32 = 262144;
pub const TrackMouse: u32 = 524288;
pub const UsePalette: u32 = 1048576;
pub const UseUnicode: u32 = 2097152;
pub const SetUpCaret: u32 = 4194304;
pub const StandardWindow: u32 = 992;
pub const GA_Visible: u32 = 1;
pub const GA_Enabled: u32 = 2;
pub const GA_Checked: u32 = 4;
pub const GA_Highlighted: u32 = 8;
pub const GA_Armed: u32 = 16;
pub const GA_Focus: u32 = 32;
pub const YES: u32 = 1;
pub const NO: i32 = -1;
pub const CANCEL: u32 = 0;
unsafe extern "C" {
    #[doc = "  General functions."]
    pub fn GA_initapp(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn GA_exitapp();
    pub fn GA_drawall();
    pub fn GA_peekevent() -> ::std::os::raw::c_int;
    pub fn GA_waitevent();
    pub fn GA_doevent() -> ::std::os::raw::c_int;
    pub fn mainloop();
    pub fn GA_execapp(cmd: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    #[doc = "  Point and rectangle arithmetic."]
    pub fn GA_newpoint(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> point;
    pub fn GA_newrect(
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> rect;
    pub fn GA_rpt(min: point, max: point) -> rect;
    pub fn GA_topleft(r: rect) -> point;
    pub fn GA_bottomright(r: rect) -> point;
    pub fn GA_topright(r: rect) -> point;
    pub fn GA_bottomleft(r: rect) -> point;
    pub fn GA_addpt(p1: point, p2: point) -> point;
    pub fn GA_subpt(p1: point, p2: point) -> point;
    pub fn GA_midpt(p1: point, p2: point) -> point;
    pub fn GA_mulpt(p1: point, i: ::std::os::raw::c_int) -> point;
    pub fn GA_divpt(p1: point, i: ::std::os::raw::c_int) -> point;
    pub fn GA_rmove(r: rect, p: point) -> rect;
    pub fn GA_raddpt(r: rect, p: point) -> rect;
    pub fn GA_rsubpt(r: rect, p: point) -> rect;
    pub fn GA_rmul(r: rect, i: ::std::os::raw::c_int) -> rect;
    pub fn GA_rdiv(r: rect, i: ::std::os::raw::c_int) -> rect;
    pub fn GA_growr(r: rect, w: ::std::os::raw::c_int, h: ::std::os::raw::c_int) -> rect;
    pub fn GA_insetr(r: rect, i: ::std::os::raw::c_int) -> rect;
    pub fn GA_rcenter(r1: rect, r2: rect) -> rect;
    pub fn GA_ptinr(p: point, r: rect) -> ::std::os::raw::c_int;
    pub fn GA_rinr(r1: rect, r2: rect) -> ::std::os::raw::c_int;
    pub fn GA_rxr(r1: rect, r2: rect) -> ::std::os::raw::c_int;
    pub fn GA_equalpt(p1: point, p2: point) -> ::std::os::raw::c_int;
    pub fn GA_equalr(r1: rect, r2: rect) -> ::std::os::raw::c_int;
    pub fn GA_clipr(r1: rect, r2: rect) -> rect;
    pub fn GA_rcanon(r: rect) -> rect;
    pub fn GA_setrgb(c: rgb);
    #[doc = "  Context functions for bitmaps, windows, controls."]
    pub fn GA_addto(dest: control);
    pub fn GA_drawto(dest: drawing);
    pub fn GA_setlinewidth(width: ::std::os::raw::c_int);
    #[doc = "  Transfer modes for drawing operations, S=source, D=destination.\n  The modes are arranged so that, for example, (~D)|S == notDorS."]
    pub fn GA_setdrawmode(mode: ::std::os::raw::c_int);
    #[doc = "  Drawing functions."]
    pub fn GA_bitblt(dest: drawing, src: drawing, dp: point, sr: rect, mode: ::std::os::raw::c_int);
    pub fn GA_scrollrect(dp: point, sr: rect);
    pub fn GA_copyrect(src: drawing, dp: point, sr: rect);
    pub fn GA_texturerect(src: drawing, r: rect);
    pub fn GA_invertrect(r: rect);
    pub fn GA_getpixel(p: point) -> rgb;
    pub fn GA_setpixel(p: point, c: rgb);
    #[doc = "  Drawing using the current colour."]
    pub fn GA_moveto(p: point);
    pub fn GA_lineto(p: point);
    pub fn GA_drawpoint(p: point);
    pub fn GA_drawline(p1: point, p2: point);
    pub fn GA_drawrect(r: rect);
    pub fn GA_fillrect(r: rect);
    pub fn GA_drawarc(
        r: rect,
        start_angle: ::std::os::raw::c_int,
        end_angle: ::std::os::raw::c_int,
    );
    pub fn GA_fillarc(
        r: rect,
        start_angle: ::std::os::raw::c_int,
        end_angle: ::std::os::raw::c_int,
    );
    pub fn GA_drawellipse(r: rect);
    pub fn GA_fillellipse(r: rect);
    pub fn GA_drawroundrect(r: rect);
    pub fn GA_fillroundrect(r: rect);
    pub fn GA_drawpolygon(p: *mut point, n: ::std::os::raw::c_int);
    pub fn GA_fillpolygon(p: *mut point, n: ::std::os::raw::c_int);
    #[doc = "  Drawing text, selecting fonts."]
    pub fn GA_newfont(
        name: *const ::std::os::raw::c_char,
        style: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
    ) -> font;
    pub fn GA_setfont(f: font);
    pub fn GA_fontwidth(f: font) -> ::std::os::raw::c_int;
    pub fn GA_fontheight(f: font) -> ::std::os::raw::c_int;
    pub fn GA_fontascent(f: font) -> ::std::os::raw::c_int;
    pub fn GA_fontdescent(f: font) -> ::std::os::raw::c_int;
    pub fn GA_strwidth(f: font, s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn GA_strsize(f: font, s: *const ::std::os::raw::c_char) -> point;
    pub fn GA_strrect(f: font, s: *const ::std::os::raw::c_char) -> rect;
    pub fn GA_drawstr(p: point, str_: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn GA_textheight(
        width: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn GA_drawtext(
        r: rect,
        alignment: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    pub fn GA_gprintf(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    #[doc = "  Find the current state of drawing."]
    pub fn GA_currentdrawing() -> drawing;
    pub fn GA_currentrgb() -> rgb;
    pub fn GA_currentmode() -> ::std::os::raw::c_int;
    pub fn GA_currentpoint() -> point;
    pub fn GA_currentlinewidth() -> ::std::os::raw::c_int;
    pub fn GA_currentfont() -> font;
    pub fn GA_currentcursor() -> cursor;
    #[doc = "  Find current keyboard state."]
    pub fn GA_getkeystate() -> ::std::os::raw::c_int;
    #[doc = "  Bitmaps."]
    pub fn GA_newbitmap(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        depth: ::std::os::raw::c_int,
    ) -> bitmap;
    pub fn GA_loadbitmap(name: *const ::std::os::raw::c_char) -> bitmap;
    pub fn GA_imagetobitmap(img: image) -> bitmap;
    pub fn GA_createbitmap(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        depth: ::std::os::raw::c_int,
        data: *mut GAbyte,
    ) -> bitmap;
    pub fn GA_setbitmapdata(b: bitmap, data: *mut GAbyte);
    pub fn GA_getbitmapdata(b: bitmap, data: *mut GAbyte);
    pub fn GA_getbitmapdata2(b: bitmap, data: *mut *mut GAbyte);
    #[doc = "  Images."]
    pub fn GA_newimage(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        depth: ::std::os::raw::c_int,
    ) -> image;
    pub fn GA_copyimage(img: image) -> image;
    pub fn GA_delimage(img: image);
    pub fn GA_imagedepth(img: image) -> ::std::os::raw::c_int;
    pub fn GA_imagewidth(img: image) -> ::std::os::raw::c_int;
    pub fn GA_imageheight(img: image) -> ::std::os::raw::c_int;
    pub fn GA_setpixels(img: image, pixels: *mut GAbyte);
    pub fn GA_getpixels(img: image) -> *mut GAbyte;
    pub fn GA_setpalette(img: image, length: ::std::os::raw::c_int, cmap: *mut rgb);
    pub fn GA_getpalette(img: image) -> *mut rgb;
    pub fn GA_getpalettesize(img: image) -> ::std::os::raw::c_int;
    pub fn GA_scaleimage(src: image, dr: rect, sr: rect) -> image;
    pub fn GA_convert32to8(img: image) -> image;
    pub fn GA_convert8to32(img: image) -> image;
    pub fn GA_sortpalette(img: image);
    pub fn GA_loadimage(filename: *const ::std::os::raw::c_char) -> image;
    pub fn GA_saveimage(img: image, filename: *const ::std::os::raw::c_char);
    pub fn GA_drawimage(img: image, dr: rect, sr: rect);
    pub fn GA_drawmonochrome(img: image, dr: rect, sr: rect);
    pub fn GA_drawgreyscale(img: image, dr: rect, sr: rect);
    pub fn GA_drawdarker(img: image, dr: rect, sr: rect);
    pub fn GA_drawbrighter(img: image, dr: rect, sr: rect);
    #[doc = "  Windows."]
    pub fn GA_newwindow(
        name: *const ::std::os::raw::c_char,
        r: rect,
        flags: ::std::os::raw::c_long,
    ) -> window;
    pub fn GA_show(w: window);
    pub fn GA_hide(w: window);
    pub fn GA_GetCurrentWinPos(obj: window) -> rect;
    #[doc = "  Functions which work for bitmaps, windows and controls."]
    pub fn GA_objdepth(obj: objptr) -> ::std::os::raw::c_int;
    pub fn GA_objrect(obj: objptr) -> rect;
    pub fn GA_objwidth(obj: objptr) -> ::std::os::raw::c_int;
    pub fn GA_objheight(obj: objptr) -> ::std::os::raw::c_int;
    pub fn GA_delobj(obj: objptr);
    #[doc = "  Setting window and control callback functions."]
    pub fn GA_setaction(c: control, fn_: actionfn);
    pub fn GA_sethit(c: control, fn_: intfn);
    pub fn GA_setdel(c: control, fn_: actionfn);
    pub fn GA_setclose(c: control, fn_: actionfn);
    pub fn GA_setredraw(c: control, fn_: drawfn);
    pub fn GA_setresize(c: control, fn_: drawfn);
    pub fn GA_setkeydown(c: control, fn_: keyfn);
    pub fn GA_setkeyaction(c: control, fn_: keyfn);
    pub fn GA_setmousedown(c: control, fn_: mousefn);
    pub fn GA_setmousedrag(c: control, fn_: mousefn);
    pub fn GA_setmouseup(c: control, fn_: mousefn);
    pub fn GA_setmousemove(c: control, fn_: mousefn);
    pub fn GA_setmouserepeat(c: control, fn_: mousefn);
    pub fn GA_setdrop(c: control, fn_: dropfn);
    pub fn GA_setonfocus(c: control, fn_: actionfn);
    pub fn GA_setim(c: control, fn_: imfn);
    #[doc = "  Using windows and controls."]
    pub fn GA_clear(c: control);
    pub fn GA_draw(c: control);
    pub fn GA_redraw(c: control);
    pub fn GA_resize(c: control, r: rect);
    pub fn GA_isvisible(c: control) -> ::std::os::raw::c_int;
    pub fn GA_enable(c: control);
    pub fn GA_disable(c: control);
    pub fn GA_isenabled(c: control) -> ::std::os::raw::c_int;
    pub fn GA_check(c: control);
    pub fn GA_uncheck(c: control);
    pub fn GA_ischecked(c: control) -> ::std::os::raw::c_int;
    pub fn GA_highlight(c: control);
    pub fn GA_unhighlight(c: control);
    pub fn GA_ishighlighted(c: control) -> ::std::os::raw::c_int;
    pub fn GA_flashcontrol(c: control);
    pub fn GA_activatecontrol(c: control);
    #[doc = "  Changing the state of a control."]
    pub fn GA_settext(c: control, newtext: *const ::std::os::raw::c_char);
    pub fn GA_gettext(c: control) -> *mut ::std::os::raw::c_char;
    pub fn GA_settextfont(c: control, f: font);
    pub fn GA_gettextfont(c: control) -> font;
    pub fn GA_setforeground(c: control, fg: rgb);
    pub fn GA_getforeground(c: control) -> rgb;
    pub fn GA_setbackground(c: control, bg: rgb);
    pub fn GA_getbackground(c: control) -> rgb;
    pub fn GA_setvalue(c: control, value: ::std::os::raw::c_int);
    pub fn GA_getvalue(c: control) -> ::std::os::raw::c_int;
    pub fn GA_setdata(c: control, data: *mut ::std::os::raw::c_void);
    pub fn GA_getdata(c: control) -> *mut ::std::os::raw::c_void;
    pub fn GA_parentwindow(c: control) -> window;
    #[doc = "  Create buttons, scrollbars, controls etc on the current window."]
    pub fn GA_newcontrol(text: *const ::std::os::raw::c_char, r: rect) -> control;
    pub fn GA_newdrawing(r: rect, fn_: drawfn) -> drawing;
    pub fn GA_newpicture(img: image, r: rect) -> drawing;
    pub fn GA_newbutton(text: *const ::std::os::raw::c_char, r: rect, fn_: actionfn) -> button;
    pub fn GA_newimagebutton(img: image, r: rect, fn_: actionfn) -> button;
    pub fn GA_setimage(c: control, img: image);
    pub fn GA_newcheckbox(text: *const ::std::os::raw::c_char, r: rect, fn_: actionfn) -> checkbox;
    pub fn GA_newimagecheckbox(img: image, r: rect, fn_: actionfn) -> checkbox;
    pub fn GA_newradiobutton(
        text: *const ::std::os::raw::c_char,
        r: rect,
        fn_: actionfn,
    ) -> radiobutton;
    pub fn GA_newradiogroup() -> radiogroup;
    pub fn GA_newscrollbar(
        r: rect,
        max: ::std::os::raw::c_int,
        pagesize: ::std::os::raw::c_int,
        fn_: scrollfn,
    ) -> scrollbar;
    pub fn GA_changescrollbar(
        s: scrollbar,
        where_: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
    );
    pub fn GA_newlabel(
        text: *const ::std::os::raw::c_char,
        r: rect,
        alignment: ::std::os::raw::c_int,
    ) -> label;
    pub fn GA_newfield(text: *const ::std::os::raw::c_char, r: rect) -> field;
    pub fn GA_newpassword(text: *const ::std::os::raw::c_char, r: rect) -> field;
    pub fn GA_newtextbox(text: *const ::std::os::raw::c_char, r: rect) -> textbox;
    pub fn GA_newtextarea(text: *const ::std::os::raw::c_char, r: rect) -> textbox;
    pub fn GA_newrichtextarea(text: *const ::std::os::raw::c_char, r: rect) -> textbox;
    pub fn GA_newlistbox(
        list: *mut *const ::std::os::raw::c_char,
        r: rect,
        fn_: scrollfn,
        dble: actionfn,
    ) -> listbox;
    pub fn GA_newdroplist(
        list: *mut *const ::std::os::raw::c_char,
        r: rect,
        fn_: scrollfn,
    ) -> listbox;
    pub fn GA_newdropfield(
        list: *mut *const ::std::os::raw::c_char,
        r: rect,
        fn_: scrollfn,
    ) -> listbox;
    pub fn GA_newmultilist(
        list: *mut *const ::std::os::raw::c_char,
        r: rect,
        fn_: scrollfn,
        dble: actionfn,
    ) -> listbox;
    pub fn GA_isselected(b: listbox, index: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn GA_setlistitem(b: listbox, index: ::std::os::raw::c_int);
    pub fn GA_getlistitem(b: listbox) -> ::std::os::raw::c_int;
    pub fn GA_changelistbox(b: listbox, new_list: *mut *const ::std::os::raw::c_char);
    pub fn GA_newprogressbar(
        r: rect,
        pmin: ::std::os::raw::c_int,
        pmax: ::std::os::raw::c_int,
        incr: ::std::os::raw::c_int,
        smooth: ::std::os::raw::c_int,
    ) -> progressbar;
    pub fn GA_setprogressbar(obj: progressbar, n: ::std::os::raw::c_int);
    pub fn GA_stepprogressbar(obj: progressbar, n: ::std::os::raw::c_int);
    pub fn GA_setprogressbarrange(
        obj: progressbar,
        pbmin: ::std::os::raw::c_int,
        pbmax: ::std::os::raw::c_int,
    );
    pub fn GA_newmenubar(adjust_menus: actionfn) -> menubar;
    pub fn GA_newsubmenu(parent: menu, name: *const ::std::os::raw::c_char) -> menu;
    pub fn GA_newmenu(name: *const ::std::os::raw::c_char) -> menu;
    pub fn GA_newmenuitem(
        name: *const ::std::os::raw::c_char,
        key: ::std::os::raw::c_int,
        fn_: menufn,
    ) -> menuitem;
    #[doc = "  Text editing functions."]
    pub fn GA_undotext(t: textbox);
    pub fn GA_cuttext(t: textbox);
    pub fn GA_copytext(t: textbox);
    pub fn GA_cleartext(t: textbox);
    pub fn GA_pastetext(t: textbox);
    pub fn GA_inserttext(t: textbox, text: *const ::std::os::raw::c_char);
    pub fn GA_selecttext(t: textbox, start: ::std::os::raw::c_long, end: ::std::os::raw::c_long);
    pub fn GA_textselection(
        t: textbox,
        start: *mut ::std::os::raw::c_long,
        end: *mut ::std::os::raw::c_long,
    );
    pub fn GA_apperror(errstr: *const ::std::os::raw::c_char);
    pub fn GA_askok(info: *const ::std::os::raw::c_char);
    pub fn GA_askokcancel(question: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn GA_askyesno(question: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn GA_askyesnocancel(question: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn GA_askstring(
        question: *const ::std::os::raw::c_char,
        default_string: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn GA_askpassword(
        question: *const ::std::os::raw::c_char,
        default_string: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn GA_askfilename(
        title: *const ::std::os::raw::c_char,
        default_name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn GA_askfilenamewithdir(
        title: *const ::std::os::raw::c_char,
        default_name: *const ::std::os::raw::c_char,
        dir: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn GA_askfilesave(
        title: *const ::std::os::raw::c_char,
        default_name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn GA_askUserPass(title: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn GA_clickbutton(w: window, b: button);
    #[doc = "  Time functions."]
    pub fn GA_settimer(millisec: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn GA_settimerfn(timeout: timerfn, data: *mut ::std::os::raw::c_void);
    pub fn GA_setmousetimer(millisec: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
    pub fn GA_delay(millisec: ::std::os::raw::c_uint);
    pub fn GA_currenttime() -> ::std::os::raw::c_long;
    #[doc = "  Cursors."]
    pub fn GA_newcursor(hotspot: point, img: image) -> cursor;
    pub fn GA_createcursor(
        offset: point,
        white_mask: *mut GAbyte,
        black_shape: *mut GAbyte,
    ) -> cursor;
    pub fn GA_loadcursor(name: *const ::std::os::raw::c_char) -> cursor;
    pub fn GA_setcursor(c: cursor);
    #[doc = "  Change the drawing state."]
    pub fn GA_copydrawstate() -> drawstate;
    pub fn GA_setdrawstate(saved_state: drawstate);
    pub fn GA_restoredrawstate(saved_state: drawstate);
    pub fn GA_resetdrawstate();
    #[doc = "  Caret-related functions."]
    pub fn GA_setcaret(
        c: control,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
    pub fn GA_showcaret(c: control, showing: ::std::os::raw::c_int);
    #[doc = " fixed-width font"]
    pub static mut GA_FixedFont: font;
    #[doc = " normal arrow cursor"]
    pub static mut GA_ArrowCursor: cursor;
    #[doc = " invisible cursor"]
    pub static mut GA_BlankCursor: cursor;
    #[doc = " wait for the computer"]
    pub static mut GA_WatchCursor: cursor;
    #[doc = " insert text"]
    pub static mut GA_CaretCursor: cursor;
    #[doc = " insert text"]
    pub static mut GA_TextCursor: cursor;
    #[doc = " hand pointer"]
    pub static mut GA_HandCursor: cursor;
    #[doc = " cross pointer"]
    pub static mut GA_CrossCursor: cursor;
    #[doc = " system font"]
    pub static mut GA_SystemFont: font;
    #[doc = " times roman font (serif)"]
    pub static mut GA_Times: font;
    #[doc = " helvetica font (sans serif)"]
    pub static mut GA_Helvetica: font;
    #[doc = " courier font (fixed width)"]
    pub static mut GA_Courier: font;
}
