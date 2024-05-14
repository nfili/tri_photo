use std::cell::RefCell;
use gtk::{TemplateChild,glib,CompositeTemplate,Window,
    subclass::{
        prelude::{ObjectSubclassExt,ObjectImplExt,ObjectSubclass,ObjectImpl,WidgetImpl,DialogImpl,WindowImpl,},
        widget::{WidgetImplExt,WidgetClassExt,CompositeTemplateInitializingExt,CompositeTemplateClass
        }
    },
};
use crate::app::Cmd;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/tri_photo/popup_accueil.ui")]
pub struct PopupAccueil {
    #[template_child]
    pub label: TemplateChild<gtk::Label>,
    #[template_child]
    pub button_load: TemplateChild<gtk::Button>,
    #[template_child]
    pub button_cancel: TemplateChild<gtk::Button>,
    #[template_child]
    pub button_ok: TemplateChild<gtk::Button>,
    pub action: RefCell<Cmd>,
}

#[glib::object_subclass]
impl ObjectSubclass for PopupAccueil {
    const NAME: &'static str = "PopupAccueil";
    type Type = super::PopupAccueil;
    type ParentType = Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
impl ObjectImpl for PopupAccueil {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().init_hb();
        self.obj().init_label();
        self.obj().set_icon_button();
    }
    // Needed for direct subclasses of GtkWidget; Here you need to unparent all direct children of your template.
    fn dispose(&self) {
    }
}
impl WidgetImpl for PopupAccueil {
   fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
    }
}
impl DialogImpl for PopupAccueil {}
impl WindowImpl for PopupAccueil {}

