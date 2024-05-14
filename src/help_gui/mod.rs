mod imp;
use gtk::{
        Window,
        glib::{self, clone},
        subclass::prelude::ObjectSubclassIsExt,
        traits::{ButtonExt, GtkWindowExt}
    };
use crate::{app::{WIDTH,HEIGHT}, header_bar};

glib::wrapper! {
    pub struct HelpGui(ObjectSubclass<imp::HelpGui>) 
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}
impl HelpGui{
    /// Fonction créant une nouvelle fenetre d'aide
    pub fn new(_title: &str,parent: &Window) -> Self {
        let help_gui: HelpGui = glib::Object::builder()
            .property("default-width", WIDTH-20)
            .property("default-height",HEIGHT-60)
            .property("transient-for", &parent)
            .property("hide-on-close", true)
            .build();
        let this = help_gui.clone();
        help_gui.set_titlebar(Some(&header_bar::new("Aide de TriPhoto", move || {this.close()})));
        help_gui
    }
    /// function de la gestion du click sur ok
    fn init_ok(&self){
        self.imp().on_help_gui.connect_clicked(clone!(@weak self as this=>move |_| {
            this.close();
        }));
    }
}