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

// pub mod functions {
// pub use super::auto::functions::*;
// }
