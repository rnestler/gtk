// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;

struct_Widget!(SeparatorMenuItem);

impl SeparatorMenuItem {
    pub fn new() -> Option<SeparatorMenuItem> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_separator_menu_item_new() };
        check_pointer!(tmp_pointer, SeparatorMenuItem)
    }
}

impl_drop!(SeparatorMenuItem);
impl_TraitWidget!(SeparatorMenuItem);

impl ::ContainerTrait for SeparatorMenuItem {}
impl ::BinTrait for SeparatorMenuItem {}
impl ::MenuItemTrait for SeparatorMenuItem {}
