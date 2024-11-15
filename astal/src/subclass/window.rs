use crate::Window;
use gtk::subclass::prelude::*;

pub trait AstalWindowImpl: WindowImpl {}

unsafe impl<T: AstalWindowImpl> IsSubclassable<T> for Window {}
