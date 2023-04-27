use std::cell::RefCell;
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate, Entry, ListView,PopoverMenuBar};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/myapp/window.ui")]
pub struct Window {
    #[template_child]
    pub menubar: TemplateChild<PopoverMenuBar>
    // pub entry: TemplateChild<Entry>,
    // #[template_child]
    // pub tasks_list: TemplateChild<ListView>,
    // pub tasks: RefCell<Option<gio::ListStore>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "myapp";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Connect to "clicked" signal of `button`
        
    }
    
}
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}