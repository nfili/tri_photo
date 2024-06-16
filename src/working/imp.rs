use std::sync::{Arc,Mutex};
use gtk::subclass::widget::CompositeTemplateInitializingExt;
use gtk::subclass::widget::WidgetClassExt;
use gtk::{TemplateChild,CompositeTemplate,glib,subclass::{
		prelude::{ObjectImpl,ObjectSubclass,ObjectImplExt, ObjectSubclassExt,},
		widget::{WidgetImpl,CompositeTemplateClass},
		window::WindowImpl,
	},	
};
use crate::gest_files::GestFiles;
use super::{StateCopy,StateTrans};

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/tri_photo/working.ui")]
pub struct Working {
	#[template_child]
	pub title: TemplateChild<gtk::Label>,
	#[template_child]
	pub data_image: TemplateChild<gtk::Picture>,
	#[template_child]
	pub data_name: TemplateChild<gtk::Label>,
	#[template_child]
	pub chemin: TemplateChild<gtk::Label>,
	#[template_child]
	pub data_path: TemplateChild<gtk::Label>,
	#[template_child]
	pub date: TemplateChild<gtk::Label>,
	#[template_child]
	pub data_date: TemplateChild<gtk::Label>,
	#[template_child]
	pub lieu: TemplateChild<gtk::Label>,
	#[template_child]
	pub data_geoloc: TemplateChild<gtk::Label>,
	#[template_child]
	pub status: TemplateChild<gtk::Label>,
	#[template_child]
	pub copy_status: TemplateChild<gtk::Label>,
	#[template_child]
	pub progress_bar: TemplateChild<gtk::ProgressBar>,
	#[template_child]
	pub progress_end: TemplateChild<gtk::Label>,
	#[template_child]
	pub progress_status: TemplateChild<gtk::Label>,
	#[template_child]
	pub quit: TemplateChild<gtk::Button>,
	#[template_child]
	pub quit_label: TemplateChild<gtk::Label>,
	#[template_child]
	pub rec_quit: TemplateChild<gtk::Button>,
	#[template_child]
	pub save_and_quit: TemplateChild<gtk::Label>,

	// Attribut de fonctionnement
	pub files: Arc<Mutex<GestFiles>>,
	pub option: Arc<Mutex<Vec<bool>>>,
	pub state_copy: Arc<Mutex<StateCopy>>,
	pub state_trans: Arc<Mutex<StateTrans>>,
	pub is_copied: Arc<Mutex<bool>>,
	pub is_renamed: Arc<Mutex<bool>>,
	pub is_warning: Arc<Mutex<bool>>,
	pub is_choosed: Arc<Mutex<bool>>,
	pub current_id_file: Arc<Mutex<usize>>,
}
#[glib::object_subclass]
impl ObjectSubclass for Working{
	const NAME: &'static str="Working";	
	type Type = super::Working;
	type ParentType = gtk::Window;

	// Within class_init() you must set the template.
    // The CompositeTemplate derive macro provides a convenience function
    // bind_template() to set the template and bind all children at once.
    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Working {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().init();
        self.obj().init_progress();
        
    }
    fn dispose(&self) {

    }
}
impl WidgetImpl for Working {}
impl WindowImpl for Working {}