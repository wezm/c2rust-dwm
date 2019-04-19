use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    pub type _FcCharSet;
    pub type _FcPattern;
    pub type _FcConfig;
    pub type _XftDraw;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn XCreateFontCursor(_: *mut Display, _: libc::c_uint) -> Cursor;
    #[no_mangle]
    fn XCreateGC(_: *mut Display, _: Drawable, _: libc::c_ulong,
                 _: *mut XGCValues) -> GC;
    #[no_mangle]
    fn XCreatePixmap(_: *mut Display, _: Drawable, _: libc::c_uint,
                     _: libc::c_uint, _: libc::c_uint) -> Pixmap;
    #[no_mangle]
    fn XCopyArea(_: *mut Display, _: Drawable, _: Drawable, _: GC,
                 _: libc::c_int, _: libc::c_int, _: libc::c_uint,
                 _: libc::c_uint, _: libc::c_int, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn XDrawRectangle(_: *mut Display, _: Drawable, _: GC, _: libc::c_int,
                      _: libc::c_int, _: libc::c_uint, _: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn XFillRectangle(_: *mut Display, _: Drawable, _: GC, _: libc::c_int,
                      _: libc::c_int, _: libc::c_uint, _: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn XFreeCursor(_: *mut Display, _: Cursor) -> libc::c_int;
    #[no_mangle]
    fn XFreeGC(_: *mut Display, _: GC) -> libc::c_int;
    #[no_mangle]
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> libc::c_int;
    #[no_mangle]
    fn XSetForeground(_: *mut Display, _: GC, _: libc::c_ulong)
     -> libc::c_int;
    #[no_mangle]
    fn XSetLineAttributes(_: *mut Display, _: GC, _: libc::c_uint,
                          _: libc::c_int, _: libc::c_int, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn FcConfigSubstitute(config: *mut FcConfig, p: *mut FcPattern,
                          kind: FcMatchKind) -> FcBool;
    #[no_mangle]
    fn FcCharSetCreate() -> *mut FcCharSet;
    #[no_mangle]
    fn FcCharSetDestroy(fcs: *mut FcCharSet);
    #[no_mangle]
    fn FcCharSetAddChar(fcs: *mut FcCharSet, ucs4: FcChar32) -> FcBool;
    #[no_mangle]
    fn FcDefaultSubstitute(pattern: *mut FcPattern);
    #[no_mangle]
    fn FcNameParse(name: *const FcChar8) -> *mut FcPattern;
    #[no_mangle]
    fn FcPatternDuplicate(p: *const FcPattern) -> *mut FcPattern;
    #[no_mangle]
    fn FcPatternDestroy(p: *mut FcPattern);
    #[no_mangle]
    fn FcPatternAddCharSet(p: *mut FcPattern, object: *const libc::c_char,
                           c: *const FcCharSet) -> FcBool;
    #[no_mangle]
    fn FcPatternAddBool(p: *mut FcPattern, object: *const libc::c_char,
                        b: FcBool) -> FcBool;
    #[no_mangle]
    fn FcPatternGetBool(p: *const FcPattern, object: *const libc::c_char,
                        n: libc::c_int, b: *mut FcBool) -> FcResult;
    #[no_mangle]
    fn XftColorAllocName(dpy: *mut Display, visual: *const Visual,
                         cmap: Colormap, name: *const libc::c_char,
                         result: *mut XftColor) -> libc::c_int;
    #[no_mangle]
    fn XftDrawCreate(dpy: *mut Display, drawable: Drawable,
                     visual: *mut Visual, colormap: Colormap) -> *mut XftDraw;
    #[no_mangle]
    fn XftDrawDestroy(draw: *mut XftDraw);
    #[no_mangle]
    fn XftDrawStringUtf8(draw: *mut XftDraw, color: *const XftColor,
                         pub_0: *mut XftFont, x: libc::c_int, y: libc::c_int,
                         string: *const FcChar8, len: libc::c_int);
    #[no_mangle]
    fn XftTextExtentsUtf8(dpy: *mut Display, pub_0: *mut XftFont,
                          string: *const FcChar8, len: libc::c_int,
                          extents: *mut XGlyphInfo);
    #[no_mangle]
    fn XftFontMatch(dpy: *mut Display, screen: libc::c_int,
                    pattern: *const FcPattern, result: *mut FcResult)
     -> *mut FcPattern;
    #[no_mangle]
    fn XftFontOpenName(dpy: *mut Display, screen: libc::c_int,
                       name: *const libc::c_char) -> *mut XftFont;
    #[no_mangle]
    fn XftFontOpenPattern(dpy: *mut Display, pattern: *mut FcPattern)
     -> *mut XftFont;
    #[no_mangle]
    fn XftFontClose(dpy: *mut Display, pub_0: *mut XftFont);
    #[no_mangle]
    fn XftCharExists(dpy: *mut Display, pub_0: *mut XftFont, ucs4: FcChar32)
     -> FcBool;
    #[no_mangle]
    fn ecalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    /* See LICENSE file for copyright and license details. */
    #[no_mangle]
    fn die(fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type XID = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type XPointer = *mut libc::c_char;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option<unsafe extern "C" fn(_: *mut _XExtData)
                                 -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XGCValues {
    pub function: libc::c_int,
    pub plane_mask: libc::c_ulong,
    pub foreground: libc::c_ulong,
    pub background: libc::c_ulong,
    pub line_width: libc::c_int,
    pub line_style: libc::c_int,
    pub cap_style: libc::c_int,
    pub join_style: libc::c_int,
    pub fill_style: libc::c_int,
    pub fill_rule: libc::c_int,
    pub arc_mode: libc::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: libc::c_int,
    pub ts_y_origin: libc::c_int,
    pub font: Font,
    pub subwindow_mode: libc::c_int,
    pub graphics_exposures: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: libc::c_int,
    pub dashes: libc::c_char,
}
pub type GC = *mut _XGC;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
pub type Display = _XDisplay;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                                   -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(_: *mut _XDisplay)
                              -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut unnamed;
pub type FcChar8 = libc::c_uchar;
pub type FcChar32 = libc::c_uint;
pub type FcBool = libc::c_int;
pub type FcCharSet = _FcCharSet;
pub type _FcResult = libc::c_uint;
pub const FcResultOutOfMemory: _FcResult = 4;
pub const FcResultNoId: _FcResult = 3;
pub const FcResultTypeMismatch: _FcResult = 2;
pub const FcResultNoMatch: _FcResult = 1;
pub const FcResultMatch: _FcResult = 0;
pub type FcResult = _FcResult;
pub type FcPattern = _FcPattern;
pub type _FcMatchKind = libc::c_uint;
pub const FcMatchKindBegin: _FcMatchKind = 0;
pub const FcMatchKindEnd: _FcMatchKind = 3;
pub const FcMatchScan: _FcMatchKind = 2;
pub const FcMatchFont: _FcMatchKind = 1;
pub const FcMatchPattern: _FcMatchKind = 0;
pub type FcMatchKind = _FcMatchKind;
pub type FcConfig = _FcConfig;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XRenderColor {
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub alpha: libc::c_ushort,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _XGlyphInfo {
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
    pub x: libc::c_short,
    pub y: libc::c_short,
    pub xOff: libc::c_short,
    pub yOff: libc::c_short,
}
pub type XGlyphInfo = _XGlyphInfo;
pub type XftChar8 = FcChar8;
pub type XftResult = FcResult;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _XftFont {
    pub ascent: libc::c_int,
    pub descent: libc::c_int,
    pub height: libc::c_int,
    pub max_advance_width: libc::c_int,
    pub charset: *mut FcCharSet,
    pub pattern: *mut FcPattern,
}
pub type XftFont = _XftFont;
pub type XftDraw = _XftDraw;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _XftColor {
    pub pixel: libc::c_ulong,
    pub color: XRenderColor,
}
pub type XftColor = _XftColor;
/* See LICENSE file for copyright and license details. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Cur {
    pub cursor: Cursor,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Fnt {
    pub dpy: *mut Display,
    pub h: libc::c_uint,
    pub xfont: *mut XftFont,
    pub pattern: *mut FcPattern,
    pub next: *mut Fnt,
}
pub type unnamed_0 = libc::c_uint;
/* Clr scheme index */
pub const ColBorder: unnamed_0 = 2;
pub const ColBg: unnamed_0 = 1;
pub const ColFg: unnamed_0 = 0;
pub type Clr = XftColor;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Drw {
    pub w: libc::c_uint,
    pub h: libc::c_uint,
    pub dpy: *mut Display,
    pub screen: libc::c_int,
    pub root: Window,
    pub drawable: Drawable,
    pub gc: GC,
    pub scheme: *mut Clr,
    pub fonts: *mut Fnt,
}
/* Drawable abstraction */
#[no_mangle]
pub unsafe extern "C" fn drw_create(mut dpy: *mut Display,
                                    mut screen: libc::c_int, mut root: Window,
                                    mut w: libc::c_uint, mut h: libc::c_uint)
 -> *mut Drw {
    let mut drw: *mut Drw =
        ecalloc(1i32 as size_t, ::std::mem::size_of::<Drw>() as libc::c_ulong)
            as *mut Drw;
    (*drw).dpy = dpy;
    (*drw).screen = screen;
    (*drw).root = root;
    (*drw).w = w;
    (*drw).h = h;
    (*drw).drawable =
        XCreatePixmap(dpy, root, w, h,
                      (*(*(dpy as
                               _XPrivDisplay)).screens.offset(screen as
                                                                  isize)).root_depth
                          as libc::c_uint);
    (*drw).gc =
        XCreateGC(dpy, root, 0i32 as libc::c_ulong, 0 as *mut XGCValues);
    XSetLineAttributes(dpy, (*drw).gc, 1i32 as libc::c_uint, 0i32, 1i32,
                       0i32);
    return drw;
}
#[no_mangle]
pub unsafe extern "C" fn drw_resize(mut drw: *mut Drw, mut w: libc::c_uint,
                                    mut h: libc::c_uint) {
    if drw.is_null() { return }
    (*drw).w = w;
    (*drw).h = h;
    if 0 != (*drw).drawable { XFreePixmap((*drw).dpy, (*drw).drawable); }
    (*drw).drawable =
        XCreatePixmap((*drw).dpy, (*drw).root, w, h,
                      (*(*((*drw).dpy as
                               _XPrivDisplay)).screens.offset((*drw).screen as
                                                                  isize)).root_depth
                          as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn drw_free(mut drw: *mut Drw) {
    XFreePixmap((*drw).dpy, (*drw).drawable);
    XFreeGC((*drw).dpy, (*drw).gc);
    free(drw as *mut libc::c_void);
}
/* Fnt abstraction */
#[no_mangle]
pub unsafe extern "C" fn drw_fontset_create(mut drw: *mut Drw,
                                            mut fonts:
                                                *mut *const libc::c_char,
                                            mut fontcount: size_t)
 -> *mut Fnt {
    let mut cur: *mut Fnt = 0 as *mut Fnt;
    let mut ret: *mut Fnt = 0 as *mut Fnt;
    let mut i: size_t = 0;
    if drw.is_null() || fonts.is_null() { return 0 as *mut Fnt }
    i = 1i32 as size_t;
    while i <= fontcount {
        cur =
            xfont_create(drw,
                         *fonts.offset(fontcount.wrapping_sub(i) as isize),
                         0 as *mut FcPattern);
        if !cur.is_null() { (*cur).next = ret; ret = cur }
        i = i.wrapping_add(1)
    }
    (*drw).fonts = ret;
    return (*drw).fonts;
}
/* This function is an implementation detail. Library users should use
 * drw_fontset_create instead.
 */
unsafe extern "C" fn xfont_create(mut drw: *mut Drw,
                                  mut fontname: *const libc::c_char,
                                  mut fontpattern: *mut FcPattern)
 -> *mut Fnt {
    let mut font: *mut Fnt = 0 as *mut Fnt;
    let mut xfont: *mut XftFont = 0 as *mut XftFont;
    let mut pattern: *mut FcPattern = 0 as *mut FcPattern;
    if !fontname.is_null() {
        xfont = XftFontOpenName((*drw).dpy, (*drw).screen, fontname);
        if xfont.is_null() {
            fprintf(stderr,
                    b"error, cannot load font from name: \'%s\'\n\x00" as
                        *const u8 as *const libc::c_char, fontname);
            return 0 as *mut Fnt
        }
        pattern = FcNameParse(fontname as *mut FcChar8);
        if pattern.is_null() {
            fprintf(stderr,
                    b"error, cannot parse font name to pattern: \'%s\'\n\x00"
                        as *const u8 as *const libc::c_char, fontname);
            XftFontClose((*drw).dpy, xfont);
            return 0 as *mut Fnt
        }
    } else if !fontpattern.is_null() {
        xfont = XftFontOpenPattern((*drw).dpy, fontpattern);
        if xfont.is_null() {
            fprintf(stderr,
                    b"error, cannot load font from pattern.\n\x00" as
                        *const u8 as *const libc::c_char);
            return 0 as *mut Fnt
        }
    } else {
        die(b"no font specified.\x00" as *const u8 as *const libc::c_char);
    }
    /* Do not allow using color fonts. This is a workaround for a BadLength
	 * error from Xft with color glyphs. Modelled on the Xterm workaround. See
	 * https://bugzilla.redhat.com/show_bug.cgi?id=1498269
	 * https://lists.suckless.org/dev/1701/30932.html
	 * https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=916349
	 * and lots more all over the internet.
	 */
    let mut iscol: FcBool = 0;
    if FcPatternGetBool((*xfont).pattern,
                        b"color\x00" as *const u8 as *const libc::c_char,
                        0i32, &mut iscol) as libc::c_uint ==
           FcResultMatch as libc::c_int as libc::c_uint && 0 != iscol {
        XftFontClose((*drw).dpy, xfont);
        return 0 as *mut Fnt
    }
    font =
        ecalloc(1i32 as size_t, ::std::mem::size_of::<Fnt>() as libc::c_ulong)
            as *mut Fnt;
    (*font).xfont = xfont;
    (*font).pattern = pattern;
    (*font).h = ((*xfont).ascent + (*xfont).descent) as libc::c_uint;
    (*font).dpy = (*drw).dpy;
    return font;
}
#[no_mangle]
pub unsafe extern "C" fn drw_fontset_free(mut font: *mut Fnt) {
    if !font.is_null() { drw_fontset_free((*font).next); xfont_free(font); };
}
unsafe extern "C" fn xfont_free(mut font: *mut Fnt) {
    if font.is_null() { return }
    if !(*font).pattern.is_null() { FcPatternDestroy((*font).pattern); }
    XftFontClose((*font).dpy, (*font).xfont);
    free(font as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn drw_fontset_getwidth(mut drw: *mut Drw,
                                              mut text: *const libc::c_char)
 -> libc::c_uint {
    if drw.is_null() || (*drw).fonts.is_null() || text.is_null() {
        return 0i32 as libc::c_uint
    }
    return drw_text(drw, 0i32, 0i32, 0i32 as libc::c_uint,
                    0i32 as libc::c_uint, 0i32 as libc::c_uint, text, 0i32) as
               libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn drw_text(mut drw: *mut Drw, mut x: libc::c_int,
                                  mut y: libc::c_int, mut w: libc::c_uint,
                                  mut h: libc::c_uint, mut lpad: libc::c_uint,
                                  mut text: *const libc::c_char,
                                  mut invert: libc::c_int) -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ty: libc::c_int = 0;
    let mut ew: libc::c_uint = 0;
    let mut d: *mut XftDraw = 0 as *mut XftDraw;
    let mut usedfont: *mut Fnt = 0 as *mut Fnt;
    let mut curfont: *mut Fnt = 0 as *mut Fnt;
    let mut nextfont: *mut Fnt = 0 as *mut Fnt;
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    let mut utf8strlen: libc::c_int = 0;
    let mut utf8charlen: libc::c_int = 0;
    let mut render: libc::c_int =
        (0 != x || 0 != y || 0 != w || 0 != h) as libc::c_int;
    let mut utf8codepoint: libc::c_long = 0i32 as libc::c_long;
    let mut utf8str: *const libc::c_char = 0 as *const libc::c_char;
    let mut fccharset: *mut FcCharSet = 0 as *mut FcCharSet;
    let mut fcpattern: *mut FcPattern = 0 as *mut FcPattern;
    let mut match_0: *mut FcPattern = 0 as *mut FcPattern;
    let mut result: XftResult = FcResultMatch;
    let mut charexists: libc::c_int = 0i32;
    if drw.is_null() || 0 != render && (*drw).scheme.is_null() ||
           text.is_null() || (*drw).fonts.is_null() {
        return 0i32
    }
    if 0 == render {
        w = !w
    } else {
        XSetForeground((*drw).dpy, (*drw).gc,
                       (*(*drw).scheme.offset((if 0 != invert {
                                                   ColFg as libc::c_int
                                               } else {
                                                   ColBg as libc::c_int
                                               }) as isize)).pixel);
        XFillRectangle((*drw).dpy, (*drw).drawable, (*drw).gc, x, y, w, h);
        d =
            XftDrawCreate((*drw).dpy, (*drw).drawable,
                          (*(*((*drw).dpy as
                                   _XPrivDisplay)).screens.offset((*drw).screen
                                                                      as
                                                                      isize)).root_visual,
                          (*(*((*drw).dpy as
                                   _XPrivDisplay)).screens.offset((*drw).screen
                                                                      as
                                                                      isize)).cmap);
        x =
            (x as libc::c_uint).wrapping_add(lpad) as libc::c_int as
                libc::c_int;
        w = w.wrapping_sub(lpad)
    }
    usedfont = (*drw).fonts;
    loop  {
        utf8strlen = 0i32;
        utf8str = text;
        nextfont = 0 as *mut Fnt;
        while 0 != *text {
            utf8charlen =
                utf8decode(text, &mut utf8codepoint, 4i32 as size_t) as
                    libc::c_int;
            curfont = (*drw).fonts;
            while !curfont.is_null() {
                charexists =
                    (0 != charexists ||
                         0 !=
                             XftCharExists((*drw).dpy, (*curfont).xfont,
                                           utf8codepoint as FcChar32)) as
                        libc::c_int;
                if 0 != charexists {
                    if curfont == usedfont {
                        utf8strlen += utf8charlen;
                        text = text.offset(utf8charlen as isize)
                    } else { nextfont = curfont }
                    break ;
                } else { curfont = (*curfont).next }
            }
            if 0 == charexists || !nextfont.is_null() { break ; }
            charexists = 0i32
        }
        if 0 != utf8strlen {
            drw_font_getexts(usedfont, utf8str, utf8strlen as libc::c_uint,
                             &mut ew, 0 as *mut libc::c_uint);
            len =
                if (utf8strlen as libc::c_ulong) <
                       (::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong)
                   {
                    utf8strlen as libc::c_ulong
                } else {
                    (::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong)
                };
            while 0 != len && ew > w {
                drw_font_getexts(usedfont, utf8str, len as libc::c_uint,
                                 &mut ew, 0 as *mut libc::c_uint);
                len = len.wrapping_sub(1)
            }
            if 0 != len {
                memcpy(buf.as_mut_ptr() as *mut libc::c_void,
                       utf8str as *const libc::c_void, len);
                buf[len as usize] = '\u{0}' as i32 as libc::c_char;
                if len < utf8strlen as libc::c_ulong {
                    i = len;
                    while 0 != i &&
                              i > len.wrapping_sub(3i32 as libc::c_ulong) {
                        i = i.wrapping_sub(1);
                        buf[i as usize] = '.' as i32 as libc::c_char
                    }
                }
                if 0 != render {
                    ty =
                        (y as
                             libc::c_uint).wrapping_add(h.wrapping_sub((*usedfont).h).wrapping_div(2i32
                                                                                                       as
                                                                                                       libc::c_uint)).wrapping_add((*(*usedfont).xfont).ascent
                                                                                                                                       as
                                                                                                                                       libc::c_uint)
                            as libc::c_int;
                    XftDrawStringUtf8(d,
                                      &mut *(*drw).scheme.offset((if 0 !=
                                                                         invert
                                                                     {
                                                                      ColBg as
                                                                          libc::c_int
                                                                  } else {
                                                                      ColFg as
                                                                          libc::c_int
                                                                  }) as
                                                                     isize),
                                      (*usedfont).xfont, x, ty,
                                      buf.as_mut_ptr() as *mut XftChar8,
                                      len as libc::c_int);
                }
                x =
                    (x as libc::c_uint).wrapping_add(ew) as libc::c_int as
                        libc::c_int;
                w = w.wrapping_sub(ew)
            }
        }
        if 0 == *text { break ; }
        if !nextfont.is_null() {
            charexists = 0i32;
            usedfont = nextfont
        } else {
            charexists = 1i32;
            fccharset = FcCharSetCreate();
            FcCharSetAddChar(fccharset, utf8codepoint as FcChar32);
            if (*(*drw).fonts).pattern.is_null() {
                die(b"the first font in the cache must be loaded from a font string.\x00"
                        as *const u8 as *const libc::c_char);
            }
            fcpattern = FcPatternDuplicate((*(*drw).fonts).pattern);
            FcPatternAddCharSet(fcpattern,
                                b"charset\x00" as *const u8 as
                                    *const libc::c_char, fccharset);
            FcPatternAddBool(fcpattern,
                             b"scalable\x00" as *const u8 as
                                 *const libc::c_char, 1i32);
            FcPatternAddBool(fcpattern,
                             b"color\x00" as *const u8 as *const libc::c_char,
                             0i32);
            FcConfigSubstitute(0 as *mut FcConfig, fcpattern, FcMatchPattern);
            FcDefaultSubstitute(fcpattern);
            match_0 =
                XftFontMatch((*drw).dpy, (*drw).screen, fcpattern,
                             &mut result);
            FcCharSetDestroy(fccharset);
            FcPatternDestroy(fcpattern);
            if !match_0.is_null() {
                usedfont =
                    xfont_create(drw, 0 as *const libc::c_char, match_0);
                if !usedfont.is_null() &&
                       0 !=
                           XftCharExists((*drw).dpy, (*usedfont).xfont,
                                         utf8codepoint as FcChar32) {
                    curfont = (*drw).fonts;
                    while !(*curfont).next.is_null() {
                        curfont = (*curfont).next
                    }
                    (*curfont).next = usedfont
                } else { xfont_free(usedfont); usedfont = (*drw).fonts }
            }
        }
    }
    if !d.is_null() { XftDrawDestroy(d); }
    return (x as
                libc::c_uint).wrapping_add(if 0 != render {
                                               w
                                           } else { 0i32 as libc::c_uint }) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn drw_font_getexts(mut font: *mut Fnt,
                                          mut text: *const libc::c_char,
                                          mut len: libc::c_uint,
                                          mut w: *mut libc::c_uint,
                                          mut h: *mut libc::c_uint) {
    let mut ext: XGlyphInfo =
        _XGlyphInfo{width: 0, height: 0, x: 0, y: 0, xOff: 0, yOff: 0,};
    if font.is_null() || text.is_null() { return }
    XftTextExtentsUtf8((*font).dpy, (*font).xfont, text as *mut XftChar8,
                       len as libc::c_int, &mut ext);
    if !w.is_null() { *w = ext.xOff as libc::c_uint }
    if !h.is_null() { *h = (*font).h };
}
unsafe extern "C" fn utf8decode(mut c: *const libc::c_char,
                                mut u: *mut libc::c_long, mut clen: size_t)
 -> size_t {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut len: size_t = 0;
    let mut type_0: size_t = 0;
    let mut udecoded: libc::c_long = 0;
    *u = 0xfffdi32 as libc::c_long;
    if 0 == clen { return 0i32 as size_t }
    udecoded = utf8decodebyte(*c.offset(0isize), &mut len);
    if !(1i32 as libc::c_ulong <= len && len <= 4i32 as libc::c_ulong) {
        return 1i32 as size_t
    }
    i = 1i32 as size_t;
    j = 1i32 as size_t;
    while i < clen && j < len {
        udecoded =
            udecoded << 6i32 |
                utf8decodebyte(*c.offset(i as isize), &mut type_0);
        if 0 != type_0 { return j }
        i = i.wrapping_add(1);
        j = j.wrapping_add(1)
    }
    if j < len { return 0i32 as size_t }
    *u = udecoded;
    utf8validate(u, len);
    return len;
}
unsafe extern "C" fn utf8validate(mut u: *mut libc::c_long, mut i: size_t)
 -> size_t {
    if !(utfmin[i as usize] <= *u && *u <= utfmax[i as usize]) ||
           0xd800i32 as libc::c_long <= *u && *u <= 0xdfffi32 as libc::c_long
       {
        *u = 0xfffdi32 as libc::c_long
    }
    i = 1i32 as size_t;
    while *u > utfmax[i as usize] { i = i.wrapping_add(1) }
    return i;
}
static mut utfmax: [libc::c_long; 5] =
    [0x10ffffi32 as libc::c_long, 0x7fi32 as libc::c_long,
     0x7ffi32 as libc::c_long, 0xffffi32 as libc::c_long,
     0x10ffffi32 as libc::c_long];
static mut utfmin: [libc::c_long; 5] =
    [0i32 as libc::c_long, 0i32 as libc::c_long, 0x80i32 as libc::c_long,
     0x800i32 as libc::c_long, 0x10000i32 as libc::c_long];
unsafe extern "C" fn utf8decodebyte(c: libc::c_char, mut i: *mut size_t)
 -> libc::c_long {
    *i = 0i32 as size_t;
    while *i < (4i32 + 1i32) as libc::c_ulong {
        if c as libc::c_uchar as libc::c_int &
               utfmask[*i as usize] as libc::c_int ==
               utfbyte[*i as usize] as libc::c_int {
            return (c as libc::c_uchar as libc::c_int &
                        !(utfmask[*i as usize] as libc::c_int)) as
                       libc::c_long
        }
        *i = (*i).wrapping_add(1)
    }
    return 0i32 as libc::c_long;
}
static mut utfmask: [libc::c_uchar; 5] =
    [0xc0i32 as libc::c_uchar, 0x80i32 as libc::c_uchar,
     0xe0i32 as libc::c_uchar, 0xf0i32 as libc::c_uchar,
     0xf8i32 as libc::c_uchar];
/* See LICENSE file for copyright and license details. */
static mut utfbyte: [libc::c_uchar; 5] =
    [0x80i32 as libc::c_uchar, 0i32 as libc::c_uchar,
     0xc0i32 as libc::c_uchar, 0xe0i32 as libc::c_uchar,
     0xf0i32 as libc::c_uchar];
/* Colorscheme abstraction */
#[no_mangle]
pub unsafe extern "C" fn drw_clr_create(mut drw: *mut Drw, mut dest: *mut Clr,
                                        mut clrname: *const libc::c_char) {
    if drw.is_null() || dest.is_null() || clrname.is_null() { return }
    if 0 ==
           XftColorAllocName((*drw).dpy,
                             (*(*((*drw).dpy as
                                      _XPrivDisplay)).screens.offset((*drw).screen
                                                                         as
                                                                         isize)).root_visual,
                             (*(*((*drw).dpy as
                                      _XPrivDisplay)).screens.offset((*drw).screen
                                                                         as
                                                                         isize)).cmap,
                             clrname, dest) {
        die(b"error, cannot allocate color \'%s\'\x00" as *const u8 as
                *const libc::c_char, clrname);
    };
}
#[no_mangle]
pub unsafe extern "C" fn drw_scm_create(mut drw: *mut Drw,
                                        mut clrnames:
                                            *mut *const libc::c_char,
                                        mut clrcount: size_t) -> *mut Clr {
    let mut i: size_t = 0;
    let mut ret: *mut Clr = 0 as *mut Clr;
    if drw.is_null() || clrnames.is_null() || clrcount < 2i32 as libc::c_ulong
           ||
           {
               ret =
                   ecalloc(clrcount,
                           ::std::mem::size_of::<XftColor>() as libc::c_ulong)
                       as *mut Clr;
               ret.is_null()
           } {
        return 0 as *mut Clr
    }
    i = 0i32 as size_t;
    while i < clrcount {
        drw_clr_create(drw, &mut *ret.offset(i as isize),
                       *clrnames.offset(i as isize));
        i = i.wrapping_add(1)
    }
    return ret;
}
/* Cursor abstraction */
#[no_mangle]
pub unsafe extern "C" fn drw_cur_create(mut drw: *mut Drw,
                                        mut shape: libc::c_int) -> *mut Cur {
    let mut cur: *mut Cur = 0 as *mut Cur;
    if drw.is_null() ||
           {
               cur =
                   ecalloc(1i32 as size_t,
                           ::std::mem::size_of::<Cur>() as libc::c_ulong) as
                       *mut Cur;
               cur.is_null()
           } {
        return 0 as *mut Cur
    }
    (*cur).cursor = XCreateFontCursor((*drw).dpy, shape as libc::c_uint);
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn drw_cur_free(mut drw: *mut Drw,
                                      mut cursor: *mut Cur) {
    if cursor.is_null() { return }
    XFreeCursor((*drw).dpy, (*cursor).cursor);
    free(cursor as *mut libc::c_void);
}
/* Drawing context manipulation */
#[no_mangle]
pub unsafe extern "C" fn drw_setfontset(mut drw: *mut Drw,
                                        mut set: *mut Fnt) {
    if !drw.is_null() { (*drw).fonts = set };
}
#[no_mangle]
pub unsafe extern "C" fn drw_setscheme(mut drw: *mut Drw, mut scm: *mut Clr) {
    if !drw.is_null() { (*drw).scheme = scm };
}
/* Drawing functions */
#[no_mangle]
pub unsafe extern "C" fn drw_rect(mut drw: *mut Drw, mut x: libc::c_int,
                                  mut y: libc::c_int, mut w: libc::c_uint,
                                  mut h: libc::c_uint,
                                  mut filled: libc::c_int,
                                  mut invert: libc::c_int) {
    if drw.is_null() || (*drw).scheme.is_null() { return }
    XSetForeground((*drw).dpy, (*drw).gc,
                   if 0 != invert {
                       (*(*drw).scheme.offset(ColBg as libc::c_int as
                                                  isize)).pixel
                   } else {
                       (*(*drw).scheme.offset(ColFg as libc::c_int as
                                                  isize)).pixel
                   });
    if 0 != filled {
        XFillRectangle((*drw).dpy, (*drw).drawable, (*drw).gc, x, y, w, h);
    } else {
        XDrawRectangle((*drw).dpy, (*drw).drawable, (*drw).gc, x, y,
                       w.wrapping_sub(1i32 as libc::c_uint),
                       h.wrapping_sub(1i32 as libc::c_uint));
    };
}
/* Map functions */
#[no_mangle]
pub unsafe extern "C" fn drw_map(mut drw: *mut Drw, mut win: Window,
                                 mut x: libc::c_int, mut y: libc::c_int,
                                 mut w: libc::c_uint, mut h: libc::c_uint) {
    if drw.is_null() { return }
    XCopyArea((*drw).dpy, (*drw).drawable, win, (*drw).gc, x, y, w, h, x, y);
    XSync((*drw).dpy, 0i32);
}