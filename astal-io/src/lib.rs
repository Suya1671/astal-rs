#![allow(unused_imports, dead_code, unused_braces)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// No-op.
macro_rules! assert_initialized_main_thread {
    () => {};
}

pub use auto::*;
use ffi;
mod auto;
pub mod subclass;

pub mod prelude {
    pub use crate::auto::traits::*;
}

pub mod functions {
    use glib::{
        object::IsA,
        translate::{from_glib_full, FromGlibPtrNone, ToGlibPtr},
        GString,
    };

    pub use super::auto::functions::*;

    #[doc(alias = "astal_io_write_sock")]
    pub fn write_sock<Q: FnOnce(Result<(), glib::Error>) + 'static>(
        conn: &impl IsA<gio::SocketConnection>,
        response: &str,
        callback: Q,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box<glib::thread_guard::ThreadGuard<Q>> =
            Box::new(glib::thread_guard::ThreadGuard::new(callback));

        unsafe extern "C" fn trampoline<Q: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            result: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_io_write_sock_finish(result, &mut error);

            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };

            let callback: Box<glib::thread_guard::ThreadGuard<Q>> =
                Box::from_raw(user_data as *mut _);

            let callback = callback.into_inner();

            callback(result);
        }

        unsafe {
            ffi::astal_io_write_sock(
                conn.as_ref().to_glib_none().0,
                response.to_glib_none().0,
                Some(trampoline::<Q>),
                Box::into_raw(user_data) as *mut _,
            )
        }
    }
}
