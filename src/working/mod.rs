mod imp;
use glib::IsA;
use gtk::{
	gdk_pixbuf::Pixbuf,
	gio::prelude::FileExt,
	glib::ControlFlow, 
	prelude::{ApplicationExt, BoxExt, DialogExtManual, FileChooserExt}, 
	subclass::prelude::ObjectSubclassIsExt, 
	traits::{ButtonExt, GtkWindowExt, WidgetExt},
	Align, Box, Button, Image, Label, Orientation, Picture, TemplateChild, Window
};
use crate::{
	app::{HEIGHT, WIDTH}, 
	file::File,
	file_choser::new_save, 
	gest_files::GestFiles,
	header_bar, 
	message::{Message, TypeMessage},
	rename::Rename
};
use std::{io::Write, thread, time::Duration, usize};
use conv::ValueFrom;

const EXTENSION: &str = ".tps";

///Enumération représentant l'état de la copy
#[derive(Copy,Clone,Debug,Default,PartialEq)]
pub enum StateCopy{
	#[default] Init,
	Wait,
	Coping,
	Copied,
	Stop,
	Warning,
	Warned,
	Rename,
	Renamed,
	Deleted,
	ChoosingOnDemand,
	ChoosedOnDemand,
	Finished
}
//Enumération représentant l'état de la transmission
#[derive(Copy,Clone,Debug,Default,PartialEq)]
pub enum StateTrans{
	#[default] Init,
	Wait,
	Sending,
	Sended,
	Receiving,
	Received,
	Finished
}
//Enumération représentant la tâche à effectuer
#[derive(Copy,Clone,Debug,Default,PartialEq)]
pub enum Tache {
	#[default] Wait,
	Copy,
	Rename,
	Delete,
	Send,
	Update,
	Verif,
	None
}

glib::wrapper! {
	/// Creation de l'interface Working pour la procédure de copi
    pub struct Working(ObjectSubclass<imp::Working>)
        @extends gtk::Widget, gtk::Window, gtk::ProgressBar,
         @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Working {
	/**
	 Fonction créant une nouvelle instance de la structure Working 
	 * app (gtk::Application)  l'application qui appelle cet objet
	 * tri (GestFiles) liste des données sur les fichiers à copier
	 * option (Vec bool) option de tri et d'aprés traitement
	 * 
	 # -> retourne un objet Working
	 * */
	pub fn new<P: glib::IsA<gtk::Application>>(app: &P,tri: GestFiles,option:Vec<bool>) -> Self {
         let mut working: Working =  glib::Object::builder()
         	.property("application", app)
         	.property("default-width", WIDTH)
         	.property("default-height", HEIGHT)
         	.build();

         working.set_option(option);
         working.set_decorated(false);
         working.set_state_copy(StateCopy::Init);
      	 working.set_state_trans(StateTrans::Init);
		 working.set_files(tri);
		 working
	}
	
	/// Fonction qui gère l'initialisation des boutons de l'interface gtk
	pub fn init(&self){
	    self.connect_close_request(|obj|{
	     	let app =obj.application().unwrap();
	     	app.quit();
	     	true.into()
	    });
	    let imp = self.imp();
		imp.progress_end.set_text(self.nb_files().to_string().as_str());
	    let this = self.clone();
	    imp.rec_quit.set_cursor_from_name(Some("pointer"));
	    imp.rec_quit.connect_clicked( move |obj| {
         	let mut this = this.clone();
            obj.set_cursor_from_name(Some("grabbing"));
            let _ = this.save(&this.clone(),this.state_copy(), this.state_trans());
            obj.set_cursor_from_name(Some("pointer"));

        });
        imp.quit.set_cursor_from_name(Some("pointer"));
		let this=self.clone();
        imp.quit.connect_clicked( move |obj|{
        	let mut this = this.clone();
        	obj.set_cursor_from_name(Some("grabbing"));	
        	let tmp_sta_copy = this.state_copy();
            let tmp_sta_trans = this.state_trans();
			this.quit(tmp_sta_copy, tmp_sta_trans)
        });
    }
    /**
     Fonction d'avertissement lorsque l'on quitte le programme
     * 
     * tmp_sta_copy: StateCoy, état initiale de la copie
     * tmp_sta_trans: StateTrans, état initiale de la transmission
     * */
    fn quit(&mut self,tmp_sta_copy: StateCopy,tmp_sta_trans: StateTrans){
    	self.set_state_copy(StateCopy::Stop);
		self.set_state_trans(StateTrans::Wait);
		let message= "Vous allez quitter le programme,
 le listing des fichiers à copier/renomer sera perdu,
 je vous conseille de sauvegarder pour quitter.";
        let quit_test = Message::new(&self.application().unwrap(),"Quitter le programme",message,
        	Some(&(("Retour","/org/gtk_rs/tri_photo/retour.png"),("Quitter","/org/gtk_rs/tri_photo/quit.png"))),
        	TypeMessage::Information,&self);
        let this = self.clone();
        quit_test.connect_close_request(move | obj |{
        	let mut this = this.clone();
        	if *obj.imp().status.borrow(){
        		this.application().unwrap().quit();
        		glib::Propagation::Stop
        	}
        	else {
        		this.set_states(tmp_sta_copy,tmp_sta_trans);
        		glib::Propagation::Proceed
        	}
        });
        quit_test.show();
    }
    /**
     Fonction de sauvegarde de la progression et quitte
     * 
     * parent: &Window, fenetre appelant la demande de sauvegarde
     * tmp_sta_copy: StateCoy, état initiale de la copie
     * tmp_sta_trans: StateTrans, état initiale de la transmission
     * */
    fn save(&mut self,parent:&impl IsA<Window>, sta_copy: StateCopy, sta_trans:StateTrans){
    	let rep = new_save(Some(parent));
    	self.set_state_copy(StateCopy::Stop);
        self.set_state_trans(StateTrans::Wait);
            
        rep.clone().run_async({
        	let this = self.clone();
       		move |objs,answer| {
            	objs.hide();
            	let mut this = this.clone();
                match answer{
                    gtk::ResponseType::Ok =>{
                    	let tmp_path = this.verif_extension(objs.file().unwrap().path().unwrap().display().to_string(),EXTENSION);
                		let mut file = match std::fs::File::create(tmp_path) {
        					Err(why) => panic!("couldn't create {}", why),
        					Ok(file) => {
        						file
        					}
    					};
    					let mut options:String = String::from("options;");
    					for option in this.option().iter(){
    						options += &(option.to_string() +",");
    					}
    					let serialise_file = this.files().serialize();
    					let tmp_serialize = options +"*\n"+&serialise_file;
    					let _ = file.write_all(tmp_serialize.as_bytes());
                       	this.application().unwrap().quit();
                    } 
                    _ => {
                        	this.set_state_copy(sta_copy); 
                        	this.set_state_trans(sta_trans); 
                    }
                }
        }});
    }
    /**
     Fonction créant la fenetre d'avertissement de la suppression des sources
     * 
     # -> retourne la fenetre Message
     * */
	fn init_warning(&mut self) -> Message {
		let message = "Vous avez choisi de supprimer
 les anciens fichiers cette action est irréversible,
 les fichiers supprimés ne seront pas récupérable";

		let warning = Message::new(&self.application().unwrap(),"Avertissement critique",message,
        	Some(&(("Quitter","/org/gtk_rs/tri_photo/quit.png"),("J'ai compris","/org/gtk_rs/tri_photo/ok_hand.png"))),
        	TypeMessage::Attention,&self);
		//  Bouton valider
		warning.connect_close_request({
			let this= self.clone();
			move | obj |{
        	let mut this = this.clone();
        	if !*obj.imp().status.borrow(){
        		this.application().unwrap().quit();
        		glib::Propagation::Stop
        	}
        	else {
        		let _ = this.end_warning();
        		glib::Propagation::Proceed
        	}
        }});
		warning
	}
	/**
	 Fonction créant la fenetre de gestion des fichiers à la demande
	 *
	 * file: File, donnée du fichier à taiter
	 * 
	 # -> retourne la fenetre Window 
	 * */
	fn choose_on_demand(&self, file: File)->Window{
		let win = Window::builder()
        	.hide_on_close(true)
        	.modal(true)
        	.build();
        let this = self.clone();
        win.set_titlebar(Some(&header_bar::new("Tri Photo : Choix pour le fichiers", move || { this.application().unwrap().quit() } )));
		let v_box = Box::new(Orientation::Vertical,10);
		v_box.set_valign(Align::Center);
		let lab_message = Label::new(Some("Vous avez choisi de gérer les fichiers au fur et à mesure"));
		// Boutton copier
		let h_box_copier = Box::new(Orientation::Horizontal,5);
		let copier = Button::builder().build();
		let img_copier = Image::builder()
			.resource("/org/gtk_rs/tri_photo/copier.png")
			.build();
		img_copier.set_icon_size(gtk::IconSize::Large);
		let lab_copier = Label::new(Some("Copier"));
		h_box_copier.append(&img_copier);
		h_box_copier.append(&lab_copier);
		copier.set_child(Some(&h_box_copier));
		copier .set_css_classes(&["btn"]);
		copier.set_cursor_from_name(Some("pointer"));
		//Boutton rename
		let h_box_rename = Box::new(Orientation::Horizontal,5);
		let rename = Button::builder().build();
		let img_rename = Image::builder()
			.resource("/org/gtk_rs/tri_photo/renomer.png")
			.build();
		img_rename.set_icon_size(gtk::IconSize::Large);
		let lab_rename = Label::new(Some("Renomer"));
		h_box_rename.append(&img_rename);
		h_box_rename.append(&lab_rename);
		rename.set_child(Some(&h_box_rename));
		rename.set_css_classes(&["btn"]);
		rename.set_cursor_from_name(Some("pointer"));
		//Boutton save
		let h_box_save = Box::new(Orientation::Horizontal,5);
		let save = Button::builder().build();
		let img_save = Image::builder()
			.resource("/org/gtk_rs/tri_photo/save.png")
			.build();
		img_save.set_icon_size(gtk::IconSize::Large);
		let lab_save = Label::new(Some("Sauvegarder"));
		h_box_save.append(&img_save);
		h_box_save.append(&lab_save);
		save.set_child(Some(&h_box_save));
		save.set_css_classes(&["btn"]);
		save.set_cursor_from_name(Some("pointer"));
       	// bouton quit
       	let h_box_quit = Box::new(Orientation::Horizontal,5);
		let quit = Button::builder().build();
		let img_quit = Image::builder()
			.resource("/org/gtk_rs/tri_photo/quit.png")
			.build();
		img_quit.set_icon_size(gtk::IconSize::Large);
		let lab_quit = Label::new(Some("Quitter"));
		h_box_quit.append(&img_quit);
		h_box_quit.append(&lab_quit);
		quit.set_child(Some(&h_box_quit));
		quit.set_css_classes(&["btn"]);
		quit.set_cursor_from_name(Some("pointer"));
		// bouton delete
       	let h_box_delete = Box::new(Orientation::Horizontal,5);
		let delete = Button::builder().build();
		let img_delete = Image::builder()
			.resource("/org/gtk_rs/tri_photo/supprimer.png")
			.build();
		img_delete.set_icon_size(gtk::IconSize::Large);
		let lab_delete = Label::new(Some("Supprimer"));
		h_box_delete.append(&img_delete);
		h_box_delete.append(&lab_delete);
		delete.set_child(Some(&h_box_delete));
		delete.set_css_classes(&["btn"]);
		delete.set_cursor_from_name(Some("pointer"));
		let h_box_btn = Box::new(Orientation::Horizontal, 10);
		h_box_btn.append(&quit);
		h_box_btn.append(&save);
		h_box_btn.append(&delete);
		h_box_btn.append(&rename);
		h_box_btn.append(&copier);

		v_box.append(&lab_message);
		v_box.append(&h_box_btn);

		let h_box_top_level= Box::new(Orientation::Horizontal,10);
		h_box_top_level.set_margin_bottom(10);
		h_box_top_level.set_margin_top(10);
		h_box_top_level.set_margin_start(10);
		h_box_top_level.set_margin_end(10);

        let img = Picture::new();
        img.set_pixbuf(Some(&self.create_img(file.path())));
        img.set_size_request(100,133);
        h_box_top_level.append(&img);
        h_box_top_level.append(&v_box);
		win.set_child(Some(&h_box_top_level));
		quit.connect_clicked({
			let this = self.clone();
			move |obj| {
				let mut this = this.clone();
				obj.set_cursor_from_name(Some("grabbing"));
				this.quit(this.state_copy(), this.state_trans());
		}});
		save.connect_clicked({
			let this: Working = self.clone();
			let this_win = win.clone();
			move |obj| {
				let mut this = this.clone();
				obj.set_cursor_from_name(Some("grabbing")); 
				this.save(&this_win,this.state_copy(), this.state_trans());
		}});
		delete.connect_clicked({
			let this: Working = self.clone();
			let this_win = win.clone();
			move |obj| {
				let mut this = this.clone();
				obj.set_cursor_from_name(Some("grabbing"));
				this.set_state_copy(StateCopy::Deleted);
				this_win.close();
		}});
		rename.connect_clicked({
			let this: Working = self.clone();
			let this_win = win.clone();
			move |obj| {
				obj.set_cursor_from_name(Some("grabbing"));
				let mut this = this.clone();
				this.set_state_copy(StateCopy::Rename);
				this.set_status("Renomage en cours...");
				let _= this.rename(file.clone());
				this_win.close();
		}});
		copier.connect_clicked({
			let this: Working = self.clone();
			let this_win = win.clone();
			move |obj| {
				let mut this= this.clone();
				obj.set_cursor_from_name(Some("grabbing"));
				this.set_state_copy(StateCopy::ChoosedOnDemand);
				this_win.close();
		}});
		win
	}
	/// Fonction qui lance la procédure de copi  (renomage, suppression, copi)
	pub fn init_progress(&self){
		/**
		 Fonction interne permetenant de gérer l'affichade de la barre de progession. 
		 * p_status (&TemplateChild Label) le label de la fin de la barre
		 * p_bar (&TemplateChild ProgressBar) la barre de progression
		 * nb_file (u64) le nombre de fichier total
		 * file (&File) le fichier indexé
		 * */
		 fn set_bar(p_status: &TemplateChild<Label>,p_bar: &TemplateChild<gtk::ProgressBar>,p_end: &TemplateChild<Label>,
		 	nb_file: u64,file: &File){
			/**
			 Fonction permetenant de calculer la fraction de la progression de la barre. 
			 * int (f64) : l'index du fichier en cours
			 * i (f64) : le nombre de fichier total
			 * 
			 # -> retourne un f64 indiquant la representation du fichier sur l'ensemble
			 * */
			 fn fraction(int:f64,t:u64)-> f64 { (int+1.00) / f64::value_from(t).unwrap()}
			/**
			 Fonction permetenant d'indiquer la progression de la barre. 
			 * p_status (&TemplateChild Label) le label de la fin de la barre
			 * value (u64) l'index du fichier en cours,
			 * nb_file (u64) le nombre de fichier total
			 * */
			 fn advancement(p_status: &TemplateChild<Label>,p_end: &TemplateChild<Label>, value: u64, nb_file: u64){
			 	let new_index= value+1;
			 	let result = format!("{}/{}",new_index,nb_file);
			 	p_status.set_text(&result);
				p_end.set_text(new_index.to_string().as_str());
			 }
			 advancement(p_status, p_end, file.id_number(), nb_file);
			 p_bar.set_fraction(fraction(f64::value_from(file.id_number()).unwrap(),nb_file));
			}
		/**
		 Fonction interne permetenant de gérer l'affichade des données de l'image. 
		 * working (&Working) : le texte à couper
		 * file (&File) : la largeur maximale de la ligne de texte
		 * */
		fn set_data(working: &Working,file: &File){
			/**
			 Fonction permetenant de limité la taille du texte à affiché. 
			 * binding (String) : le texte à couper
			 * taille (usize) : la largeur maximale de la ligne de texte
			 * 
			 # -> retourne un String de la largeur d'affichage voulue
			 * */
			 fn get_name_label(binding: String, taille: usize) -> String{
			 	let path: String;
			 	let binding = binding.trim_end_matches('/');
			 	if binding.len() > taille {
			 		let path_start = &binding[0..3];
			 		let path_end = &binding[binding.len()-15..];
			 		path = path_start.to_string() +"..."+  path_end ;
			 		return path;
			 	}
			 	return binding.to_string()
			 }
			/**
			 Fonction permetenant de limité la taille du chemin de répertoire à affiché. 
			 * path (&str) : le texte à couper
			 * 
			 # -> retourne un String de la largeur d'affichage voulue
			 * */
			 fn get_path_data(path: &str) -> String{
			 	let binding: Vec<_>= path.rmatch_indices('/').collect();
			 	get_name_label(path[..binding[0].0].to_string(),45)
			 }
			/**
			 Fonction permetenant de convertir la date en string. 
			 * date ((u16,18,18)) : le texte à couper
			 * 
			 # ->retourne un String de la date
			 * */
			fn get_date_label(date: (u16,u8,u8)) -> String {
			 	if date.0 == 1970 && date.1 == 1 && date.2 == 1{
			 		return String::from("inconnue")
			 	}
			 	date.0.to_string() +"/"+ &date.1.to_string() + "/" + &date.2.to_string()
			}
			/**
			 Fonction permettant de modifié le contenu de la localisation, si celle ci est connu. 
			 * binding (String) : le texte à couper
			 * 
			 # -> retourne un String de la localisation
			 * */
			fn get_geoloc_data(binding: String) -> String{
			 	if binding == "None"{
			 		return String::from("Inconue")
			 	}
			 	return binding
			 }
			let imp = working.imp();
			// ajout du thumbail
			imp.data_image.set_pixbuf(Some(&working.create_img(file.path())));
			imp.data_image.set_size_request(100, 133);
			// data du fichier
			imp.data_name.set_label(&get_name_label(file.name(),45));
			imp.data_path.set_label(&get_path_data(&file.path()));
			imp.data_geoloc.set_label(&get_geoloc_data(file.localisation()));
			imp.data_date.set_label(&get_date_label(file.date()));
		}

		// Initialisation de la barre de progression
		self.imp().progress_bar.set_fraction(0.0);
		
		// lancement du script à l'affichage de la fenêtre 
		self.connect_map(move |obj|{
			//configuration du channel
			let (tx, rx) = std::sync::mpsc::channel();
			// copie des 
			let files= obj.files();
			//  ENVOIE
            // thread 1 Boucle d'envoi
            let mut this = obj.clone();
            let handle = thread::Builder::new().name("boucle d'envoie".into());
            let _thread1 = handle.spawn(move || {
            	let mut id_file:usize=0;
            	let nb_file:usize = this.nb_files().try_into().unwrap();
            	// boucle d'envoi conditionné par le nombre de fichier à transmettre
            	while id_file<nb_file {
            		// Récuperation de l'index courrent, si depuis une svg, current sera à la valeur enregistré sinon 0 
            		let current = files.current();
            		//  Récuperation du fichier
            		let v = &this.file_at(id_file);
        			//  initialisation de de l'index du premier fichier
        			if id_file == 0 {
       					this.set_current_id_file(0);
        			}
        			//  initialisation des états et taches
        			let mut tache: Tache = Tache::Wait;
        			match this.state_trans(){
        				StateTrans::Init => this.set_state_trans(StateTrans::Wait),
        				_ => ()
        			};
        			match this.state_copy(){
        				StateCopy::Init => this.set_state_copy(StateCopy::Wait),
        				_ => ()
        			};
        			// la boucle d'envoi est préte à transmettre des données
        			if this.current_id_file() == id_file && 
        				this.state_copy() == StateCopy::Wait && 
        				this.state_trans() == StateTrans::Wait {
        				if current >= v.id_number() && v.is_copy() == false{
        					tache = Tache::Send;
        				}
        				else if current >= v.id_number() && v.is_copy() == true{
        					tache = Tache::Verif;
        				}
        				else if current < v.id_number(){
        					tache=Tache::Send;
        				}
        			}
        			//  le fichier avec l'id à été copié,verifé et transmis 
        			else if this.current_id_file() == id_file &&
        			 	this.state_copy() == StateCopy::Finished &&
        			 	this.state_trans() == StateTrans::Finished {
        			 	tache=Tache::Update;
        			}
        			else if this.current_id_file() == id_file  &&
        			 	this.state_copy() == StateCopy::Warned && 
        			 	this.state_trans() == StateTrans::Finished {
        			 		tache=Tache::None;
        			}
        			// le fichier avec l'id à été renomer et transmis
        			else if this.current_id_file() == id_file  &&
        			 	this.state_copy() == StateCopy::Renamed && 
        			 	this.state_trans() == StateTrans::Finished {
        			 	tache=Tache::Send;
        			}
        			// le fichier avec l'id à été renomer et transmis
        			else if this.current_id_file() == id_file  &&
        			 	this.state_copy() == StateCopy::Deleted && 
        			 	this.state_trans() == StateTrans::Finished {
        			 		this.set_state_copy(StateCopy::Finished);
        			 		tache=Tache::Delete;
        			}
        			else if this.current_id_file() == id_file  &&
        			 	this.state_copy() == StateCopy::ChoosedOnDemand && 
        			 	this.state_trans() == StateTrans::Finished {
        			 		tache=Tache::None;
        			}
        			// le fichier avec l'id à été Copié et transmis
        			 else if this.current_id_file() == id_file  &&
        			 	this.state_copy() == StateCopy::Copied && 
        			 	this.state_trans() == StateTrans::Finished {
        			 	tache=Tache::Verif;
        			}
        			else {
        				();
        			}
        			// Gestion des tâches, une tache par bouclage,
        			// Send (envoie du ficher),
        			// update ( mise à jours des états et variables),
        			// verif (verification du fichier copié et attributions des états)
        			match tache{
        				Tache::Send => {
   							this.sender(tx.clone(), v.clone(), &mut id_file);
        				},
        				Tache::Update => {
        					id_file+=1;
        					this.set_current_id_file(id_file);
        					this.set_current(id_file);
        					this.set_is_renamed(false);
        					this.set_is_choosing(false);
        					this.set_states(StateCopy::Wait,StateTrans::Wait);
        				},
        				Tache::Verif =>  {
        					let photo = match v.is_photo(){
								true =>"",
								false=>"/not_photo",
        					};
        					let path = v.create_path_dir(this.path_dest()+photo,
    						&this.option())+"/"+&v.id_number().to_string()+"_"+v.name().as_str();
    						match std::fs::File::open(path){
        						Ok(file) => {
        							if file.metadata().unwrap().len() != v.size(){
        								this.sender(tx.clone(),v.clone(),&mut id_file);
        							}else{
        								this.set_states(StateCopy::Finished,StateTrans::Finished);
        							}
        						}
        						Err(_e) => {
        							this.sender(tx.clone(), v.clone(),&mut id_file);
        						}
        					};
        				},
        				Tache::Delete => {
        					this.set_state_copy(StateCopy::Finished);
        				}
        				Tache::None => {
        					this.set_states(StateCopy::Wait,StateTrans::Wait);
        				}
        				_ => (),
        			};
            	}
            });
		    //  RECEPTION
		    let mut this= obj.clone();
		    // Utilisation de timeout_add_local pour la gestion de la boucle de réception, utilise moins de ressource que idle_add_local
		    glib::timeout_add_local( Duration::from_millis(25), move || match rx.try_recv(){
		    	Ok(mut value) => {
		    		//  Vérification de l'index du fichier recu
		    		if value.id_number() as usize == this.current_id_file() {
		    			this.set_state_trans(StateTrans::Receiving);
						
						let mut this =this.clone();
						//  Vérification de l'etat de copy, soit Renamed soit autre chose
			    		if this.state_copy() != StateCopy::Renamed || this.state_copy() != StateCopy::Warned{
			    			set_data(&this,&value);
			    			set_bar(&this.imp().progress_status, &this.imp().progress_bar,&this.imp().progress_end, this.nb_files(), &value);
							let warning = this.option().get(5).unwrap().clone();
							let rename = this.option().get(6).unwrap().clone();
							let choose_on_demand= this.option().get(7).unwrap().clone();
							if warning.to_owned() == true && !this.is_warning(){
								this.set_state_copy(StateCopy::Warning);
			     				this.warning();
					     	}							
							//  si on doit renomer et que la tâche n'est pas accomplie
			    			else if rename.to_owned() == true  && !this.is_renamed(){
								this.set_state_copy(StateCopy::Rename);
								this.set_status("Renomage en cours...");
			    				let _ = this.rename(value.clone());
			    			}
			    			// si on a choisir de décider au dernier moment
			    			else if choose_on_demand.to_owned() == true && !this.is_choosed(){
			    				this.set_state_copy(StateCopy::ChoosingOnDemand);
			    				this.set_status("Gestion personnalisée en cours...");
			    				let _ = this.choose(value.clone());
			    			}
			    			// sinon on modifi l'état de copy à Coping
			    			else {
			    				this.set_state_copy(StateCopy::Coping);
			    			}	
				    	}
				    	// sinon on modifi l'état de copy à Coping
				    	else {
				    		this.set_state_copy(StateCopy::Coping);
				    	}
						this.set_status("Copie en cours...");
						//  Vérification de l'état de copi, si Coping, on lance la copi
				    	if this.state_copy() == StateCopy::Coping{
			    			set_data(&this,&value);
			    			set_bar(&this.imp().progress_status, &this.imp().progress_bar,&this.imp().progress_end, this.nb_files(), &value);
			    			value=this.files().copy(
			    				this.path_dest(),&this.option(),value.clone()).1;
			    			let a = value.id_number() as usize;
			    			let _ = this.end_copy(a,value);
			    		}
		    		}
		    		ControlFlow::Continue
		    	}
		    	Err(std::sync::mpsc::TryRecvError::Empty) => ControlFlow::Continue,
		    	Err(std::sync::mpsc::TryRecvError::Disconnected) => {
		    		let win = Message::quit(&this.application().unwrap(), "Copie terminée", "La copie des fichiers est terminée", &this);
			        win.connect_close_request({
			        	let this= this.clone();
			        	move |_obj|{
			        		this.application().unwrap().quit();
			        		glib::Propagation::Stop
			        	}
			        });
		    		win.show();
		    		ControlFlow::Break
		    	},
		    });
		});
	}

	/**
	 Fonction gérant  l'envoie du fichier à traiter
	 * tx (Sender) le type de données transmit via le canal
	 * v (File) le fichier à transmettre pour traitement
	 * i (&mut usize) l'index dans la liste de fichier
	 * */
	fn sender(&mut self,tx:std::sync::mpsc::Sender<File>,v:File,i:&mut usize){
		self.set_status("Initialisation ");
		self.set_state_trans(StateTrans::Sending);
		let tx = tx.clone();
		self.set_current_id_file(*i);
		let handle = thread::Builder::new().name("Envoie du message".into());
		let thread2 = handle.spawn(move || {
			let _ = tx.send(v).unwrap();
		}).unwrap();
		thread2.join().unwrap();
		self.set_state_trans(StateTrans::Sended);
		thread::sleep(std::time::Duration::from_millis(375));
	}
	/**
	 Fonction qui lance la fenêtre de gestion du fichier
	 * value (File) fichier à traiter
	 * */
	fn choose(&mut self, file: File){
		let choose = self.choose_on_demand(file);
		self.set_state_copy(StateCopy::ChoosingOnDemand);
		let this = self.clone();
		choose.connect_close_request(
			move |_|{
				let mut this = this.clone();
				let _ = this.end_choose();
				glib::Propagation::Proceed
		});
		choose.show();
	}
	/**
	 Fonction qui lance la fenêtre de renomage
	 * value (File) fichier à renomer
	 * */
	fn rename(&mut self,value:File){
		let rename= Rename::new(&self.application().unwrap(),"Renomer le fichier",&value.clone().name(),&self);
		self.set_state_copy(StateCopy::Rename);
		let this = self.clone();
		rename.connect_close_request(move |obj|{
			let mut this= this.clone();
			let  extension = ".".to_owned()+&value.extension();
			let tmp_path = this.verif_extension(obj.new_name(),&extension);
			let _ = this.end_rename(tmp_path, &mut value.clone());
			glib::Propagation::Proceed
		});
		rename.show();	
	}
	///Fonction qui lance la fenêtre de d'avertissement de la suppression des sources
	fn warning(&mut self){
		self.set_state_copy(StateCopy::Warning);
		let warning =  self.init_warning();
		warning.show();
	}
	/**
	 Fonction qui gére la fin de la procédure de copi
	 * index (usize) index dans la liste
	 * file (File) Fichier a copié
	 * 
	 # -> Retourne un resultat de bool ou une erreur
	 * */
	fn end_copy(&mut self,index: usize,file: File) -> Result<bool,&'static str> {
		self.set_file_at(index,file.clone());
		self.set_status("Copié");
		self.set_is_copied(true);
		self.set_states(StateCopy::Copied,StateTrans::Finished);
		Ok(true)
	}
	/// Fonction qui gère la fin du processus de gestion du fichier
	fn end_choose(&mut self) -> Result<bool,&'static str>{
		self.set_is_choosing(true);
		self.set_status("Choix effectué");
		self.set_state_trans(StateTrans::Finished);
		Ok(true)
	}
	/**
	 Fonction qui gére la fin de la procédure de renomage
	 * name (String) nouveau nom du fichier
	 * file (&mut File) Le fichier à renomer
	 * 
	 # -> Retourne un resultat de bool ou une erreur
	 * */
	fn end_rename(&mut self, name: String, file: &mut File) -> Result<bool, &'static str> {
		self.set_name_of_file(name, file);
		self.set_status("Renomé");
		self.set_is_renamed(true);
		self.set_states(StateCopy::Renamed,StateTrans::Finished);
		Ok(true)
	}
	/// Fonction qui gère la fin du processus d'avertissement de suppression des sources
	fn end_warning(&mut self) -> Result<bool,&'static str>{
		self.set_is_warning(true);
		self.set_states(StateCopy::Warned,StateTrans::Finished);
		Ok(true)
	}
	/**
	 Fonction qui verifie si l'extention existe ou pas, si elle n'existe pas il la rajoute
	 * 
	 * path: String, le chemin à verifier
	 * extension: &str, l'extenstion à chercher
	 * 
	 # -> Retourne le path avec l'extension si besoin
	 * */
	fn verif_extension(&self,path: String, extension: &str) -> String{
		if !path.contains(&extension){
            return path + extension;
       	}
        path  	
	}
	fn set_states(&mut self, copy: StateCopy, trans: StateTrans){
		self.set_state_copy(copy);
		self.set_state_trans(trans);
	}
	/**
	 Fonction créant le thumbail du fichier passer par le chemin de celui-ci
	 * 
	 * path: String, le chemin de l'image
	 * */
	fn create_img(&self, path:String) -> Pixbuf {
		let mut pixbuf = gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(std::path::Path::new(&path),100,133,true).unwrap();
			pixbuf = pixbuf.apply_embedded_orientation().unwrap();
			pixbuf
	}
	//setter
	fn set_state_trans(&mut self, state: StateTrans) {	*self.imp().state_trans.lock().unwrap() = state; }
	fn set_state_copy(&mut self, state: StateCopy) { *self.imp().state_copy.lock().unwrap() = state; }
	fn set_is_copied(&mut self, copied: bool) { *self.imp().is_copied.lock().unwrap() = copied; }
	fn set_is_renamed(&mut self, renamed: bool) { *self.imp().is_renamed.lock().unwrap() = renamed;	}
	fn set_is_warning(&mut self, warning: bool) { *self.imp().is_warning.lock().unwrap() = warning;	}
	fn set_is_choosing(&mut self, choosing: bool) { *self.imp().is_choosed.lock().unwrap() = choosing; }
	fn set_current_id_file(&mut self, id: usize) { *self.imp().current_id_file.lock().unwrap() = id }
	fn set_current(&mut self, id: usize) { self.imp().files.lock().unwrap().set_current(id.try_into().unwrap())}
	fn set_option(&mut self, option:Vec<bool>) { *self.imp().option.lock().unwrap() = option; }
	fn set_file_at(&mut self, index:usize, file: File) { self.imp().files.lock().unwrap().set_file_at(index, file); }
	fn set_files(&mut self, files: GestFiles) { *self.imp().files.lock().unwrap() = files; }
	fn set_name_of_file(&mut self, name: String, file: &mut File) {
		let _ = file.set_name(name);
		let index: usize = file.id_number() as usize;
		self.set_file_at(index, file.clone());
	}
	// setter gtk
	fn set_status(&mut self,status:&str){ self.imp().copy_status.set_label(status); }

	//getter
	fn state_copy(&self) -> StateCopy { *self.imp().state_copy.lock().unwrap() }
	fn state_trans(&self) -> StateTrans { *self.imp().state_trans.lock().unwrap() }
	fn _is_copied(&self) -> bool { *self.imp().is_copied.lock().unwrap() }
	fn is_renamed(&self) -> bool { *self.imp().is_renamed.lock().unwrap() }
	fn is_warning(&self) -> bool { *self.imp().is_warning.lock().unwrap() }
	fn is_choosed(&self) -> bool { *self.imp().is_choosed.lock().unwrap() }
	fn current_id_file(&self) -> usize { *self.imp().current_id_file.lock().unwrap() }
	fn files(&self) -> GestFiles { self.imp().files.lock().unwrap().get() }
	fn file_at(&self,index: usize) -> File { self.files().file_at(index) }
	fn nb_files(&self) -> u64 { self.files().nb_file() }
	fn path_dest(&self) -> String  { self.files().path_dest() }
	fn option(&self) -> Vec<bool> { self.imp().option.lock().unwrap().to_vec() }
}
unsafe impl Send for Working {}
unsafe impl Sync for Working {}