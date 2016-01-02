// This file was generated by gir (b745e7b) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use SizeGroupMode;
use Widget;
use ffi;
use glib::object::Upcast;
use glib::translate::*;

glib_wrapper! {
    pub struct SizeGroup(Object<ffi::GtkSizeGroup>): Buildable;

    match fn {
        get_type => || ffi::gtk_size_group_get_type(),
    }
}

impl SizeGroup {
    pub fn new(mode: SizeGroupMode) -> SizeGroup {
        unsafe {
            from_glib_full(ffi::gtk_size_group_new(mode))
        }
    }

    pub fn add_widget<T: Upcast<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_size_group_add_widget(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    pub fn get_ignore_hidden(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_size_group_get_ignore_hidden(self.to_glib_none().0))
        }
    }

    pub fn get_mode(&self) -> SizeGroupMode {
        unsafe {
            ffi::gtk_size_group_get_mode(self.to_glib_none().0)
        }
    }

    //pub fn get_widgets(&self) -> /*Unknown conversion*/Unknown rust type: "SList TypeId { ns_id: 1, id: 3 }" {
    //    unsafe { TODO: call ffi::gtk_size_group_get_widgets() }
    //}

    pub fn remove_widget<T: Upcast<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_size_group_remove_widget(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    pub fn set_ignore_hidden(&self, ignore_hidden: bool) {
        unsafe {
            ffi::gtk_size_group_set_ignore_hidden(self.to_glib_none().0, ignore_hidden.to_glib());
        }
    }

    pub fn set_mode(&self, mode: SizeGroupMode) {
        unsafe {
            ffi::gtk_size_group_set_mode(self.to_glib_none().0, mode);
        }
    }

}
