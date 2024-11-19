use ffi::AstalCavaCava;
use glib::boxed::Boxed;
use glib::ffi::g_array_get_element_size;
use glib::object::{IsA, ObjectType};
use glib::translate::*;

use crate::Cava;

pub trait CavaExtManual: 'static {
    fn values(&self) -> Vec<f64>;
}

impl<O: IsA<Cava> + ObjectType<GlibType = AstalCavaCava>> CavaExtManual for O {
    fn values(&self) -> Vec<f64> {
        unsafe {
            let garray = ffi::astal_cava_cava_get_values(self.to_glib_none().0);
            let val = *garray;
            let slice: &[f64] =
                std::slice::from_raw_parts(val.data as *const f64, val.len as usize);
            slice.to_vec()
        }
    }
}
