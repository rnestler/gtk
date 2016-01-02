// This file was generated by gir (b745e7b) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Popover;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct PopoverMenu(Object<ffi::GtkPopoverMenu>): Widget, Container, Bin, Popover, Buildable;

    match fn {
        get_type => || ffi::gtk_popover_menu_get_type(),
    }
}

impl PopoverMenu {
    #[cfg(gtk_3_16)]
    pub fn new() -> PopoverMenu {
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_menu_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_16)]
    pub fn open_submenu(&self, name: &str) {
        unsafe {
            ffi::gtk_popover_menu_open_submenu(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

}
