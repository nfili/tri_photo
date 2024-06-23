use gtk::{
    TemplateChild,glib,CompositeTemplate, Window,
    subclass::{
            prelude::{ObjectSubclassExt,ObjectImplExt,ObjectSubclass,ObjectImpl,WidgetImpl,WindowImpl},
            widget::{WidgetImplExt,CompositeTemplateInitializingExt,CompositeTemplateClass,WidgetClassExt},
    },
};

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/tri_photo/rename.ui")]
pub struct Rename {
    #[template_child]
    pub name: TemplateChild<gtk::Label>,
    #[template_child]
    pub new_name: TemplateChild<gtk::Entry>,
    #[template_child]
    pub quit: TemplateChild<gtk::Button>,
    #[template_child]
    pub rec_quit: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for Rename {
    const NAME: &'static str = "Rename";
    type Type = super::Rename;
    type ParentType = Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
impl ObjectImpl for Rename {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_icon_button();
    }
    // Needed for direct subclasses of GtkWidget;
    // Here you need to unparent all direct children
    // of your template.
    fn dispose(&self) {
       // self.dispose_template();
    }
}
impl WidgetImpl for Rename {
   fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
    }
}
// impl DialogImpl for Rename {}
impl WindowImpl for Rename {}

