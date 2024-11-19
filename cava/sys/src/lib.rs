// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir-astal
// from ../../gir-files
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use gobject_sys as gobject;
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
pub type AstalCavaInput = c_int;
pub const ASTAL_CAVA_INPUT_FIFO: AstalCavaInput = 0;
pub const ASTAL_CAVA_INPUT_PORTAUDIO: AstalCavaInput = 1;
pub const ASTAL_CAVA_INPUT_PIPEWIRE: AstalCavaInput = 2;
pub const ASTAL_CAVA_INPUT_ALSA: AstalCavaInput = 3;
pub const ASTAL_CAVA_INPUT_PULSE: AstalCavaInput = 4;
pub const ASTAL_CAVA_INPUT_SNDIO: AstalCavaInput = 5;
pub const ASTAL_CAVA_INPUT_OSS: AstalCavaInput = 6;
pub const ASTAL_CAVA_INPUT_JACK: AstalCavaInput = 7;
pub const ASTAL_CAVA_INPUT_SHMEM: AstalCavaInput = 8;
pub const ASTAL_CAVA_INPUT_WINSCAP: AstalCavaInput = 9;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalCavaCavaClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalCavaCavaClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalCavaCavaClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

// Classes
#[repr(C)]
#[allow(dead_code)]
pub struct AstalCavaCava {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalCavaCava {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalCavaCava @ {self:p}"))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // AstalCavaCava
    //=========================================================================
    pub fn astal_cava_cava_get_type() -> GType;
    pub fn astal_cava_cava_get_default() -> *mut AstalCavaCava;
    pub fn astal_cava_cava_get_active(self_: *mut AstalCavaCava) -> gboolean;
    pub fn astal_cava_cava_get_autosens(self_: *mut AstalCavaCava) -> gboolean;
    pub fn astal_cava_cava_get_bars(self_: *mut AstalCavaCava) -> c_int;
    pub fn astal_cava_cava_get_channels(self_: *mut AstalCavaCava) -> c_int;
    pub fn astal_cava_cava_get_framerate(self_: *mut AstalCavaCava) -> c_int;
    pub fn astal_cava_cava_get_high_cutoff(self_: *mut AstalCavaCava) -> c_int;
    pub fn astal_cava_cava_get_input(self_: *mut AstalCavaCava) -> AstalCavaInput;
    pub fn astal_cava_cava_get_low_cutoff(self_: *mut AstalCavaCava) -> c_int;
    pub fn astal_cava_cava_get_noise_reduction(self_: *mut AstalCavaCava) -> c_double;
    pub fn astal_cava_cava_get_samplerate(self_: *mut AstalCavaCava) -> c_int;
    pub fn astal_cava_cava_get_source(self_: *mut AstalCavaCava) -> *mut c_char;
    pub fn astal_cava_cava_get_stereo(self_: *mut AstalCavaCava) -> gboolean;
    pub fn astal_cava_cava_get_values(self_: *mut AstalCavaCava) -> *mut glib::GArray;
    pub fn astal_cava_cava_set_active(self_: *mut AstalCavaCava, active: gboolean);
    pub fn astal_cava_cava_set_autosens(self_: *mut AstalCavaCava, autosens: gboolean);
    pub fn astal_cava_cava_set_bars(self_: *mut AstalCavaCava, bars: c_int);
    pub fn astal_cava_cava_set_channels(self_: *mut AstalCavaCava, channels: c_int);
    pub fn astal_cava_cava_set_framerate(self_: *mut AstalCavaCava, framerate: c_int);
    pub fn astal_cava_cava_set_high_cutoff(self_: *mut AstalCavaCava, high_cutoff: c_int);
    pub fn astal_cava_cava_set_input(self_: *mut AstalCavaCava, input: AstalCavaInput);
    pub fn astal_cava_cava_set_low_cutoff(self_: *mut AstalCavaCava, low_cutoff: c_int);
    pub fn astal_cava_cava_set_noise_reduction(self_: *mut AstalCavaCava, noise: c_double);
    pub fn astal_cava_cava_set_samplerate(self_: *mut AstalCavaCava, samplerate: c_int);
    pub fn astal_cava_cava_set_source(self_: *mut AstalCavaCava, source: *const c_char);
    pub fn astal_cava_cava_set_stereo(self_: *mut AstalCavaCava, stereo: gboolean);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn astal_cava_get_default() -> *mut AstalCavaCava;

}
