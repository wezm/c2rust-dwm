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
    #[no_mangle]
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t)
     -> __sighandler_t;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn setsid() -> __pid_t;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
               __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn XGetModifierMapping(_: *mut Display) -> *mut XModifierKeymap;
    #[no_mangle]
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    #[no_mangle]
    fn XInternAtom(_: *mut Display, _: *const libc::c_char, _: libc::c_int)
     -> Atom;
    #[no_mangle]
    fn XCreateSimpleWindow(_: *mut Display, _: Window, _: libc::c_int,
                           _: libc::c_int, _: libc::c_uint, _: libc::c_uint,
                           _: libc::c_uint, _: libc::c_ulong,
                           _: libc::c_ulong) -> Window;
    #[no_mangle]
    fn XCreateWindow(_: *mut Display, _: Window, _: libc::c_int,
                     _: libc::c_int, _: libc::c_uint, _: libc::c_uint,
                     _: libc::c_uint, _: libc::c_int, _: libc::c_uint,
                     _: *mut Visual, _: libc::c_ulong,
                     _: *mut XSetWindowAttributes) -> Window;
    #[no_mangle]
    fn XKeycodeToKeysym(_: *mut Display, _: KeyCode, _: libc::c_int)
     -> KeySym;
    #[no_mangle]
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
    #[no_mangle]
    fn XGetWMProtocols(_: *mut Display, _: Window, _: *mut *mut Atom,
                       _: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn XFreeStringList(_: *mut *mut libc::c_char);
    #[no_mangle]
    fn XAllowEvents(_: *mut Display, _: libc::c_int, _: Time) -> libc::c_int;
    #[no_mangle]
    fn XChangeProperty(_: *mut Display, _: Window, _: Atom, _: Atom,
                       _: libc::c_int, _: libc::c_int,
                       _: *const libc::c_uchar, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn XChangeWindowAttributes(_: *mut Display, _: Window, _: libc::c_ulong,
                               _: *mut XSetWindowAttributes) -> libc::c_int;
    #[no_mangle]
    fn XCheckMaskEvent(_: *mut Display, _: libc::c_long, _: *mut XEvent)
     -> libc::c_int;
    #[no_mangle]
    fn XCloseDisplay(_: *mut Display) -> libc::c_int;
    #[no_mangle]
    fn XConfigureWindow(_: *mut Display, _: Window, _: libc::c_uint,
                        _: *mut XWindowChanges) -> libc::c_int;
    #[no_mangle]
    fn XDefineCursor(_: *mut Display, _: Window, _: Cursor) -> libc::c_int;
    #[no_mangle]
    fn XDeleteProperty(_: *mut Display, _: Window, _: Atom) -> libc::c_int;
    #[no_mangle]
    fn XDestroyWindow(_: *mut Display, _: Window) -> libc::c_int;
    #[no_mangle]
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn XFreeModifiermap(_: *mut XModifierKeymap) -> libc::c_int;
    #[no_mangle]
    fn XGetTransientForHint(_: *mut Display, _: Window, _: *mut Window)
     -> libc::c_int;
    #[no_mangle]
    fn XGetWindowProperty(_: *mut Display, _: Window, _: Atom,
                          _: libc::c_long, _: libc::c_long, _: libc::c_int,
                          _: Atom, _: *mut Atom, _: *mut libc::c_int,
                          _: *mut libc::c_ulong, _: *mut libc::c_ulong,
                          _: *mut *mut libc::c_uchar) -> libc::c_int;
    #[no_mangle]
    fn XGetWindowAttributes(_: *mut Display, _: Window,
                            _: *mut XWindowAttributes) -> libc::c_int;
    #[no_mangle]
    fn XGrabButton(_: *mut Display, _: libc::c_uint, _: libc::c_uint,
                   _: Window, _: libc::c_int, _: libc::c_uint, _: libc::c_int,
                   _: libc::c_int, _: Window, _: Cursor) -> libc::c_int;
    #[no_mangle]
    fn XGrabKey(_: *mut Display, _: libc::c_int, _: libc::c_uint, _: Window,
                _: libc::c_int, _: libc::c_int, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn XGrabPointer(_: *mut Display, _: Window, _: libc::c_int,
                    _: libc::c_uint, _: libc::c_int, _: libc::c_int,
                    _: Window, _: Cursor, _: Time) -> libc::c_int;
    #[no_mangle]
    fn XGrabServer(_: *mut Display) -> libc::c_int;
    #[no_mangle]
    fn XKeysymToKeycode(_: *mut Display, _: KeySym) -> KeyCode;
    #[no_mangle]
    fn XKillClient(_: *mut Display, _: XID) -> libc::c_int;
    #[no_mangle]
    fn XMapRaised(_: *mut Display, _: Window) -> libc::c_int;
    #[no_mangle]
    fn XMapWindow(_: *mut Display, _: Window) -> libc::c_int;
    #[no_mangle]
    fn XMaskEvent(_: *mut Display, _: libc::c_long, _: *mut XEvent)
     -> libc::c_int;
    #[no_mangle]
    fn XMoveResizeWindow(_: *mut Display, _: Window, _: libc::c_int,
                         _: libc::c_int, _: libc::c_uint, _: libc::c_uint)
     -> libc::c_int;
    #[no_mangle]
    fn XMoveWindow(_: *mut Display, _: Window, _: libc::c_int, _: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> libc::c_int;
    #[no_mangle]
    fn XQueryPointer(_: *mut Display, _: Window, _: *mut Window,
                     _: *mut Window, _: *mut libc::c_int, _: *mut libc::c_int,
                     _: *mut libc::c_int, _: *mut libc::c_int,
                     _: *mut libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn XQueryTree(_: *mut Display, _: Window, _: *mut Window, _: *mut Window,
                  _: *mut *mut Window, _: *mut libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn XRaiseWindow(_: *mut Display, _: Window) -> libc::c_int;
    #[no_mangle]
    fn XRefreshKeyboardMapping(_: *mut XMappingEvent) -> libc::c_int;
    #[no_mangle]
    fn XSelectInput(_: *mut Display, _: Window, _: libc::c_long)
     -> libc::c_int;
    #[no_mangle]
    fn XSendEvent(_: *mut Display, _: Window, _: libc::c_int, _: libc::c_long,
                  _: *mut XEvent) -> libc::c_int;
    #[no_mangle]
    fn XSetCloseDownMode(_: *mut Display, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn XSetInputFocus(_: *mut Display, _: Window, _: libc::c_int, _: Time)
     -> libc::c_int;
    #[no_mangle]
    fn XSetWindowBorder(_: *mut Display, _: Window, _: libc::c_ulong)
     -> libc::c_int;
    #[no_mangle]
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn XUngrabButton(_: *mut Display, _: libc::c_uint, _: libc::c_uint,
                     _: Window) -> libc::c_int;
    #[no_mangle]
    fn XUngrabKey(_: *mut Display, _: libc::c_int, _: libc::c_uint, _: Window)
     -> libc::c_int;
    #[no_mangle]
    fn XUngrabPointer(_: *mut Display, _: Time) -> libc::c_int;
    #[no_mangle]
    fn XUngrabServer(_: *mut Display) -> libc::c_int;
    #[no_mangle]
    fn XUnmapWindow(_: *mut Display, _: Window) -> libc::c_int;
    #[no_mangle]
    fn XWarpPointer(_: *mut Display, _: Window, _: Window, _: libc::c_int,
                    _: libc::c_int, _: libc::c_uint, _: libc::c_uint,
                    _: libc::c_int, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn XSupportsLocale() -> libc::c_int;
    #[no_mangle]
    fn XGetClassHint(_: *mut Display, _: Window, _: *mut XClassHint)
     -> libc::c_int;
    #[no_mangle]
    fn XGetTextProperty(_: *mut Display, _: Window, _: *mut XTextProperty,
                        _: Atom) -> libc::c_int;
    #[no_mangle]
    fn XGetWMHints(_: *mut Display, _: Window) -> *mut XWMHints;
    #[no_mangle]
    fn XGetWMNormalHints(_: *mut Display, _: Window, _: *mut XSizeHints,
                         _: *mut libc::c_long) -> libc::c_int;
    #[no_mangle]
    fn XSetClassHint(_: *mut Display, _: Window, _: *mut XClassHint)
     -> libc::c_int;
    #[no_mangle]
    fn XSetWMHints(_: *mut Display, _: Window, _: *mut XWMHints)
     -> libc::c_int;
    #[no_mangle]
    fn XmbTextPropertyToTextList(display: *mut Display,
                                 text_prop: *const XTextProperty,
                                 list_return: *mut *mut *mut libc::c_char,
                                 count_return: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn XineramaIsActive(dpy_0: *mut Display) -> libc::c_int;
    #[no_mangle]
    fn XineramaQueryScreens(dpy_0: *mut Display, number: *mut libc::c_int)
     -> *mut XineramaScreenInfo;
    /* Drawable abstraction */
    #[no_mangle]
    fn drw_create(dpy_0: *mut Display, screen_0: libc::c_int, win: Window,
                  w: libc::c_uint, h: libc::c_uint) -> *mut Drw;
    #[no_mangle]
    fn drw_resize(drw_0: *mut Drw, w: libc::c_uint, h: libc::c_uint);
    #[no_mangle]
    fn drw_free(drw_0: *mut Drw);
    /* Fnt abstraction */
    #[no_mangle]
    fn drw_fontset_create(drw_0: *mut Drw, fonts_0: *mut *const libc::c_char,
                          fontcount: size_t) -> *mut Fnt;
    #[no_mangle]
    fn drw_fontset_getwidth(drw_0: *mut Drw, text: *const libc::c_char)
     -> libc::c_uint;
    #[no_mangle]
    fn drw_scm_create(drw_0: *mut Drw, clrnames: *mut *const libc::c_char,
                      clrcount: size_t) -> *mut Clr;
    /* Cursor abstraction */
    #[no_mangle]
    fn drw_cur_create(drw_0: *mut Drw, shape: libc::c_int) -> *mut Cur;
    #[no_mangle]
    fn drw_cur_free(drw_0: *mut Drw, cursor_0: *mut Cur);
    #[no_mangle]
    fn drw_setscheme(drw_0: *mut Drw, scm: *mut Clr);
    /* Drawing functions */
    #[no_mangle]
    fn drw_rect(drw_0: *mut Drw, x: libc::c_int, y: libc::c_int,
                w: libc::c_uint, h: libc::c_uint, filled: libc::c_int,
                invert: libc::c_int);
    #[no_mangle]
    fn drw_text(drw_0: *mut Drw, x: libc::c_int, y: libc::c_int,
                w: libc::c_uint, h: libc::c_uint, lpad: libc::c_uint,
                text: *const libc::c_char, invert: libc::c_int)
     -> libc::c_int;
    /* Map functions */
    #[no_mangle]
    fn drw_map(drw_0: *mut Drw, win: Window, x: libc::c_int, y: libc::c_int,
               w: libc::c_uint, h: libc::c_uint);
    /* See LICENSE file for copyright and license details. */
    #[no_mangle]
    fn die(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn ecalloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type size_t = libc::c_ulong;
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
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type KeySym = XID;
pub type KeyCode = libc::c_uchar;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: libc::c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: libc::c_ulong,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XWindowAttributes {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub depth: libc::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: libc::c_int,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub colormap: Colormap,
    pub map_installed: libc::c_int,
    pub map_state: libc::c_int,
    pub all_event_masks: libc::c_long,
    pub your_event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub screen: *mut Screen,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XWindowChanges {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub sibling: Window,
    pub stack_mode: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XModifierKeymap {
    pub max_keypermod: libc::c_int,
    pub modifiermap: *mut KeyCode,
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
pub type XButtonPressedEvent = XButtonEvent;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: unnamed_0,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union unnamed_0 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
pub type XEvent = _XEvent;
pub type XErrorHandler
    =
    Option<unsafe extern "C" fn(_: *mut Display, _: *mut XErrorEvent)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XSizeHints {
    pub flags: libc::c_long,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub min_width: libc::c_int,
    pub min_height: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
    pub width_inc: libc::c_int,
    pub height_inc: libc::c_int,
    pub min_aspect: unnamed_1,
    pub max_aspect: unnamed_1,
    pub base_width: libc::c_int,
    pub base_height: libc::c_int,
    pub win_gravity: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct unnamed_1 {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XWMHints {
    pub flags: libc::c_long,
    pub input: libc::c_int,
    pub initial_state: libc::c_int,
    pub icon_pixmap: Pixmap,
    pub icon_window: Window,
    pub icon_x: libc::c_int,
    pub icon_y: libc::c_int,
    pub icon_mask: Pixmap,
    pub window_group: XID,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XTextProperty {
    pub value: *mut libc::c_uchar,
    pub encoding: Atom,
    pub format: libc::c_int,
    pub nitems: libc::c_ulong,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XClassHint {
    pub res_name: *mut libc::c_char,
    pub res_class: *mut libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct XineramaScreenInfo {
    pub screen_number: libc::c_int,
    pub x_org: libc::c_short,
    pub y_org: libc::c_short,
    pub width: libc::c_short,
    pub height: libc::c_short,
}
pub type FcCharSet = _FcCharSet;
pub type FcPattern = _FcPattern;
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
pub struct _XftFont {
    pub ascent: libc::c_int,
    pub descent: libc::c_int,
    pub height: libc::c_int,
    pub max_advance_width: libc::c_int,
    pub charset: *mut FcCharSet,
    pub pattern: *mut FcPattern,
}
pub type XftFont = _XftFont;
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
pub type unnamed_2 = libc::c_uint;
/* Clr scheme index */
pub const ColBorder: unnamed_2 = 2;
pub const ColBg: unnamed_2 = 1;
pub const ColFg: unnamed_2 = 0;
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
/* See LICENSE file for copyright and license details.
 *
 * dynamic window manager is designed like any other X client as well. It is
 * driven through handling X events. In contrast to other X clients, a window
 * manager selects for SubstructureRedirectMask on the root window, to receive
 * events about window (dis-)appearance. Only one X connection at a time is
 * allowed to select for this event mask.
 *
 * The event handlers of dwm are organized in an array which is accessed
 * whenever a new event has been fetched. This allows event dispatching
 * in O(1) time.
 *
 * Each child of the root window is called a client, except windows which have
 * set the override_redirect flag. Clients are organized in a linked client
 * list on each monitor, the focus history is remembered through a stack list
 * on each monitor. Each client contains a bit array to indicate the tags of a
 * client.
 *
 * Keys and tagging rules are organized as arrays and defined in config.h.
 *
 * To understand everything else, start reading main().
 */
/* XINERAMA */
/* macros */
/* enums */
pub type unnamed_3 = libc::c_uint;
/* cursor */
pub const CurLast: unnamed_3 = 3;
pub const CurMove: unnamed_3 = 2;
pub const CurResize: unnamed_3 = 1;
pub const CurNormal: unnamed_3 = 0;
pub type unnamed_4 = libc::c_uint;
/* color schemes */
pub const SchemeSel: unnamed_4 = 1;
pub const SchemeNorm: unnamed_4 = 0;
pub type unnamed_5 = libc::c_uint;
/* EWMH atoms */
pub const NetLast: unnamed_5 = 9;
pub const NetClientList: unnamed_5 = 8;
pub const NetWMWindowTypeDialog: unnamed_5 = 7;
pub const NetWMWindowType: unnamed_5 = 6;
pub const NetActiveWindow: unnamed_5 = 5;
pub const NetWMFullscreen: unnamed_5 = 4;
pub const NetWMCheck: unnamed_5 = 3;
pub const NetWMState: unnamed_5 = 2;
pub const NetWMName: unnamed_5 = 1;
pub const NetSupported: unnamed_5 = 0;
pub type unnamed_6 = libc::c_uint;
/* default atoms */
pub const WMLast: unnamed_6 = 4;
pub const WMTakeFocus: unnamed_6 = 3;
pub const WMState: unnamed_6 = 2;
pub const WMDelete: unnamed_6 = 1;
pub const WMProtocols: unnamed_6 = 0;
pub type unnamed_7 = libc::c_uint;
/* clicks */
pub const ClkLast: unnamed_7 = 6;
pub const ClkRootWin: unnamed_7 = 5;
pub const ClkClientWin: unnamed_7 = 4;
pub const ClkWinTitle: unnamed_7 = 3;
pub const ClkStatusText: unnamed_7 = 2;
pub const ClkLtSymbol: unnamed_7 = 1;
pub const ClkTagBar: unnamed_7 = 0;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union Arg {
    pub i: libc::c_int,
    pub ui: libc::c_uint,
    pub f: libc::c_float,
    pub v: *const libc::c_void,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Button {
    pub click: libc::c_uint,
    pub mask: libc::c_uint,
    pub button: libc::c_uint,
    pub func: Option<unsafe extern "C" fn(_: *const Arg) -> ()>,
    pub arg: Arg,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Monitor {
    pub ltsymbol: [libc::c_char; 16],
    pub mfact: libc::c_float,
    pub nmaster: libc::c_int,
    pub num: libc::c_int,
    pub by: libc::c_int,
    pub mx: libc::c_int,
    pub my: libc::c_int,
    pub mw: libc::c_int,
    pub mh: libc::c_int,
    pub wx: libc::c_int,
    pub wy: libc::c_int,
    pub ww: libc::c_int,
    pub wh: libc::c_int,
    pub seltags: libc::c_uint,
    pub sellt: libc::c_uint,
    pub tagset: [libc::c_uint; 2],
    pub showbar: libc::c_int,
    pub topbar: libc::c_int,
    pub clients: *mut Client,
    pub sel: *mut Client,
    pub stack: *mut Client,
    pub next: *mut Monitor,
    pub barwin: Window,
    pub lt: [*const Layout; 2],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Layout {
    pub symbol: *const libc::c_char,
    pub arrange: Option<unsafe extern "C" fn(_: *mut Monitor) -> ()>,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Client {
    pub name: [libc::c_char; 256],
    pub mina: libc::c_float,
    pub maxa: libc::c_float,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub oldx: libc::c_int,
    pub oldy: libc::c_int,
    pub oldw: libc::c_int,
    pub oldh: libc::c_int,
    pub basew: libc::c_int,
    pub baseh: libc::c_int,
    pub incw: libc::c_int,
    pub inch: libc::c_int,
    pub maxw: libc::c_int,
    pub maxh: libc::c_int,
    pub minw: libc::c_int,
    pub minh: libc::c_int,
    pub bw: libc::c_int,
    pub oldbw: libc::c_int,
    pub tags: libc::c_uint,
    pub isfixed: libc::c_int,
    pub isfloating: libc::c_int,
    pub isurgent: libc::c_int,
    pub neverfocus: libc::c_int,
    pub oldstate: libc::c_int,
    pub isfullscreen: libc::c_int,
    pub next: *mut Client,
    pub snext: *mut Client,
    pub mon: *mut Monitor,
    pub win: Window,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Key {
    pub mod_0: libc::c_uint,
    pub keysym: KeySym,
    pub func: Option<unsafe extern "C" fn(_: *const Arg) -> ()>,
    pub arg: Arg,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Rule {
    pub class: *const libc::c_char,
    pub instance: *const libc::c_char,
    pub title: *const libc::c_char,
    pub tags: libc::c_uint,
    pub isfloating: libc::c_int,
    pub monitor: libc::c_int,
}
/* function declarations */
unsafe extern "C" fn applyrules(mut c: *mut Client) {
    let mut class: *const libc::c_char = 0 as *const libc::c_char;
    let mut instance: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    let mut r: *const Rule = 0 as *const Rule;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut ch: XClassHint =
        XClassHint{res_name: 0 as *mut libc::c_char,
                   res_class: 0 as *mut libc::c_char,};
    (*c).isfloating = 0i32;
    (*c).tags = 0i32 as libc::c_uint;
    XGetClassHint(dpy, (*c).win, &mut ch);
    class =
        if !ch.res_class.is_null() {
            ch.res_class as *const libc::c_char
        } else { broken.as_ptr() };
    instance =
        if !ch.res_name.is_null() {
            ch.res_name as *const libc::c_char
        } else { broken.as_ptr() };
    i = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[Rule; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<Rule>()
                                                   as libc::c_ulong) {
        r = &*rules.as_ptr().offset(i as isize) as *const Rule;
        if ((*r).title.is_null() ||
                !strstr((*c).name.as_mut_ptr(), (*r).title).is_null()) &&
               ((*r).class.is_null() || !strstr(class, (*r).class).is_null())
               &&
               ((*r).instance.is_null() ||
                    !strstr(instance, (*r).instance).is_null()) {
            (*c).isfloating = (*r).isfloating;
            (*c).tags |= (*r).tags;
            m = mons;
            while !m.is_null() && (*m).num != (*r).monitor { m = (*m).next }
            if !m.is_null() { (*c).mon = m }
        }
        i = i.wrapping_add(1)
    }
    if !ch.res_class.is_null() { XFree(ch.res_class as *mut libc::c_void); }
    if !ch.res_name.is_null() { XFree(ch.res_name as *mut libc::c_void); }
    (*c).tags =
        if 0 !=
               (*c).tags &
                   ((1i32 <<
                         (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                              as
                                                              libc::c_ulong))
                        - 1i32) as libc::c_uint {
            (*c).tags &
                ((1i32 <<
                      (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                           as libc::c_ulong))
                     - 1i32) as libc::c_uint
        } else { (*(*c).mon).tagset[(*(*c).mon).seltags as usize] };
}
/*               fg         bg         border   */
/* tagging */
static mut tags: [*const libc::c_char; 9] =
    [b"1\x00" as *const u8 as *const libc::c_char,
     b"2\x00" as *const u8 as *const libc::c_char,
     b"3\x00" as *const u8 as *const libc::c_char,
     b"4\x00" as *const u8 as *const libc::c_char,
     b"5\x00" as *const u8 as *const libc::c_char,
     b"6\x00" as *const u8 as *const libc::c_char,
     b"7\x00" as *const u8 as *const libc::c_char,
     b"8\x00" as *const u8 as *const libc::c_char,
     b"9\x00" as *const u8 as *const libc::c_char];
static mut mons: *mut Monitor = 0 as *const Monitor as *mut Monitor;
static mut rules: [Rule; 2] =
    [Rule{class: b"Gimp\x00" as *const u8 as *const libc::c_char,
          instance: 0 as *const libc::c_char,
          title: 0 as *const libc::c_char,
          tags: 0i32 as libc::c_uint,
          isfloating: 1i32,
          monitor: -1i32,},
     Rule{class: b"Firefox\x00" as *const u8 as *const libc::c_char,
          instance: 0 as *const libc::c_char,
          title: 0 as *const libc::c_char,
          tags: (1i32 << 8i32) as libc::c_uint,
          isfloating: 0i32,
          monitor: -1i32,}];
/* variables */
static mut broken: [libc::c_char; 7] = [98, 114, 111, 107, 101, 110, 0];
static mut dpy: *mut Display = 0 as *const Display as *mut Display;
unsafe extern "C" fn applysizehints(mut c: *mut Client,
                                    mut x: *mut libc::c_int,
                                    mut y: *mut libc::c_int,
                                    mut w: *mut libc::c_int,
                                    mut h: *mut libc::c_int,
                                    mut interact: libc::c_int)
 -> libc::c_int {
    let mut baseismin: libc::c_int = 0;
    let mut m: *mut Monitor = (*c).mon;
    *w = if 1i32 > *w { 1i32 } else { *w };
    *h = if 1i32 > *h { 1i32 } else { *h };
    if 0 != interact {
        if *x > sw { *x = sw - ((*c).w + 2i32 * (*c).bw) }
        if *y > sh { *y = sh - ((*c).h + 2i32 * (*c).bw) }
        if *x + *w + 2i32 * (*c).bw < 0i32 { *x = 0i32 }
        if *y + *h + 2i32 * (*c).bw < 0i32 { *y = 0i32 }
    } else {
        if *x >= (*m).wx + (*m).ww {
            *x = (*m).wx + (*m).ww - ((*c).w + 2i32 * (*c).bw)
        }
        if *y >= (*m).wy + (*m).wh {
            *y = (*m).wy + (*m).wh - ((*c).h + 2i32 * (*c).bw)
        }
        if *x + *w + 2i32 * (*c).bw <= (*m).wx { *x = (*m).wx }
        if *y + *h + 2i32 * (*c).bw <= (*m).wy { *y = (*m).wy }
    }
    if *h < bh { *h = bh }
    if *w < bh { *w = bh }
    if 0 != resizehints || 0 != (*c).isfloating ||
           (*(*(*c).mon).lt[(*(*c).mon).sellt as usize]).arrange.is_none() {
        baseismin =
            ((*c).basew == (*c).minw && (*c).baseh == (*c).minh) as
                libc::c_int;
        if 0 == baseismin { *w -= (*c).basew; *h -= (*c).baseh }
        if (*c).mina > 0i32 as libc::c_float &&
               (*c).maxa > 0i32 as libc::c_float {
            if (*c).maxa < *w as libc::c_float / *h as libc::c_float {
                *w =
                    ((*h as libc::c_float * (*c).maxa) as libc::c_double +
                         0.5f64) as libc::c_int
            } else if (*c).mina < *h as libc::c_float / *w as libc::c_float {
                *h =
                    ((*w as libc::c_float * (*c).mina) as libc::c_double +
                         0.5f64) as libc::c_int
            }
        }
        if 0 != baseismin { *w -= (*c).basew; *h -= (*c).baseh }
        if 0 != (*c).incw { *w -= *w % (*c).incw }
        if 0 != (*c).inch { *h -= *h % (*c).inch }
        *w =
            if *w + (*c).basew > (*c).minw {
                *w + (*c).basew
            } else { (*c).minw };
        *h =
            if *h + (*c).baseh > (*c).minh {
                *h + (*c).baseh
            } else { (*c).minh };
        if 0 != (*c).maxw { *w = if *w < (*c).maxw { *w } else { (*c).maxw } }
        if 0 != (*c).maxh { *h = if *h < (*c).maxh { *h } else { (*c).maxh } }
    }
    return (*x != (*c).x || *y != (*c).y || *w != (*c).w || *h != (*c).h) as
               libc::c_int;
}
/* 1 means respect size hints in tiled resizals */
static mut resizehints: libc::c_int = 1i32;
static mut bh: libc::c_int = 0;
/* X display screen geometry width, height */
static mut sh: libc::c_int = 0;
static mut sw: libc::c_int = 0;
unsafe extern "C" fn arrange(mut m: *mut Monitor) {
    if !m.is_null() {
        showhide((*m).stack);
    } else {
        m = mons;
        while !m.is_null() { showhide((*m).stack); m = (*m).next }
    }
    if !m.is_null() {
        arrangemon(m);
        restack(m);
    } else { m = mons; while !m.is_null() { arrangemon(m); m = (*m).next } };
}
unsafe extern "C" fn arrangemon(mut m: *mut Monitor) {
    strncpy((*m).ltsymbol.as_mut_ptr(),
            (*(*m).lt[(*m).sellt as usize]).symbol,
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong);
    if (*(*m).lt[(*m).sellt as usize]).arrange.is_some() {
        (*(*m).lt[(*m).sellt as
                      usize]).arrange.expect("non-null function pointer")(m);
    };
}
unsafe extern "C" fn restack(mut m: *mut Monitor) {
    let mut c: *mut Client = 0 as *mut Client;
    let mut ev: XEvent = _XEvent{type_0: 0,};
    let mut wc: XWindowChanges =
        XWindowChanges{x: 0,
                       y: 0,
                       width: 0,
                       height: 0,
                       border_width: 0,
                       sibling: 0,
                       stack_mode: 0,};
    drawbar(m);
    if (*m).sel.is_null() { return }
    if 0 != (*(*m).sel).isfloating ||
           (*(*m).lt[(*m).sellt as usize]).arrange.is_none() {
        XRaiseWindow(dpy, (*(*m).sel).win);
    }
    if (*(*m).lt[(*m).sellt as usize]).arrange.is_some() {
        wc.stack_mode = 1i32;
        wc.sibling = (*m).barwin;
        c = (*m).stack;
        while !c.is_null() {
            if 0 == (*c).isfloating &&
                   0 !=
                       (*c).tags &
                           (*(*c).mon).tagset[(*(*c).mon).seltags as usize] {
                XConfigureWindow(dpy, (*c).win,
                                 (1i32 << 5i32 | 1i32 << 6i32) as
                                     libc::c_uint, &mut wc);
                wc.sibling = (*c).win
            }
            c = (*c).snext
        }
    }
    XSync(dpy, 0i32);
    while 0 != XCheckMaskEvent(dpy, 1i64 << 4i32, &mut ev) { };
}
unsafe extern "C" fn drawbar(mut m: *mut Monitor) {
    let mut x: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut sw_0: libc::c_int = 0i32;
    let mut boxs: libc::c_int =
        (*(*drw).fonts).h.wrapping_div(9i32 as libc::c_uint) as libc::c_int;
    let mut boxw: libc::c_int =
        (*(*drw).fonts).h.wrapping_div(6i32 as
                                           libc::c_uint).wrapping_add(2i32 as
                                                                          libc::c_uint)
            as libc::c_int;
    let mut i: libc::c_uint = 0;
    let mut occ: libc::c_uint = 0i32 as libc::c_uint;
    let mut urg: libc::c_uint = 0i32 as libc::c_uint;
    let mut c: *mut Client = 0 as *mut Client;
    if m == selmon {
        drw_setscheme(drw,
                      *scheme.offset(SchemeNorm as libc::c_int as isize));
        sw_0 =
            drw_fontset_getwidth(drw,
                                 stext.as_mut_ptr()).wrapping_add(lrpad as
                                                                      libc::c_uint).wrapping_sub(lrpad
                                                                                                     as
                                                                                                     libc::c_uint).wrapping_add(2i32
                                                                                                                                    as
                                                                                                                                    libc::c_uint)
                as libc::c_int;
        drw_text(drw, (*m).ww - sw_0, 0i32, sw_0 as libc::c_uint,
                 bh as libc::c_uint, 0i32 as libc::c_uint, stext.as_mut_ptr(),
                 0i32);
    }
    c = (*m).clients;
    while !c.is_null() {
        occ |= (*c).tags;
        if 0 != (*c).isurgent { urg |= (*c).tags }
        c = (*c).next
    }
    x = 0i32;
    i = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        w =
            drw_fontset_getwidth(drw,
                                 tags[i as
                                          usize]).wrapping_add(lrpad as
                                                                   libc::c_uint)
                as libc::c_int;
        drw_setscheme(drw,
                      *scheme.offset((if 0 !=
                                             (*m).tagset[(*m).seltags as
                                                             usize] &
                                                 (1i32 << i) as libc::c_uint {
                                          SchemeSel as libc::c_int
                                      } else { SchemeNorm as libc::c_int }) as
                                         isize));
        drw_text(drw, x, 0i32, w as libc::c_uint, bh as libc::c_uint,
                 (lrpad / 2i32) as libc::c_uint, tags[i as usize],
                 (urg & (1i32 << i) as libc::c_uint) as libc::c_int);
        if 0 != occ & (1i32 << i) as libc::c_uint {
            drw_rect(drw, x + boxs, boxs, boxw as libc::c_uint,
                     boxw as libc::c_uint,
                     (m == selmon && !(*selmon).sel.is_null() &&
                          0 !=
                              (*(*selmon).sel).tags &
                                  (1i32 << i) as libc::c_uint) as libc::c_int,
                     (urg & (1i32 << i) as libc::c_uint) as libc::c_int);
        }
        x += w;
        i = i.wrapping_add(1)
    }
    blw =
        drw_fontset_getwidth(drw,
                             (*m).ltsymbol.as_mut_ptr()).wrapping_add(lrpad as
                                                                          libc::c_uint)
            as libc::c_int;
    w = blw;
    drw_setscheme(drw, *scheme.offset(SchemeNorm as libc::c_int as isize));
    x =
        drw_text(drw, x, 0i32, w as libc::c_uint, bh as libc::c_uint,
                 (lrpad / 2i32) as libc::c_uint, (*m).ltsymbol.as_mut_ptr(),
                 0i32);
    w = (*m).ww - sw_0 - x;
    if w > bh {
        if !(*m).sel.is_null() {
            drw_setscheme(drw,
                          *scheme.offset((if m == selmon {
                                              SchemeSel as libc::c_int
                                          } else {
                                              SchemeNorm as libc::c_int
                                          }) as isize));
            drw_text(drw, x, 0i32, w as libc::c_uint, bh as libc::c_uint,
                     (lrpad / 2i32) as libc::c_uint,
                     (*(*m).sel).name.as_mut_ptr(), 0i32);
            if 0 != (*(*m).sel).isfloating {
                drw_rect(drw, x + boxs, boxs, boxw as libc::c_uint,
                         boxw as libc::c_uint, (*(*m).sel).isfixed, 0i32);
            }
        } else {
            drw_setscheme(drw,
                          *scheme.offset(SchemeNorm as libc::c_int as isize));
            drw_rect(drw, x, 0i32, w as libc::c_uint, bh as libc::c_uint,
                     1i32, 1i32);
        }
    }
    drw_map(drw, (*m).barwin, 0i32, 0i32, (*m).ww as libc::c_uint,
            bh as libc::c_uint);
}
static mut drw: *mut Drw = 0 as *const Drw as *mut Drw;
static mut scheme: *mut *mut Clr = 0 as *const *mut Clr as *mut *mut Clr;
/* sum of left and right padding for text */
static mut lrpad: libc::c_int = 0;
static mut selmon: *mut Monitor = 0 as *const Monitor as *mut Monitor;
/* bar geometry */
static mut blw: libc::c_int = 0i32;
static mut stext: [libc::c_char; 256] = [0; 256];
unsafe extern "C" fn showhide(mut c: *mut Client) {
    if c.is_null() { return }
    if 0 != (*c).tags & (*(*c).mon).tagset[(*(*c).mon).seltags as usize] {
        XMoveWindow(dpy, (*c).win, (*c).x, (*c).y);
        if ((*(*(*c).mon).lt[(*(*c).mon).sellt as usize]).arrange.is_none() ||
                0 != (*c).isfloating) && 0 == (*c).isfullscreen {
            resize(c, (*c).x, (*c).y, (*c).w, (*c).h, 0i32);
        }
        showhide((*c).snext);
    } else {
        showhide((*c).snext);
        XMoveWindow(dpy, (*c).win, ((*c).w + 2i32 * (*c).bw) * -2i32, (*c).y);
    };
}
unsafe extern "C" fn resize(mut c: *mut Client, mut x: libc::c_int,
                            mut y: libc::c_int, mut w: libc::c_int,
                            mut h: libc::c_int, mut interact: libc::c_int) {
    if 0 != applysizehints(c, &mut x, &mut y, &mut w, &mut h, interact) {
        resizeclient(c, x, y, w, h);
    };
}
unsafe extern "C" fn resizeclient(mut c: *mut Client, mut x: libc::c_int,
                                  mut y: libc::c_int, mut w: libc::c_int,
                                  mut h: libc::c_int) {
    let mut wc: XWindowChanges =
        XWindowChanges{x: 0,
                       y: 0,
                       width: 0,
                       height: 0,
                       border_width: 0,
                       sibling: 0,
                       stack_mode: 0,};
    (*c).oldx = (*c).x;
    wc.x = x;
    (*c).x = wc.x;
    (*c).oldy = (*c).y;
    wc.y = y;
    (*c).y = wc.y;
    (*c).oldw = (*c).w;
    wc.width = w;
    (*c).w = wc.width;
    (*c).oldh = (*c).h;
    wc.height = h;
    (*c).h = wc.height;
    wc.border_width = (*c).bw;
    XConfigureWindow(dpy, (*c).win,
                     (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32 |
                          1i32 << 3i32 | 1i32 << 4i32) as libc::c_uint,
                     &mut wc);
    configure(c);
    XSync(dpy, 0i32);
}
unsafe extern "C" fn configure(mut c: *mut Client) {
    let mut ce: XConfigureEvent =
        XConfigureEvent{type_0: 0,
                        serial: 0,
                        send_event: 0,
                        display: 0 as *mut Display,
                        event: 0,
                        window: 0,
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                        border_width: 0,
                        above: 0,
                        override_redirect: 0,};
    ce.type_0 = 22i32;
    ce.display = dpy;
    ce.event = (*c).win;
    ce.window = (*c).win;
    ce.x = (*c).x;
    ce.y = (*c).y;
    ce.width = (*c).w;
    ce.height = (*c).h;
    ce.border_width = (*c).bw;
    ce.above = 0i64 as Window;
    ce.override_redirect = 0i32;
    XSendEvent(dpy, (*c).win, 0i32, 1i64 << 17i32,
               &mut ce as *mut XConfigureEvent as *mut XEvent);
}
unsafe extern "C" fn attach(mut c: *mut Client) {
    (*c).next = (*(*c).mon).clients;
    (*(*c).mon).clients = c;
}
unsafe extern "C" fn attachstack(mut c: *mut Client) {
    (*c).snext = (*(*c).mon).stack;
    (*(*c).mon).stack = c;
}
unsafe extern "C" fn buttonpress(mut e: *mut XEvent) {
    let mut i: libc::c_uint = 0;
    let mut x: libc::c_uint = 0;
    let mut click: libc::c_uint = 0;
    let mut arg: Arg = Arg{i: 0i32,};
    let mut c: *mut Client = 0 as *mut Client;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut ev: *mut XButtonPressedEvent = &mut (*e).xbutton;
    click = ClkRootWin as libc::c_int as libc::c_uint;
    m = wintomon((*ev).window);
    if !m.is_null() && m != selmon {
        unfocus((*selmon).sel, 1i32);
        selmon = m;
        focus(0 as *mut Client);
    }
    if (*ev).window == (*selmon).barwin {
        x = 0i32 as libc::c_uint;
        i = x;
        loop  {
            x =
                x.wrapping_add(drw_fontset_getwidth(drw,
                                                    tags[i as
                                                             usize]).wrapping_add(lrpad
                                                                                      as
                                                                                      libc::c_uint));
            if !((*ev).x as libc::c_uint >= x &&
                     {
                         i = i.wrapping_add(1);
                         (i as libc::c_ulong) <
                             (::std::mem::size_of::<[*const libc::c_char; 9]>()
                                  as
                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                                  as
                                                                  libc::c_ulong)
                     }) {
                break ;
            }
        }
        if (i as libc::c_ulong) <
               (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                    as libc::c_ulong) {
            click = ClkTagBar as libc::c_int as libc::c_uint;
            arg.ui = (1i32 << i) as libc::c_uint
        } else if ((*ev).x as libc::c_uint) <
                      x.wrapping_add(blw as libc::c_uint) {
            click = ClkLtSymbol as libc::c_int as libc::c_uint
        } else if (*ev).x as libc::c_uint >
                      ((*selmon).ww as
                           libc::c_uint).wrapping_sub(drw_fontset_getwidth(drw,
                                                                           stext.as_mut_ptr()).wrapping_add(lrpad
                                                                                                                as
                                                                                                                libc::c_uint))
         {
            click = ClkStatusText as libc::c_int as libc::c_uint
        } else { click = ClkWinTitle as libc::c_int as libc::c_uint }
    } else {
        c = wintoclient((*ev).window);
        if !c.is_null() {
            focus(c);
            restack(selmon);
            XAllowEvents(dpy, 2i32, 0i64 as Time);
            click = ClkClientWin as libc::c_int as libc::c_uint
        }
    }
    i = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[Button; 11]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<Button>()
                                                   as libc::c_ulong) {
        if click == buttons[i as usize].click &&
               buttons[i as usize].func.is_some() &&
               buttons[i as usize].button == (*ev).button &&
               buttons[i as usize].mask &
                   !(numlockmask | (1i32 << 1i32) as libc::c_uint) &
                   (1i32 << 0i32 | 1i32 << 2i32 | 1i32 << 3i32 | 1i32 << 4i32
                        | 1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) as
                       libc::c_uint ==
                   (*ev).state &
                       !(numlockmask | (1i32 << 1i32) as libc::c_uint) &
                       (1i32 << 0i32 | 1i32 << 2i32 | 1i32 << 3i32 |
                            1i32 << 4i32 | 1i32 << 5i32 | 1i32 << 6i32 |
                            1i32 << 7i32) as libc::c_uint {
            buttons[i as
                        usize].func.expect("non-null function pointer")(if click
                                                                               ==
                                                                               ClkTagBar
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                               &&
                                                                               buttons[i
                                                                                           as
                                                                                           usize].arg.i
                                                                                   ==
                                                                                   0i32
                                                                           {
                                                                            &mut arg
                                                                                as
                                                                                *mut Arg
                                                                                as
                                                                                *const Arg
                                                                        } else {
                                                                            &(*buttons.as_mut_ptr().offset(i
                                                                                                               as
                                                                                                               isize)).arg
                                                                        });
        }
        i = i.wrapping_add(1)
    };
}
// Initialized in run_static_initializers
static mut buttons: [Button; 11] =
    [Button{click: 0, mask: 0, button: 0, func: None, arg: Arg{i: 0,},}; 11];
unsafe extern "C" fn toggletag(mut arg: *const Arg) {
    let mut newtags: libc::c_uint = 0;
    if (*selmon).sel.is_null() { return }
    newtags =
        (*(*selmon).sel).tags ^
            (*arg).ui &
                ((1i32 <<
                      (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                           as libc::c_ulong))
                     - 1i32) as libc::c_uint;
    if 0 != newtags {
        (*(*selmon).sel).tags = newtags;
        focus(0 as *mut Client);
        arrange(selmon);
    };
}
unsafe extern "C" fn focus(mut c: *mut Client) {
    if c.is_null() ||
           0 == (*c).tags & (*(*c).mon).tagset[(*(*c).mon).seltags as usize] {
        c = (*selmon).stack;
        while !c.is_null() &&
                  0 ==
                      (*c).tags &
                          (*(*c).mon).tagset[(*(*c).mon).seltags as usize] {
            c = (*c).snext
        }
    }
    if !(*selmon).sel.is_null() && (*selmon).sel != c {
        unfocus((*selmon).sel, 0i32);
    }
    if !c.is_null() {
        if (*c).mon != selmon { selmon = (*c).mon }
        if 0 != (*c).isurgent { seturgent(c, 0i32); }
        detachstack(c);
        attachstack(c);
        grabbuttons(c, 1i32);
        XSetWindowBorder(dpy, (*c).win,
                         (*(*scheme.offset(SchemeSel as libc::c_int as
                                               isize)).offset(ColBorder as
                                                                  libc::c_int
                                                                  as
                                                                  isize)).pixel);
        setfocus(c);
    } else {
        XSetInputFocus(dpy, root, 1i64 as libc::c_int, 0i64 as Time);
        XDeleteProperty(dpy, root,
                        netatom[NetActiveWindow as libc::c_int as usize]);
    }
    (*selmon).sel = c;
    drawbars();
}
unsafe extern "C" fn drawbars() {
    let mut m: *mut Monitor = 0 as *mut Monitor;
    m = mons;
    while !m.is_null() { drawbar(m); m = (*m).next };
}
static mut netatom: [Atom; 9] = [0; 9];
static mut root: Window = 0;
unsafe extern "C" fn setfocus(mut c: *mut Client) {
    if 0 == (*c).neverfocus {
        XSetInputFocus(dpy, (*c).win, 1i64 as libc::c_int, 0i64 as Time);
        XChangeProperty(dpy, root,
                        netatom[NetActiveWindow as libc::c_int as usize],
                        33i32 as Atom, 32i32, 0i32,
                        &mut (*c).win as *mut Window as *mut libc::c_uchar,
                        1i32);
    }
    sendevent(c, wmatom[WMTakeFocus as libc::c_int as usize]);
}
static mut wmatom: [Atom; 4] = [0; 4];
unsafe extern "C" fn sendevent(mut c: *mut Client, mut proto: Atom)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut protocols: *mut Atom = 0 as *mut Atom;
    let mut exists: libc::c_int = 0i32;
    let mut ev: XEvent = _XEvent{type_0: 0,};
    if 0 != XGetWMProtocols(dpy, (*c).win, &mut protocols, &mut n) {
        while 0 == exists && { let fresh0 = n; n = n - 1; 0 != fresh0 } {
            exists = (*protocols.offset(n as isize) == proto) as libc::c_int
        }
        XFree(protocols as *mut libc::c_void);
    }
    if 0 != exists {
        ev.type_0 = 33i32;
        ev.xclient.window = (*c).win;
        ev.xclient.message_type = wmatom[WMProtocols as libc::c_int as usize];
        ev.xclient.format = 32i32;
        ev.xclient.data.l[0usize] = proto as libc::c_long;
        ev.xclient.data.l[1usize] = 0i64;
        XSendEvent(dpy, (*c).win, 0i32, 0i64, &mut ev);
    }
    return exists;
}
unsafe extern "C" fn grabbuttons(mut c: *mut Client,
                                 mut focused: libc::c_int) {
    updatenumlockmask();
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut modifiers: [libc::c_uint; 4] =
        [0i32 as libc::c_uint, (1i32 << 1i32) as libc::c_uint, numlockmask,
         numlockmask | (1i32 << 1i32) as libc::c_uint];
    XUngrabButton(dpy, 0i64 as libc::c_uint, (1i32 << 15i32) as libc::c_uint,
                  (*c).win);
    if 0 == focused {
        XGrabButton(dpy, 0i64 as libc::c_uint,
                    (1i32 << 15i32) as libc::c_uint, (*c).win, 0i32,
                    (1i64 << 2i32 | 1i64 << 3i32) as libc::c_uint, 0i32, 0i32,
                    0i64 as Window, 0i64 as Cursor);
    }
    i = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[Button; 11]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<Button>()
                                                   as libc::c_ulong) {
        if buttons[i as usize].click ==
               ClkClientWin as libc::c_int as libc::c_uint {
            j = 0i32 as libc::c_uint;
            while (j as libc::c_ulong) <
                      (::std::mem::size_of::<[libc::c_uint; 4]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_uint>()
                                                           as libc::c_ulong) {
                XGrabButton(dpy, buttons[i as usize].button,
                            buttons[i as usize].mask | modifiers[j as usize],
                            (*c).win, 0i32,
                            (1i64 << 2i32 | 1i64 << 3i32) as libc::c_uint,
                            1i32, 0i32, 0i64 as Window, 0i64 as Cursor);
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    };
}
static mut numlockmask: libc::c_uint = 0i32 as libc::c_uint;
unsafe extern "C" fn updatenumlockmask() {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut modmap: *mut XModifierKeymap = 0 as *mut XModifierKeymap;
    numlockmask = 0i32 as libc::c_uint;
    modmap = XGetModifierMapping(dpy);
    i = 0i32 as libc::c_uint;
    while i < 8i32 as libc::c_uint {
        j = 0i32 as libc::c_uint;
        while j < (*modmap).max_keypermod as libc::c_uint {
            if *(*modmap).modifiermap.offset(i.wrapping_mul((*modmap).max_keypermod
                                                                as
                                                                libc::c_uint).wrapping_add(j)
                                                 as isize) as libc::c_int ==
                   XKeysymToKeycode(dpy, 0xff7fi32 as KeySym) as libc::c_int {
                numlockmask = (1i32 << i) as libc::c_uint
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    XFreeModifiermap(modmap);
}
unsafe extern "C" fn detachstack(mut c: *mut Client) {
    let mut tc: *mut *mut Client = 0 as *mut *mut Client;
    let mut t: *mut Client = 0 as *mut Client;
    tc = &mut (*(*c).mon).stack;
    while !(*tc).is_null() && *tc != c { tc = &mut (**tc).snext }
    *tc = (*c).snext;
    if c == (*(*c).mon).sel {
        t = (*(*c).mon).stack;
        while !t.is_null() &&
                  0 ==
                      (*t).tags &
                          (*(*t).mon).tagset[(*(*t).mon).seltags as usize] {
            t = (*t).snext
        }
        (*(*c).mon).sel = t
    };
}
unsafe extern "C" fn seturgent(mut c: *mut Client, mut urg: libc::c_int) {
    let mut wmh: *mut XWMHints = 0 as *mut XWMHints;
    (*c).isurgent = urg;
    wmh = XGetWMHints(dpy, (*c).win);
    if wmh.is_null() { return }
    (*wmh).flags =
        if 0 != urg {
            (*wmh).flags | 1i64 << 8i32
        } else { (*wmh).flags & !(1i64 << 8i32) };
    XSetWMHints(dpy, (*c).win, wmh);
    XFree(wmh as *mut libc::c_void);
}
unsafe extern "C" fn unfocus(mut c: *mut Client,
                             mut setfocus_0: libc::c_int) {
    if c.is_null() { return }
    grabbuttons(c, 0i32);
    XSetWindowBorder(dpy, (*c).win,
                     (*(*scheme.offset(SchemeNorm as libc::c_int as
                                           isize)).offset(ColBorder as
                                                              libc::c_int as
                                                              isize)).pixel);
    if 0 != setfocus_0 {
        XSetInputFocus(dpy, root, 1i64 as libc::c_int, 0i64 as Time);
        XDeleteProperty(dpy, root,
                        netatom[NetActiveWindow as libc::c_int as usize]);
    };
}
unsafe extern "C" fn tag(mut arg: *const Arg) {
    if !(*selmon).sel.is_null() &&
           0 !=
               (*arg).ui &
                   ((1i32 <<
                         (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                              as
                                                              libc::c_ulong))
                        - 1i32) as libc::c_uint {
        (*(*selmon).sel).tags =
            (*arg).ui &
                ((1i32 <<
                      (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                           as libc::c_ulong))
                     - 1i32) as libc::c_uint;
        focus(0 as *mut Client);
        arrange(selmon);
    };
}
unsafe extern "C" fn toggleview(mut arg: *const Arg) {
    let mut newtagset: libc::c_uint =
        (*selmon).tagset[(*selmon).seltags as usize] ^
            (*arg).ui &
                ((1i32 <<
                      (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                           as libc::c_ulong))
                     - 1i32) as libc::c_uint;
    if 0 != newtagset {
        (*selmon).tagset[(*selmon).seltags as usize] = newtagset;
        focus(0 as *mut Client);
        arrange(selmon);
    };
}
unsafe extern "C" fn view(mut arg: *const Arg) {
    if (*arg).ui &
           ((1i32 <<
                 (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                      libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                      as libc::c_ulong)) -
                1i32) as libc::c_uint ==
           (*selmon).tagset[(*selmon).seltags as usize] {
        return
    }
    (*selmon).seltags ^= 1i32 as libc::c_uint;
    if 0 !=
           (*arg).ui &
               ((1i32 <<
                     (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                          libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                          as libc::c_ulong)) -
                    1i32) as libc::c_uint {
        (*selmon).tagset[(*selmon).seltags as usize] =
            (*arg).ui &
                ((1i32 <<
                      (::std::mem::size_of::<[*const libc::c_char; 9]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                           as libc::c_ulong))
                     - 1i32) as libc::c_uint
    }
    focus(0 as *mut Client);
    arrange(selmon);
}
unsafe extern "C" fn resizemouse(mut arg: *const Arg) {
    let mut ocx: libc::c_int = 0;
    let mut ocy: libc::c_int = 0;
    let mut nw: libc::c_int = 0;
    let mut nh: libc::c_int = 0;
    let mut c: *mut Client = 0 as *mut Client;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut ev: XEvent = _XEvent{type_0: 0,};
    let mut lasttime: Time = 0i32 as Time;
    c = (*selmon).sel;
    if c.is_null() { return }
    if 0 != (*c).isfullscreen { return }
    restack(selmon);
    ocx = (*c).x;
    ocy = (*c).y;
    if XGrabPointer(dpy, root, 0i32,
                    (1i64 << 2i32 | 1i64 << 3i32 | 1i64 << 6i32) as
                        libc::c_uint, 1i32, 1i32, 0i64 as Window,
                    (*cursor[CurResize as libc::c_int as usize]).cursor,
                    0i64 as Time) != 0i32 {
        return
    }
    XWarpPointer(dpy, 0i64 as Window, (*c).win, 0i32, 0i32,
                 0i32 as libc::c_uint, 0i32 as libc::c_uint,
                 (*c).w + (*c).bw - 1i32, (*c).h + (*c).bw - 1i32);
    loop  {
        XMaskEvent(dpy,
                   1i64 << 2i32 | 1i64 << 3i32 | 1i64 << 6i32 | 1i64 << 15i32
                       | 1i64 << 20i32, &mut ev);
        match ev.type_0 {
            23 | 12 | 20 => {
                handler[ev.type_0 as
                            usize].expect("non-null function pointer")(&mut ev);
            }
            6 => {
                if !(ev.xmotion.time.wrapping_sub(lasttime) <=
                         (1000i32 / 60i32) as libc::c_ulong) {
                    lasttime = ev.xmotion.time;
                    nw =
                        if ev.xmotion.x - ocx - 2i32 * (*c).bw + 1i32 > 1i32 {
                            ev.xmotion.x - ocx - 2i32 * (*c).bw + 1i32
                        } else { 1i32 };
                    nh =
                        if ev.xmotion.y - ocy - 2i32 * (*c).bw + 1i32 > 1i32 {
                            ev.xmotion.y - ocy - 2i32 * (*c).bw + 1i32
                        } else { 1i32 };
                    if (*(*c).mon).wx + nw >= (*selmon).wx &&
                           (*(*c).mon).wx + nw <= (*selmon).wx + (*selmon).ww
                           && (*(*c).mon).wy + nh >= (*selmon).wy &&
                           (*(*c).mon).wy + nh <= (*selmon).wy + (*selmon).wh
                       {
                        if 0 == (*c).isfloating &&
                               (*(*selmon).lt[(*selmon).sellt as
                                                  usize]).arrange.is_some() &&
                               (abs(nw - (*c).w) as libc::c_uint > snap ||
                                    abs(nh - (*c).h) as libc::c_uint > snap) {
                            togglefloating(0 as *const Arg);
                        }
                    }
                    if (*(*selmon).lt[(*selmon).sellt as
                                          usize]).arrange.is_none() ||
                           0 != (*c).isfloating {
                        resize(c, (*c).x, (*c).y, nw, nh, 1i32);
                    }
                }
            }
            _ => { }
        }
        if !(ev.type_0 != 5i32) { break ; }
    }
    XWarpPointer(dpy, 0i64 as Window, (*c).win, 0i32, 0i32,
                 0i32 as libc::c_uint, 0i32 as libc::c_uint,
                 (*c).w + (*c).bw - 1i32, (*c).h + (*c).bw - 1i32);
    XUngrabPointer(dpy, 0i64 as Time);
    while 0 != XCheckMaskEvent(dpy, 1i64 << 4i32, &mut ev) { }
    m = recttomon((*c).x, (*c).y, (*c).w, (*c).h);
    if m != selmon { sendmon(c, m); selmon = m; focus(0 as *mut Client); };
}
unsafe extern "C" fn sendmon(mut c: *mut Client, mut m: *mut Monitor) {
    if (*c).mon == m { return }
    unfocus(c, 1i32);
    detach(c);
    detachstack(c);
    (*c).mon = m;
    (*c).tags = (*m).tagset[(*m).seltags as usize];
    attach(c);
    attachstack(c);
    focus(0 as *mut Client);
    arrange(0 as *mut Monitor);
}
unsafe extern "C" fn detach(mut c: *mut Client) {
    let mut tc: *mut *mut Client = 0 as *mut *mut Client;
    tc = &mut (*(*c).mon).clients;
    while !(*tc).is_null() && *tc != c { tc = &mut (**tc).next }
    *tc = (*c).next;
}

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
unsafe fn intersect(x: i32, y: i32, w: i32, h: i32, m: *mut Monitor) -> i32 {
   (max(0, min((x)+(w),(*m).wx+(*m).ww) - max((x),(*m).wx))
                               * max(0, min((y)+(h),(*m).wy+(*m).wh) - max((y),(*m).wy)))
}
unsafe extern "C" fn recttomon(mut x: libc::c_int, mut y: libc::c_int,
                               mut w: libc::c_int, mut h: libc::c_int)
 -> *mut Monitor {
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut r: *mut Monitor = selmon;
    let mut a: libc::c_int = 0;
    let mut area: libc::c_int = 0i32;
    m = mons;
    while !m.is_null() {
        a = intersect(x, y, w, h, m);
//            if 0i32 >
//                   if x + w < (*m).wx + (*m).ww {
//                       x + w
//                   } else { (*m).wx + (*m).ww } -
//                       if x > (*m).wx { x } else { (*m).wx } {
//                0i32
//            } else {
//                if x + w < (*m).wx + (*m).ww {
//                    x + w
//                } else { (*m).wx + (*m).ww } -
//                    if x > (*m).wx { x } else { (*m).wx }
//            } *
//                if 0i32 >
//                       if y + h < (*m).wy + (*m).wh {
//                           y + h
//                       } else { (*m).wy + (*m).wh } -
//                           if y > (*m).wy { y } else { (*m).wy } {
//                    0i32
//                } else {
//                    if y + h < (*m).wy + (*m).wh {
//                        y + h
//                    } else { (*m).wy + (*m).wh } -
//                        if y > (*m).wy { y } else { (*m).wy }
//                };
        if a > area { area = a; r = m }
        m = (*m).next
    }
    return r;
}
unsafe extern "C" fn togglefloating(mut arg: *const Arg) {
    if (*selmon).sel.is_null() { return }
    if 0 != (*(*selmon).sel).isfullscreen { return }
    (*(*selmon).sel).isfloating =
        (0 == (*(*selmon).sel).isfloating || 0 != (*(*selmon).sel).isfixed) as
            libc::c_int;
    if 0 != (*(*selmon).sel).isfloating {
        resize((*selmon).sel, (*(*selmon).sel).x, (*(*selmon).sel).y,
               (*(*selmon).sel).w, (*(*selmon).sel).h, 0i32);
    }
    arrange(selmon);
}
/* snap pixel */
static mut snap: libc::c_uint = 32i32 as libc::c_uint;
static mut handler: [Option<unsafe extern "C" fn(_: *mut XEvent) -> ()>; 36] =
    [None, None, Some(keypress), None, Some(buttonpress), None,
     Some(motionnotify), Some(enternotify), None, Some(focusin), None, None,
     Some(expose), None, None, None, None, Some(destroynotify),
     Some(unmapnotify), None, Some(maprequest), None, Some(configurenotify),
     Some(configurerequest), None, None, None, None, Some(propertynotify),
     None, None, None, None, Some(clientmessage), Some(mappingnotify), None];
unsafe extern "C" fn unmapnotify(mut e: *mut XEvent) {
    let mut c: *mut Client = 0 as *mut Client;
    let mut ev: *mut XUnmapEvent = &mut (*e).xunmap;
    c = wintoclient((*ev).window);
    if !c.is_null() {
        if 0 != (*ev).send_event {
            setclientstate(c, 0i32 as libc::c_long);
        } else { unmanage(c, 0i32); }
    };
}
unsafe extern "C" fn unmanage(mut c: *mut Client,
                              mut destroyed: libc::c_int) {
    let mut m: *mut Monitor = (*c).mon;
    let mut wc: XWindowChanges =
        XWindowChanges{x: 0,
                       y: 0,
                       width: 0,
                       height: 0,
                       border_width: 0,
                       sibling: 0,
                       stack_mode: 0,};
    detach(c);
    detachstack(c);
    if 0 == destroyed {
        wc.border_width = (*c).oldbw;
        XGrabServer(dpy);
        XSetErrorHandler(Some(xerrordummy));
        XConfigureWindow(dpy, (*c).win, (1i32 << 4i32) as libc::c_uint,
                         &mut wc);
        XUngrabButton(dpy, 0i64 as libc::c_uint,
                      (1i32 << 15i32) as libc::c_uint, (*c).win);
        setclientstate(c, 0i32 as libc::c_long);
        XSync(dpy, 0i32);
        XSetErrorHandler(Some(xerror));
        XUngrabServer(dpy);
    }
    free(c as *mut libc::c_void);
    focus(0 as *mut Client);
    updateclientlist();
    arrange(m);
}
unsafe extern "C" fn updateclientlist() {
    let mut c: *mut Client = 0 as *mut Client;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    XDeleteProperty(dpy, root,
                    netatom[NetClientList as libc::c_int as usize]);
    m = mons;
    while !m.is_null() {
        c = (*m).clients;
        while !c.is_null() {
            XChangeProperty(dpy, root,
                            netatom[NetClientList as libc::c_int as usize],
                            33i32 as Atom, 32i32, 2i32,
                            &mut (*c).win as *mut Window as
                                *mut libc::c_uchar, 1i32);
            c = (*c).next
        }
        m = (*m).next
    };
}
unsafe extern "C" fn xerror(mut dpy_0: *mut Display, mut ee: *mut XErrorEvent)
 -> libc::c_int {
    if (*ee).error_code as libc::c_int == 3i32 ||
           (*ee).request_code as libc::c_int == 42i32 &&
               (*ee).error_code as libc::c_int == 8i32 ||
           (*ee).request_code as libc::c_int == 74i32 &&
               (*ee).error_code as libc::c_int == 9i32 ||
           (*ee).request_code as libc::c_int == 70i32 &&
               (*ee).error_code as libc::c_int == 9i32 ||
           (*ee).request_code as libc::c_int == 66i32 &&
               (*ee).error_code as libc::c_int == 9i32 ||
           (*ee).request_code as libc::c_int == 12i32 &&
               (*ee).error_code as libc::c_int == 8i32 ||
           (*ee).request_code as libc::c_int == 28i32 &&
               (*ee).error_code as libc::c_int == 10i32 ||
           (*ee).request_code as libc::c_int == 33i32 &&
               (*ee).error_code as libc::c_int == 10i32 ||
           (*ee).request_code as libc::c_int == 62i32 &&
               (*ee).error_code as libc::c_int == 9i32 {
        return 0i32
    }
    fprintf(stderr,
            b"dwm: fatal error: request code=%d, error code=%d\n\x00" as
                *const u8 as *const libc::c_char,
            (*ee).request_code as libc::c_int,
            (*ee).error_code as libc::c_int);
    return xerrorxlib.expect("non-null function pointer")(dpy_0, ee);
}
static mut xerrorxlib:
       Option<unsafe extern "C" fn(_: *mut Display, _: *mut XErrorEvent)
                  -> libc::c_int> =
    None;
unsafe extern "C" fn setclientstate(mut c: *mut Client,
                                    mut state: libc::c_long) {
    let mut data: [libc::c_long; 2] = [state, 0i64];
    XChangeProperty(dpy, (*c).win, wmatom[WMState as libc::c_int as usize],
                    wmatom[WMState as libc::c_int as usize], 32i32, 0i32,
                    data.as_mut_ptr() as *mut libc::c_uchar, 2i32);
}
unsafe extern "C" fn xerrordummy(mut dpy_0: *mut Display,
                                 mut ee: *mut XErrorEvent) -> libc::c_int {
    return 0i32;
}
unsafe extern "C" fn wintoclient(mut w: Window) -> *mut Client {
    let mut c: *mut Client = 0 as *mut Client;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    m = mons;
    while !m.is_null() {
        c = (*m).clients;
        while !c.is_null() { if (*c).win == w { return c } c = (*c).next }
        m = (*m).next
    }
    return 0 as *mut Client;
}
unsafe extern "C" fn propertynotify(mut e: *mut XEvent) {
    let mut c: *mut Client = 0 as *mut Client;
    let mut trans: Window = 0;
    let mut ev: *mut XPropertyEvent = &mut (*e).xproperty;
    if (*ev).window == root && (*ev).atom == 39i32 as Atom {
        updatestatus();
    } else if (*ev).state == 1i32 {
        return
    } else {
        c = wintoclient((*ev).window);
        if !c.is_null() {
            match (*ev).atom {
                68 => {
                    if 0 == (*c).isfloating &&
                           0 !=
                               XGetTransientForHint(dpy, (*c).win, &mut trans)
                           &&
                           {
                               (*c).isfloating =
                                   (wintoclient(trans) !=
                                        0 as *mut libc::c_void as *mut Client)
                                       as libc::c_int;
                               0 != (*c).isfloating
                           } {
                        arrange((*c).mon);
                    }
                }
                40 => { updatesizehints(c); }
                35 => { updatewmhints(c); drawbars(); }
                _ => { }
            }
            if (*ev).atom == 39i32 as Atom ||
                   (*ev).atom == netatom[NetWMName as libc::c_int as usize] {
                updatetitle(c);
                if c == (*(*c).mon).sel { drawbar((*c).mon); }
            }
            if (*ev).atom == netatom[NetWMWindowType as libc::c_int as usize]
               {
                updatewindowtype(c);
            }
        }
    };
}
unsafe extern "C" fn updatewindowtype(mut c: *mut Client) {
    let mut state: Atom =
        getatomprop(c, netatom[NetWMState as libc::c_int as usize]);
    let mut wtype: Atom =
        getatomprop(c, netatom[NetWMWindowType as libc::c_int as usize]);
    if state == netatom[NetWMFullscreen as libc::c_int as usize] {
        setfullscreen(c, 1i32);
    }
    if wtype == netatom[NetWMWindowTypeDialog as libc::c_int as usize] {
        (*c).isfloating = 1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn getatomprop(mut c: *mut Client, mut prop: Atom)
 -> Atom {
    let mut di: libc::c_int = 0;
    let mut dl: libc::c_ulong = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut da: Atom = 0;
    let mut atom: Atom = 0i64 as Atom;
    if XGetWindowProperty(dpy, (*c).win, prop, 0i64,
                          ::std::mem::size_of::<Atom>() as libc::c_ulong as
                              libc::c_long, 0i32, 4i32 as Atom, &mut da,
                          &mut di, &mut dl, &mut dl, &mut p) == 0i32 &&
           !p.is_null() {
        atom = *(p as *mut Atom);
        XFree(p as *mut libc::c_void);
    }
    return atom;
}
unsafe extern "C" fn setfullscreen(mut c: *mut Client,
                                   mut fullscreen: libc::c_int) {
    if 0 != fullscreen && 0 == (*c).isfullscreen {
        XChangeProperty(dpy, (*c).win,
                        netatom[NetWMState as libc::c_int as usize],
                        4i32 as Atom, 32i32, 0i32,
                        &mut *netatom.as_mut_ptr().offset(NetWMFullscreen as
                                                              libc::c_int as
                                                              isize) as
                            *mut Atom as *mut libc::c_uchar, 1i32);
        (*c).isfullscreen = 1i32;
        (*c).oldstate = (*c).isfloating;
        (*c).oldbw = (*c).bw;
        (*c).bw = 0i32;
        (*c).isfloating = 1i32;
        resizeclient(c, (*(*c).mon).mx, (*(*c).mon).my, (*(*c).mon).mw,
                     (*(*c).mon).mh);
        XRaiseWindow(dpy, (*c).win);
    } else if 0 == fullscreen && 0 != (*c).isfullscreen {
        XChangeProperty(dpy, (*c).win,
                        netatom[NetWMState as libc::c_int as usize],
                        4i32 as Atom, 32i32, 0i32, 0 as *mut libc::c_uchar,
                        0i32);
        (*c).isfullscreen = 0i32;
        (*c).isfloating = (*c).oldstate;
        (*c).bw = (*c).oldbw;
        (*c).x = (*c).oldx;
        (*c).y = (*c).oldy;
        (*c).w = (*c).oldw;
        (*c).h = (*c).oldh;
        resizeclient(c, (*c).x, (*c).y, (*c).w, (*c).h);
        arrange((*c).mon);
    };
}
unsafe extern "C" fn updatetitle(mut c: *mut Client) {
    if 0 ==
           gettextprop((*c).win, netatom[NetWMName as libc::c_int as usize],
                       (*c).name.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 256]>() as
                           libc::c_ulong as libc::c_uint) {
        gettextprop((*c).win, 39i32 as Atom, (*c).name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as
                        libc::c_ulong as libc::c_uint);
    }
    if (*c).name[0usize] as libc::c_int == '\u{0}' as i32 {
        strcpy((*c).name.as_mut_ptr(), broken.as_ptr());
    };
}
unsafe extern "C" fn gettextprop(mut w: Window, mut atom: Atom,
                                 mut text: *mut libc::c_char,
                                 mut size: libc::c_uint) -> libc::c_int {
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut name: XTextProperty =
        XTextProperty{value: 0 as *mut libc::c_uchar,
                      encoding: 0,
                      format: 0,
                      nitems: 0,};
    if text.is_null() || size == 0i32 as libc::c_uint { return 0i32 }
    *text.offset(0isize) = '\u{0}' as i32 as libc::c_char;
    if 0 == XGetTextProperty(dpy, w, &mut name, atom) || 0 == name.nitems {
        return 0i32
    }
    if name.encoding == 31i32 as Atom {
        strncpy(text, name.value as *mut libc::c_char,
                size.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong);
    } else if XmbTextPropertyToTextList(dpy, &mut name, &mut list, &mut n) >=
                  0i32 && n > 0i32 && !(*list).is_null() {
        strncpy(text, *list,
                size.wrapping_sub(1i32 as libc::c_uint) as libc::c_ulong);
        XFreeStringList(list);
    }
    *text.offset(size.wrapping_sub(1i32 as libc::c_uint) as isize) =
        '\u{0}' as i32 as libc::c_char;
    XFree(name.value as *mut libc::c_void);
    return 1i32;
}
unsafe extern "C" fn updatewmhints(mut c: *mut Client) {
    let mut wmh: *mut XWMHints = 0 as *mut XWMHints;
    wmh = XGetWMHints(dpy, (*c).win);
    if !wmh.is_null() {
        if c == (*selmon).sel && 0 != (*wmh).flags & 1i64 << 8i32 {
            (*wmh).flags &= !(1i64 << 8i32);
            XSetWMHints(dpy, (*c).win, wmh);
        } else {
            (*c).isurgent =
                if 0 != (*wmh).flags & 1i64 << 8i32 { 1i32 } else { 0i32 }
        }
        if 0 != (*wmh).flags & 1i64 << 0i32 {
            (*c).neverfocus = (0 == (*wmh).input) as libc::c_int
        } else { (*c).neverfocus = 0i32 }
        XFree(wmh as *mut libc::c_void);
    };
}
unsafe extern "C" fn updatesizehints(mut c: *mut Client) {
    let mut msize: libc::c_long = 0;
    let mut size: XSizeHints =
        XSizeHints{flags: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   min_width: 0,
                   min_height: 0,
                   max_width: 0,
                   max_height: 0,
                   width_inc: 0,
                   height_inc: 0,
                   min_aspect: unnamed_1{x: 0, y: 0,},
                   max_aspect: unnamed_1{x: 0, y: 0,},
                   base_width: 0,
                   base_height: 0,
                   win_gravity: 0,};
    if 0 == XGetWMNormalHints(dpy, (*c).win, &mut size, &mut msize) {
        size.flags = 1i64 << 3i32
    }
    if 0 != size.flags & 1i64 << 8i32 {
        (*c).basew = size.base_width;
        (*c).baseh = size.base_height
    } else if 0 != size.flags & 1i64 << 4i32 {
        (*c).basew = size.min_width;
        (*c).baseh = size.min_height
    } else { (*c).baseh = 0i32; (*c).basew = (*c).baseh }
    if 0 != size.flags & 1i64 << 6i32 {
        (*c).incw = size.width_inc;
        (*c).inch = size.height_inc
    } else { (*c).inch = 0i32; (*c).incw = (*c).inch }
    if 0 != size.flags & 1i64 << 5i32 {
        (*c).maxw = size.max_width;
        (*c).maxh = size.max_height
    } else { (*c).maxh = 0i32; (*c).maxw = (*c).maxh }
    if 0 != size.flags & 1i64 << 4i32 {
        (*c).minw = size.min_width;
        (*c).minh = size.min_height
    } else if 0 != size.flags & 1i64 << 8i32 {
        (*c).minw = size.base_width;
        (*c).minh = size.base_height
    } else { (*c).minh = 0i32; (*c).minw = (*c).minh }
    if 0 != size.flags & 1i64 << 7i32 {
        (*c).mina =
            size.min_aspect.y as libc::c_float /
                size.min_aspect.x as libc::c_float;
        (*c).maxa =
            size.max_aspect.x as libc::c_float /
                size.max_aspect.y as libc::c_float
    } else { (*c).mina = 0.0f64 as libc::c_float; (*c).maxa = (*c).mina }
    (*c).isfixed =
        (0 != (*c).maxw && 0 != (*c).maxh && (*c).maxw == (*c).minw &&
             (*c).maxh == (*c).minh) as libc::c_int;
}
unsafe extern "C" fn updatestatus() {
    if 0 ==
           gettextprop(root, 39i32 as Atom, stext.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 256]>() as
                           libc::c_ulong as libc::c_uint) {
        strcpy(stext.as_mut_ptr(),
               b"dwm-6.2\x00" as *const u8 as *const libc::c_char);
    }
    drawbar(selmon);
}
unsafe extern "C" fn motionnotify(mut e: *mut XEvent) {
    static mut mon: *mut Monitor = 0 as *const Monitor as *mut Monitor;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut ev: *mut XMotionEvent = &mut (*e).xmotion;
    if (*ev).window != root { return }
    m = recttomon((*ev).x_root, (*ev).y_root, 1i32, 1i32);
    if m != mon && !mon.is_null() {
        unfocus((*selmon).sel, 1i32);
        selmon = m;
        focus(0 as *mut Client);
    }
    mon = m;
}
unsafe extern "C" fn maprequest(mut e: *mut XEvent) {
    static mut wa: XWindowAttributes =
        XWindowAttributes{x: 0,
                          y: 0,
                          width: 0,
                          height: 0,
                          border_width: 0,
                          depth: 0,
                          visual: 0 as *const Visual as *mut Visual,
                          root: 0,
                          class: 0,
                          bit_gravity: 0,
                          win_gravity: 0,
                          backing_store: 0,
                          backing_planes: 0,
                          backing_pixel: 0,
                          save_under: 0,
                          colormap: 0,
                          map_installed: 0,
                          map_state: 0,
                          all_event_masks: 0,
                          your_event_mask: 0,
                          do_not_propagate_mask: 0,
                          override_redirect: 0,
                          screen: 0 as *const Screen as *mut Screen,};
    let mut ev: *mut XMapRequestEvent = &mut (*e).xmaprequest;
    if 0 == XGetWindowAttributes(dpy, (*ev).window, &mut wa) { return }
    if 0 != wa.override_redirect { return }
    if wintoclient((*ev).window).is_null() { manage((*ev).window, &mut wa); };
}
unsafe extern "C" fn manage(mut w: Window, mut wa: *mut XWindowAttributes) {
    let mut c: *mut Client = 0 as *mut Client;
    let mut t: *mut Client = 0 as *mut Client;
    let mut trans: Window = 0i64 as Window;
    let mut wc: XWindowChanges =
        XWindowChanges{x: 0,
                       y: 0,
                       width: 0,
                       height: 0,
                       border_width: 0,
                       sibling: 0,
                       stack_mode: 0,};
    c =
        ecalloc(1i32 as size_t,
                ::std::mem::size_of::<Client>() as libc::c_ulong) as
            *mut Client;
    (*c).win = w;
    (*c).oldx = (*wa).x;
    (*c).x = (*c).oldx;
    (*c).oldy = (*wa).y;
    (*c).y = (*c).oldy;
    (*c).oldw = (*wa).width;
    (*c).w = (*c).oldw;
    (*c).oldh = (*wa).height;
    (*c).h = (*c).oldh;
    (*c).oldbw = (*wa).border_width;
    updatetitle(c);
    if 0 != XGetTransientForHint(dpy, w, &mut trans) &&
           { t = wintoclient(trans); !t.is_null() } {
        (*c).mon = (*t).mon;
        (*c).tags = (*t).tags
    } else { (*c).mon = selmon; applyrules(c); }
    if (*c).x + ((*c).w + 2i32 * (*c).bw) > (*(*c).mon).mx + (*(*c).mon).mw {
        (*c).x = (*(*c).mon).mx + (*(*c).mon).mw - ((*c).w + 2i32 * (*c).bw)
    }
    if (*c).y + ((*c).h + 2i32 * (*c).bw) > (*(*c).mon).my + (*(*c).mon).mh {
        (*c).y = (*(*c).mon).my + (*(*c).mon).mh - ((*c).h + 2i32 * (*c).bw)
    }
    (*c).x = if (*c).x > (*(*c).mon).mx { (*c).x } else { (*(*c).mon).mx };
    (*c).y =
        if (*c).y >
               if (*(*c).mon).by == (*(*c).mon).my &&
                      (*c).x + (*c).w / 2i32 >= (*(*c).mon).wx &&
                      (*c).x + (*c).w / 2i32 < (*(*c).mon).wx + (*(*c).mon).ww
                  {
                   bh
               } else { (*(*c).mon).my } {
            (*c).y
        } else if (*(*c).mon).by == (*(*c).mon).my &&
                      (*c).x + (*c).w / 2i32 >= (*(*c).mon).wx &&
                      (*c).x + (*c).w / 2i32 < (*(*c).mon).wx + (*(*c).mon).ww
         {
            bh
        } else { (*(*c).mon).my };
    (*c).bw = borderpx as libc::c_int;
    wc.border_width = (*c).bw;
    XConfigureWindow(dpy, w, (1i32 << 4i32) as libc::c_uint, &mut wc);
    XSetWindowBorder(dpy, w,
                     (*(*scheme.offset(SchemeNorm as libc::c_int as
                                           isize)).offset(ColBorder as
                                                              libc::c_int as
                                                              isize)).pixel);
    configure(c);
    updatewindowtype(c);
    updatesizehints(c);
    updatewmhints(c);
    XSelectInput(dpy, w,
                 1i64 << 4i32 | 1i64 << 21i32 | 1i64 << 22i32 |
                     1i64 << 17i32);
    grabbuttons(c, 0i32);
    if 0 == (*c).isfloating {
        (*c).oldstate =
            (trans != 0i64 as libc::c_ulong || 0 != (*c).isfixed) as
                libc::c_int;
        (*c).isfloating = (*c).oldstate
    }
    if 0 != (*c).isfloating { XRaiseWindow(dpy, (*c).win); }
    attach(c);
    attachstack(c);
    XChangeProperty(dpy, root, netatom[NetClientList as libc::c_int as usize],
                    33i32 as Atom, 32i32, 2i32,
                    &mut (*c).win as *mut Window as *mut libc::c_uchar, 1i32);
    XMoveResizeWindow(dpy, (*c).win, (*c).x + 2i32 * sw, (*c).y,
                      (*c).w as libc::c_uint, (*c).h as libc::c_uint);
    setclientstate(c, 1i32 as libc::c_long);
    if (*c).mon == selmon { unfocus((*selmon).sel, 0i32); }
    (*(*c).mon).sel = c;
    arrange((*c).mon);
    XMapWindow(dpy, (*c).win);
    focus(0 as *mut Client);
}
/* See LICENSE file for copyright and license details. */
/* appearance */
/* border pixel of windows */
static mut borderpx: libc::c_uint = 1i32 as libc::c_uint;
unsafe extern "C" fn mappingnotify(mut e: *mut XEvent) {
    let mut ev: *mut XMappingEvent = &mut (*e).xmapping;
    XRefreshKeyboardMapping(ev);
    if (*ev).request == 1i32 { grabkeys(); };
}
unsafe extern "C" fn grabkeys() {
    updatenumlockmask();
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut modifiers: [libc::c_uint; 4] =
        [0i32 as libc::c_uint, (1i32 << 1i32) as libc::c_uint, numlockmask,
         numlockmask | (1i32 << 1i32) as libc::c_uint];
    let mut code: KeyCode = 0;
    XUngrabKey(dpy, 0i64 as libc::c_int, (1i32 << 15i32) as libc::c_uint,
               root);
    i = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[Key; 60]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<Key>() as
                                                   libc::c_ulong) {
        code = XKeysymToKeycode(dpy, keys[i as usize].keysym);
        if 0 != code {
            j = 0i32 as libc::c_uint;
            while (j as libc::c_ulong) <
                      (::std::mem::size_of::<[libc::c_uint; 4]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_uint>()
                                                           as libc::c_ulong) {
                XGrabKey(dpy, code as libc::c_int,
                         keys[i as usize].mod_0 | modifiers[j as usize], root,
                         1i32, 1i32, 1i32);
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    };
}
// Initialized in run_static_initializers
static mut keys: [Key; 60] =
    [Key{mod_0: 0, keysym: 0, func: None, arg: Arg{i: 0,},}; 60];
unsafe extern "C" fn quit(mut arg: *const Arg) { running = 0i32; }
static mut running: libc::c_int = 1i32;
unsafe extern "C" fn tagmon(mut arg: *const Arg) {
    if (*selmon).sel.is_null() || (*mons).next.is_null() { return }
    sendmon((*selmon).sel, dirtomon((*arg).i));
}
unsafe extern "C" fn dirtomon(mut dir: libc::c_int) -> *mut Monitor {
    let mut m: *mut Monitor = 0 as *mut Monitor;
    if dir > 0i32 {
        m = (*selmon).next;
        if m.is_null() { m = mons }
    } else if selmon == mons {
        m = mons;
        while !(*m).next.is_null() { m = (*m).next }
    } else { m = mons; while (*m).next != selmon { m = (*m).next } }
    return m;
}
unsafe extern "C" fn focusmon(mut arg: *const Arg) {
    let mut m: *mut Monitor = 0 as *mut Monitor;
    if (*mons).next.is_null() { return }
    m = dirtomon((*arg).i);
    if m == selmon { return }
    unfocus((*selmon).sel, 0i32);
    selmon = m;
    focus(0 as *mut Client);
}
unsafe extern "C" fn setlayout(mut arg: *const Arg) {
    if arg.is_null() || (*arg).v.is_null() ||
           (*arg).v !=
               (*selmon).lt[(*selmon).sellt as usize] as *const libc::c_void {
        (*selmon).sellt ^= 1i32 as libc::c_uint
    }
    if !arg.is_null() && !(*arg).v.is_null() {
        (*selmon).lt[(*selmon).sellt as usize] = (*arg).v as *mut Layout
    }
    strncpy((*selmon).ltsymbol.as_mut_ptr(),
            (*(*selmon).lt[(*selmon).sellt as usize]).symbol,
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong);
    if !(*selmon).sel.is_null() {
        arrange(selmon);
    } else { drawbar(selmon); };
}
static mut layouts: [Layout; 3] =
    [Layout{symbol: b"[]=\x00" as *const u8 as *const libc::c_char,
            arrange: Some(tile),},
     Layout{symbol: b"><>\x00" as *const u8 as *const libc::c_char,
            arrange: None,},
     Layout{symbol: b"[M]\x00" as *const u8 as *const libc::c_char,
            arrange: Some(monocle),}];
unsafe extern "C" fn monocle(mut m: *mut Monitor) {
    let mut n: libc::c_uint = 0i32 as libc::c_uint;
    let mut c: *mut Client = 0 as *mut Client;
    c = (*m).clients;
    while !c.is_null() {
        if 0 != (*c).tags & (*(*c).mon).tagset[(*(*c).mon).seltags as usize] {
            n = n.wrapping_add(1)
        }
        c = (*c).next
    }
    if n > 0i32 as libc::c_uint {
        snprintf((*m).ltsymbol.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                 b"[%d]\x00" as *const u8 as *const libc::c_char, n);
    }
    c = nexttiled((*m).clients);
    while !c.is_null() {
        resize(c, (*m).wx, (*m).wy, (*m).ww - 2i32 * (*c).bw,
               (*m).wh - 2i32 * (*c).bw, 0i32);
        c = nexttiled((*c).next)
    };
}
unsafe extern "C" fn nexttiled(mut c: *mut Client) -> *mut Client {
    while !c.is_null() &&
              (0 != (*c).isfloating ||
                   0 ==
                       (*c).tags &
                           (*(*c).mon).tagset[(*(*c).mon).seltags as usize]) {
        c = (*c).next
    }
    return c;
}
unsafe extern "C" fn tile(mut m: *mut Monitor) {
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    let mut mw: libc::c_uint = 0;
    let mut my: libc::c_uint = 0;
    let mut ty: libc::c_uint = 0;
    let mut c: *mut Client = 0 as *mut Client;
    n = 0i32 as libc::c_uint;
    c = nexttiled((*m).clients);
    while !c.is_null() { c = nexttiled((*c).next); n = n.wrapping_add(1) }
    if n == 0i32 as libc::c_uint { return }
    if n > (*m).nmaster as libc::c_uint {
        mw =
            (if 0 != (*m).nmaster {
                 (*m).ww as libc::c_float * (*m).mfact
             } else { 0i32 as libc::c_float }) as libc::c_uint
    } else { mw = (*m).ww as libc::c_uint }
    ty = 0i32 as libc::c_uint;
    my = ty;
    i = my;
    c = nexttiled((*m).clients);
    while !c.is_null() {
        if i < (*m).nmaster as libc::c_uint {
            h =
                ((*m).wh as
                     libc::c_uint).wrapping_sub(my).wrapping_div(if n <
                                                                        (*m).nmaster
                                                                            as
                                                                            libc::c_uint
                                                                    {
                                                                     n
                                                                 } else {
                                                                     (*m).nmaster
                                                                         as
                                                                         libc::c_uint
                                                                 }.wrapping_sub(i));
            resize(c, (*m).wx,
                   ((*m).wy as libc::c_uint).wrapping_add(my) as libc::c_int,
                   mw.wrapping_sub((2i32 * (*c).bw) as libc::c_uint) as
                       libc::c_int,
                   h.wrapping_sub((2i32 * (*c).bw) as libc::c_uint) as
                       libc::c_int, 0i32);
            my = my.wrapping_add(((*c).h + 2i32 * (*c).bw) as libc::c_uint)
        } else {
            h =
                ((*m).wh as
                     libc::c_uint).wrapping_sub(ty).wrapping_div(n.wrapping_sub(i));
            resize(c,
                   ((*m).wx as libc::c_uint).wrapping_add(mw) as libc::c_int,
                   ((*m).wy as libc::c_uint).wrapping_add(ty) as libc::c_int,
                   ((*m).ww as
                        libc::c_uint).wrapping_sub(mw).wrapping_sub((2i32 *
                                                                         (*c).bw)
                                                                        as
                                                                        libc::c_uint)
                       as libc::c_int,
                   h.wrapping_sub((2i32 * (*c).bw) as libc::c_uint) as
                       libc::c_int, 0i32);
            ty = ty.wrapping_add(((*c).h + 2i32 * (*c).bw) as libc::c_uint)
        }
        c = nexttiled((*c).next);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn killclient(mut arg: *const Arg) {
    if (*selmon).sel.is_null() { return }
    if 0 == sendevent((*selmon).sel, wmatom[WMDelete as libc::c_int as usize])
       {
        XGrabServer(dpy);
        XSetErrorHandler(Some(xerrordummy));
        XSetCloseDownMode(dpy, 0i32);
        XKillClient(dpy, (*(*selmon).sel).win);
        XSync(dpy, 0i32);
        XSetErrorHandler(Some(xerror));
        XUngrabServer(dpy);
    };
}
unsafe extern "C" fn zoom(mut arg: *const Arg) {
    let mut c: *mut Client = (*selmon).sel;
    if (*(*selmon).lt[(*selmon).sellt as usize]).arrange.is_none() ||
           !(*selmon).sel.is_null() && 0 != (*(*selmon).sel).isfloating {
        return
    }
    if c == nexttiled((*selmon).clients) {
        if c.is_null() || { c = nexttiled((*c).next); c.is_null() } { return }
    }
    pop(c);
}
unsafe extern "C" fn pop(mut c: *mut Client) {
    detach(c);
    attach(c);
    focus(c);
    arrange((*c).mon);
}
unsafe extern "C" fn setmfact(mut arg: *const Arg) {
    let mut f: libc::c_float = 0.;
    if arg.is_null() ||
           (*(*selmon).lt[(*selmon).sellt as usize]).arrange.is_none() {
        return
    }
    f =
        (if ((*arg).f as libc::c_double) < 1.0f64 {
             ((*arg).f + (*selmon).mfact) as libc::c_double
         } else { (*arg).f as libc::c_double - 1.0f64 }) as libc::c_float;
    if (f as libc::c_double) < 0.1f64 || f as libc::c_double > 0.9f64 {
        return
    }
    (*selmon).mfact = f;
    arrange(selmon);
}
unsafe extern "C" fn incnmaster(mut arg: *const Arg) {
    (*selmon).nmaster =
        if (*selmon).nmaster + (*arg).i > 0i32 {
            (*selmon).nmaster + (*arg).i
        } else { 0i32 };
    arrange(selmon);
}
unsafe extern "C" fn focusstack(mut arg: *const Arg) {
    let mut c: *mut Client = 0 as *mut Client;
    let mut i: *mut Client = 0 as *mut Client;
    if (*selmon).sel.is_null() { return }
    if (*arg).i > 0i32 {
        c = (*(*selmon).sel).next;
        while !c.is_null() &&
                  0 ==
                      (*c).tags &
                          (*(*c).mon).tagset[(*(*c).mon).seltags as usize] {
            c = (*c).next
        }
        if c.is_null() {
            c = (*selmon).clients;
            while !c.is_null() &&
                      0 ==
                          (*c).tags &
                              (*(*c).mon).tagset[(*(*c).mon).seltags as usize]
                  {
                c = (*c).next
            }
        }
    } else {
        i = (*selmon).clients;
        while i != (*selmon).sel {
            if 0 !=
                   (*i).tags &
                       (*(*i).mon).tagset[(*(*i).mon).seltags as usize] {
                c = i
            }
            i = (*i).next
        }
        if c.is_null() {
            while !i.is_null() {
                if 0 !=
                       (*i).tags &
                           (*(*i).mon).tagset[(*(*i).mon).seltags as usize] {
                    c = i
                }
                i = (*i).next
            }
        }
    }
    if !c.is_null() { focus(c); restack(selmon); };
}
unsafe extern "C" fn togglebar(mut arg: *const Arg) {
    (*selmon).showbar = (0 == (*selmon).showbar) as libc::c_int;
    updatebarpos(selmon);
    XMoveResizeWindow(dpy, (*selmon).barwin, (*selmon).wx, (*selmon).by,
                      (*selmon).ww as libc::c_uint, bh as libc::c_uint);
    arrange(selmon);
}
unsafe extern "C" fn updatebarpos(mut m: *mut Monitor) {
    (*m).wy = (*m).my;
    (*m).wh = (*m).mh;
    if 0 != (*m).showbar {
        (*m).wh -= bh;
        (*m).by = if 0 != (*m).topbar { (*m).wy } else { (*m).wy + (*m).wh };
        (*m).wy = if 0 != (*m).topbar { (*m).wy + bh } else { (*m).wy }
    } else { (*m).by = -bh };
}
static mut termcmd: [*const libc::c_char; 2] =
    [b"st\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
unsafe extern "C" fn spawn(mut arg: *const Arg) {
    if (*arg).v == dmenucmd.as_mut_ptr() as *const libc::c_void {
        dmenumon[0usize] = ('0' as i32 + (*selmon).num) as libc::c_char
    }
    if fork() == 0i32 {
        if !dpy.is_null() { close((*(dpy as _XPrivDisplay)).fd); }
        setsid();
        execvp(*((*arg).v as *mut *mut libc::c_char).offset(0isize),
               (*arg).v as *mut *mut libc::c_char as
                   *const *mut libc::c_char);
        fprintf(stderr,
                b"dwm: execvp %s\x00" as *const u8 as *const libc::c_char,
                *((*arg).v as *mut *mut libc::c_char).offset(0isize));
        perror(b" failed\x00" as *const u8 as *const libc::c_char);
        exit(0i32);
    };
}
/* symbol     arrange function */
/* first entry is default */
/* no layout function means floating behavior */
/* key definitions */
/* helper for spawning shell commands in the pre dwm-5.0 fashion */
/* commands */
/* component of dmenucmd, manipulated in spawn() */
static mut dmenumon: [libc::c_char; 2] = [48, 0];
static mut dmenucmd: [*const libc::c_char; 14] =
    unsafe {
        [b"dmenu_run\x00" as *const u8 as *const libc::c_char,
         b"-m\x00" as *const u8 as *const libc::c_char,
         dmenumon.as_ptr() as *mut u8 as *const libc::c_char,
         b"-fn\x00" as *const u8 as *const libc::c_char, dmenufont.as_ptr(),
         b"-nb\x00" as *const u8 as *const libc::c_char, col_gray1.as_ptr(),
         b"-nf\x00" as *const u8 as *const libc::c_char, col_gray3.as_ptr(),
         b"-sb\x00" as *const u8 as *const libc::c_char, col_cyan.as_ptr(),
         b"-sf\x00" as *const u8 as *const libc::c_char, col_gray4.as_ptr(),
         0 as *const libc::c_char]
    };
static mut col_gray4: [libc::c_char; 8] =
    [35, 101, 101, 101, 101, 101, 101, 0];
static mut col_cyan: [libc::c_char; 8] = [35, 48, 48, 53, 53, 55, 55, 0];
static mut col_gray3: [libc::c_char; 8] = [35, 98, 98, 98, 98, 98, 98, 0];
static mut col_gray1: [libc::c_char; 8] = [35, 50, 50, 50, 50, 50, 50, 0];
static mut dmenufont: [libc::c_char; 18] =
    [109, 111, 110, 111, 115, 112, 97, 99, 101, 58, 115, 105, 122, 101, 61,
     49, 48, 0];
unsafe extern "C" fn keypress(mut e: *mut XEvent) {
    let mut i: libc::c_uint = 0;
    let mut keysym: KeySym = 0;
    let mut ev: *mut XKeyEvent = 0 as *mut XKeyEvent;
    ev = &mut (*e).xkey;
    keysym = XKeycodeToKeysym(dpy, (*ev).keycode as KeyCode, 0i32);
    i = 0i32 as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[Key; 60]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<Key>() as
                                                   libc::c_ulong) {
        if keysym == keys[i as usize].keysym &&
               keys[i as usize].mod_0 &
                   !(numlockmask | (1i32 << 1i32) as libc::c_uint) &
                   (1i32 << 0i32 | 1i32 << 2i32 | 1i32 << 3i32 | 1i32 << 4i32
                        | 1i32 << 5i32 | 1i32 << 6i32 | 1i32 << 7i32) as
                       libc::c_uint ==
                   (*ev).state &
                       !(numlockmask | (1i32 << 1i32) as libc::c_uint) &
                       (1i32 << 0i32 | 1i32 << 2i32 | 1i32 << 3i32 |
                            1i32 << 4i32 | 1i32 << 5i32 | 1i32 << 6i32 |
                            1i32 << 7i32) as libc::c_uint &&
               keys[i as usize].func.is_some() {
            keys[i as
                     usize].func.expect("non-null function pointer")(&(*keys.as_mut_ptr().offset(i
                                                                                                     as
                                                                                                     isize)).arg);
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn focusin(mut e: *mut XEvent) {
    let mut ev: *mut XFocusChangeEvent = &mut (*e).xfocus;
    if !(*selmon).sel.is_null() && (*ev).window != (*(*selmon).sel).win {
        setfocus((*selmon).sel);
    };
}
unsafe extern "C" fn expose(mut e: *mut XEvent) {
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut ev: *mut XExposeEvent = &mut (*e).xexpose;
    if (*ev).count == 0i32 && { m = wintomon((*ev).window); !m.is_null() } {
        drawbar(m);
    };
}
unsafe extern "C" fn wintomon(mut w: Window) -> *mut Monitor {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut c: *mut Client = 0 as *mut Client;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    if w == root && 0 != getrootptr(&mut x, &mut y) {
        return recttomon(x, y, 1i32, 1i32)
    }
    m = mons;
    while !m.is_null() { if w == (*m).barwin { return m } m = (*m).next }
    c = wintoclient(w);
    if !c.is_null() { return (*c).mon }
    return selmon;
}
unsafe extern "C" fn getrootptr(mut x: *mut libc::c_int,
                                mut y: *mut libc::c_int) -> libc::c_int {
    let mut di: libc::c_int = 0;
    let mut dui: libc::c_uint = 0;
    let mut dummy: Window = 0;
    return XQueryPointer(dpy, root, &mut dummy, &mut dummy, x, y, &mut di,
                         &mut di, &mut dui);
}
unsafe extern "C" fn enternotify(mut e: *mut XEvent) {
    let mut c: *mut Client = 0 as *mut Client;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut ev: *mut XCrossingEvent = &mut (*e).xcrossing;
    if ((*ev).mode != 0i32 || (*ev).detail == 2i32) && (*ev).window != root {
        return
    }
    c = wintoclient((*ev).window);
    m = if !c.is_null() { (*c).mon } else { wintomon((*ev).window) };
    if m != selmon {
        unfocus((*selmon).sel, 1i32);
        selmon = m
    } else if c.is_null() || c == (*selmon).sel { return }
    focus(c);
}
unsafe extern "C" fn destroynotify(mut e: *mut XEvent) {
    let mut c: *mut Client = 0 as *mut Client;
    let mut ev: *mut XDestroyWindowEvent = &mut (*e).xdestroywindow;
    c = wintoclient((*ev).window);
    if !c.is_null() { unmanage(c, 1i32); };
}
unsafe extern "C" fn configurenotify(mut e: *mut XEvent) {
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut c: *mut Client = 0 as *mut Client;
    let mut ev: *mut XConfigureEvent = &mut (*e).xconfigure;
    let mut dirty: libc::c_int = 0;
    if (*ev).window == root {
        dirty = (sw != (*ev).width || sh != (*ev).height) as libc::c_int;
        sw = (*ev).width;
        sh = (*ev).height;
        if 0 != updategeom() || 0 != dirty {
            drw_resize(drw, sw as libc::c_uint, bh as libc::c_uint);
            updatebars();
            m = mons;
            while !m.is_null() {
                c = (*m).clients;
                while !c.is_null() {
                    if 0 != (*c).isfullscreen {
                        resizeclient(c, (*m).mx, (*m).my, (*m).mw, (*m).mh);
                    }
                    c = (*c).next
                }
                XMoveResizeWindow(dpy, (*m).barwin, (*m).wx, (*m).by,
                                  (*m).ww as libc::c_uint,
                                  bh as libc::c_uint);
                m = (*m).next
            }
            focus(0 as *mut Client);
            arrange(0 as *mut Monitor);
        }
    };
}
unsafe extern "C" fn updatebars() {
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut wa: XSetWindowAttributes =
        XSetWindowAttributes{background_pixmap: 1i64 as Pixmap,
                             background_pixel: 0,
                             border_pixmap: 0,
                             border_pixel: 0,
                             bit_gravity: 0,
                             win_gravity: 0,
                             backing_store: 0,
                             backing_planes: 0,
                             backing_pixel: 0,
                             save_under: 0,
                             event_mask: 1i64 << 2i32 | 1i64 << 15i32,
                             do_not_propagate_mask: 0,
                             override_redirect: 1i32,
                             colormap: 0,
                             cursor: 0,};
    let mut ch: XClassHint =
        XClassHint{res_name:
                       b"dwm\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   res_class:
                       b"dwm\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,};
    m = mons;
    while !m.is_null() {
        if !(0 != (*m).barwin) {
            (*m).barwin =
                XCreateWindow(dpy, root, (*m).wx, (*m).by,
                              (*m).ww as libc::c_uint, bh as libc::c_uint,
                              0i32 as libc::c_uint,
                              (*(*(dpy as
                                       _XPrivDisplay)).screens.offset(screen
                                                                          as
                                                                          isize)).root_depth,
                              0i64 as libc::c_uint,
                              (*(*(dpy as
                                       _XPrivDisplay)).screens.offset(screen
                                                                          as
                                                                          isize)).root_visual,
                              (1i64 << 9i32 | 1i64 << 0i32 | 1i64 << 11i32) as
                                  libc::c_ulong, &mut wa);
            XDefineCursor(dpy, (*m).barwin,
                          (*cursor[CurNormal as libc::c_int as
                                       usize]).cursor);
            XMapRaised(dpy, (*m).barwin);
            XSetClassHint(dpy, (*m).barwin, &mut ch);
        }
        m = (*m).next
    };
}
static mut cursor: [*mut Cur; 3] = [0 as *const Cur as *mut Cur; 3];
static mut screen: libc::c_int = 0;
unsafe extern "C" fn updategeom() -> libc::c_int {
    let mut dirty: libc::c_int = 0i32;
    if 0 != XineramaIsActive(dpy) {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        let mut nn: libc::c_int = 0;
        let mut c: *mut Client = 0 as *mut Client;
        let mut m: *mut Monitor = 0 as *mut Monitor;
        let mut info: *mut XineramaScreenInfo =
            XineramaQueryScreens(dpy, &mut nn);
        let mut unique: *mut XineramaScreenInfo =
            0 as *mut XineramaScreenInfo;
        n = 0i32;
        m = mons;
        while !m.is_null() { m = (*m).next; n += 1 }
        unique =
            ecalloc(nn as size_t,
                    ::std::mem::size_of::<XineramaScreenInfo>() as
                        libc::c_ulong) as *mut XineramaScreenInfo;
        i = 0i32;
        j = 0i32;
        while i < nn {
            if 0 !=
                   isuniquegeom(unique, j as size_t,
                                &mut *info.offset(i as isize)) {
                let fresh1 = j;
                j = j + 1;
                memcpy(&mut *unique.offset(fresh1 as isize) as
                           *mut XineramaScreenInfo as *mut libc::c_void,
                       &mut *info.offset(i as isize) as
                           *mut XineramaScreenInfo as *const libc::c_void,
                       ::std::mem::size_of::<XineramaScreenInfo>() as
                           libc::c_ulong);
            }
            i += 1
        }
        XFree(info as *mut libc::c_void);
        nn = j;
        if n <= nn {
            i = 0i32;
            while i < nn - n {
                m = mons;
                while !m.is_null() && !(*m).next.is_null() { m = (*m).next }
                if !m.is_null() {
                    (*m).next = createmon()
                } else { mons = createmon() }
                i += 1
            }
            i = 0i32;
            m = mons;
            while i < nn && !m.is_null() {
                if i >= n ||
                       (*unique.offset(i as isize)).x_org as libc::c_int !=
                           (*m).mx ||
                       (*unique.offset(i as isize)).y_org as libc::c_int !=
                           (*m).my ||
                       (*unique.offset(i as isize)).width as libc::c_int !=
                           (*m).mw ||
                       (*unique.offset(i as isize)).height as libc::c_int !=
                           (*m).mh {
                    dirty = 1i32;
                    (*m).num = i;
                    (*m).wx =
                        (*unique.offset(i as isize)).x_org as libc::c_int;
                    (*m).mx = (*m).wx;
                    (*m).wy =
                        (*unique.offset(i as isize)).y_org as libc::c_int;
                    (*m).my = (*m).wy;
                    (*m).ww =
                        (*unique.offset(i as isize)).width as libc::c_int;
                    (*m).mw = (*m).ww;
                    (*m).wh =
                        (*unique.offset(i as isize)).height as libc::c_int;
                    (*m).mh = (*m).wh;
                    updatebarpos(m);
                }
                m = (*m).next;
                i += 1
            }
        } else {
            i = nn;
            while i < n {
                m = mons;
                while !m.is_null() && !(*m).next.is_null() { m = (*m).next }
                loop  {
                    c = (*m).clients;
                    if c.is_null() { break ; }
                    dirty = 1i32;
                    (*m).clients = (*c).next;
                    detachstack(c);
                    (*c).mon = mons;
                    attach(c);
                    attachstack(c);
                }
                if m == selmon { selmon = mons }
                cleanupmon(m);
                i += 1
            }
        }
        free(unique as *mut libc::c_void);
    } else {
        if mons.is_null() { mons = createmon() }
        if (*mons).mw != sw || (*mons).mh != sh {
            dirty = 1i32;
            (*mons).ww = sw;
            (*mons).mw = (*mons).ww;
            (*mons).wh = sh;
            (*mons).mh = (*mons).wh;
            updatebarpos(mons);
        }
    }
    if 0 != dirty { selmon = mons; selmon = wintomon(root) }
    return dirty;
}
unsafe extern "C" fn createmon() -> *mut Monitor {
    let mut m: *mut Monitor = 0 as *mut Monitor;
    m =
        ecalloc(1i32 as size_t,
                ::std::mem::size_of::<Monitor>() as libc::c_ulong) as
            *mut Monitor;
    (*m).tagset[1usize] = 1i32 as libc::c_uint;
    (*m).tagset[0usize] = (*m).tagset[1usize];
    (*m).mfact = mfact;
    (*m).nmaster = nmaster;
    (*m).showbar = showbar;
    (*m).topbar = topbar;
    (*m).lt[0usize] = &*layouts.as_ptr().offset(0isize) as *const Layout;
    (*m).lt[1usize] =
        &*layouts.as_ptr().offset((1i32 as
                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<[Layout; 3]>()
                                                                        as
                                                                        libc::c_ulong).wrapping_div(::std::mem::size_of::<Layout>()
                                                                                                        as
                                                                                                        libc::c_ulong))
                                      as isize) as *const Layout;
    strncpy((*m).ltsymbol.as_mut_ptr(), layouts[0usize].symbol,
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong);
    return m;
}
/* 0 means bottom bar */
static mut topbar: libc::c_int = 1i32;
/* 0 means no bar */
static mut showbar: libc::c_int = 1i32;
/* number of clients in master area */
static mut nmaster: libc::c_int = 1i32;
/* xprop(1):
	 *	WM_CLASS(STRING) = instance, class
	 *	WM_NAME(STRING) = title
	 */
	/* class      instance    title       tags mask     isfloating   monitor */
/* layout(s) */
/* factor of master area size [0.05..0.95] */
static mut mfact: libc::c_float = 0.55f64 as libc::c_float;
unsafe extern "C" fn cleanupmon(mut mon: *mut Monitor) {
    let mut m: *mut Monitor = 0 as *mut Monitor;
    if mon == mons {
        mons = (*mons).next
    } else {
        m = mons;
        while !m.is_null() && (*m).next != mon { m = (*m).next }
        (*m).next = (*mon).next
    }
    XUnmapWindow(dpy, (*mon).barwin);
    XDestroyWindow(dpy, (*mon).barwin);
    free(mon as *mut libc::c_void);
}
unsafe extern "C" fn isuniquegeom(mut unique: *mut XineramaScreenInfo,
                                  mut n: size_t,
                                  mut info: *mut XineramaScreenInfo)
 -> libc::c_int {
    loop  {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(0 != fresh2) { break ; }
        if (*unique.offset(n as isize)).x_org as libc::c_int ==
               (*info).x_org as libc::c_int &&
               (*unique.offset(n as isize)).y_org as libc::c_int ==
                   (*info).y_org as libc::c_int &&
               (*unique.offset(n as isize)).width as libc::c_int ==
                   (*info).width as libc::c_int &&
               (*unique.offset(n as isize)).height as libc::c_int ==
                   (*info).height as libc::c_int {
            return 0i32
        }
    }
    return 1i32;
}
unsafe extern "C" fn configurerequest(mut e: *mut XEvent) {
    let mut c: *mut Client = 0 as *mut Client;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut ev: *mut XConfigureRequestEvent = &mut (*e).xconfigurerequest;
    let mut wc: XWindowChanges =
        XWindowChanges{x: 0,
                       y: 0,
                       width: 0,
                       height: 0,
                       border_width: 0,
                       sibling: 0,
                       stack_mode: 0,};
    c = wintoclient((*ev).window);
    if !c.is_null() {
        if 0 != (*ev).value_mask & (1i32 << 4i32) as libc::c_ulong {
            (*c).bw = (*ev).border_width
        } else if 0 != (*c).isfloating ||
                      (*(*selmon).lt[(*selmon).sellt as
                                         usize]).arrange.is_none() {
            m = (*c).mon;
            if 0 != (*ev).value_mask & (1i32 << 0i32) as libc::c_ulong {
                (*c).oldx = (*c).x;
                (*c).x = (*m).mx + (*ev).x
            }
            if 0 != (*ev).value_mask & (1i32 << 1i32) as libc::c_ulong {
                (*c).oldy = (*c).y;
                (*c).y = (*m).my + (*ev).y
            }
            if 0 != (*ev).value_mask & (1i32 << 2i32) as libc::c_ulong {
                (*c).oldw = (*c).w;
                (*c).w = (*ev).width
            }
            if 0 != (*ev).value_mask & (1i32 << 3i32) as libc::c_ulong {
                (*c).oldh = (*c).h;
                (*c).h = (*ev).height
            }
            if (*c).x + (*c).w > (*m).mx + (*m).mw && 0 != (*c).isfloating {
                (*c).x =
                    (*m).mx +
                        ((*m).mw / 2i32 - ((*c).w + 2i32 * (*c).bw) / 2i32)
            }
            if (*c).y + (*c).h > (*m).my + (*m).mh && 0 != (*c).isfloating {
                (*c).y =
                    (*m).my +
                        ((*m).mh / 2i32 - ((*c).h + 2i32 * (*c).bw) / 2i32)
            }
            if 0 !=
                   (*ev).value_mask &
                       (1i32 << 0i32 | 1i32 << 1i32) as libc::c_ulong &&
                   0 ==
                       (*ev).value_mask &
                           (1i32 << 2i32 | 1i32 << 3i32) as libc::c_ulong {
                configure(c);
            }
            if 0 !=
                   (*c).tags &
                       (*(*c).mon).tagset[(*(*c).mon).seltags as usize] {
                XMoveResizeWindow(dpy, (*c).win, (*c).x, (*c).y,
                                  (*c).w as libc::c_uint,
                                  (*c).h as libc::c_uint);
            }
        } else { configure(c); }
    } else {
        wc.x = (*ev).x;
        wc.y = (*ev).y;
        wc.width = (*ev).width;
        wc.height = (*ev).height;
        wc.border_width = (*ev).border_width;
        wc.sibling = (*ev).above;
        wc.stack_mode = (*ev).detail;
        XConfigureWindow(dpy, (*ev).window, (*ev).value_mask as libc::c_uint,
                         &mut wc);
    }
    XSync(dpy, 0i32);
}
unsafe extern "C" fn clientmessage(mut e: *mut XEvent) {
    let mut cme: *mut XClientMessageEvent = &mut (*e).xclient;
    let mut c: *mut Client = wintoclient((*cme).window);
    if c.is_null() { return }
    if (*cme).message_type == netatom[NetWMState as libc::c_int as usize] {
        if (*cme).data.l[1usize] as libc::c_ulong ==
               netatom[NetWMFullscreen as libc::c_int as usize] ||
               (*cme).data.l[2usize] as libc::c_ulong ==
                   netatom[NetWMFullscreen as libc::c_int as usize] {
            setfullscreen(c,
                          ((*cme).data.l[0usize] == 1i32 as libc::c_long ||
                               (*cme).data.l[0usize] == 2i32 as libc::c_long
                                   && 0 == (*c).isfullscreen) as libc::c_int);
        }
    } else if (*cme).message_type ==
                  netatom[NetActiveWindow as libc::c_int as usize] {
        if c != (*selmon).sel && 0 == (*c).isurgent { seturgent(c, 1i32); }
    };
}
unsafe extern "C" fn movemouse(mut arg: *const Arg) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ocx: libc::c_int = 0;
    let mut ocy: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut c: *mut Client = 0 as *mut Client;
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut ev: XEvent = _XEvent{type_0: 0,};
    let mut lasttime: Time = 0i32 as Time;
    c = (*selmon).sel;
    if c.is_null() { return }
    if 0 != (*c).isfullscreen { return }
    restack(selmon);
    ocx = (*c).x;
    ocy = (*c).y;
    if XGrabPointer(dpy, root, 0i32,
                    (1i64 << 2i32 | 1i64 << 3i32 | 1i64 << 6i32) as
                        libc::c_uint, 1i32, 1i32, 0i64 as Window,
                    (*cursor[CurMove as libc::c_int as usize]).cursor,
                    0i64 as Time) != 0i32 {
        return
    }
    if 0 == getrootptr(&mut x, &mut y) { return }
    loop  {
        XMaskEvent(dpy,
                   1i64 << 2i32 | 1i64 << 3i32 | 1i64 << 6i32 | 1i64 << 15i32
                       | 1i64 << 20i32, &mut ev);
        match ev.type_0 {
            23 | 12 | 20 => {
                handler[ev.type_0 as
                            usize].expect("non-null function pointer")(&mut ev);
            }
            6 => {
                if !(ev.xmotion.time.wrapping_sub(lasttime) <=
                         (1000i32 / 60i32) as libc::c_ulong) {
                    lasttime = ev.xmotion.time;
                    nx = ocx + (ev.xmotion.x - x);
                    ny = ocy + (ev.xmotion.y - y);
                    if (abs((*selmon).wx - nx) as libc::c_uint) < snap {
                        nx = (*selmon).wx
                    } else if (abs((*selmon).wx + (*selmon).ww -
                                       (nx + ((*c).w + 2i32 * (*c).bw))) as
                                   libc::c_uint) < snap {
                        nx =
                            (*selmon).wx + (*selmon).ww -
                                ((*c).w + 2i32 * (*c).bw)
                    }
                    if (abs((*selmon).wy - ny) as libc::c_uint) < snap {
                        ny = (*selmon).wy
                    } else if (abs((*selmon).wy + (*selmon).wh -
                                       (ny + ((*c).h + 2i32 * (*c).bw))) as
                                   libc::c_uint) < snap {
                        ny =
                            (*selmon).wy + (*selmon).wh -
                                ((*c).h + 2i32 * (*c).bw)
                    }
                    if 0 == (*c).isfloating &&
                           (*(*selmon).lt[(*selmon).sellt as
                                              usize]).arrange.is_some() &&
                           (abs(nx - (*c).x) as libc::c_uint > snap ||
                                abs(ny - (*c).y) as libc::c_uint > snap) {
                        togglefloating(0 as *const Arg);
                    }
                    if (*(*selmon).lt[(*selmon).sellt as
                                          usize]).arrange.is_none() ||
                           0 != (*c).isfloating {
                        resize(c, nx, ny, (*c).w, (*c).h, 1i32);
                    }
                }
            }
            _ => { }
        }
        if !(ev.type_0 != 5i32) { break ; }
    }
    XUngrabPointer(dpy, 0i64 as Time);
    m = recttomon((*c).x, (*c).y, (*c).w, (*c).h);
    if m != selmon { sendmon(c, m); selmon = m; focus(0 as *mut Client); };
}
unsafe extern "C" fn checkotherwm() {
    xerrorxlib = XSetErrorHandler(Some(xerrorstart));
    XSelectInput(dpy,
                 (*(*(dpy as
                          _XPrivDisplay)).screens.offset((*(dpy as
                                                                _XPrivDisplay)).default_screen
                                                             as isize)).root,
                 1i64 << 20i32);
    XSync(dpy, 0i32);
    XSetErrorHandler(Some(xerror));
    XSync(dpy, 0i32);
}
unsafe extern "C" fn xerrorstart(mut dpy_0: *mut Display,
                                 mut ee: *mut XErrorEvent) -> libc::c_int {
    die(b"dwm: another window manager is already running\x00" as *const u8 as
            *const libc::c_char);
    return -1i32;
}
unsafe extern "C" fn cleanup() {
    let mut a: Arg = Arg{ui: !0i32 as libc::c_uint,};
    let mut foo: Layout =
        Layout{symbol: b"\x00" as *const u8 as *const libc::c_char,
               arrange: None,};
    let mut m: *mut Monitor = 0 as *mut Monitor;
    let mut i: size_t = 0;
    view(&mut a);
    (*selmon).lt[(*selmon).sellt as usize] = &mut foo;
    m = mons;
    while !m.is_null() {
        while !(*m).stack.is_null() { unmanage((*m).stack, 0i32); }
        m = (*m).next
    }
    XUngrabKey(dpy, 0i64 as libc::c_int, (1i32 << 15i32) as libc::c_uint,
               root);
    while !mons.is_null() { cleanupmon(mons); }
    i = 0i32 as size_t;
    while i < CurLast as libc::c_int as libc::c_ulong {
        drw_cur_free(drw, cursor[i as usize]);
        i = i.wrapping_add(1)
    }
    i = 0i32 as size_t;
    while i <
              (::std::mem::size_of::<[[*const libc::c_char; 3]; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<[*const libc::c_char; 3]>()
                                                   as libc::c_ulong) {
        free(*scheme.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    XDestroyWindow(dpy, wmcheckwin);
    drw_free(drw);
    XSync(dpy, 0i32);
    XSetInputFocus(dpy, 1i64 as Window, 1i64 as libc::c_int, 0i64 as Time);
    XDeleteProperty(dpy, root,
                    netatom[NetActiveWindow as libc::c_int as usize]);
}
static mut wmcheckwin: Window = 0;
static mut colors: [[*const libc::c_char; 3]; 2] =
    unsafe {
        [[col_gray3.as_ptr(), col_gray1.as_ptr(), col_gray2.as_ptr()],
         [col_gray4.as_ptr(), col_cyan.as_ptr(), col_cyan.as_ptr()]]
    };
static mut col_gray2: [libc::c_char; 8] = [35, 52, 52, 52, 52, 52, 52, 0];
unsafe extern "C" fn getstate(mut w: Window) -> libc::c_long {
    let mut format: libc::c_int = 0;
    let mut result: libc::c_long = -1i32 as libc::c_long;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut n: libc::c_ulong = 0;
    let mut extra: libc::c_ulong = 0;
    let mut real: Atom = 0;
    if XGetWindowProperty(dpy, w, wmatom[WMState as libc::c_int as usize],
                          0i64, 2i64, 0i32,
                          wmatom[WMState as libc::c_int as usize], &mut real,
                          &mut format, &mut n, &mut extra,
                          &mut p as *mut *mut libc::c_uchar) != 0i32 {
        return -1i32 as libc::c_long
    }
    if n != 0i32 as libc::c_ulong { result = *p as libc::c_long }
    XFree(p as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn run() {
    let mut ev: XEvent = _XEvent{type_0: 0,};
    XSync(dpy, 0i32);
    while 0 != running && 0 == XNextEvent(dpy, &mut ev) {
        if handler[ev.type_0 as usize].is_some() {
            handler[ev.type_0 as
                        usize].expect("non-null function pointer")(&mut ev);
        }
    };
}
unsafe extern "C" fn scan() {
    let mut i: libc::c_uint = 0;
    let mut num: libc::c_uint = 0;
    let mut d1: Window = 0;
    let mut d2: Window = 0;
    let mut wins: *mut Window = 0 as *mut Window;
    let mut wa: XWindowAttributes =
        XWindowAttributes{x: 0,
                          y: 0,
                          width: 0,
                          height: 0,
                          border_width: 0,
                          depth: 0,
                          visual: 0 as *const Visual as *mut Visual,
                          root: 0,
                          class: 0,
                          bit_gravity: 0,
                          win_gravity: 0,
                          backing_store: 0,
                          backing_planes: 0,
                          backing_pixel: 0,
                          save_under: 0,
                          colormap: 0,
                          map_installed: 0,
                          map_state: 0,
                          all_event_masks: 0,
                          your_event_mask: 0,
                          do_not_propagate_mask: 0,
                          override_redirect: 0,
                          screen: 0 as *const Screen as *mut Screen,};
    if 0 != XQueryTree(dpy, root, &mut d1, &mut d2, &mut wins, &mut num) {
        i = 0i32 as libc::c_uint;
        while i < num {
            if !(0 ==
                     XGetWindowAttributes(dpy, *wins.offset(i as isize),
                                          &mut wa) ||
                     0 != wa.override_redirect ||
                     0 !=
                         XGetTransientForHint(dpy, *wins.offset(i as isize),
                                              &mut d1)) {
                if wa.map_state == 2i32 ||
                       getstate(*wins.offset(i as isize)) ==
                           3i32 as libc::c_long {
                    manage(*wins.offset(i as isize), &mut wa);
                }
            }
            i = i.wrapping_add(1)
        }
        i = 0i32 as libc::c_uint;
        while i < num {
            /* now the transients */
            if !(0 ==
                     XGetWindowAttributes(dpy, *wins.offset(i as isize),
                                          &mut wa)) {
                if 0 !=
                       XGetTransientForHint(dpy, *wins.offset(i as isize),
                                            &mut d1) &&
                       (wa.map_state == 2i32 ||
                            getstate(*wins.offset(i as isize)) ==
                                3i32 as libc::c_long) {
                    manage(*wins.offset(i as isize), &mut wa);
                }
            }
            i = i.wrapping_add(1)
        }
        if !wins.is_null() { XFree(wins as *mut libc::c_void); }
    };
}
unsafe extern "C" fn setup() {
    let mut i: libc::c_int = 0;
    let mut wa: XSetWindowAttributes =
        XSetWindowAttributes{background_pixmap: 0,
                             background_pixel: 0,
                             border_pixmap: 0,
                             border_pixel: 0,
                             bit_gravity: 0,
                             win_gravity: 0,
                             backing_store: 0,
                             backing_planes: 0,
                             backing_pixel: 0,
                             save_under: 0,
                             event_mask: 0,
                             do_not_propagate_mask: 0,
                             override_redirect: 0,
                             colormap: 0,
                             cursor: 0,};
    let mut utf8string: Atom = 0;
    sigchld(0i32);
    screen = (*(dpy as _XPrivDisplay)).default_screen;
    sw = (*(*(dpy as _XPrivDisplay)).screens.offset(screen as isize)).width;
    sh = (*(*(dpy as _XPrivDisplay)).screens.offset(screen as isize)).height;
    root = (*(*(dpy as _XPrivDisplay)).screens.offset(screen as isize)).root;
    drw =
        drw_create(dpy, screen, root, sw as libc::c_uint, sh as libc::c_uint);
    if drw_fontset_create(drw, fonts.as_mut_ptr(),
                          (::std::mem::size_of::<[*const libc::c_char; 1]>()
                               as
                               libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                               as
                                                               libc::c_ulong)).is_null()
       {
        die(b"no fonts could be loaded.\x00" as *const u8 as
                *const libc::c_char);
    }
    lrpad = (*(*drw).fonts).h as libc::c_int;
    bh = (*(*drw).fonts).h.wrapping_add(2i32 as libc::c_uint) as libc::c_int;
    updategeom();
    utf8string =
        XInternAtom(dpy,
                    b"UTF8_STRING\x00" as *const u8 as *const libc::c_char,
                    0i32);
    wmatom[WMProtocols as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"WM_PROTOCOLS\x00" as *const u8 as *const libc::c_char,
                    0i32);
    wmatom[WMDelete as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"WM_DELETE_WINDOW\x00" as *const u8 as
                        *const libc::c_char, 0i32);
    wmatom[WMState as libc::c_int as usize] =
        XInternAtom(dpy, b"WM_STATE\x00" as *const u8 as *const libc::c_char,
                    0i32);
    wmatom[WMTakeFocus as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"WM_TAKE_FOCUS\x00" as *const u8 as *const libc::c_char,
                    0i32);
    netatom[NetActiveWindow as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"_NET_ACTIVE_WINDOW\x00" as *const u8 as
                        *const libc::c_char, 0i32);
    netatom[NetSupported as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"_NET_SUPPORTED\x00" as *const u8 as *const libc::c_char,
                    0i32);
    netatom[NetWMName as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"_NET_WM_NAME\x00" as *const u8 as *const libc::c_char,
                    0i32);
    netatom[NetWMState as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"_NET_WM_STATE\x00" as *const u8 as *const libc::c_char,
                    0i32);
    netatom[NetWMCheck as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"_NET_SUPPORTING_WM_CHECK\x00" as *const u8 as
                        *const libc::c_char, 0i32);
    netatom[NetWMFullscreen as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"_NET_WM_STATE_FULLSCREEN\x00" as *const u8 as
                        *const libc::c_char, 0i32);
    netatom[NetWMWindowType as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"_NET_WM_WINDOW_TYPE\x00" as *const u8 as
                        *const libc::c_char, 0i32);
    netatom[NetWMWindowTypeDialog as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"_NET_WM_WINDOW_TYPE_DIALOG\x00" as *const u8 as
                        *const libc::c_char, 0i32);
    netatom[NetClientList as libc::c_int as usize] =
        XInternAtom(dpy,
                    b"_NET_CLIENT_LIST\x00" as *const u8 as
                        *const libc::c_char, 0i32);
    cursor[CurNormal as libc::c_int as usize] = drw_cur_create(drw, 68i32);
    cursor[CurResize as libc::c_int as usize] = drw_cur_create(drw, 120i32);
    cursor[CurMove as libc::c_int as usize] = drw_cur_create(drw, 52i32);
    scheme =
        ecalloc((::std::mem::size_of::<[[*const libc::c_char; 3]; 2]>() as
                     libc::c_ulong).wrapping_div(::std::mem::size_of::<[*const libc::c_char; 3]>()
                                                     as libc::c_ulong),
                ::std::mem::size_of::<*mut Clr>() as libc::c_ulong) as
            *mut *mut Clr;
    i = 0i32;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[[*const libc::c_char; 3]; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<[*const libc::c_char; 3]>()
                                                   as libc::c_ulong) {
        let ref mut fresh3 = *scheme.offset(i as isize);
        *fresh3 =
            drw_scm_create(drw, colors[i as usize].as_mut_ptr(),
                           3i32 as size_t);
        i += 1
    }
    updatebars();
    updatestatus();
    wmcheckwin =
        XCreateSimpleWindow(dpy, root, 0i32, 0i32, 1i32 as libc::c_uint,
                            1i32 as libc::c_uint, 0i32 as libc::c_uint,
                            0i32 as libc::c_ulong, 0i32 as libc::c_ulong);
    XChangeProperty(dpy, wmcheckwin,
                    netatom[NetWMCheck as libc::c_int as usize],
                    33i32 as Atom, 32i32, 0i32,
                    &mut wmcheckwin as *mut Window as *mut libc::c_uchar,
                    1i32);
    XChangeProperty(dpy, wmcheckwin,
                    netatom[NetWMName as libc::c_int as usize], utf8string,
                    8i32, 0i32,
                    b"dwm\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_uchar, 3i32);
    XChangeProperty(dpy, root, netatom[NetWMCheck as libc::c_int as usize],
                    33i32 as Atom, 32i32, 0i32,
                    &mut wmcheckwin as *mut Window as *mut libc::c_uchar,
                    1i32);
    XChangeProperty(dpy, root, netatom[NetSupported as libc::c_int as usize],
                    4i32 as Atom, 32i32, 0i32,
                    netatom.as_mut_ptr() as *mut libc::c_uchar,
                    NetLast as libc::c_int);
    XDeleteProperty(dpy, root,
                    netatom[NetClientList as libc::c_int as usize]);
    wa.cursor = (*cursor[CurNormal as libc::c_int as usize]).cursor;
    wa.event_mask =
        1i64 << 20i32 | 1i64 << 19i32 | 1i64 << 2i32 | 1i64 << 6i32 |
            1i64 << 4i32 | 1i64 << 5i32 | 1i64 << 17i32 | 1i64 << 22i32;
    XChangeWindowAttributes(dpy, root,
                            (1i64 << 11i32 | 1i64 << 14i32) as libc::c_ulong,
                            &mut wa);
    XSelectInput(dpy, root, wa.event_mask);
    grabkeys();
    focus(0 as *mut Client);
}
static mut fonts: [*const libc::c_char; 1] =
    [b"monospace:size=10\x00" as *const u8 as *const libc::c_char];
unsafe extern "C" fn sigchld(mut unused: libc::c_int) {
    if signal(17i32, Some(sigchld)) ==
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(-1i32 as libc::intptr_t) {
        die(b"can\'t install SIGCHLD handler:\x00" as *const u8 as
                *const libc::c_char);
    }
    while 0i32 < waitpid(-1i32, 0 as *mut libc::c_int, 1i32) { };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    if argc == 2i32 &&
           0 ==
               strcmp(b"-v\x00" as *const u8 as *const libc::c_char,
                      *argv.offset(1isize)) {
        die(b"dwm-6.2\x00" as *const u8 as *const libc::c_char);
    } else if argc != 1i32 {
        die(b"usage: dwm [-v]\x00" as *const u8 as *const libc::c_char);
    }
    if setlocale(0i32, b"\x00" as *const u8 as *const libc::c_char).is_null()
           || 0 == XSupportsLocale() {
        fputs(b"warning: no locale support\n\x00" as *const u8 as
                  *const libc::c_char, stderr);
    }
    dpy = XOpenDisplay(0 as *const libc::c_char);
    if dpy.is_null() {
        die(b"dwm: cannot open display\x00" as *const u8 as
                *const libc::c_char);
    }
    checkotherwm();
    setup();
    scan();
    run();
    cleanup();
    XCloseDisplay(dpy);
    return 0i32;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    buttons =
        {
            [Button{click: ClkLtSymbol as libc::c_int as libc::c_uint,
                    mask: 0i32 as libc::c_uint,
                    button: 1i32 as libc::c_uint,
                    func: Some(setlayout),
                    arg: Arg{i: 0i32,},},
             Button{click: ClkLtSymbol as libc::c_int as libc::c_uint,
                    mask: 0i32 as libc::c_uint,
                    button: 3i32 as libc::c_uint,
                    func: Some(setlayout),
                    arg:
                        Arg{v:
                                &*layouts.as_ptr().offset(2isize) as
                                    *const Layout as *const libc::c_void,},},
             Button{click: ClkWinTitle as libc::c_int as libc::c_uint,
                    mask: 0i32 as libc::c_uint,
                    button: 2i32 as libc::c_uint,
                    func: Some(zoom),
                    arg: Arg{i: 0i32,},},
             Button{click: ClkStatusText as libc::c_int as libc::c_uint,
                    mask: 0i32 as libc::c_uint,
                    button: 2i32 as libc::c_uint,
                    func: Some(spawn),
                    arg:
                        Arg{v:
                                termcmd.as_mut_ptr() as
                                    *const libc::c_void,},},
             Button{click: ClkClientWin as libc::c_int as libc::c_uint,
                    mask: (1i32 << 3i32) as libc::c_uint,
                    button: 1i32 as libc::c_uint,
                    func: Some(movemouse),
                    arg: Arg{i: 0i32,},},
             Button{click: ClkClientWin as libc::c_int as libc::c_uint,
                    mask: (1i32 << 3i32) as libc::c_uint,
                    button: 2i32 as libc::c_uint,
                    func: Some(togglefloating),
                    arg: Arg{i: 0i32,},},
             Button{click: ClkClientWin as libc::c_int as libc::c_uint,
                    mask: (1i32 << 3i32) as libc::c_uint,
                    button: 3i32 as libc::c_uint,
                    func: Some(resizemouse),
                    arg: Arg{i: 0i32,},},
             Button{click: ClkTagBar as libc::c_int as libc::c_uint,
                    mask: 0i32 as libc::c_uint,
                    button: 1i32 as libc::c_uint,
                    func: Some(view),
                    arg: Arg{i: 0i32,},},
             Button{click: ClkTagBar as libc::c_int as libc::c_uint,
                    mask: 0i32 as libc::c_uint,
                    button: 3i32 as libc::c_uint,
                    func: Some(toggleview),
                    arg: Arg{i: 0i32,},},
             Button{click: ClkTagBar as libc::c_int as libc::c_uint,
                    mask: (1i32 << 3i32) as libc::c_uint,
                    button: 1i32 as libc::c_uint,
                    func: Some(tag),
                    arg: Arg{i: 0i32,},},
             Button{click: ClkTagBar as libc::c_int as libc::c_uint,
                    mask: (1i32 << 3i32) as libc::c_uint,
                    button: 3i32 as libc::c_uint,
                    func: Some(toggletag),
                    arg: Arg{i: 0i32,},}]
        };
    keys =
        {
            [Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x70i32 as KeySym,
                 func: Some(spawn),
                 arg: Arg{v: dmenucmd.as_mut_ptr() as *const libc::c_void,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0xff0di32 as KeySym,
                 func: Some(spawn),
                 arg: Arg{v: termcmd.as_mut_ptr() as *const libc::c_void,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x62i32 as KeySym,
                 func: Some(togglebar),
                 arg: Arg{i: 0i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x6ai32 as KeySym,
                 func: Some(focusstack),
                 arg: Arg{i: 1i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x6bi32 as KeySym,
                 func: Some(focusstack),
                 arg: Arg{i: -1i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x69i32 as KeySym,
                 func: Some(incnmaster),
                 arg: Arg{i: 1i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x64i32 as KeySym,
                 func: Some(incnmaster),
                 arg: Arg{i: -1i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x68i32 as KeySym,
                 func: Some(setmfact),
                 arg: Arg{f: -0.05f64 as libc::c_float,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x6ci32 as KeySym,
                 func: Some(setmfact),
                 arg: Arg{f: 0.05f64 as libc::c_float,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0xff0di32 as KeySym,
                 func: Some(zoom),
                 arg: Arg{i: 0i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0xff09i32 as KeySym,
                 func: Some(view),
                 arg: Arg{i: 0i32,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x63i32 as KeySym,
                 func: Some(killclient),
                 arg: Arg{i: 0i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x74i32 as KeySym,
                 func: Some(setlayout),
                 arg:
                     Arg{v:
                             &*layouts.as_ptr().offset(0isize) as
                                 *const Layout as *const libc::c_void,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x66i32 as KeySym,
                 func: Some(setlayout),
                 arg:
                     Arg{v:
                             &*layouts.as_ptr().offset(1isize) as
                                 *const Layout as *const libc::c_void,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x6di32 as KeySym,
                 func: Some(setlayout),
                 arg:
                     Arg{v:
                             &*layouts.as_ptr().offset(2isize) as
                                 *const Layout as *const libc::c_void,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x20i32 as KeySym,
                 func: Some(setlayout),
                 arg: Arg{i: 0i32,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x20i32 as KeySym,
                 func: Some(togglefloating),
                 arg: Arg{i: 0i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x30i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: !0i32 as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x30i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: !0i32 as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x2ci32 as KeySym,
                 func: Some(focusmon),
                 arg: Arg{i: -1i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x2ei32 as KeySym,
                 func: Some(focusmon),
                 arg: Arg{i: 1i32,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x2ci32 as KeySym,
                 func: Some(tagmon),
                 arg: Arg{i: -1i32,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x2ei32 as KeySym,
                 func: Some(tagmon),
                 arg: Arg{i: 1i32,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x31i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: (1i32 << 0i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 2i32) as libc::c_uint,
                 keysym: 0x31i32 as KeySym,
                 func: Some(toggleview),
                 arg: Arg{ui: (1i32 << 0i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x31i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: (1i32 << 0i32) as libc::c_uint,},},
             Key{mod_0:
                     (1i32 << 3i32 | 1i32 << 2i32 | 1i32 << 0i32) as
                         libc::c_uint,
                 keysym: 0x31i32 as KeySym,
                 func: Some(toggletag),
                 arg: Arg{ui: (1i32 << 0i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x32i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: (1i32 << 1i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 2i32) as libc::c_uint,
                 keysym: 0x32i32 as KeySym,
                 func: Some(toggleview),
                 arg: Arg{ui: (1i32 << 1i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x32i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: (1i32 << 1i32) as libc::c_uint,},},
             Key{mod_0:
                     (1i32 << 3i32 | 1i32 << 2i32 | 1i32 << 0i32) as
                         libc::c_uint,
                 keysym: 0x32i32 as KeySym,
                 func: Some(toggletag),
                 arg: Arg{ui: (1i32 << 1i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x33i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: (1i32 << 2i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 2i32) as libc::c_uint,
                 keysym: 0x33i32 as KeySym,
                 func: Some(toggleview),
                 arg: Arg{ui: (1i32 << 2i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x33i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: (1i32 << 2i32) as libc::c_uint,},},
             Key{mod_0:
                     (1i32 << 3i32 | 1i32 << 2i32 | 1i32 << 0i32) as
                         libc::c_uint,
                 keysym: 0x33i32 as KeySym,
                 func: Some(toggletag),
                 arg: Arg{ui: (1i32 << 2i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x34i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: (1i32 << 3i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 2i32) as libc::c_uint,
                 keysym: 0x34i32 as KeySym,
                 func: Some(toggleview),
                 arg: Arg{ui: (1i32 << 3i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x34i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: (1i32 << 3i32) as libc::c_uint,},},
             Key{mod_0:
                     (1i32 << 3i32 | 1i32 << 2i32 | 1i32 << 0i32) as
                         libc::c_uint,
                 keysym: 0x34i32 as KeySym,
                 func: Some(toggletag),
                 arg: Arg{ui: (1i32 << 3i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x35i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: (1i32 << 4i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 2i32) as libc::c_uint,
                 keysym: 0x35i32 as KeySym,
                 func: Some(toggleview),
                 arg: Arg{ui: (1i32 << 4i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x35i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: (1i32 << 4i32) as libc::c_uint,},},
             Key{mod_0:
                     (1i32 << 3i32 | 1i32 << 2i32 | 1i32 << 0i32) as
                         libc::c_uint,
                 keysym: 0x35i32 as KeySym,
                 func: Some(toggletag),
                 arg: Arg{ui: (1i32 << 4i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x36i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: (1i32 << 5i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 2i32) as libc::c_uint,
                 keysym: 0x36i32 as KeySym,
                 func: Some(toggleview),
                 arg: Arg{ui: (1i32 << 5i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x36i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: (1i32 << 5i32) as libc::c_uint,},},
             Key{mod_0:
                     (1i32 << 3i32 | 1i32 << 2i32 | 1i32 << 0i32) as
                         libc::c_uint,
                 keysym: 0x36i32 as KeySym,
                 func: Some(toggletag),
                 arg: Arg{ui: (1i32 << 5i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x37i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: (1i32 << 6i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 2i32) as libc::c_uint,
                 keysym: 0x37i32 as KeySym,
                 func: Some(toggleview),
                 arg: Arg{ui: (1i32 << 6i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x37i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: (1i32 << 6i32) as libc::c_uint,},},
             Key{mod_0:
                     (1i32 << 3i32 | 1i32 << 2i32 | 1i32 << 0i32) as
                         libc::c_uint,
                 keysym: 0x37i32 as KeySym,
                 func: Some(toggletag),
                 arg: Arg{ui: (1i32 << 6i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x38i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: (1i32 << 7i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 2i32) as libc::c_uint,
                 keysym: 0x38i32 as KeySym,
                 func: Some(toggleview),
                 arg: Arg{ui: (1i32 << 7i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x38i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: (1i32 << 7i32) as libc::c_uint,},},
             Key{mod_0:
                     (1i32 << 3i32 | 1i32 << 2i32 | 1i32 << 0i32) as
                         libc::c_uint,
                 keysym: 0x38i32 as KeySym,
                 func: Some(toggletag),
                 arg: Arg{ui: (1i32 << 7i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32) as libc::c_uint,
                 keysym: 0x39i32 as KeySym,
                 func: Some(view),
                 arg: Arg{ui: (1i32 << 8i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 2i32) as libc::c_uint,
                 keysym: 0x39i32 as KeySym,
                 func: Some(toggleview),
                 arg: Arg{ui: (1i32 << 8i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x39i32 as KeySym,
                 func: Some(tag),
                 arg: Arg{ui: (1i32 << 8i32) as libc::c_uint,},},
             Key{mod_0:
                     (1i32 << 3i32 | 1i32 << 2i32 | 1i32 << 0i32) as
                         libc::c_uint,
                 keysym: 0x39i32 as KeySym,
                 func: Some(toggletag),
                 arg: Arg{ui: (1i32 << 8i32) as libc::c_uint,},},
             Key{mod_0: (1i32 << 3i32 | 1i32 << 0i32) as libc::c_uint,
                 keysym: 0x71i32 as KeySym,
                 func: Some(quit),
                 arg: Arg{i: 0i32,},}]
        }
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];