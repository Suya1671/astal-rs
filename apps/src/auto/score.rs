// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-astal
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};

glib::wrapper! {
    pub struct Score(BoxedInline<ffi::AstalAppsScore>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::astal_apps_score_get_type(), ptr as *mut _) as *mut ffi::AstalAppsScore,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::astal_apps_score_get_type(), ptr as *mut _),
        type_ => || ffi::astal_apps_score_get_type(),
    }
}
