use gtk::{
    Window,TemplateChild,CompositeTemplate,
    glib::subclass::{object::ObjectImplExt,types::ObjectSubclassExt},
    subclass::{
        prelude::{ObjectSubclass,ObjectImpl,WindowImpl,WidgetImpl},
        widget::{WidgetClassExt,WidgetImplExt,CompositeTemplateInitializingExt,CompositeTemplateClass}
    }
};

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/tri_photo/help_gui.ui")]
pub struct HelpGui {
	#[template_child]
	pub on_help_gui: TemplateChild<gtk::Button>,
}
#[glib::object_subclass]
impl ObjectSubclass for HelpGui {
    const NAME: &'static str = "HelpGui";
    type Type = super::HelpGui;
    type ParentType = Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
impl ObjectImpl for HelpGui {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().init_ok();
    }
    // Needed for direct subclasses of GtkWidget; Here you need to unparent all direct children of your template.
    fn dispose(&self) {}
}
impl WidgetImpl for HelpGui {
   fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
    }
}
impl WindowImpl for HelpGui {}