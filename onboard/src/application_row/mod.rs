mod imp;

use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gio, glib};

glib::wrapper! {
    pub struct ApplicationRow(ObjectSubclass<imp::ApplicationRow>)
        @extends gtk4::Widget, gtk4::Box;
}

impl Default for ApplicationRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationRow {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    pub fn set_app_info(&self, app_info: &gio::AppInfo) {
        let imp = self.imp();
        imp.name.set_text(&app_info.name());
        if let Some(desc) = app_info.description() {
            imp.description.set_text(&desc);
        }
        if let Some(icon) = app_info.icon() {
            imp.image.set_from_gicon(&icon);
        }
    }
}
