use gtk::glib;
use gtk::subclass::prelude::*;
use crate::MainMenu;
// Object holding the state
#[derive(Default)]
#[template(resource = "/org/gtk_rs/myapp/window.ui")]
pub struct MainMenu{
    #[template_child]
    pub menubar: TemplateChild<MainMenu>,
}
// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MainMenu {
    const NAME: &'static str = "menubar";
    type Type = super::MainMenu;
    type ParentType = gtk::PopoverMenu;
}

// Trait shared by all GObjects
impl ObjectImpl for MainMenu {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for MainMenu {}

// Trait shared by all buttons
impl ButtonImpl for MainMenu {}