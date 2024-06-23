mod imp;
use gtk::{
    glib, prelude::{GtkWindowExt, WidgetExt}, subclass::prelude::ObjectSubclassIsExt, traits::{BoxExt, ButtonExt}, Image, Label
};
use crate::{header_bar, text::Text, working::Working};

/// Type de la fenetre de message
#[derive(Copy,Clone,Debug,Default,PartialEq)]
pub enum TypeMessage{
	#[default] Information,
	Attention,
}

glib::wrapper! {
    pub struct Message(ObjectSubclass<imp::Message>)
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}
impl Message{
	/**
	 Initialisation de la fenetre de message, pour envoyer
	 *
	 * app: glib::IsA(gtk::Application), l'application en elle meme.
	 * title: &str, nom de la fenetre,
	 * label: &str, contenue du message,
	 * btns: Some(((&str,&str),(&str,&str))), contenue des boutons annuler et valider (label,resource),(label,resource). pour
	 envoyer None à btns None::<((&str,&str),(&str,&str))>.as_ref()
	 * type_mess: TypeMessage, selectionne le type de message.
	 * parent: &Working, la fenetre appelant le messsage
	 # -> retourne la structure Message
	 * */
	pub fn new<P: glib::IsA<gtk::Application>>(app:&P, title: &str, label: &str,
	 btns: Option<&((&str,&str),(&str,&str))>, type_mess: TypeMessage,parent: &Working) -> Self {
		let mut message: Message =  glib::Object::builder()
         	.property("application", app)
         	.property("title",title)
            .property("transient-for", &parent)
         	.build();
        message.imp().status.replace(false);
        message.set_titlebar(Some(&header_bar::new_without_widget(title)));
        match type_mess {
        	TypeMessage::Information => message.set_information(label),
        	TypeMessage::Attention => message.set_attention(label),
        };
        message.init_btns(btns);
        message
	}
	pub fn quit<P: glib::IsA<gtk::Application>>(app:&P, title: &str, label: &str,parent: &Working) -> Self {
		let mut message: Message =  glib::Object::builder()
         	.property("application", app)
         	.property("title",title)
            .property("transient-for", &parent)
         	.build();
        message.imp().status.replace(false);
        message.set_titlebar(Some(&header_bar::new_without_widget(title)));
        message.set_information(label);
        message.imp().valider.hide();
        let box_btn_ann = gtk::Box::new(gtk::Orientation::Horizontal,5);
        Self::set_btn(&box_btn_ann, &Text::Quitter.as_string(),"/org/gtk_rs/tri_photo/quit.png" );
        message.imp().annuler.set_child(Some(&box_btn_ann));
        message.imp().annuler.connect_clicked({
        	let this = message.clone();
        	move |_| {
        		this.close();
        	}});
        message
	}
	/**
	Affiche les bouttons en fonction des options, si pas d'options envoyer affichage par default 
	 * 
	 * btns : (annuler(label :&str, resource: &str),valider(label: &str, resource: &str))
	 * */
	fn init_btns(&mut self,btns:Option<&((&str,&str),(&str,&str))>){
		let box_btn_ann = gtk::Box::new(gtk::Orientation::Horizontal,5);
		let box_btn_val = gtk::Box::new(gtk::Orientation::Horizontal,5);
		if btns.is_some(){
			let btn_all=btns.unwrap();
			let data_ann=btn_all.0;
			let data_val=btn_all.1;
			Self::set_btn(&box_btn_ann, data_ann.0, data_ann.1);
			Self::set_btn(&box_btn_val, data_val.0, data_val.1);
		}
		else{
			// annuler
			Self::set_btn(&box_btn_ann, &Text::Cancel.as_string(), "/org/gtk_rs/tri_photo/retour.png");
	        // valider
	        Self::set_btn(&box_btn_val, &Text::Valid.as_string(), "/org/gtk_rs/tri_photo/valid.png");	        
		}
		let imp = self.imp();
		imp.valider.set_child(Some(&box_btn_val));
		imp.annuler.set_child(Some(&box_btn_ann));
		imp.annuler.set_cursor_from_name(Some("pointer"));
		imp.valider.set_cursor_from_name(Some("pointer"));
		//action sur les boutons
		imp.valider.connect_clicked({
			let this = self.clone();
			move |obj|{
				obj.set_cursor_from_name(Some("grabbing"));
				this.imp().status.replace(true);
				this.close();
			}
		});	
		imp.annuler.connect_clicked({
			let this = self.clone();
			move |obj|{
				obj.set_cursor_from_name(Some("grabbing"));
				this.imp().status.replace(false);
				this.close();
			}
		});	
	}
	fn set_btn(cont: &gtk::Box,lab : &str,resource: &str){
			let lab=Label::new(Some(lab));
        	let img=Image::from_resource(resource);
			cont.append(&img);
			cont.append(&lab);
		}
	fn set_attention(&mut self,lab_mess: &str){
		self.imp().lab_mess.set_text(lab_mess);
		self.imp().img_mess.set_resource(Some("/org/gtk_rs/tri_photo/attention.png"));
	}
	fn set_information(&mut self,lab_mess: &str){
		self.imp().lab_mess.set_text(lab_mess);
		self.imp().img_mess.set_resource(Some("/org/gtk_rs/tri_photo/information.png"));
	}
}