use regex::Regex;
use std::{
	fs::read_dir,
	io::{BufRead,BufReader}, 
	path::Path,
	sync::{Arc, Mutex}
};

use crate::{
	app::App,
	directory::is_dir,
	file::File,
	geo
};

#[derive(Debug,Clone,Default)]
pub struct GestFiles{
	path_src: String,
	path_dest: String,
	current: u64,
	nb_file: u64,
	files: Vec<File>,
	size: u64,
	status_init: bool,
}
impl GestFiles{
	/// Initialise la structure
	pub fn new() -> GestFiles {
		Self::default()
	}
	/// Initialisation par default
	fn default() -> GestFiles{
		GestFiles {
				path_src: String::new(),
				path_dest: String::new(),
				current: 0,
				nb_file: 0,
				files: Vec::new(),
				size: 0,
				status_init:false,
			}
	}

/**
 Fonction publique initialisant la recherche de fichier
 * path: Chemin du répertoire à parcourir
 * option: Option de recherche de fichier passée à la fonction
 * callback:  Option de tableau gtk::Label qui affiche les informations sur l'évolution du parcour
* */
pub fn init<F>(&mut self, path: &Path,option: &Vec<String>, mut callback: F)
	where
		F: FnMut() ->(Arc<Mutex<bool>>,std::sync::mpsc::Sender<Vec<String>>)
		{
		self.clear();
		if is_dir(path){
			self.path_src = path.to_str().unwrap().to_string();
			self.parcours_rep(path,option,callback().0,callback().1);
			self.status_init = true;
		}
	}

	/**
	 Fonction interne de recherche de fichier 
	 * path: Chemin du répertoire à parcourir
	 * option: Option de recherche de fichier passée à la fonction
	 * callback:  Option de tableau gtk::Label qui affiche les informations sur l'évolution du parcour
	 * */
	fn parcours_rep(&mut self, path: &Path,options_search: &Vec<String>,
		stop: Arc<Mutex<bool>>, data: std::sync::mpsc::Sender<Vec<String>>)
		{
		let mut only_photo = false;
		let mut options_searchs: Vec<String>= Vec::new();
		let mut i:usize = 0;
		 while i < options_search.len(){
		 	if options_search.get(i).unwrap().contains("only_photo"){
				only_photo = true;
			}
		 	else {
		 		options_searchs.push(options_search.get(i).unwrap().to_string());
		 	}
		 	i+=1
		};
		if let Ok(entries) = read_dir(path) {
			for entry in entries.into_iter() {
				let gard = match stop.lock() {
        			Ok(value) => *value,
		        	Err(poisoned) => *poisoned.into_inner(),
		        };
    			if gard{
		        	let _ = data.send(vec!["stop".to_string()]).unwrap();
					println!("Arret en cours {gard}");
					break;
		        }
				if let Ok(entry) = entry {
	      			let path = entry.path();
	      			if let Ok(metadata) = entry.metadata() {
	            		if metadata.is_dir(){
	            			let pp = path.display().to_string();
	            			let name: Vec<&str> = pp.split('/').collect();
	            			let re = Regex::new(r"^(\.[\w\d\s-]*)|(lost\+found){1}").unwrap();
	            			if re.is_match(name.last().unwrap())==false {
	            				self.parcours_rep(&path,options_search,stop.clone(),data.clone());
	            			}
	            		}
	            		else {
	            			let path = Path::new(&path);
	            			let extension: &str = match path.extension(){
	            				Some(ext) =>  ext.to_str().unwrap(),
	            				None => "none",
	            			};
	            			for option in options_searchs.iter(){
	            				// thread::sleep(Duration::from_millis(50));
	            				if extension.contains(option.as_str()){
	            					match File::new(path.display().to_string(),
	            					 extension.to_string(), metadata.len(),
	            					  self.nb_file){
	            						Ok(mut f) => {
		            						if f.is_gps(){
		            							let mut localisation = geo::get_location(geo::convert_gps(f.latitude()),geo::convert_gps(f.longitude()));
		            							if localisation.contains("\""){
		            								localisation = localisation.replace("\"", "");
		            							}
		            							f.set_localisation(localisation.clone());
		            						}
											let re = Regex::new(r"jpg|tiff").unwrap();
		            						if only_photo && re.is_match(extension) {
		            							if f.is_photo(){
		            								self.files.push(f);
													self.nb_file += 1;
		            								self.size += metadata.len();
		            							}
		            						}
		            						else {
		            							self.files.push(f);
		            							self.nb_file += 1;
		            							self.size += metadata.len();
		            						}
		            						let size = App::human_read(self.size);
		            						data.send(vec![self.nb_file().to_string(),size]).unwrap();
		            					}
	            						Err(_e) => (),
	            					};
	            				}
	            			}	            			
	            		}
	                }
	            }
			}
		}
	}
	/**
	 Fonction qui copi un fichier vers son emplacement définitif
	 * path_dest : chemin de déstination 
	 * option : tableau de bool contenant les option de copie
	 ```
			* key 0 Année
			* key 1 Mois
			* key 2 Jour
			* key 3 Localisation
			* key 4 Mois écrit en lettre
			* key 5 Effacer le fichier en cas de succées de la copie
	 ```
	 * 
	 # -> Retourne une tulpe (bool,file) qui contient l'état de la copie, et la struct file passé en paramétre
	 * */
	pub fn copy(&mut self, path_dest: String, option: &Vec<bool>,mut file: File) -> (bool,File){
		use crate::directory::create_dir;

		self.set_current(file.id_number().try_into().unwrap());
		let path:String= match file.is_photo(){
            true => file.create_path_dir(path_dest.clone(),option),
            false => file.create_path_dir(path_dest.clone()+"/not_photo",option),
        };
        
        match create_dir(path.to_string()){
            Ok(..) =>{
                match file.copy(path,option[5]){
                    Ok(..) => {
                    	file.set_is_copy(true);
                    	let a = file.id_number() as usize;
                    	self.set_file_at(a, file.clone());
                    	if option[5]{
                    		// let tmp_path = file.clone().path();
                    		self.delete_all_dir_without_file(file.clone().path());
                    	}
                    },
                    Err(_e) => return (false,file),  
                }
            }
            Err(_e) => return (false,file), 
        }
		(true,file)
	}
	/**
	 Supprime les chemins de répertoires vident
	 * 
	 * path: chemin vers le répertoire à analyser
	 * */
	fn delete_all_dir_without_file(&self, path: String){
		let mut split_path: Vec<&str> = path.split("/").collect();
		split_path.pop();
		let mut tmp_split = split_path.clone();
		for _ in split_path{
			let path_tmp = tmp_split.join("/");
			let tmp_parent = Path::new(path_tmp.as_str());
			tmp_split.pop();
			if self.count_entry_for_rep(tmp_parent) == 0{
				let _ = std::fs::remove_dir(tmp_parent);
			}
			else 
			{
				break;
			}
			if path_tmp.eq(&self.path_src){
				break;
			}
		}
	}
	/**
	 Compte si le nombre de fichiers présent dans le répertoire est superieur à zero
	 * 
	 * path : chemin vers le répertoire à analyser
	 # -> retoune 0 ou 1
	 * */
	/// 
	fn count_entry_for_rep(&self,path: &Path)-> i32 {
		let mut nb_entry=0;
		if let Ok(entries) = read_dir(path.to_owned()){
			for _entry in entries.into_iter(){
				(nb_entry+=1);
				break;
			}
		}
		nb_entry
	}
	/// Reset self.files
	fn reset_files(&mut self) { self.files.clear(); }
	/// Fonction qui RAZ la structure
	pub fn clear(&mut self){
		self.set_nb_file(0);
		self.reset_files();
		self.set_size(0);
		self.set_status_init(false);	
	}

	// setter
	pub fn set_path_dest(&mut self,i: String){ self.path_dest = i;}
	pub fn set_current(&mut self,i: u64){ self.current = i;}
	fn set_nb_file(&mut self,i: u64){ self.nb_file = i;}
	fn set_size(&mut self,i: u64) { self.size = i; }
	fn set_status_init(&mut self, i: bool) { self.status_init = i; }
	pub fn set_files(&mut self,files: Vec<File>) { self.files =  files;	}
	pub fn set_file_at(&mut self, index:usize, file: File) { self.files[index]=file; }

	// getter
	pub fn path_dest(&self) -> String { self.path_dest.clone() }
	pub fn current(&self) -> u64 { self.current }
	pub fn nb_file(&self) -> u64 { self.nb_file }
	pub fn size(&self) -> u64 { self.size	}
	pub fn files(&self) -> Vec<File> { self.files.clone() }
	pub fn status_init(&self) -> bool { self.status_init}
	pub fn get(&self) -> Self { self.clone() }
	pub fn file_at(&self, index: usize) -> File { self.files.get(index).unwrap().clone() }

	// Serialisation
	pub fn serialize(&self)-> String{
		let mut serialize: Vec<String>= Vec::new();
		serialize.push(format!("path_dest;{}",self.path_dest));
		serialize.push(format!("current;{}", self.current().to_string()));
		serialize.push(format!("nb_file;{}", self.nb_file().to_string()));
		serialize.push(format!("size;{}", self.size.to_string()));
		serialize.push(format!("status_init;{}", self.status_init.to_string()));
		for file in self.files() {
			serialize.push(String::from("[file]"));
			serialize.push(format!("id_number;{}",file.id_number()));
			serialize.push(format!("name;{}",file.name()));
			serialize.push(format!("path;{}",file.path()));
			serialize.push(format!("size;{}",file.size()));
			serialize.push(format!("extension;{}",file.extension()));
			serialize.push(format!("date;{}-{}-{}",file.date().0,file.date().1,file.date().2));
			serialize.push(format!("exposure_mode;{}",file.exposure_mode()));
			serialize.push(format!("latitude;{}-{}-{}-{}",file.latitude().0,file.latitude().1,file.latitude().2,file.latitude().3));
			serialize.push(format!("longitude;{}-{}-{}-{}",file.longitude().0,file.longitude().1,file.longitude().2,file.longitude().3));
			serialize.push(format!("localisation;{}", file.localisation()));
			serialize.push(format!("is_copy;{}",file.is_copy()));
			serialize.push(format!("is_photo;{}",file.is_photo()));
			serialize.push(format!("[end_file]"));
		}
		serialize.join("\n").to_string()
	}
	pub fn unserialize(path: String) -> std::io::Result<GestFiles>{
		let mut gest_files = Self::default();
		let mut files: Vec<File> = Vec::new();
		let mut is_file = false;
		let mut f= File::default();

		let file = std::fs::File::open(path)?;
		let bufreader = BufReader::new(file);
		for line in bufreader.lines() {
	        if let Ok(value) = line {
	        	let v:Vec<_>= value.split(';').collect();
	            match v[0]{
	            	"path_dest" => gest_files.set_path_dest(v[1].to_string()),
	            	"current" => gest_files.set_current(v[1].parse::<u64>().unwrap()),
	            	"nb_file" => gest_files.set_nb_file(v[1].parse::<u64>().unwrap()),
	            	"size" => {
	            		// eprintln!("{:?}",v[1]);
	            		if is_file {
	            			f.set_size(v[1].parse::<u64>().unwrap());
	            		}
	            		else{ gest_files.set_size(v[1].parse::<u64>().unwrap()); }
	            	},
	            	"status_init" => gest_files.set_status_init(v[1].parse::<bool>().unwrap()),
	            	"[file]" => {
	            		is_file = true;
	            		f= File::default();
	            	},
	            	"id_number" => {
	            		if is_file {
	            			f.set_id_numder(v[1].parse::<u64>().unwrap());
	            		}
	            	},
					"name" =>{
						 if is_file{
	            			f.set_name(v[1].to_string());
	            		}
	            	}
					"path" => {
						if is_file {
	            			f.set_path(v[1].to_string());
	            		}
	            	},
					"extension" => {
						if is_file{
	            			f.set_extension(v[1].to_string());
	            		}
					}
					"exposure_mode" => {
						if is_file{
	            			f.set_exposure_mode(v[1].to_string());
	            		}
					}
					"localisation" => {
						if is_file{
	            			f.set_localisation(v[1].to_string());
	            		}
					}
					"is_copy" => {
						if is_file{
	            			f.set_is_copy(v[1].parse::<bool>().unwrap());
	            		}
					}
					"is_photo" => {
						if is_file{
	            			f.set_is_photo(v[1].parse::<bool>().unwrap());
	            		}
					}	
	            	"date" => {
						if is_file{
							let date:Vec<_>= v[1].split('-').collect();
	            			f.set_date(date[0].parse::<u16>().unwrap(), date[1].parse::<u8>().unwrap(), date[2].parse::<u8>().unwrap());
	            		}
					}
					"latitude" => {
						if is_file{
							let loc:Vec<_>= v[1].split('-').collect();
	            			f.set_latitude(loc[0].parse::<f64>().unwrap(), loc[1].parse::<f64>().unwrap(), loc[2].parse::<f64>().unwrap(),
	            			loc[3].to_string()) ;
	            		}
					}
					"longitude" => {
						if is_file{
							let loc:Vec<_>= v[1].split('-').collect();
	            			f.set_longitude(loc[0].parse::<f64>().unwrap(), loc[1].parse::<f64>().unwrap(), loc[2].parse::<f64>().unwrap(),
	            			loc[3].to_string()) ;
	            		}
					}
					"[end_file]" => {
	            		if is_file {
	            			files.push(f.clone());
	            		}
	            	},
	            	_ => (),
	            }
	        }
	    }
	    gest_files.set_files(files);
	    Ok(gest_files)
	}
}
unsafe impl Send for GestFiles {}
unsafe impl Sync for GestFiles {}
