use glib::Object;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct MainMenu(ObjectSubclass<imp::MainMenu>)
        @extends gtk::PopoverMenu, gtk::Popover,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::ShortcutManager;
}

impl MainMenu {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}