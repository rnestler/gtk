// This file was generated by gir (463de47) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use ToolButton;
use ToolItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct MenuToolButton(Object<ffi::GtkMenuToolButton>): Widget, Container, Bin, ToolItem, ToolButton, Actionable, Buildable;

    match fn {
        get_type => || ffi::gtk_menu_tool_button_get_type(),
    }
}

impl MenuToolButton {
    pub fn new<T: IsA<Widget>>(icon_widget: Option<&T>, label: Option<&str>) -> MenuToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_menu_tool_button_new(icon_widget.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str) -> MenuToolButton {
        assert_initialized_main_thread!();
        unsafe {
            ToolItem::from_glib_none(ffi::gtk_menu_tool_button_new_from_stock(stock_id.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_menu(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_tool_button_get_menu(self.to_glib_none().0))
        }
    }

    pub fn set_arrow_tooltip_markup(&self, markup: &str) {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_markup(self.to_glib_none().0, markup.to_glib_none().0);
        }
    }

    pub fn set_arrow_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_menu_tool_button_set_arrow_tooltip_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn set_menu<T: IsA<Widget>>(&self, menu: &T) {
        unsafe {
            ffi::gtk_menu_tool_button_set_menu(self.to_glib_none().0, menu.to_glib_none().0);
        }
    }
}
