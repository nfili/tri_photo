use std::{cell::RefCell, rc::Rc};

use gtk::{
    glib, subclass::{
            prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, WidgetImpl, WindowImpl},
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassExt, WidgetImplExt},
    }, Button, CompositeTemplate, TemplateChild, Window
};
#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/tri_photo/message.ui")]
pub struct Message {
    #[template_child]
    pub lab_mess: TemplateChild<gtk::Label>,
    #[template_child]
    pub img_mess: TemplateChild<gtk::Image>,
    #[template_child]
    pub box_btn_mess: TemplateChild<gtk::Box>,
    #[template_child]
    pub annuler: TemplateChild<Button>,
    #[template_child]
    pub valider: TemplateChild<Button>,
    pub status: Rc<RefCell<bool>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Message {
    const NAME: &'static str = "Message";
    type Type = super::Message;
    type ParentType = Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
impl ObjectImpl for Message {
    fn constructed(&self) {
        self.parent_constructed();
    }
    // Needed for direct subclasses of GtkWidget;
    // Here you need to unparent all direct children
    // of your template.
    fn dispose(&self) {
       // self.dispose_template();
    }
}
impl WidgetImpl for Message {
   fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
    }
}
// impl DialogImpl for Rename {}
impl WindowImpl for Message {}

