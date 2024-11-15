use crate::Application;

use gtk::subclass::prelude::*;

pub trait AstalApplicationImpl: GtkApplicationImpl {}

unsafe impl<T: AstalApplicationImpl> IsSubclassable<T> for Application {}
