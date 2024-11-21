use ffi::AstalNotifdAction;
use glib::boxed::Boxed;
use glib::ffi::g_array_get_element_size;
use glib::object::IsA;
use glib::translate::*;

use crate::Action;

pub trait ActionExtManual: 'static {
    fn label(&self) -> glib::GString;
    fn id(&self) -> glib::GString;
}

impl ActionExtManual for Action {
    fn label(&self) -> glib::GString {
        unsafe {
            // get access to inner record
            let action = *self.as_ptr();
            let label_ptr = action.label;
            let label = glib::GString::from_glib_none(label_ptr);
            label
        }
    }

    fn id(&self) -> glib::GString {
        unsafe {
            // get access to inner record
            let action = *self.as_ptr();
            let id_ptr = action.id;
            let id = glib::GString::from_glib_none(id_ptr);
            id
        }
    }
}
