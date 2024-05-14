mod imp;

use gio::prelude::ApplicationExt;
use gtk::{
    glib::{self},
    subclass::prelude::ObjectSubclassIsExt,
    traits::{ButtonExt,BoxExt,},
    prelude::GtkWindowExt,
    Image
};
use crate::{app::{WIDTH,Cmd},header_bar};

glib::wrapper! {
    pub struct PopupAccueil(ObjectSubclass<imp::PopupAccueil>)
     @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}
impl PopupAccueil{
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P, title: &str,parent: &gtk::ApplicationWindow) -> Self {
            let pop: PopupAccueil = glib::Object::builder()
            .property("application", app)
            .property("title",title)
            .property("default-width", WIDTH)
            .property("transient-for", &parent)
            .property("hide-on-close", true)
            .build();

            pop.imp().action.replace(Cmd::None);
            pop
    }
    /// Initialise la "header bar" de l'interface [`PopupAccueil`].
    fn init_hb(&self){
        let this = self.clone();
        self.set_titlebar(Some(&header_bar::new("Tri Photo: Accueil",move || { this.application().unwrap().quit()})));
    }
    /// Initialise le texte de l'interface [`PopupAccueil`].
    fn init_label(&self){
        let imp = self.imp();
        imp.label.set_text("Bienvenu,

Vous pourrez trier vos \"Photos\", en fonction de leurs dates de prise de vue et de leurs géolocalisations (si disponible lors de la prise de vue).

Vous pouvez aussi choisir de renomer les photographies, ainsi que de supprimer le contenu du répertoire source.
            ");
    }
    /// Initialisation des boutons de l'interface [`PopupAccueil`].
    fn set_icon_button(&self){
        let imp = self.imp();
                
        let hbox =gtk::Box::new(gtk::Orientation::Horizontal,5);
        let image = Image::builder().resource("/org/gtk_rs/tri_photo/suivant.png").build();
        let label_ok= gtk::Label::new(Some("Suivant"));
        hbox.append(&image);
        hbox.append(&label_ok);
        imp.button_ok.set_child(Some(&hbox));
        let this0 = self.clone();
        imp.button_ok.connect_clicked(move |_| {
            let mut this0 = this0.clone();
            this0.set_action(Cmd::Conf);
            this0.close();
        });

        let hbox =gtk::Box::new(gtk::Orientation::Horizontal,5);
        let image = Image::builder().resource("/org/gtk_rs/tri_photo/quit.png").build();
        let label_ok= gtk::Label::new(Some("Quitter"));
        hbox.append(&image);
        hbox.append(&label_ok);
        imp.button_cancel.set_child(Some(&hbox));
        let this =self.clone();
        imp.button_cancel.connect_clicked(move |_| {
            let mut this = this.clone();
         this.set_action(Cmd::Quit);
         this.close();
     });

        let hbox =gtk::Box::new(gtk::Orientation::Horizontal,5);
        let image = Image::builder().resource("/org/gtk_rs/tri_photo/load.png").build();
        let label_ok= gtk::Label::new(Some("Charger"));
        hbox.append(&image);
        hbox.append(&label_ok);
        imp.button_load.set_child(Some(&hbox));
        let this1 = self.clone();
        imp.button_load.connect_clicked(move |_| {
            let mut this1= this1.clone();
            this1.set_action(Cmd::Load);
            this1.close();
        });
    }
    // getter
    pub fn action(&self) -> Cmd { self.imp().action.borrow().clone()}

    // setter
    fn set_action(&mut self, action: Cmd) {
        *self.imp().action.borrow_mut()=  action;
    }
}
