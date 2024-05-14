mod imp;
use gtk::{
    glib::{self},
    subclass::prelude::ObjectSubclassIsExt,
    traits::{ButtonExt,BoxExt},
    prelude::{EditableExt, GtkWindowExt, WidgetExt}
};
use crate::{working::Working, header_bar};

glib::wrapper! {
    pub struct Rename(ObjectSubclass<imp::Rename>)
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}
impl Rename{
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P, title: &str,name : &str, parent: &Working) -> Self {
            let rename:Rename= glib::Object::builder()
            .property("application", app)
            .property("title",title)
            .property("transient-for", &parent)
            .property("hide-on-close", true)
            .build();
            
            rename.imp().name.set_label(name);
            rename.imp().new_name.set_text("Nouveau nom");
            rename.set_titlebar(Some(&header_bar::new_without_widget("Tri Photo : Renomer")));
            rename
    }
    
    pub fn set_icon_button(&self){
        let imp = self.imp();
        
        let hbox =gtk::Box::new(gtk::Orientation::Horizontal,5);
        let pix =gtk::gdk_pixbuf::Pixbuf::from_resource("/org/gtk_rs/tri_photo/renomer.png").unwrap();
        let image = gtk::Image::from_pixbuf(Some(&pix));
        image.set_icon_size(gtk::IconSize::Inherit);
        let label_ok= gtk::Label::new(Some("Renomer"));
        hbox.append(&image);
        hbox.append(&label_ok);
        imp.rec_quit.set_child(Some(&hbox));
        imp.rec_quit.set_css_classes(&["btn"]);
        let this = self.clone();
        imp.rec_quit.connect_clicked(move |_obj|{
            this.close();
        });

        let hbox =gtk::Box::new(gtk::Orientation::Horizontal,5);
         let pix =gtk::gdk_pixbuf::Pixbuf::from_resource("/org/gtk_rs/tri_photo/save.png").unwrap();
        let image = gtk::Image::from_pixbuf(Some(&pix));
        image.set_icon_size(gtk::IconSize::Inherit);
        let label_ok= gtk::Label::new(Some("Conserver"));
        hbox.append(&image);
        hbox.append(&label_ok);
        imp.quit.set_child(Some(&hbox));
        imp.quit.set_css_classes(&["btn"]);
        let this = self.clone();
        imp.quit.connect_clicked( move |_obj|{
            this.close();
        });

    }
    //getter
    pub fn new_name(&self)-> String {
        self.imp().new_name.get().text().to_string()
    }
}
