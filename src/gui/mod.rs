mod gif_paintable;
mod help_gui;
use std::{
	borrow::BorrowMut, cell::RefCell, collections::HashMap, path::Path, rc::Rc, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration
};
use gio::prelude::{ApplicationExt, FileExt};
use glib::{ControlFlow, IsA, ObjectExt};
use gtk::{
	prelude::{
		BoxExt, ButtonExt, CheckButtonExt, DialogExtManual, EditableExt, FileChooserExt, GridExt, GtkWindowExt, WidgetExt
	}, Align, Button, CheckButton, Entry, Frame, Grid, IconSize, Image, Label, Orientation, Picture, Widget, Window
};
use crate::{
	app::{human_read, is_valid_taille, Cmd, HEIGHT, VERSION, WIDTH},
	file_choser::new_select, gest_files::GestFiles, header_bar, gui::help_gui::HelpGui, text::Text
};

use self::gif_paintable::GifPaintable;

/// Représente l'utilisation du selecteur de répertoire
 #[derive(PartialEq,Clone,Debug)]
enum EntryDir{
    /// Repertoire source
    Source,
    /// Répertoire de destination 
    Destination,
}
/// Etats de la zone d'information
pub enum StateInfo{
    Valide,
    Invalide,
    Updating,
    None,
}
/// Représente les types de fichiers à trouver
#[derive(Debug,Clone,Eq, Hash, PartialEq)]
pub enum GuiSearchFile{
    Bmp,
    Jpg,
    Gif,
    Png,
    Tiff,
    OnlyPhoto,
}
/// Représente les différentes options de GuiWindow
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum GuiTree{
    Day = 0,
    Month,
    Year,
    Geo,
    Letter,
}
#[derive(Debug,Clone,Eq, Hash, PartialEq)]
pub enum GuiAfterRun{
    Delete=5,
    Rename,
    ChooseOnDemand,
}
#[derive(Debug,Clone)]
pub struct GuiWindowWork{
	// variable de travail
	action:Rc<RefCell<Cmd>>,
	files: Rc<RefCell<GestFiles>>,
	search: Rc<RefCell<HashMap<GuiSearchFile,(bool,gtk::CheckButton)>>>,
	tree:  RefCell<HashMap<GuiTree,(bool,gtk::CheckButton)>>,
	ar: RefCell<HashMap<GuiAfterRun,(bool,gtk::CheckButton)>>,
}
impl GuiWindowWork{
	/// Fonction initialisant les variables de travail
	pub fn init() -> Self{
		Self {
    		action: Rc::new(RefCell::new(Cmd::None)),
			files: Rc::new(RefCell::new(GestFiles::new())),
			search: Rc::new(RefCell::new(HashMap::new())),
			tree: RefCell::new(HashMap::new()),
			ar: RefCell::new(HashMap::new()),
		}
	}
	/// retourne un Vec contenant le nom des options de recherche validée
	/// # -> retoune un Vec de GuiSearchFile
    fn get_options_search(&self) -> Vec<GuiSearchFile>{
        let mut  options: Vec<GuiSearchFile> = vec![];
        match self.search.try_borrow(){
            Ok(t) =>{
               for (key,value) in t.iter(){
                    if value.0 {
                        options.push(key.clone());
                    }
                } 
            }
            Err(..) => {
                options = self.get_options_search();
            }
        }
        options
    }
    /// Fonction utilisant le status des options de copy (tree et ar)  
    /// # -> retourne sous un Vec(bool) trié
    pub fn get_all_copy_option(&self) -> Vec<bool> {
        // 0 => a, 1=> mois, 2=> jours, 3 => localisation, 4=> mois noms
        let mut vec:Vec<bool> = vec![false;8];
        for opt in self.tree.borrow().clone(){
        	match opt.0 {
            GuiTree::Day => vec[0]= opt.1.1.is_active(),
            GuiTree::Month => vec[1]= opt.1.1.is_active(),
            GuiTree::Year => vec[2]= opt.1.1.is_active(),
            GuiTree::Geo => vec[3]= opt.1.1.is_active(),
            GuiTree::Letter => vec[4]= opt.1.1.is_active(),
        	};
        }
        for opt in self.ar.borrow().clone(){
        	match opt.0 {
            	GuiAfterRun::Delete => vec[5]= opt.1.1.is_active(),
            	GuiAfterRun::Rename => vec[6]= opt.1.1.is_active(),
            	GuiAfterRun::ChooseOnDemand => vec[7]= opt.1.1.is_active(),
        	}
        }
        vec
    }
    /**
     Retoune la valeur  de la recherche, si key existe  
     * key: &str (la key)
     * 
     # -> Retourne un bool dans une option , sinon None
     * */
    pub fn get_search_bool_at(&self, key: &GuiSearchFile) -> Option<bool> {
        if let Some(b) = self.search.as_ref().borrow().get(key) {
            return Some(b.0)
        }
        None
    }
    /**
     Retoune dans une option le checkButton, si key existe  
     * key: &str (la key)
     * 
     # -> Retourne CheckButton dans une Option, sinon None
     * */
    pub fn get_search_button_at(&self, key: &GuiSearchFile) -> Option<CheckButton> {
        if let Some(cb) = self.search.as_ref().borrow().get(key){
            return Some(cb.1.clone())
        }
        None
    }
    /**
     Retoune dans une option le tulpe, si key existe  
     * key: &str (la key)
     * 
     # -> Retourne (bool,CheckButton) dans une Option, sinon None
     * */
    pub fn get_search_tulpe_at(&self, key: &GuiSearchFile) ->Option<(bool,CheckButton)> {
    	if let Some(cb) = self.search.as_ref().borrow().get(key){
            return Some(cb.clone())
        }
        None
    }
    /**
     Fonction modifiant l'état des checkbutton de GuiWindow
     * b (bool) true ou false
     * hashs : HashMap (T,(bool,CheckButton))  T : GuiSearchFile,GuiAfterRun, GuiTree
     * */
    pub fn set_state_check_button<T: std::fmt::Debug>(&mut self,b: bool,hashs:&mut HashMap<T,(bool,CheckButton)>){
    	for opt in hashs.borrow_mut().iter_mut(){
    		opt.1.1.set_sensitive(b);
    	}
    }
    /**
     Fonction modifiant la valeur checked des boutons  de  Hasmpap "search"
     * value: (bool) true ou false 
     * hashs HashMap (GuiSearchFile,(bool,CheckButton))
     * */
	pub fn set_search_checked(&mut self, b: bool,hashs:&mut HashMap<GuiSearchFile,(bool,CheckButton)>){
    	for opt in hashs.borrow_mut().iter_mut(){
    		opt.1.1.set_active(b);
    	}
    }
    /**
     Fonction modifiant le boolean du Hasmpap "search"
     * key (GuiSearchFile): Bmp, Png, Tiff, Gif, Jpg, OnlyPhoto
     * value: (bool) true ou false 
     * */
	pub fn set_search_bool_at(&mut self, key: &GuiSearchFile, value: bool){
    	if let Some((b,_rc)) = self.search.as_ref().borrow_mut().get_mut(&key) {
            *b=value;
        }
    }
    /**
     Fonction modifiant le boolean du Hasmpap "tree"
     * key (GuiTree): Day,Month,Year,GeoLoc,Letter 
     * value: (bool) true ou false 
     * */
    pub fn set_tree_bool_at(&mut self, key: &GuiTree, value: bool){
    	if let Some((b,_cb)) = self.tree.borrow_mut().get_mut(&key) {
            *b=value;
        }
    }
    /**
     Fonction modifiant le boolean du Hasmpap "ar"
     * key (GuiAfterRun): Delete, Rename
     * value: (bool) true ou false 
     * */
    pub fn set_ar_bool_at(&mut self, key: &GuiAfterRun, value: bool){
    	if let Some((b,_cb)) = self.ar.borrow_mut().get_mut(&key) {
            *b=value;
        }
    }

	/**
     Fonction qui converti les options de recherche validées en un tableau de nom d'extension
     * 
     # -> Retourne Vec(String) contenant les extensions à rechercher
     * */
    pub fn shearch_validate_to_vec_string(&self) -> Vec<String>{
    	let options = self.get_options_search();
    	let mut opt_string:Vec<String> = Vec::new();
            let _ = options.iter().for_each(|x| {
                 match x{
                    GuiSearchFile::Bmp => opt_string.push("bmp".to_string()),
                    GuiSearchFile::Jpg =>  opt_string.push("jpg".to_string()),
                    GuiSearchFile::Png =>  opt_string.push("png".to_string()),
                    GuiSearchFile::Gif =>  opt_string.push("gif".to_string()),
                    GuiSearchFile::Tiff =>  opt_string.push("tiff".to_string()),
                    GuiSearchFile::OnlyPhoto => opt_string.push("only_photo".to_string()),
                }
            });
            opt_string
    }
   	/**
   	 Verifie si les checkbutton de search sont séléctionner 
   	 *
   	 # -> Retourne true si un des checkboutton est coché sinon false
   	 * */
    pub fn search_checkbutton_is_active(&self) -> bool{
    	for opt in self.search.as_ref().borrow().iter(){
    		if opt.1.1.is_active(){
    			return true
    		}
    	}
    	return false;
    }
	// getter
	pub fn action(&self)-> Cmd  { self.action.as_ref().borrow().clone()}
	pub fn files(&self) ->GestFiles { self.files.as_ref().borrow().clone() }
	pub fn search(&self)-> HashMap<GuiSearchFile,(bool,gtk::CheckButton)> { self.search.as_ref().borrow().clone() }
	pub fn tree(&self)-> HashMap<GuiTree,(bool,gtk::CheckButton)> { self.tree.borrow().clone()}
	pub fn ar(&self)-> HashMap<GuiAfterRun,(bool,gtk::CheckButton)> { self.ar.borrow().clone() }

	// setter
	pub fn set_action(&mut self, action: Cmd ) { self.action.replace(action); }
	pub fn set_files(&mut self, files: GestFiles ) { self.files.replace(files); }
	pub fn set_search(&mut self, searchs: HashMap<GuiSearchFile,(bool,gtk::CheckButton)> ) { self.search.borrow_mut().replace(searchs); }
	pub fn set_tree(&mut self, tree: HashMap<GuiTree,(bool,gtk::CheckButton)> ) { self.tree.replace(tree); }
	pub fn set_ar(&mut self, ar: HashMap<GuiAfterRun,(bool,gtk::CheckButton)> ) { self.ar.replace(ar); }
}
unsafe impl Send for GuiWindowWork{}

#[derive(Clone)]
pub struct GuiWindow{
	// Gtk
	window: Window,
	source: Entry,
	dest: Entry,
	btn_source: Button,
	btn_dest: Button,
	annuler: Button,
	valider: Button,
	nb_file: Label,
	tt_file: Label,
	tt_dur: Label,
	info: Label,
	img_info: Picture,
	jpg: CheckButton,
	png: CheckButton,
	gif: CheckButton,
	bmp: CheckButton,
	tiff: CheckButton,
	only_photo: CheckButton,
	search: Button,
	year: CheckButton,
	month: CheckButton,
	day: CheckButton,
	geo_loc: CheckButton,
	letter: CheckButton,
	rename: CheckButton,
	delete: CheckButton,
	choose_on_demand: CheckButton,
	apercu: Image,
	// variable de travail
	gww: GuiWindowWork,
	height: i32,
	width: i32,
}

impl GuiWindow{
	/**
	Fonction créant une nouvelle interface de configuration de travail
	 * app: L'application en elle même
	 * title: Le titre de la fenêtre
	 * parent: La fenêtre appelant cette interface
	 # -> Retourne la structure compléte GuiWindow
	 * */
	pub fn new<P: glib::IsA<gtk::Application>>(app: &P,title: &str,parent: &gtk::ApplicationWindow) ->Self {
		// Initialisation des variables
		let mut gui =Self::init_values(app, title, parent);
		// Création de l'interface
		gui.create_hb(title);
		// Conteneur principale
		let v_box_top_level = Self::create_box(Orientation::Vertical, Align::Baseline, 18, 24, 24, 18, 18);
		v_box_top_level.append(&gui.create_space_work());
		v_box_top_level.append(&gui.create_option_search());
		v_box_top_level.append(&gui.create_option_tri());
		v_box_top_level.append(&gui.create_option_after_tri());
		v_box_top_level.append(&gui.create_gest_btn());
		// Ajout du conteneur principale à la fenêtre
		gui.window.set_child(Some(&v_box_top_level));
		//  Initialise les actions
		gui.init_btn_clicked();
		gui.init_state_checkbox();
		gui.init_entrys();
		gui.mod_apercu();
		gui.checkbox_clicked();
		gui.search_connect_clicked();
		gui.ar_connect_clicked();
		// Retourne ma structure
		gui
	}
	/**
	Fonction initialisant les variables de la structure
	 * app: L'application en elle même
	 * title: Le titre de la fenêtre
	 * parent: La fenêtre appelant cette interface
	 * 
	 # -> Retourne la structure complétement initialisé
	 * */
	fn init_values<P: glib::IsA<gtk::Application>>(app: &P,title: &str,parent: &gtk::ApplicationWindow) -> Self {
		let window = Window::builder()
			.application(app)
            .title(title)
            .default_width(WIDTH)
            .default_height(HEIGHT)
            .transient_for(parent)
            .hide_on_close(true)
            .icon_name("tp")
            .build();
        let mut rsc = Rsc::new();
		let source = Self::create_entry(&Text::GuiEntrySource.as_string());
		let dest = Self::create_entry(&Text::GuiEntryDestination.as_string());
		let btn_source = Self::create_btn_sel(rsc.mod_path("rep_chooser.png").unwrap());
		let btn_dest = Self::create_btn_sel(rsc.mod_path("rep_chooser.png").unwrap());
		let annuler = Self::create_btn(&Text::Quitter.as_string(), rsc.mod_path("quit.png").unwrap(), "btn", true);
		annuler.child().unwrap().last_child().unwrap().set_size_request(32,32);
		annuler.set_margin_end(12);
		let valider = Self::create_btn(&Text::GuiDemarrerCopy.as_string(), rsc.mod_path("copier.png").unwrap(), "btn", false);
		valider.child().unwrap().last_child().unwrap().set_size_request(32,32);
		let nb_file = Label::builder().label("0").margin_end(0).build();
		let tt_file = Label::builder().label("0").build();
		let tt_dur = Label::builder().label("/0GiB").build();
		let info = Label::builder()
			.width_request(310)
			.label(&Text::GuiSelWait.as_string())
			.css_classes(["infos"])
			.build();
		let img_info = Picture::builder()
			.css_classes(["img_infos"])
			.width_request(16)
			.height_request(16)
			.build();
		let jpg = Self::create_checkbutton("jpg");
		let png = Self::create_checkbutton("png");
		let gif = Self::create_checkbutton("gif");
		let bmp = Self::create_checkbutton("bmp");
		let tiff = Self::create_checkbutton("tiff");
		let only_photo = Self::create_checkbutton(&Text::GuiOnlyPhoto.as_string());
		only_photo.set_halign(gtk::Align::Center);
		only_photo.set_width_request(150);
		let search = Self::create_btn_sel(rsc.mod_path("icone_search.png").unwrap());
		search.set_tooltip_text(Some(&Text::GuiChercher.as_string()));
		search.hide();
		search.set_halign(gtk::Align::Center);
		search.set_cursor_from_name(Some("pointer"));
		let year = Self::create_checkbutton(&Text::GuiYear.as_string());
		let month = Self::create_checkbutton(&Text::GuiMonth.as_string());
		let day = Self::create_checkbutton(&Text::GuiDay.as_string());
		let geo_loc = Self::create_checkbutton(&Text::GuiLieu.as_string());
		let letter = Self::create_checkbutton(&Text::GuiLetter.as_string());
		letter.set_halign(gtk::Align::Center);
		letter.set_hexpand(false);
		letter.set_margin_bottom(8);

		let rename = Self::create_checkbutton(&Text::GuiRename.as_string());
		let delete = Self::create_checkbutton(&Text::GuiDelete.as_string());
		let choose_on_demand = Self::create_checkbutton(&Text::GuiChooseOnDemand.as_string());
		let apercu = Image::builder()
		.resource(rsc.mod_path("apercu_none.png").unwrap())
		.height_request(100)
		.width_request(100)
		.build();
		let height: i32 = 0;
		let width: i32 = 0;
		GuiWindow{
			window: window.clone(),
			source,
			dest,
			btn_source,
			btn_dest,
			annuler,
			valider,
			nb_file,
			tt_file,
			tt_dur,
			info,
			img_info,
			jpg,
			png,
			gif,
			bmp,
			tiff,
			only_photo,
			search,
			year,
			month,
			day,
			geo_loc,
			letter,
			rename,
			delete,
			choose_on_demand,
			apercu,
			//  variable de travail
			gww: GuiWindowWork::init(),
			height,
			width,
		}
	}
	/*-----------------------------------------------------------------------------------------------*/
	/*-------------------------------CREATION DE LA VUE DE L'INTERFACE-------------------------------*/
	/*-----------------------------------------------------------------------------------------------*/
	/// Création de la header bar
	fn create_hb(&self,title:&str){
		fn create_btn_hb(res: &str) -> Button{
			let img = Image::builder()
		  	.resource(res)
		  	.margin_start(0)
	        .width_request(16)
	        .css_classes(["btn_lab_hb"])
		  	.build();

			Button::builder()
			.css_classes(["btn_hb"])
			.child(&img)
	        .build()
		}
		let mut rsc = Rsc::new();
		let hb_help = create_btn_hb(rsc.mod_path("help.png").unwrap());
        let help = HelpGui::new("Aide", &self.window);
        hb_help.set_cursor_from_name(Some("pointer"));
        hb_help.connect_clicked(move |obj|{
            obj.set_cursor_from_name(Some("grabbing"));
            help.show();
            obj.set_cursor_from_name(Some("pointer"));
        });
        let hb_about = create_btn_hb(rsc.mod_path("about.png").unwrap());
        hb_about.set_cursor_from_name(Some("pointer")); 
        hb_about.connect_clicked( move |obj|{
            obj.set_cursor_from_name(Some("grabbing"));
            let about = gtk::AboutDialog::new();
            about.set_modal(true);
            about.set_icon_name(Some("tp"));
            about.set_css_classes(&["about"]);
            about.set_hide_on_close(true);
            about.set_version(Some(VERSION));
            about.set_program_name(Some("TriPhoto"));
            about.set_license_type(gtk::License::MitX11);
            about.set_comments(Some(&Text::GuiAboutShortDescription.as_string()));
            about.set_copyright(Some("© 2023 Nicolas Filippozzi"));
            about.set_artists(&["Nicolas Filippozzi","https://thenounproject.com/browse/icons/term/like/ Ilham Fitrotul Hayat"]);
            about.set_authors(&[ "Nicolas Filippozzi"]);
            about.set_documenters(&["Nicolas Filippozzi"]);
            about.set_logo_icon_name(Some("tp"));
            
            about.show();
            obj.set_cursor_from_name(Some("pointer"));
        });
        self.window.set_titlebar(Some(&header_bar::new_with_widget_at_end(title
            ,vec![&hb_about,&hb_help],{
            let this:GuiWindow = self.clone();
            move || { this.quit() }
             })));
	}
	/**
	 Fonction de création de gtkEntry pour l'espace de travail
	 * s: Texte 
	 * 
	 # -> Retoune la gtkEntry initialisé
	 * */
	fn create_entry(s: &str) -> Entry{
 		Entry::builder()
 			.editable(false)
 			.text(s)
 			.width_request(310)
 			.build()
	}
	/**
	 Fonction de création de gtkButton pour la sélection de répertoire
	 * res: Resource image 
	 * 
	 # -> Retoune le gtkButton initialisé
	 * */
  	fn create_btn_sel(res: &str) -> Button{
  		let img = Image::builder()
	  	.resource(res)
	  	.build();
  		Button::builder()
  			.child(&img)
	  		.css_classes(["btn_sel"])
	  		.build()
	}
	/**
	Fonction de création de gtkButton
	 * label: Texte du bouton
	 * res: Resource image 
	 * class: Nom de la classe de style
	 * is_active: Le bouton doit etre actif ou pas
	 * 
	 # -> Retoune le gtkButton initialisé
	 * */
	fn create_btn(label : &str, res: &str,class: &str,is_active: bool) -> Button {
		let img = Image::builder()
	  	.resource(res)
	  	.build();
	  	let label = Label::builder()
	  	.label(label)
	  	.build();
	  	let h_box = gtk::Box::new(gtk::Orientation::Horizontal,5);
	  	h_box.append(&label);
	  	h_box.append(&img);

		Button::builder()
		.css_classes([class])
		.sensitive(is_active)
		.child(&h_box)
		.build()
  	}
  	/**
  	Fonction de création de gtkCheckButton 
	 * label: Texte du CheckButton 
	 * 
	 # -> Retoune le gtkCheckButton initialisé
	 * */
  	fn create_checkbutton(label: &str) -> CheckButton {
        CheckButton::builder()
        	.label(label)
        	.css_classes(["check_label"])
        	.build()
  	}
  	/**
  	Fonction de création de gtkFrame, conteneur principale de chaque section
	 * title: Titre de la section 
	 * child: Le widget de contenu de la section
	 * 
	 # -> Retoune le gtkFrame initialisé
	 * */
  	fn create_frame(title: &str,child: &impl IsA<Widget> )->Frame{
  		let lab_title = Label::builder()
  			.label(title)
  			.can_focus(false)
  			.hexpand(true)
  			.margin_start(12)
  			.build();
  		let frame = Frame::builder()
  			.label_widget(&lab_title)
  			.child(child)
  			.build();
  		frame
  	}
  	/**
  	Fonction créant le conteneur gtkBox principal de chaque section
  	 * 
  	 # -> Retourne le gtkBox  initialisé
  	 * */
  	fn create_conteneur0(&self) -> gtk::Box{
  		let box_0 = Self::create_box(gtk::Orientation::Vertical, gtk::Align::Baseline, 5, 0, 0, 0, 0);
  		box_0.set_widget_name("box_conteneur0");
  		box_0
  	}
  	/**
  	Fonction de création de gtkBox
	 * orientation:  L'orientation de l'orientable.
	 * align: Alignement des contenus
	 * spacing: espace entre chaque enfant
	 * mar_bottom: marge depuis le bas
	 * mar_top: marge depuis le haut
	 * mar_start: marge depuis le coté gauche
	 * mar_end: marge depuis le coté droit
	 * 
	 # -> Retoune le gtkBox initialisé
	 * */
  	fn create_box(orientation: Orientation,halign: Align,spacing: i32,
  		mar_bottom: i32, mar_top: i32,mar_start: i32, mar_end:i32) -> gtk::Box {
		gtk::Box::builder()
			.orientation(orientation)
			.margin_top(mar_top)
			.margin_bottom(mar_bottom)
			.margin_start(mar_start)
			.margin_end(mar_end)
			.halign(halign)
			.spacing(spacing)
			.build()
  	}
  	/// # -> Retourne dans un gtkFrame la section Répertoire de travail
  	fn create_space_work(&self) -> Frame{
  		let height = 1;
  		let width = 1;
  		let grid = Grid::builder()
	  		.column_spacing(14)
	  		.row_spacing(14)
	  		.margin_bottom(8)
	  		.margin_top(14)
	  		.margin_start(12)
	  		.margin_end(12)
	  		.build();
  		grid.attach(&self.source, 0, 0, width, height);
  		grid.attach(&self.btn_source, 1, 0, width, height);
  		grid.attach(&self.dest, 0, 1, width, height);
  		grid.attach(&self.btn_dest,1, 1, width, height);

  		let h_box0 = Self::create_box(gtk::Orientation::Horizontal, gtk::Align::Center, 0, 10, 0, 12, 12);
		let lab_nb_file = Label::builder()
			.label(&Text::GuiFileFound.as_string())
			.margin_start(0)
			.build();
		h_box0.append(&lab_nb_file);
		h_box0.append(&self.nb_file);

		let h_box1 = Self::create_box(gtk::Orientation::Horizontal, gtk::Align::Start, 0, 0, 0, 12, 12);
		h_box1.append(&self.tt_file);
		h_box1.append(&self.tt_dur);

		h_box0.append(&h_box1);

		let h_box2 = Self::create_box(gtk::Orientation::Horizontal, gtk::Align::Center, 0, 0, 0, 0, 0);
		h_box2.append(&self.info);
		h_box2.append(&self.img_info);


		let conteneur0 = Self::create_conteneur0(&self);
		conteneur0.append(&grid);
		conteneur0.append(&h_box0);
		conteneur0.append(&h_box2);
		let frame = Self::create_frame(&Text::GuiWorkDir.as_string(),&conteneur0);
		frame
  	}
  	/// # -> Retourne dans un gtkFrame la section Fichier à trouver
  	fn create_option_search(&self) -> Frame{
		let h_box0 = Self::create_box(gtk::Orientation::Vertical, gtk::Align::Center, 8, 8, 0, 0, 0);
		let h_box1 = Self::create_box(gtk::Orientation::Horizontal, gtk::Align::Center, 10, 0, 14, 0, 0);
		h_box1.set_hexpand(true);

		h_box1.append(&self.png);
		h_box1.append(&self.jpg);
		h_box1.append(&self.bmp);
		h_box1.append(&self.gif);
		h_box1.append(&self.tiff);

		h_box0.append(&h_box1);
		h_box0.append(&self.only_photo);
		h_box0.append(&self.search);

  		let conteneur0 = Self::create_conteneur0(&self);
		conteneur0.append(&h_box0);
		let frame = Self::create_frame(&Text::GuiFileAtFound.as_string(),&conteneur0);
		frame
  	}
  	/// # -> Retourne dans un gtkFrame la section Arborescense de tri
  	fn create_option_tri(&self) -> Frame {
  		let h_box0 = Self::create_box(gtk::Orientation::Horizontal, gtk::Align::Center, 60, 8, 14, 0, 0);
  		h_box0.set_homogeneous(true);
  		h_box0.set_hexpand(true);
		let h_box1 = Self::create_box(gtk::Orientation::Vertical, gtk::Align::Baseline, 8, 0, 12, 0, 0);
		let h_box2 = Self::create_box(gtk::Orientation::Vertical, gtk::Align::Baseline, 0, 0, 12, 0, 0);
		h_box2.set_hexpand(true);

		h_box1.append(&self.year);
		h_box1.append(&self.month);
		h_box1.append(&self.day);
		h_box1.append(&self.geo_loc);

		let lab_apercu = Label::new(Some(&Text::GuiView.as_string()));
		h_box2.append(&lab_apercu);
		h_box2.append(&self.apercu);

		h_box0.append(&h_box1);
		h_box0.append(&h_box2);

		let conteneur0 = Self::create_conteneur0(&self);
		conteneur0.append(&h_box0);
		conteneur0.append(&self.letter);
		let frame = Self::create_frame(&Text::GuiSortingTree.as_string(),&conteneur0);
		frame 
  	}
  	/// # -> Retourne dans un gtkFrame la section Option d'aprés tri
  	fn create_option_after_tri(&self) -> Frame {
  		let h_box0 = Self::create_box(gtk::Orientation::Horizontal, gtk::Align::Center, 10, 0, 0, 0, 0);
  		h_box0.set_hexpand(true);

  		h_box0.append(&self.rename);
  		h_box0.append(&self.delete);

		let h_box1 = Self::create_box(gtk::Orientation::Horizontal, gtk::Align::Center, 10, 0, 0, 0, 0);
		h_box1.append(&self.choose_on_demand);

		let v_box0 = Self::create_box(gtk::Orientation::Vertical, gtk::Align::Center, 8,8,14,0,0);
		v_box0.append(&h_box0);
		v_box0.append(&h_box1);

  		let conteneur0 = Self::create_conteneur0(&self);
		conteneur0.append(&v_box0);
		let frame = Self::create_frame(&Text::GuiTraitement.as_string(),&conteneur0);
		frame 
  	}
  	/// # -> Retourne dans un gtkBox les boutons d'action de l'interface( Quitter/Valider )
  	fn create_gest_btn(&self) -> gtk::Box{
		let h_box0 = Self::create_box(gtk::Orientation::Horizontal, gtk::Align::End, 0, 0, 0, 0, 0);
		h_box0.append(&self.annuler);
		h_box0.append(&self.valider);
		h_box0
  	}
	/*-----------------------------------------------------------------------------------------------*/
	
	/*-----------------------------------------------------------------------------------------------*/
	/*------------------------------GESTION DES EVENEMENTS SUR LES BOUTTONS--------------------------*/
	/*-----------------------------------------------------------------------------------------------*/
	/// Initialise les actions des boutons Annuler et Valider
	fn init_btn_clicked(&self) {
        let this = self.clone();
        self.annuler.connect_clicked(move |_| {
            let mut this = this.clone();
            this.set_action(Cmd::Close);
            this.quit();
        });
        let this = self.clone();
        self.valider.connect_clicked( move |_| {
            let mut this = this.clone();
            this.set_action(Cmd::Run);
            this.close();
        });
    }
	/// Initialise les actions des boutons rep_chooser
	fn init_entrys(&mut self){
		/**
		 Fonction modifiant le statut des autres widgets en cas d'erreur, ou d'annulation
		 * gui (GuiWindow) la struct 
		 * text (str) les infos sur le comportement
		 * state (l'etat que la fonction set_info doit traité )
		 * */
		 fn set_all_widgets_on_echec(gui: &mut GuiWindow,text:&str,state:StateInfo){
			 gui.info.set_label(text);
		     gui.set_info(state);
		     gui.gww.set_search_checked(false,&mut gui.search());
		     gui.gww.set_state_check_button(false,&mut gui.search());
		     gui.gww.set_state_check_button(false,&mut gui.tree());
		     gui.gww.set_state_check_button(false,&mut gui.ar());
		     gui.tt_dur.set_label("/0GiB");
		     gui.tt_file.set_label("0");
		     gui.nb_file.set_label("0");
		     gui.init_state_checkbox();
		     gui.search.set_sensitive(false);
		 }
		 fn set_false_search(gui: &mut GuiWindow){
		 	gui.gww.set_search_checked(false, &mut gui.search());
		 }
		 /**
		 Fonction innitialsant le widget entry et le bouton de choix de répertoire
		 * gui (GuiWindow) la struct 
		 * entry_dir (EntryDir) le type de EntryDir::Source, EntryDir::Destination
		 * */
        fn init_entry(gui: &mut GuiWindow,entry_dir: EntryDir){
            let btn: gtk::Button;
            let src: gtk::Entry;
            let title: String;
            let tooltip: String;
            match entry_dir{
                EntryDir::Source => {
                    src= gui.source.clone();
                    btn= gui.btn_source.clone();
                    title= Text::GuiEntrySource.as_string();
                    tooltip= Text::GuiTooltipSource.as_string();
                },
                EntryDir::Destination => {
                    src = gui.dest.clone();
                    btn = gui.btn_dest.clone();
                    title = Text::GuiEntryDestination.as_string();
                    tooltip = Text::GuiTooltipDestination.as_string();
                }
            };
            src.set_text(&title.clone());
            src.set_halign(Align::Fill);
            btn.set_cursor_from_name(Some("pointer"));
            let rep = new_select(Some(&gui.window),&title,&Text::Choice.as_string(),&Text::Cancel.as_string());
            btn.set_tooltip_text(Some(&tooltip));
            btn.connect_clicked({ 
            	let this = gui.clone();
            	let src = src.clone();
            	let rep = rep.clone();
                let title = title.clone();
            	  move |obj| {
            	this.valider.set_sensitive(false);
            	this.nb_file.set_label("0");
            	this.tt_file.set_label("0");
            	this.tt_dur.set_label("/0Gib");
            	this.search.child().unwrap().set_properties(&[("resource",&"/org/gtk_rs/tri_photo/icone_search.png")]);
            	this.search.hide();
            	set_false_search(&mut this.clone());
            	this.init_default(this.search());
                obj.set_cursor_from_name(Some("grabbing"));
                rep.clone().run_async({
                	let mut this = this.clone();
                	let src = src.clone();
                	let entry = entry_dir.clone();
                    let title = title.clone();
                	move |objs,answer| {
	                    objs.hide();
	                    while !this.gww.files.try_borrow_mut().is_ok(){
	                    }
	                    this.gww.files.as_ref().borrow_mut().clear();
	                    match answer{
	                        gtk::ResponseType::Ok =>{
	                        	if objs.file() != None{
	                        	let tmp=objs.file().unwrap().path().unwrap();
	                        	let path = tmp.to_str().unwrap();
	                            src.set_text(&path);
	                            if entry == EntryDir::Destination{
	                            	let taille="/".to_owned()+  
               						 &human_read(fs2::available_space(Path::new(path)).unwrap_or_default());
	                        		this.tt_dur.set_label(taille.as_str());
	                        	}
	                        	}
	                        } 
	                        _ => {
	                        	src.set_text(&title.clone());
	                        	set_all_widgets_on_echec(&mut this,&Text::DataWait.as_string(),StateInfo::None);
	                        }
	                    }
	                    match GuiWindow::verif_dir_status( this.source.clone(), this.dest.clone()) {
	                        Ok(b) => {
	                        	match b{
	                        		true => {
	                        			this.info.set_label(&Text::GuiDirValid.as_string());
	                            		this.gww.set_state_check_button(true,&mut this.search());
	                            		this.set_info(StateInfo::Valide);	                            		
	                        		}
                            		false => {
                            			set_all_widgets_on_echec(&mut this,&Text::DataWait.as_string(),StateInfo::Invalide);
                            		},
	                        	}
	                        },
	                        Err(e) => {
	                        	set_all_widgets_on_echec(&mut this,e.as_str(),StateInfo::Invalide);
	                        },
	                    }
                }});
                obj.set_cursor_from_name(Some("pointer"));            
            }});
        }
        init_entry(self, EntryDir::Source);
        init_entry(self, EntryDir::Destination);
    }
    /** 
     Regle les cases à cocher listée dans le hashmap 
     * hash: HashMap (T,(bool,gtk::CheckButton))
     * */
    fn init_default<T: 'static>(&self, mut hash: HashMap<T,(bool,gtk::CheckButton)>){
        for (_k, v) in hash.iter_mut(){
            if v.0{
                 v.1.set_active(true);
            }
            v.1.set_cursor_from_name(Some("pointer"));
            v.1.set_sensitive(false);
        }
    }
    /**
     Initialise les états des differentes case à cocher
     * Enregistre l'état de chaque case à cocher dans le Hashmap correspondant
     */
    fn init_state_checkbox(&mut self){
        // Case à cocher pour le choix des fichiers à trouver
        self.set_search(HashMap::from([
        (GuiSearchFile::Bmp,(false,self.bmp.clone())),
        (GuiSearchFile::Jpg,(false,self.jpg.clone())),
        (GuiSearchFile::Gif,(false,self.gif.clone())),
        (GuiSearchFile::Png,(false,self.png.clone())),
        (GuiSearchFile::Tiff,(false,self.tiff.clone())),
        (GuiSearchFile::OnlyPhoto,(false,self.only_photo.clone())),
        ]));
        self.init_default(self.search());
        
        // Case à cocher pour le choix de la hiérachie des fichiers copiés
        self.gww.set_tree(HashMap::from([
        (GuiTree::Year,(true,self.year.clone())),
        (GuiTree::Month,(true,self.month.clone())),
        (GuiTree::Day,(true,self.day.clone())),
        (GuiTree::Geo,(false,self.geo_loc.clone())),
        (GuiTree::Letter,(false,self.letter.clone())),
        ]));
        self.init_default(self.tree());

        self.gww.set_ar(HashMap::from([
        (GuiAfterRun::Delete,(false,self.delete.clone())),
        (GuiAfterRun::Rename,(false,self.rename.clone())),
        (GuiAfterRun::ChooseOnDemand,(false,self.choose_on_demand.clone()))
        ]));
        self.init_default(self.ar());
    }
    /**
     Fonction gérant le style css des informations et de l'icone d'état
     * state (StateInfo) Valide, Invalide, Update, None
     * */
    fn set_info(&self,state: StateInfo){
        let mut cmd: Vec<&str> = Vec::new();
        match state{
            StateInfo::Valide => {
                cmd.push("info_valide");
                cmd.push("info_updating");
                cmd.push("info_invalide");
                cmd.push("valide.png");
            }
            StateInfo::Invalide => {
                cmd.push("info_invalide");
                cmd.push("info_valide");
                cmd.push("info_updating");
                cmd.push("invalide.png");
            }
            StateInfo::Updating => {
                cmd.push("info_updating");
                cmd.push("info_invalide");
                cmd.push("info_valide");
                cmd.push("load.gif");
            }
            StateInfo::None =>{
                self.info.remove_css_class("info_valide");
                self.info.remove_css_class("info_invalide");
                self.info.remove_css_class("info_updating");
                self.set_pictures("none",&self.img_info).ok();  
                return ();
            }
        }
        self.info.remove_css_class(cmd[1]);
        self.info.remove_css_class(cmd[2]);
        self.info.add_css_class(cmd[0]);
        self.set_pictures(cmd[3],&self.img_info).ok();  
    }
    /**
     Fonction qui initialise l'image de l'aperçu de la hierachie et
      active le comportement lors des cliques sur les cases à cocher des options de tri
     * */
    fn mod_apercu(&self){
        /**
        fonction interne à la fonction init_tri
        * permet d'afficher une image en fonction de ses choix de tri
        */
        fn set_apercu(gui: &GuiWindow){
            let file:&str;
            let mut is_active:String=String::new();
            let mut rsc=Rsc::new();
            if gui.year.is_active() {
                is_active += "y";
            }
            if gui.month.is_active() {
                is_active += "m";
            }
            if gui.day.is_active(){
                is_active += "d";
            }
            if gui.geo_loc.is_active() {
                is_active += "g"
            }
            match is_active.as_str() {
                "y" => file = "apercu_year.png",
                "m" => file = "apercu_month.png",
                "d" => file = "apercu_day.png",
                "g" => file = "apercu_geo.png",
                "ym" => file = "apercu_year-month.png",
                "yd" => file ="apercu_year-day.png",
                "yg" => file = "apercu_year-geo.png",
                "ymd" => file ="apercu_year-month-day.png",
                "ymg" => file = "apercu_year-month-geo.png",
                "ydg" => file = "apercu_year-day-geo.png",
                "ymdg" => file ="apercu_year-month-day-geo.png",
                "md" => file ="apercu_month-day.png",
                "mg" => file = "apercu_month-geo.png",
                "mdg" => file = "apercu_month-day-geo.png",
                "dg" => file = "apercu_day-geo.png",
                _ =>{
                    file = "apercu_none.png";
                }
            }
            gui.apercu.set_from_resource(rsc.mod_path(file));
        }

        let mut rsc=Rsc::new();

        // aperçu de la hierachie du tri par default
        self.apercu.set_from_resource(rsc.mod_path("apercu_year-month-day.png"));
        self.apercu.set_size_request(100,100);

        // action lors de la modifivation d'une case à cocher des option du tri
        self.year.connect_toggled({
        	let this = self.clone();
        		move |_| {
            		set_apercu(&this);
        }});
        self.month.connect_toggled({
        	let this = self.clone();
        		move |_| {
            		set_apercu(&this);
        }});
        self.day.connect_toggled({
        	let this = self.clone();
        		move |_| {
            		set_apercu(&this);
        }});
        self.geo_loc.connect_toggled({
        	let this = self.clone();
        		move |_| {
            		set_apercu(&this);
        }});
    }

    /**
     Fonction qui charge l'image Gif dans un Paintable pret à intégre
     * s: &str (path de l'image Gif)
     * 
     # -> retourne si s est un chemin valide un resultat Option(GifPaintable) sinon Error  
     * */
     fn set_gif (&self, s: &str) -> Result< Option<GifPaintable>, Box<dyn std::error::Error>>{
     	let mut rsc = Rsc::new();
     	let uri = "resource:///".to_string() + &rsc.mod_path(s).unwrap();
     	// let uri = "resource:///".to_string() + crate::app::APP_RSC + "/" + s;
        let file = gio::File::for_uri(&uri);
            let paintable = self::gif_paintable::GifPaintable::new();
            let (bytes, _) = file.load_contents(gio::Cancellable::NONE)?;
            paintable.load_from_bytes(&bytes)?;
            Ok(Some(paintable))
     }
    /**
     Fonction qui charge l'image ( gif ou autre format)
     * s: &str (path de l'image)
     * */
    pub fn set_pictures(&self, s: &str, img: &Picture) -> Result<(), Box<dyn std::error::Error>>{
        if s.contains(".gif"){
            let p = self.set_gif(s);
            img.set_paintable(p.unwrap().as_ref());
        }
        else {
        	let mut rsc = Rsc::new();
            img.set_resource(rsc.mod_path(s));
        }
        Ok(())
    }
    /// Configuration des actions pour la sélection des checkbutton de search
  	fn checkbox_clicked(&self) {
  		/// Désactive les checkbutton "bmp", "gif", "png" de search quand le chekcbutton "only_photo" est coché
  		fn desactivate_checkbutton(gww: &mut GuiWindowWork){
			gww.get_search_button_at(&GuiSearchFile::Bmp).unwrap().set_sensitive(false);
			gww.get_search_button_at(&GuiSearchFile::Gif).unwrap().set_sensitive(false);
			gww.get_search_button_at(&GuiSearchFile::Png).unwrap().set_sensitive(false);
			// on decoche tous les autres checkbox de recherche
			gww.get_search_button_at(&GuiSearchFile::Bmp).unwrap().set_active(false);
			gww.get_search_button_at(&GuiSearchFile::Gif).unwrap().set_active(false);
			gww.get_search_button_at(&GuiSearchFile::Png).unwrap().set_active(false);
			// on definie les valeurs associé à false 
			gww.set_search_bool_at(&GuiSearchFile::Bmp,false);
			gww.set_search_bool_at(&GuiSearchFile::Gif,false);
			gww.set_search_bool_at(&GuiSearchFile::Png,false);
  		}
  		/// Réactive les checkbutton de search quand le chekcbutton "only_photo" n'est pas coché
  		fn reactivate_checkbox(gww: &GuiWindowWork){
  			gww.get_search_button_at(&GuiSearchFile::Bmp).unwrap().set_sensitive(true);
			gww.get_search_button_at(&GuiSearchFile::Gif).unwrap().set_sensitive(true);
			gww.get_search_button_at(&GuiSearchFile::Png).unwrap().set_sensitive(true);
  		}
  		/**
  		 Active le bouton search en fonction d'un boolean
  		 * b (bool): si true active le bouton, si false le désactive
  		 * 
  		 * */
  		fn activate_btn_search(gui: &GuiWindow, b: bool){
  			gui.search.set_sensitive(b);
  			if b{
  				gui.info.set_label(&Text::GuiFileTypeOk.as_string());
  				gui.set_info(StateInfo::Valide);
  				gui.search.show();
  				gui.search.child().unwrap().set_properties(&[("resource",&"/org/gtk_rs/tri_photo/icone_search.png")]);
  			}
  			else {
  				gui.info.set_label(&Text::GuiFileTypeNone.as_string());
  				gui.set_info(StateInfo::Invalide);
  				gui.search.hide();
  			}
  		}
  		/**
  		 Vérifie si un checkbutton de search est coché
  		 * si oui, on active le bouton search
  		 * si non, on désactive le bouton search et le bouton valider
  		 * 
  		 * */
  		fn gww_search_checkbutton_is_active(gui: &mut GuiWindow){
  			if !gui.gww.search_checkbutton_is_active(){
				activate_btn_search(gui,false);
				gui.valider.set_sensitive(false);
	        }
	        else{
	        	activate_btn_search(gui,true);
			}
			gui.valider.set_sensitive(false);
			gui.gww.set_state_check_button(false,&mut gui.tree());
			gui.gww.set_state_check_button(false,&mut gui.ar());
  		}
  		for item in self.search().iter_mut(){
	  		match item.0{
	  			GuiSearchFile::Jpg => if let Some(btn) = self.gww.get_search_button_at(&item.0){
	        		btn.connect_toggled({
		        		let this = self.clone();
						let typ = item.0.clone();
	        			move |obj|{
	        				let mut this = this.clone();
	        				if obj.is_active(){
	        					this.gww.set_search_bool_at(&typ,true);
	        				}
	        				else{
	        					this.gww.set_search_bool_at(&typ,false);
	        					this.gww.get_search_button_at(&GuiSearchFile::OnlyPhoto).unwrap().set_active(false);
	        					this.gww.set_search_bool_at(&GuiSearchFile::OnlyPhoto,false);		        					
	        				}
	        				gww_search_checkbutton_is_active(&mut this);
	        			}
	        		});
		        },
		        GuiSearchFile::Tiff => if let Some(btn) = self.gww.get_search_button_at(&item.0){
		        	btn.connect_toggled({
						let this = self.clone();
						let typ = item.0.clone();
	        			move |obj|{
	        				let mut this = this.clone();
	        				if obj.is_active(){
	        					this.gww.set_search_bool_at(&typ,true);
	        				}
	        				else{
	        					this.gww.set_search_bool_at(&typ,false);
	        					this.gww.get_search_button_at(&GuiSearchFile::OnlyPhoto).unwrap().set_active(false);
	        					this.gww.set_search_bool_at(&GuiSearchFile::OnlyPhoto,false);
	        				}
	        				gww_search_checkbutton_is_active(&mut this);
	        			}
	        		});
		        },
		        GuiSearchFile::OnlyPhoto =>{
		        	if let Some(btn) = self.gww.get_search_button_at(&GuiSearchFile::OnlyPhoto){
		        		btn.connect_toggled({
			        		let this = self.clone();
			        		let typ = item.0.clone();
		        			move |obj|{
		        				let mut this = this.clone();
			        			if obj.is_active(){
			        				this.gww.set_search_bool_at(&typ,true);
			        				this.gww.set_search_bool_at(&GuiSearchFile::Jpg,true);
			        				this.gww.set_search_bool_at(&GuiSearchFile::Tiff,true);
			        				this.gww.get_search_button_at(&GuiSearchFile::Jpg).unwrap().set_active(true);
			        				this.gww.get_search_button_at(&GuiSearchFile::Tiff).unwrap().set_active(true);
			        				// on desactive les autres checkboxs de recherche
			        				desactivate_checkbutton(&mut this.gww);
			        			}
			        			else {
			        				this.gww.set_search_bool_at(&typ,false);
			        				//  on réactive les autres checkbuttons
			        				reactivate_checkbox(&this.gww);
			        			}
			        			gww_search_checkbutton_is_active(&mut this);
		        			}
		        		});
		        	}
		        },
		        _ => if let Some(btn) = self.gww.get_search_button_at(&item.0){
	        		btn.connect_toggled({
						let this = self.clone();
						let typ = item.0.clone();
	        			move |obj|{
	        				let mut this = this.clone();
	        				if obj.is_active(){
	        					this.gww.set_search_bool_at(&typ,true);
	        				}
	        				else{
	        					this.gww.set_search_bool_at(&typ,false);
	        				}
	        				gww_search_checkbutton_is_active(&mut this);
	        			}
	        		});
		        }
	    	};
  		}
  	}
  	/// Configuration des actions pour le bouton search
  	fn search_connect_clicked(&self){
  		self.search.connect_clicked({
  			let this = self.clone();
  			move |obj| {
  				obj.set_cursor_from_name(Some("grabbing"));
	  			let img = this.set_gif("icone_search.gif");
	  			obj.child().unwrap().set_properties(&[("paintable",&img.unwrap())]);
	  			obj.child().unwrap().set_properties(&[("icon-size",&IconSize::Normal)]);
	  			this.set_sensitive();
	  			obj.set_sensitive(false);
	  			obj.set_cursor_from_name(Some("pointer"));
  		}});
  	}
  	fn ar_connect_clicked(&self){
  		fn set_img_btn(gui: &GuiWindow,img: &str){
  			let mut rsc = Rsc::new();
  			gui.valider.child().unwrap().last_child().unwrap().set_properties(&[("resource",&rsc.mod_path(img).unwrap())]);
  			gui.valider.child().unwrap().last_child().unwrap().set_properties(&[("icon-size",&IconSize::Normal)]);
  		}
  			self.delete.connect_toggled({
  				let this = self.clone();
  				move |x|{
  					if x.is_active(){
  						if this.choose_on_demand.is_active(){
  							this.valider.child().unwrap().first_child().unwrap()
                            .set_properties(&[("label",&Text::GuiDemarrerMoveGestion.as_string())]);
  							set_img_btn(&this,"gestion_deplacer.png");
  						}
  						else {
  							this.valider.child().unwrap().first_child().unwrap()
                            .set_properties(&[("label",&Text::GuiDemarrerMove.as_string())]);
  							set_img_btn(&this,"deplacer.png");
  						}
  					}
  					else {
  						if this.choose_on_demand.is_active(){
  							this.valider.child().unwrap().first_child().unwrap()
                            .set_properties(&[("label",&Text::GuiDemarrerGestion.as_string())]);
  							set_img_btn(&this,"gestion.png");
  						}
  						else {
  							this.valider.child().unwrap().first_child().unwrap()
                            .set_properties(&[("label",&Text::GuiDemarrerCopy.as_string())]);
  							set_img_btn(&this,"copier.png");
  						}
  					}
  				}
  			});
  			self.choose_on_demand.connect_toggled({
  				let this = self.clone();
  				move |x| {
					if x.is_active(){
						if this.delete.is_active(){
  							this.valider.child().unwrap().first_child().unwrap()
                            .set_properties(&[("label",&Text::GuiDemarrerMoveGestion.as_string())]);
  							set_img_btn(&this,"gestion_deplacer.png");
  						}
  						else {
  							this.valider.child().unwrap().first_child().unwrap()
                            .set_properties(&[("label",&Text::GuiDemarrerGestion.as_string())]);
  							set_img_btn(&this,"gestion.png");
  						}
  						this.rename.set_active(false);
  						this.rename.set_sensitive(false);
  					}
  					else {
  						if this.delete.is_active(){
  							this.valider.child().unwrap().first_child().unwrap()
                            .set_properties(&[("label",&Text::GuiDemarrerMove.as_string())]);
  							set_img_btn(&this,"deplacer.png");
  						}
  						else {
  							this.valider.child().unwrap().first_child().unwrap()
                            .set_properties(&[("label",&Text::GuiDemarrerCopy.as_string())]);
  							set_img_btn(&this,"copier.png");
  						}
  						this.rename.set_sensitive(true);
  					}
  				}
  			});
  	}
	/*-----------------------------------------------------------------------------------------------*/

	/*-----------------------------------------------------------------------------------------------*/
	/*---------------------------------Gestion de la structure de GUIWINDOW--------------------------*/
	/*-----------------------------------------------------------------------------------------------*/
	/**
     Fonction verifiant la validité des repertoires choisis
     *  Err 1: sélection identique
     *  Err 2: en attente du repertoire de copie 
     *  Err 3: en attente de sélection
     *
     # -> Renvoi true si aucune erreur, sinon false
    */
	pub fn verif_dir_status(src: Entry,dst: Entry)-> Result <bool, String> {
        if src.text().as_str() != Text::GuiEntrySource.as_string(){
            if dst.text().as_str() != Text::GuiEntryDestination.as_string(){
                if src.text().as_str() != dst.text().as_str(){
                	if dst.text().contains(src.text().as_str()){
                		return Err(Text::GuiErrorDestBis.as_string());
                	}
                    return Ok(true);
                }
                else{
                    return Err(Text::GuiErrorSel.as_string());
                }
            }
            else{
                return Err(Text::GuiErrorDest.as_string());
            }
        }
        Err(Text::GuiSelWait.as_string())
    }
	/**
     Fonction permettant de gérer la fonction de recherche et de copie
     * active le bouton démarrer
     * lance la recherche de fichier
     * lance le calcul de la taille des données
     * verifie la validité des répertoires choisis 
     * */
    pub fn set_sensitive(&self){
        /**
         Fontion qui lance la procédure de listing des fichiers repondant aux critères de recherche
         * gui: GuiWindow (la fenêtre en cours d'utilisation)
         * options: Vec(String) (options de recherche)
         * */
        fn get_files(gui: &mut GuiWindow,tx: std::sync::mpsc::Sender<Vec<String>>, options: Vec<String>){
            let src = gui.source.clone();
            let dst = gui.dest.clone();
            let v = gui.valider.clone();
            
            if let Ok(mut search) = gui.clone().gww.files.try_borrow_mut(){
                let _ =  search.borrow_mut().init(Path::new(src.text().as_str()), &options,{
                	let _this=gui.clone();
		                 move ||{
		                    (Arc::new(Mutex::new(false)),tx.clone())//Temporaire
		                }});               
                let taille="/".to_owned()+  
                &human_read(fs2::available_space(Path::new(dst.text().as_str())).unwrap_or_default());
                gui.tt_dur.set_label(&taille);
               

                if is_valid_taille(dst.text().as_str(), search.size()){
                    gui.set_info(StateInfo::Valide);
                    v.set_sensitive(true);
                    gui.search.set_sensitive(true);
                    gui.search.child().unwrap().set_properties(&[("resource",&"/org/gtk_rs/tri_photo/icone_search_ok.png")]);
                    sleep(Duration::from_millis(800));
                    gui.search.hide();
                    gui.window.set_default_height(gui.height);
  					gui.gww.set_state_check_button(true, &mut gui.tree());
            		gui.gww.set_state_check_button(true, &mut gui.ar());
                    gui.info.set_label(&Text::GuiReadyCopy.as_string());
                }
                else{
                    gui.set_info(StateInfo::Invalide);
                }
            }
        }
        let (tx,rx) = std::sync::mpsc::channel();
        match Self::verif_dir_status(self.source.clone(),self.dest.clone()){
            Ok(_b) => {
            	
                self.window.set_cursor_from_name(Some("progress"));
                self.info.set_label(&Text::GuiScanningProgress.as_string());
                self.set_info(StateInfo::Updating);
                self.gww.files.as_ref().borrow_mut().clear();
                self.tt_file.set_text("0");
                self.nb_file.set_text("0");
                let _handle = thread::spawn({
                	let mut this = self.clone();
                	let tx = tx.clone();
                	let options = this.gww.shearch_validate_to_vec_string();
                		move || {
                			get_files(&mut this,tx,options);
                }});
			},
            Err(e) => {
                self.info.set_label(&e);
                self.set_info(StateInfo::Invalide);
                self.valider.set_sensitive(false);
            }
		}
		glib::timeout_add_local(Duration::from_millis(5),{
					let this = self.clone();
    				move || {
	                    match rx.try_recv(){
	                        Ok(value) => {
	                            this.nb_file.set_label(value[0].as_str());
	                            this.tt_file.set_label(value[1].as_str());
	                            glib::ControlFlow::Continue
	                        },
	                        Err(std::sync::mpsc::TryRecvError::Empty) => ControlFlow::Continue,
	                        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
	                            ControlFlow::Break
	                        },
	                    }
	            }});
        self.window.set_cursor_from_name(Some("normal"));
    }

  	/// Fonction qui permet de quitter l'application
  	fn quit(&self){	self.window.application().unwrap().quit() }
  	/// Fonction qui ferme la fenêtre
  	fn close(&self){ self.window.close(); }
  	/// Fonction qui affiche l'interface
	pub fn present(&self){
		self.window.present();
	}
	/*-----------------------------------------------------------------------------------------------*/

	// getter
	pub fn dest(&self) -> String {self.dest.text().to_string()}
	pub fn window(&self) -> Window {self.window.clone()}
	// getter gww
    pub fn action(&self) -> Cmd {self.gww.action()}
    pub fn search(&self) -> HashMap<GuiSearchFile,(bool,CheckButton)> { self.gww.search() }
	pub fn tree(&self) -> HashMap<GuiTree,(bool,CheckButton)> { self.gww.tree() }
	pub fn ar(&self) -> HashMap<GuiAfterRun,(bool,CheckButton)> { self.gww.ar()}
	pub fn files(&self) -> GestFiles { self.gww.files()	}
	pub fn options_copy(&self) -> Vec<bool> {self.gww.get_all_copy_option()	}
	// setter
	pub fn set_size(&mut self){
		self.height = self.window.default_height();
		self.width = self.window.default_width();
	}
	// gww
    fn set_action(&mut self, action: Cmd) { self.gww.set_action(action); }
	fn set_search(&mut self, search: HashMap<GuiSearchFile,(bool,CheckButton)>) { self.gww.set_search(search); }
}
unsafe impl Send for GuiWindow {}

/// Getsion des ressourses de l'application
#[derive(Debug,Default)]
pub struct Rsc{
    path: String
}
impl Rsc {
    pub fn new() -> Self {
        Rsc::default()
    }
    pub fn default() -> Self{
        Rsc{
            path:String::new(),
        }
    }
    /// # -> Retourne dans une option le chemin complet de la ressource
    pub fn mod_path(& mut self, s:&str) -> Option<&str> {
        self.path = crate::app::APP_RSC.to_string() + "/" + s;
        Some( &self.path)
    }
}