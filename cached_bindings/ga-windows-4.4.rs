/* automatically generated by rust-bindgen 0.71.1 */

/* OS: windows */
/* Platform:  */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.4.3 */

pub type popup = menu ; # [doc = " printer.c"] pub type printer = objptr ; # [doc = " metafile.c"] pub type metafile = objptr ; # [doc = " gmenus.c"] # [repr (C)] pub struct MenuItem { pub nm : * mut :: std :: os :: raw :: c_char , pub fn_ : menufn , pub key : :: std :: os :: raw :: c_int , pub m : menuitem , } pub const DblClick : u32 = 16 ; pub const Border : u32 = 269484032 ; pub const lSolid : u32 = 0 ; pub const lDash : u32 = 69 ; pub const lShortDash : u32 = 67 ; pub const lLongDash : u32 = 72 ; pub const lDot : u32 = 65 ; pub const lDashDot : u32 = 16709 ; pub const lShortDashDot : u32 = 16707 ; pub const lLongDashDot : u32 = 16712 ; pub const lDashDotDot : u32 = 4272453 ; pub const lShortDashDotDot : u32 = 4272451 ; pub const lLongDashDotDot : u32 = 4272456 ; pub const HWINSB : u32 = 0 ; pub const VWINSB : u32 = 1 ; pub const CONTROLSB : u32 = 2 ; unsafe extern "C" { # [doc = " renamed functions"] pub fn GA_gamainloop () ; pub fn GA_gabeep () ; pub fn GA_appcleanup () ; pub fn GA_ismdi () -> :: std :: os :: raw :: c_int ; pub fn GA_isUnicodeWindow (c : control) -> :: std :: os :: raw :: c_int ; pub fn GA_isiconic (w : window) -> :: std :: os :: raw :: c_int ; pub fn GA_screen_coords (c : control) -> rect ; pub fn GA_newmdimenu () -> menu ; pub fn GA_newpopup (fn_ : actionfn) -> popup ; pub fn GA_gmenubar (fn_ : actionfn , arg1 : * mut MenuItem) -> menubar ; pub fn GA_gpopup (fn_ : actionfn , arg1 : * mut MenuItem) -> popup ; pub fn GA_gchangepopup (w : window , p : popup) ; # [doc = " next is limited to current window..."] pub fn GA_gchangemenubar (mb : menubar) ; # [doc = " tooltips.c"] pub fn GA_addtooltip (c : control , tp : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ; # [doc = " status.c"] pub fn GA_addstatusbar () -> :: std :: os :: raw :: c_int ; pub fn GA_delstatusbar () -> :: std :: os :: raw :: c_int ; pub fn GA_setstatus (text : * const :: std :: os :: raw :: c_char) ; # [doc = " dialogs.c"] pub fn GA_setuserfilter (arg1 : * const :: std :: os :: raw :: c_char) ; pub fn GA_askchangedir () ; pub fn GA_askcdstring (question : * const :: std :: os :: raw :: c_char , default_string : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ; pub fn GA_askfilesavewithdir (title : * const :: std :: os :: raw :: c_char , default_name : * const :: std :: os :: raw :: c_char , dir : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ; pub fn GA_askfilenames (title : * const :: std :: os :: raw :: c_char , default_name : * const :: std :: os :: raw :: c_char , multi : :: std :: os :: raw :: c_int , filters : * const :: std :: os :: raw :: c_char , filterindex : :: std :: os :: raw :: c_int , strbuf : * mut :: std :: os :: raw :: c_char , bufsize : :: std :: os :: raw :: c_int , dir : * const :: std :: os :: raw :: c_char) -> * mut :: std :: os :: raw :: c_char ; pub fn GA_countFilenames (strbuf : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ; pub fn GA_setuserfilterW (arg1 : * const wchar_t) ; pub fn GA_askfilenameW (title : * const :: std :: os :: raw :: c_char , default_name : * const :: std :: os :: raw :: c_char) -> * mut wchar_t ; pub fn GA_askfilenamesW (title : * const wchar_t , default_name : * const wchar_t , multi : :: std :: os :: raw :: c_int , filters : * const wchar_t , filterindex : :: std :: os :: raw :: c_int , dir : * const wchar_t) -> * mut wchar_t ; pub fn GA_askfilesaveW (title : * const :: std :: os :: raw :: c_char , default_name : * const :: std :: os :: raw :: c_char) -> * mut wchar_t ; # [doc = "  rgb.c"] pub fn GA_nametorgb (colourname : * const :: std :: os :: raw :: c_char) -> rgb ; pub fn GA_rgbtoname (in_ : rgb) -> * const :: std :: os :: raw :: c_char ; pub fn GA_rgbtonum (in_ : rgb) -> :: std :: os :: raw :: c_int ; pub fn GA_myGetSysColor (arg1 : :: std :: os :: raw :: c_int) -> rgb ; pub fn GA_dialog_bg () -> rgb ; # [doc = " clipboard.c"] pub fn GA_copytoclipboard (src : drawing) ; pub fn GA_copystringtoclipboard (str_ : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ; pub fn GA_getstringfromclipboard (str_ : * mut :: std :: os :: raw :: c_char , n : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ; pub fn GA_clipboardhastext () -> :: std :: os :: raw :: c_int ; # [doc = " gimage.c"] pub fn GA_bitmaptoimage (bm : bitmap) -> image ; pub fn GA_newprinter (w : f64 , h : f64 , name : * const :: std :: os :: raw :: c_char) -> printer ; pub fn GA_nextpage (p : printer) ; pub fn GA_newmetafile (name : * const :: std :: os :: raw :: c_char , width : f64 , height : f64 , xpinch : f64 , ypinch : f64) -> metafile ; pub fn GA_ggetcliprect (d : drawing) -> rect ; pub fn GA_gsetcliprect (d : drawing , r : rect) ; pub fn GA_gbitblt (db : bitmap , sb : bitmap , p : point , r : rect) ; pub fn GA_gscroll (d : drawing , dp : point , r : rect) ; pub fn GA_ginvert (d : drawing , r : rect) ; pub fn GA_ggetpixel (d : drawing , p : point) -> rgb ; pub fn GA_gsetpixel (d : drawing , p : point , c : rgb) ; pub fn GA_gdrawline (d : drawing , width : :: std :: os :: raw :: c_int , style : :: std :: os :: raw :: c_int , c : rgb , p1 : point , p2 : point , fast : :: std :: os :: raw :: c_int , lend : :: std :: os :: raw :: c_int , ljoin : :: std :: os :: raw :: c_int , lmitre : f32) ; pub fn GA_gdrawrect (d : drawing , width : :: std :: os :: raw :: c_int , style : :: std :: os :: raw :: c_int , c : rgb , r : rect , fast : :: std :: os :: raw :: c_int , lend : :: std :: os :: raw :: c_int , ljoin : :: std :: os :: raw :: c_int , lmitre : f32) ; pub fn GA_gfillrect (d : drawing , fill : rgb , r : rect) ; pub fn GA_gcopy (d : drawing , d2 : drawing , r : rect) ; pub fn GA_gcopyalpha (d : drawing , d2 : drawing , r : rect , alpha : :: std :: os :: raw :: c_int) ; pub fn GA_gcopyalpha2 (d : drawing , src : image , r : rect) ; pub fn GA_gdrawellipse (d : drawing , width : :: std :: os :: raw :: c_int , border : rgb , r : rect , fast : :: std :: os :: raw :: c_int , lend : :: std :: os :: raw :: c_int , ljoin : :: std :: os :: raw :: c_int , lmitre : f32) ; pub fn GA_gfillellipse (d : drawing , fill : rgb , r : rect) ; pub fn GA_gdrawpolyline (d : drawing , width : :: std :: os :: raw :: c_int , style : :: std :: os :: raw :: c_int , c : rgb , p : * mut point , n : :: std :: os :: raw :: c_int , closepath : :: std :: os :: raw :: c_int , fast : :: std :: os :: raw :: c_int , lend : :: std :: os :: raw :: c_int , ljoin : :: std :: os :: raw :: c_int , lmitre : f32) ; pub fn GA_gsetpolyfillmode (d : drawing , oddeven : :: std :: os :: raw :: c_int) ; pub fn GA_gfillpolygon (d : drawing , fill : rgb , p : * mut point , n : :: std :: os :: raw :: c_int) ; pub fn GA_gfillpolypolygon (d : drawing , fill : rgb , p : * mut point , npoly : :: std :: os :: raw :: c_int , nper : * mut :: std :: os :: raw :: c_int) ; pub fn GA_gdrawimage (d : drawing , img : image , dr : rect , sr : rect) ; pub fn GAI_gmaskimage (d : drawing , img : image , dr : rect , sr : rect , mask : image) ; pub fn GA_gdrawstr (d : drawing , f : font , c : rgb , p : point , s : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ; pub fn GA_gdrawstr1 (d : drawing , f : font , c : rgb , p : point , s : * const :: std :: os :: raw :: c_char , hadj : f64) ; pub fn GA_gstrrect (d : drawing , f : font , s : * const :: std :: os :: raw :: c_char) -> rect ; pub fn GA_gstrsize (d : drawing , f : font , s : * const :: std :: os :: raw :: c_char) -> point ; pub fn GA_gstrwidth (d : drawing , f : font , s : * const :: std :: os :: raw :: c_char) -> :: std :: os :: raw :: c_int ; pub fn GA_gcharmetric (d : drawing , f : font , c : :: std :: os :: raw :: c_int , ascent : * mut :: std :: os :: raw :: c_int , descent : * mut :: std :: os :: raw :: c_int , width : * mut :: std :: os :: raw :: c_int) ; pub fn GA_gnewfont (d : drawing , face : * const :: std :: os :: raw :: c_char , style : :: std :: os :: raw :: c_int , size : :: std :: os :: raw :: c_int , rot : f64 , usePoints : :: std :: os :: raw :: c_int) -> font ; pub fn GA_gnewfont2 (d : drawing , face : * const :: std :: os :: raw :: c_char , style : :: std :: os :: raw :: c_int , size : :: std :: os :: raw :: c_int , rot : f64 , usePoints : :: std :: os :: raw :: c_int , quality : :: std :: os :: raw :: c_int) -> font ; pub fn GA_ghasfixedwidth (f : font) -> :: std :: os :: raw :: c_int ; pub fn GA_newfield_no_border (text : * const :: std :: os :: raw :: c_char , r : rect) -> field ; pub fn GA_gdrawwcs (d : drawing , f : font , c : rgb , p : point , s : * const wchar_t) -> :: std :: os :: raw :: c_int ; pub fn GA_gwcswidth (d : drawing , f : font , s : * const wchar_t) -> :: std :: os :: raw :: c_int ; pub fn GA_gwcharmetric (d : drawing , f : font , c : :: std :: os :: raw :: c_int , ascent : * mut :: std :: os :: raw :: c_int , descent : * mut :: std :: os :: raw :: c_int , width : * mut :: std :: os :: raw :: c_int) ; pub fn GA_gwdrawstr1 (d : drawing , f : font , c : rgb , p : point , s : * const wchar_t , cnt : :: std :: os :: raw :: c_int , hadj : f64) ; pub fn GA_gstrwidth1 (d : drawing , f : font , s : * const :: std :: os :: raw :: c_char , enc : :: std :: os :: raw :: c_int) -> :: std :: os :: raw :: c_int ; # [doc = " pixels"] pub fn GA_devicewidth (dev : drawing) -> :: std :: os :: raw :: c_int ; pub fn GA_deviceheight (dev : drawing) -> :: std :: os :: raw :: c_int ; # [doc = " mm"] pub fn GA_devicewidthmm (dev : drawing) -> :: std :: os :: raw :: c_int ; pub fn GA_deviceheightmm (dev : drawing) -> :: std :: os :: raw :: c_int ; # [doc = " pixels per inch"] pub fn GA_devicepixelsx (dev : drawing) -> :: std :: os :: raw :: c_int ; pub fn GA_devicepixelsy (dev : drawing) -> :: std :: os :: raw :: c_int ; pub fn GA_isTopmost (w : window) -> :: std :: os :: raw :: c_int ; pub fn GA_BringToTop (w : window , stay : :: std :: os :: raw :: c_int) ; pub fn GA_getHandle (w : window) -> * mut :: std :: os :: raw :: c_void ; pub fn GA_msgWindow (c : window , type_ : :: std :: os :: raw :: c_int) ; pub fn GA_gchangescrollbar (sb : scrollbar , which : :: std :: os :: raw :: c_int , where_ : :: std :: os :: raw :: c_int , max : :: std :: os :: raw :: c_int , pagesize : :: std :: os :: raw :: c_int , disablenoscroll : :: std :: os :: raw :: c_int) ; pub fn GA_gsetcursor (d : drawing , c : cursor) ; pub fn GA_newtoolbar (height : :: std :: os :: raw :: c_int) -> control ; pub fn GA_newtoolbutton (img : image , r : rect , fn_ : actionfn) -> button ; pub fn GA_scrolltext (c : textbox , lines : :: std :: os :: raw :: c_int) ; pub fn GA_ggetkeystate () -> :: std :: os :: raw :: c_int ; pub fn GA_scrollcaret (c : textbox , lines : :: std :: os :: raw :: c_int) ; pub fn GA_gsetmodified (c : textbox , modified : :: std :: os :: raw :: c_int) ; pub fn GA_ggetmodified (c : textbox) -> :: std :: os :: raw :: c_int ; pub fn GA_getlinelength (c : textbox) -> :: std :: os :: raw :: c_int ; pub fn GA_getcurrentline (c : textbox , line : * mut :: std :: os :: raw :: c_char , length : :: std :: os :: raw :: c_int) ; pub fn GA_getseltext (c : textbox , text : * mut :: std :: os :: raw :: c_char) ; pub fn GA_setlimittext (t : textbox , limit : :: std :: os :: raw :: c_long) ; pub fn GA_getlimittext (t : textbox) -> :: std :: os :: raw :: c_long ; pub fn GA_checklimittext (t : textbox , n : :: std :: os :: raw :: c_long) ; pub fn GA_getpastelength () -> :: std :: os :: raw :: c_long ; pub fn GA_textselectionex (obj : control , start : * mut :: std :: os :: raw :: c_long , end : * mut :: std :: os :: raw :: c_long) ; pub fn GA_selecttextex (obj : control , start : :: std :: os :: raw :: c_long , end : :: std :: os :: raw :: c_long) ; pub fn GA_finddialog (t : textbox) ; pub fn GA_replacedialog (t : textbox) ; pub fn GA_modeless_active () -> :: std :: os :: raw :: c_int ; # [doc = " menus.c"] pub fn GA_remove_menu_item (obj : menuitem) ; # [doc = " events.c"] pub fn GA_toolbar_show () ; pub fn GA_toolbar_hide () ; }