// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir-astal
// from ../../gir-files
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use gtk_sys as gtk;
use gdk_sys as gdk;
use gio_sys as gio;
use glib_sys as glib;

#[allow(unused_imports)]
use std::ffi::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong, c_void};
#[allow(unused_imports)]
use libc::{size_t, ssize_t, time_t, off_t, intptr_t, uintptr_t, FILE};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type AstalExclusivity = c_int;
pub const ASTAL_EXCLUSIVITY_NORMAL: AstalExclusivity = 0;
pub const ASTAL_EXCLUSIVITY_EXCLUSIVE: AstalExclusivity = 1;
pub const ASTAL_EXCLUSIVITY_IGNORE: AstalExclusivity = 2;

pub type AstalKeymode = c_int;
pub const ASTAL_KEYMODE_NONE: AstalKeymode = 0;
pub const ASTAL_KEYMODE_EXCLUSIVE: AstalKeymode = 1;
pub const ASTAL_KEYMODE_ON_DEMAND: AstalKeymode = 2;

pub type AstalLayer = c_int;
pub const ASTAL_LAYER_BACKGROUND: AstalLayer = 0;
pub const ASTAL_LAYER_BOTTOM: AstalLayer = 1;
pub const ASTAL_LAYER_TOP: AstalLayer = 2;
pub const ASTAL_LAYER_OVERLAY: AstalLayer = 3;

// Constants
pub const ASTAL_MAJOR_VERSION: c_int = 4;
pub const ASTAL_MINOR_VERSION: c_int = 0;
pub const ASTAL_MICRO_VERSION: c_int = 0;

// Flags
pub type AstalWindowAnchor = c_uint;
pub const ASTAL_WINDOW_ANCHOR_NONE: AstalWindowAnchor = 1;
pub const ASTAL_WINDOW_ANCHOR_TOP: AstalWindowAnchor = 2;
pub const ASTAL_WINDOW_ANCHOR_RIGHT: AstalWindowAnchor = 4;
pub const ASTAL_WINDOW_ANCHOR_LEFT: AstalWindowAnchor = 8;
pub const ASTAL_WINDOW_ANCHOR_BOTTOM: AstalWindowAnchor = 16;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalApplicationClass {
    pub parent_class: gtk::GtkApplicationClass,
    pub request: Option<unsafe extern "C" fn(*mut AstalApplication, *const c_char, *mut gio::GSocketConnection)>,
}

impl ::std::fmt::Debug for AstalApplicationClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalApplicationClass @ {self:p}"))
         .field("request", &self.request)
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalApplicationPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalApplicationPrivate = _AstalApplicationPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBoxClass {
    pub parent_class: gtk::GtkBoxClass,
}

impl ::std::fmt::Debug for AstalBoxClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBoxClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalBoxPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalBoxPrivate = _AstalBoxPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalSliderClass {
    pub parent_class: gtk::GtkScaleClass,
}

impl ::std::fmt::Debug for AstalSliderClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalSliderClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalSliderPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalSliderPrivate = _AstalSliderPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWindowClass {
    pub parent_class: gtk::GtkWindowClass,
}

impl ::std::fmt::Debug for AstalWindowClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWindowClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalWindowPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalWindowPrivate = _AstalWindowPrivate;

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalApplication {
    pub parent_instance: gtk::GtkApplication,
    pub priv_: *mut AstalApplicationPrivate,
}

impl ::std::fmt::Debug for AstalApplication {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalApplication @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBox {
    pub parent_instance: gtk::GtkBox,
    pub priv_: *mut AstalBoxPrivate,
}

impl ::std::fmt::Debug for AstalBox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBox @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalSlider {
    pub parent_instance: gtk::GtkScale,
    pub priv_: *mut AstalSliderPrivate,
}

impl ::std::fmt::Debug for AstalSlider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalSlider @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWindow {
    pub parent_instance: gtk::GtkWindow,
    pub priv_: *mut AstalWindowPrivate,
}

impl ::std::fmt::Debug for AstalWindow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWindow @ {self:p}"))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // AstalExclusivity
    //=========================================================================
    pub fn astal_exclusivity_get_type() -> GType;

    //=========================================================================
    // AstalKeymode
    //=========================================================================
    pub fn astal_keymode_get_type() -> GType;

    //=========================================================================
    // AstalLayer
    //=========================================================================
    pub fn astal_layer_get_type() -> GType;

    //=========================================================================
    // AstalWindowAnchor
    //=========================================================================
    pub fn astal_window_anchor_get_type() -> GType;

    //=========================================================================
    // AstalApplication
    //=========================================================================
    pub fn astal_application_get_type() -> GType;
    pub fn astal_application_reset_css(self_: *mut AstalApplication);
    pub fn astal_application_get_window(self_: *mut AstalApplication, name: *const c_char) -> *mut gtk::GtkWindow;
    pub fn astal_application_apply_css(self_: *mut AstalApplication, style: *const c_char, reset: gboolean);
    pub fn astal_application_add_icons(self_: *mut AstalApplication, path: *const c_char);
    pub fn astal_application_request(self_: *mut AstalApplication, msg: *const c_char, conn: *mut gio::GSocketConnection);
    pub fn astal_application_new() -> *mut AstalApplication;
    pub fn astal_application_get_monitors(self_: *mut AstalApplication) -> *mut glib::GList;
    pub fn astal_application_get_windows(self_: *mut AstalApplication) -> *mut glib::GList;
    pub fn astal_application_get_gtk_theme(self_: *mut AstalApplication) -> *mut c_char;
    pub fn astal_application_set_gtk_theme(self_: *mut AstalApplication, value: *const c_char);
    pub fn astal_application_get_icon_theme(self_: *mut AstalApplication) -> *mut c_char;
    pub fn astal_application_set_icon_theme(self_: *mut AstalApplication, value: *const c_char);
    pub fn astal_application_get_cursor_theme(self_: *mut AstalApplication) -> *mut c_char;
    pub fn astal_application_set_cursor_theme(self_: *mut AstalApplication, value: *const c_char);

    //=========================================================================
    // AstalBox
    //=========================================================================
    pub fn astal_box_get_type() -> GType;
    pub fn astal_box_new() -> *mut AstalBox;
    pub fn astal_box_get_vertical(self_: *mut AstalBox) -> gboolean;
    pub fn astal_box_set_vertical(self_: *mut AstalBox, value: gboolean);
    pub fn astal_box_get_children(self_: *mut AstalBox) -> *mut glib::GList;
    pub fn astal_box_set_children(self_: *mut AstalBox, value: *mut glib::GList);
    pub fn astal_box_get_child(self_: *mut AstalBox) -> *mut gtk::GtkWidget;
    pub fn astal_box_set_child(self_: *mut AstalBox, value: *mut gtk::GtkWidget);

    //=========================================================================
    // AstalSlider
    //=========================================================================
    pub fn astal_slider_get_type() -> GType;
    pub fn astal_slider_new() -> *mut AstalSlider;
    pub fn astal_slider_get_value(self_: *mut AstalSlider) -> c_double;
    pub fn astal_slider_set_value(self_: *mut AstalSlider, value: c_double);
    pub fn astal_slider_get_min(self_: *mut AstalSlider) -> c_double;
    pub fn astal_slider_set_min(self_: *mut AstalSlider, value: c_double);
    pub fn astal_slider_get_max(self_: *mut AstalSlider) -> c_double;
    pub fn astal_slider_set_max(self_: *mut AstalSlider, value: c_double);
    pub fn astal_slider_get_step(self_: *mut AstalSlider) -> c_double;
    pub fn astal_slider_set_step(self_: *mut AstalSlider, value: c_double);

    //=========================================================================
    // AstalWindow
    //=========================================================================
    pub fn astal_window_get_type() -> GType;
    pub fn astal_window_new() -> *mut AstalWindow;
    pub fn astal_window_get_namespace(self_: *mut AstalWindow) -> *const c_char;
    pub fn astal_window_set_namespace(self_: *mut AstalWindow, value: *const c_char);
    pub fn astal_window_get_anchor(self_: *mut AstalWindow) -> AstalWindowAnchor;
    pub fn astal_window_set_anchor(self_: *mut AstalWindow, value: AstalWindowAnchor);
    pub fn astal_window_get_exclusivity(self_: *mut AstalWindow) -> AstalExclusivity;
    pub fn astal_window_set_exclusivity(self_: *mut AstalWindow, value: AstalExclusivity);
    pub fn astal_window_get_layer(self_: *mut AstalWindow) -> AstalLayer;
    pub fn astal_window_set_layer(self_: *mut AstalWindow, value: AstalLayer);
    pub fn astal_window_get_keymode(self_: *mut AstalWindow) -> AstalKeymode;
    pub fn astal_window_set_keymode(self_: *mut AstalWindow, value: AstalKeymode);
    pub fn astal_window_get_gdkmonitor(self_: *mut AstalWindow) -> *mut gdk::GdkMonitor;
    pub fn astal_window_set_gdkmonitor(self_: *mut AstalWindow, value: *mut gdk::GdkMonitor);
    pub fn astal_window_get_margin_top(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_margin_top(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_get_margin_bottom(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_margin_bottom(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_get_margin_left(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_margin_left(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_get_margin_right(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_margin_right(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_set_margin(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_get_monitor(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_monitor(self_: *mut AstalWindow, value: c_int);

}
