// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-astal
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,translate::*};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AstalMprisLoop")]
pub enum Loop {
    #[doc(alias = "ASTAL_MPRIS_LOOP_UNSUPPORTED")]
    Unsupported,
    #[doc(alias = "ASTAL_MPRIS_LOOP_NONE")]
    None,
    #[doc(alias = "ASTAL_MPRIS_LOOP_TRACK")]
    Track,
    #[doc(alias = "ASTAL_MPRIS_LOOP_PLAYLIST")]
    Playlist,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for Loop {
    type GlibType = ffi::AstalMprisLoop;

    #[inline]
fn into_glib(self) -> ffi::AstalMprisLoop {
match self {
            Self::Unsupported => ffi::ASTAL_MPRIS_LOOP_UNSUPPORTED,
            Self::None => ffi::ASTAL_MPRIS_LOOP_NONE,
            Self::Track => ffi::ASTAL_MPRIS_LOOP_TRACK,
            Self::Playlist => ffi::ASTAL_MPRIS_LOOP_PLAYLIST,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::AstalMprisLoop> for Loop {
    #[inline]
unsafe fn from_glib(value: ffi::AstalMprisLoop) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::ASTAL_MPRIS_LOOP_UNSUPPORTED => Self::Unsupported,
            ffi::ASTAL_MPRIS_LOOP_NONE => Self::None,
            ffi::ASTAL_MPRIS_LOOP_TRACK => Self::Track,
            ffi::ASTAL_MPRIS_LOOP_PLAYLIST => Self::Playlist,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for Loop {
                #[inline]
    #[doc(alias = "astal_mpris_loop_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_mpris_loop_get_type()) }
                }
            }

impl glib::HasParamSpec for Loop {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for Loop {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Loop {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Loop {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Loop> for glib::Value {
    #[inline]
    fn from(v: Loop) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AstalMprisPlaybackStatus")]
pub enum PlaybackStatus {
    #[doc(alias = "ASTAL_MPRIS_PLAYBACK_STATUS_PLAYING")]
    Playing,
    #[doc(alias = "ASTAL_MPRIS_PLAYBACK_STATUS_PAUSED")]
    Paused,
    #[doc(alias = "ASTAL_MPRIS_PLAYBACK_STATUS_STOPPED")]
    Stopped,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for PlaybackStatus {
    type GlibType = ffi::AstalMprisPlaybackStatus;

    #[inline]
fn into_glib(self) -> ffi::AstalMprisPlaybackStatus {
match self {
            Self::Playing => ffi::ASTAL_MPRIS_PLAYBACK_STATUS_PLAYING,
            Self::Paused => ffi::ASTAL_MPRIS_PLAYBACK_STATUS_PAUSED,
            Self::Stopped => ffi::ASTAL_MPRIS_PLAYBACK_STATUS_STOPPED,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::AstalMprisPlaybackStatus> for PlaybackStatus {
    #[inline]
unsafe fn from_glib(value: ffi::AstalMprisPlaybackStatus) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::ASTAL_MPRIS_PLAYBACK_STATUS_PLAYING => Self::Playing,
            ffi::ASTAL_MPRIS_PLAYBACK_STATUS_PAUSED => Self::Paused,
            ffi::ASTAL_MPRIS_PLAYBACK_STATUS_STOPPED => Self::Stopped,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for PlaybackStatus {
                #[inline]
    #[doc(alias = "astal_mpris_playback_status_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_mpris_playback_status_get_type()) }
                }
            }

impl glib::HasParamSpec for PlaybackStatus {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for PlaybackStatus {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for PlaybackStatus {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PlaybackStatus {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PlaybackStatus> for glib::Value {
    #[inline]
    fn from(v: PlaybackStatus) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "AstalMprisShuffle")]
pub enum Shuffle {
    #[doc(alias = "ASTAL_MPRIS_SHUFFLE_UNSUPPORTED")]
    Unsupported,
    #[doc(alias = "ASTAL_MPRIS_SHUFFLE_ON")]
    On,
    #[doc(alias = "ASTAL_MPRIS_SHUFFLE_OFF")]
    Off,
#[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for Shuffle {
    type GlibType = ffi::AstalMprisShuffle;

    #[inline]
fn into_glib(self) -> ffi::AstalMprisShuffle {
match self {
            Self::Unsupported => ffi::ASTAL_MPRIS_SHUFFLE_UNSUPPORTED,
            Self::On => ffi::ASTAL_MPRIS_SHUFFLE_ON,
            Self::Off => ffi::ASTAL_MPRIS_SHUFFLE_OFF,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::AstalMprisShuffle> for Shuffle {
    #[inline]
unsafe fn from_glib(value: ffi::AstalMprisShuffle) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::ASTAL_MPRIS_SHUFFLE_UNSUPPORTED => Self::Unsupported,
            ffi::ASTAL_MPRIS_SHUFFLE_ON => Self::On,
            ffi::ASTAL_MPRIS_SHUFFLE_OFF => Self::Off,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for Shuffle {
                #[inline]
    #[doc(alias = "astal_mpris_shuffle_get_type")]
   fn static_type() -> glib::Type {
                    unsafe { from_glib(ffi::astal_mpris_shuffle_get_type()) }
                }
            }

impl glib::HasParamSpec for Shuffle {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    Self::ParamSpec::builder_with_default
                }
}

impl glib::value::ValueType for Shuffle {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Shuffle {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Shuffle {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Shuffle> for glib::Value {
    #[inline]
    fn from(v: Shuffle) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

