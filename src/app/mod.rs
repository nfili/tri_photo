/**
    Ce module permet de gérer le comportement du programme au lancement,
    initialise les différentes fenetres du programme, les constantes de fonctionnement
    et la langue
    * */
    use std::{io::{BufRead, BufReader}, path::{Path, PathBuf}, process::exit};
    use gtk::{
        traits::{GtkWindowExt, FileChooserExt},
        prelude::{GtkApplicationExt,ApplicationExt,ApplicationExtManual,DialogExtManual,FileExt},
        gio::prelude::ActionMapExt
    };
    use crate::{file_choser::new_load, gest_files::GestFiles, gui::GuiWindow, popup_accueil::PopupAccueil, text::Text, working::Working};

    pub const APP_ID: &str = "org.gtk_rs.tri_photo";
    pub const APP_RSC: &'static str = "/org/gtk_rs/tri_photo";
    pub const NAME: &str="Tri Photo";
    pub const VERSION: & str = env!("CARGO_PKG_VERSION");
    pub const HEIGHT: i32 = 250;
    pub const WIDTH: i32 = 400;

    #[derive(Default,Debug,Clone,Eq,Hash,PartialEq)]
    pub enum Cmd {
        #[default] None,
        Quit,
        Load,
        Conf,
        Run,
        Close
    }

    /// Fonction qui lance et gère le comportement du programme
    pub fn run_gui(){
        /// Chargement du CSS
        fn load_css() {
            let provider = gtk::CssProvider::new();
            provider.load_from_data(include_str!("style.css"));
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
        }
        /**
         Fonction qui cré la fenetre principale du programme
         * app : le gtkApplication du programme
         # -> Retourne gtkApplicationWindow
         * */
         fn create_main_widow(app: &gtk::Application) -> gtk::ApplicationWindow {
            let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title(format!("{} {}",NAME," main window").as_str())
            .build();
            window.set_default_size(WIDTH,HEIGHT);
            let icon_theme= gtk::IconTheme::default();
            icon_theme.set_theme_name(Some("hicolor"));
            icon_theme.set_search_path(&[
                Path::new("/usr/share/icons/hicolor/8x8/apps/"),
                Path::new("/usr/share/icons/hicolor/16x16/apps/"),
                Path::new("/usr/share/icons/hicolor/20x20/apps/"),
                Path::new("/usr/share/icons/hicolor/22x22/apps/"),
                Path::new("/usr/share/icons/hicolor/24x24/apps/"),
                Path::new("/usr/share/icons/hicolor/32x32/apps/"),
                Path::new("/usr/share/icons/hicolor/36x36/apps/"),
                Path::new("/usr/share/icons/hicolor/44x44/apps/"),
                Path::new("/usr/share/icons/hicolor/48x48/apps/"),
                Path::new("/usr/share/icons/hicolor/64x64/apps/"),
                Path::new("/usr/share/icons/hicolor/72x72/apps/"),
                Path::new("/usr/share/icons/hicolor/80x80/apps/"),
                Path::new("/usr/share/icons/hicolor/128x128/apps/"),
                Path::new("/usr/share/icons/hicolor/256x256/apps/"),
                Path::new("/usr/share/icons/hicolor/512x512/apps/")]);
                
            window.set_icon_name(Some("tp"));
            let action_close = gtk::gio::SimpleAction::new("close", None);
            action_close.connect_activate(glib::clone!(@weak window, @weak app =>  move |_, _| {
                window.close();
                app.quit();
            }),
            );
            window.add_action(&action_close);
            app.set_accels_for_action("win.close", &["<Ctrl>W"]);
            window
        }
        // Initialisation de gtk, inclusion et enregistrement des resources
        let _ = gtk::init();
        let _ = crate::gresources::init();
        // Création de l'application
        let application = gtk::Application::new(
            Some(APP_ID),
            Default::default(),
            );
        application.connect_startup(|_| load_css());
        application.connect_activate(move |app| {
            let main_window = create_main_widow(app);
            let win = GuiWindow::new(app,&(NAME.to_owned()+&Text::AppConfigTitle.as_string()),&main_window);
            let accueil = PopupAccueil::new(app,&(NAME.to_owned()+&Text::AppHomeTitle.as_string()),&main_window);
            
            accueil.connect_close_request({
               let app = app.clone();
               let win = win.clone();
               move |obj|{
                let mut win = win.clone();
                match obj.action(){
                    Cmd::None=> (),
                    Cmd::Conf=> {
                        win.window().connect_close_request({
                            let app1 = app.clone();
                            let gui = win.clone();
                            move |_| {
                                match gui.action(){
                                    Cmd::Run => {
                                        let mut files = gui.files();
                                        let _tri = gui.tree();
                                        let option_tri =  gui.options_copy();
                                        files.set_path_dest(gui.dest());                                        
                                        let working = Working::new(&app1,files,option_tri);
                                        working.present();
                                    },
                                    Cmd::Close => app1.quit(),
                                    _ => (),
                                }
                                glib::Propagation::Proceed
                            }
                        });
                        win.clone().present();
                        win.set_size();
                    },
                    Cmd::Quit=> app.quit(),
                    Cmd::Load => {
                        let load = new_load(Some(&main_window), &(NAME.to_owned()+&Text::AppLoadTitle.as_string()));
                        let app1=app.clone();
                        load.run_async(move |obj,answer|{
                            obj.close();
                            match answer{
                                gtk::ResponseType::Ok => {
                                    let path = obj.file().unwrap().path();
                                    let unserialize_option:Result<Vec<bool>,std::io::Error> = (move|path:PathBuf| -> Result<Vec<bool>, std::io::Error> {
                                        let mut unserialize:Vec<bool> = Vec::new();
                                        let file = std::fs::File::open(path.into_os_string())?;
                                        let bufreader = BufReader::new(file);
                                        for line in bufreader.lines() {
                                            if let Ok(value) = line {
                                                let v:Vec<_>= value.split('*').collect();
                                                let first_line: Vec<_> = v[0].split(";").collect();
                                                match first_line[0]{
                                                    "options" => {
                                                        let opt_bool: Vec<_> = first_line[1].split(',').collect();
                                                        opt_bool.iter().for_each(|b| {
                                                            if b.to_owned() != ""{
                                                                unserialize.push(b.parse().unwrap());
                                                            }
                                                        });
                                                        break;
                                                    }
                                                    _ => (),
                                                }
                                            }
                                        }
                                        Ok(unserialize)
                                    })(path.clone().unwrap());
                                    let tri = GestFiles::unserialize(path.unwrap().to_str().unwrap().to_string()).unwrap();
                                    let working = Working::new(&app1,tri,unserialize_option.unwrap());
                                    working.present();
                                }                                ,
                                gtk::ResponseType::Cancel => app1.quit(),
                                _ => todo!(),
                            }
                        });
                    },
                    _ => (),
                }
                glib::Propagation::Proceed
            }});
            accueil.present();
        });
        application.run_with_args(&["None"]);
        exit(0);
    }
    /**
     Fonction qui verifie que la taille du repertpoire de destination
     est superieur à la taille des fichiers à copier.
     * dest: &str (repertoire de destination)
     * taille_copie: u64 (taille des fichiers à copier)
     # -> Retourne true si la condition est validée, sinon retourne false.
     * */
     pub fn is_valid_taille(dest: &str, taille_copie: u64 ) -> bool {
        match fs2::available_space(dest) {
            Ok(space)=>{
                if space > taille_copie+(100*4096) {
                    return true;
                }
            }
            Err(_) => (),
        }
        false
    }
    /** 
     Fonction qui convertit une taille d'octet en caractère lisible facilement
     * s: u64 (taille en octet)
     
     # Example
     ```
     use crate::app::App;
     
     let taille_comprehensible = human_read(10000);
     assert_eq!("10Ko",taille_conprehensible);
     ```
     # -> Renourne dans un string la taille lisible facilement
     * */
     pub fn human_read(s: u64) -> String {
        let byte = byte_unit::Byte::from_bytes( u128::try_from(s).unwrap());
        byte.get_appropriate_unit(true).to_string()
    }


