#![allow(unused_imports, unused_braces)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// Asserts that this is the main thread and either `gdk::init` or `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

pub use auto::*;
use ffi;
mod auto;
pub mod subclass;

pub mod functions {
    // pub use super::auto::functions::*;
}
