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
    pub welcome: TemplateChild<gtk::Label>,
    #[template_child]
    pub working: TemplateChild<gtk::Frame>,
    #[template_child]
    pub working_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_source: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_source_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_destination: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_destination_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_file_found: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_file_found_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_file_found_description2: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_file_found_description3: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_info: TemplateChild<gtk::Label>,
    #[template_child]
    pub working_info_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub file_at_found: TemplateChild<gtk::Frame>,
    #[template_child]
    pub file_at_found_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbos: TemplateChild<gtk::Frame>,
    #[template_child]
    pub arbo_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_annee: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_annee_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_mois: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_mois_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_jours: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_jours_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_lieu: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_lieu_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_letter: TemplateChild<gtk::Label>,
    #[template_child]
    pub arbo_letter_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub traitement: TemplateChild<gtk::Frame>,
    #[template_child]
    pub traitement_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub traitement_renomer: TemplateChild<gtk::Label>,
    #[template_child]
    pub traitement_renomer_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub traitement_delete: TemplateChild<gtk::Label>,
    #[template_child]
    pub traitement_delete_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub traitement_gestion: TemplateChild<gtk::Label>,
    #[template_child]
    pub traitement_gestion_description: TemplateChild<gtk::Label>,
    #[template_child]
    pub traitement_gestion_description2: TemplateChild<gtk::Label>,
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