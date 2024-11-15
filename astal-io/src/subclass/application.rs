use gio::SocketConnection;
use glib::{
    object::Cast,
    subclass::{
        object::ObjectImpl,
        types::{
            InstanceStructExt, IsImplementable, IsSubclassable, ObjectSubclass, ObjectSubclassExt,
        },
    },
    translate::{from_glib_borrow, from_glib_full, from_glib_none, Borrowed, ToGlibPtr},
};

use crate::Application;

pub trait AstalIOApplicationImpl: AstalIOApplicationImplExt + ObjectImpl {
    fn quit(&self) {}
    fn inspector(&self) {}
    fn toggle_window(&self, window: &str) {}
    fn request(&self, msg: &str, conn: &SocketConnection) -> Result<(), glib::Error> {
        self.parent_request(msg, conn)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AstalIOApplicationImpl> Sealed for T {}
}

pub trait AstalIOApplicationImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_request(&self, msg: &str, conn: &SocketConnection) -> Result<(), glib::Error> {
        unsafe {
            let data = Self::type_data();
            let parent_iface = data.as_ref().parent_interface::<Application>()
                as *mut ffi::AstalIOApplicationIface;

            let mut error = std::ptr::null_mut();

            if let Some(f) = (*parent_iface).request {
                f(
                    self.obj().unsafe_cast_ref::<Application>().to_glib_none().0,
                    msg.to_glib_none().0,
                    conn.to_glib_none().0,
                    &mut error,
                )
            }

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<T: AstalIOApplicationImpl> AstalIOApplicationImplExt for T {}

unsafe impl<T: AstalIOApplicationImpl> IsImplementable<T> for Application {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.request = Some(application_request::<T>);
    }
}

unsafe extern "C" fn application_request<T: AstalIOApplicationImpl>(
    application: *mut ffi::AstalIOApplication,
    msg: *const libc::c_char,
    conn: *mut gio::ffi::GSocketConnection,
    error: *mut *mut glib::ffi::GError,
) {
    let instance = &*(application as *mut T::Instance);
    let imp = instance.imp();

    let msg: Borrowed<glib::GString> = from_glib_borrow(msg);

    match imp.request(msg.as_ref(), &from_glib_borrow(conn)) {
        Ok(_) => {}
        Err(err) => {
            *error = err.as_ptr();
        }
    }
}
