use chrono::{Datelike,Utc};
use std::{io::{Error,ErrorKind},path::Path};
use exif::Field;


use crate::text::Text;

/**
 Structure contenant les données d'un fichier image tel que le nom, la date, la taille, l'extention, son identifiant
 * la donnée exposure mode permet avec une date valide de verifier si une image est une photo ou pas
 * les données de localisation ne sont pas forcément présente
 * la données is_copy permet de savoir si oui ou non la photo présente a été copiée
 * la donnée is_photo est réglé à true si c'est une photo sinon false
 * */
#[derive(Default,Debug,PartialEq,Clone)]
pub struct File {
	id_number: u64,
	name: String,
	path: String,
	extension: String,
	size: u64,
	date:(u16,u8,u8),
	exposure_mode: String,
	pub latitude:(f64,f64,f64,String),
	pub longitude:(f64,f64,f64,String),
	localisation:String,
	is_copy:bool,
	is_photo:bool,
}

impl File {
	/// valeur par default de la structure File
	pub fn default() -> Self{
		File {
			id_number: 0,
			name: String::new(),
			path: String::new(),
			extension: String::new(),
			size: 0,
			date: ( 0,0,0),
			exposure_mode: String::new(),
			latitude:(0.0,0.0,0.0,String::new()),
			longitude:(0.0,0.0,0.0,String::new()),
			localisation:"None".to_string(),
			is_copy: false,
			is_photo:false,
		}
	}
	/**
	 Fonction créant un nouvel objet File 
	 * 
	 * */
	pub fn new(path: String, extension: String, size: u64, id_number: u64) -> Result <File, String>{
		let f= match std::fs::File::open(path.clone()){
        	Ok(fi) => fi,
        	Err(..) => return Err(Text::FileErrOpen.as_string()),
    	};	

    	let name_array: Vec<&str> =  path.split('/').collect();
    	let name: String = name_array.last().unwrap().to_string();
    	
    	let mut file:File =	File {
			// id: String::new(),
			id_number,
			name,
			path: path.clone(),
			extension,
			size,
			date: ( 0, 0, 0),
			exposure_mode: String::new(),
			latitude:(0.0,0.0,0.0,String::new()),
			longitude:(0.0,0.0,0.0,String::new()),
			localisation:"None".to_string(),
			is_copy: false,
			is_photo:false,
		};

    	match Path::new(&path).extension().unwrap().to_str().unwrap(){
    		"jpg" | "tiff" => {
    			match file.open_jpg_tiff(f){
    				Ok(..)=> (),
    				Err(e) => return Err(e)
    			}
    		},
    		_ => {
    			match file.open_other(path){
    				Ok(..) =>(),
    				Err(e) => return Err(format!("{}",e)),
    			};
			},
		}
		Ok(file)
	}
	/**
	 Fonction de traitement des fichiers jpg et tiff utilise exif
	 * file (std::fs::File) le fichier ouvert
	 * 
	 # -> Retourne une erreur en cas d'échec
	 * */
	fn open_jpg_tiff(&mut self, file: std::fs::File) -> Result<(), String > {
		let mut bufreader = std::io::BufReader::new(&file);
		let exif = match exif::Reader::new().read_from_container(&mut bufreader){
			Ok(ex) => ex,
			Err(_e) => {
				let image_desc = Field {
					tag: exif::Tag::ImageDescription,
					ifd_num: exif::In::PRIMARY,
					value: exif::Value::Ascii(vec![b"Sample".to_vec()]),
				};
				let mut writer = exif::experimental::Writer::new();
				let mut buf = std::io::Cursor::new(Vec::new());
				writer.push_field(&image_desc);
				match writer.write(&mut buf, false){
					Ok(()) =>{
						match exif::Reader::new().read_raw(buf.get_ref().to_vec()){
							Ok(ex) => ex,
							Err(..) => {
								return Err(Text::FileErrNoExif.as_string())
							}
						}
					},
					Err(..) => return Err(Text::FileErrNoWriteExif.as_string()),
				}
			}
		};
		for f in exif.fields() {
			match f.tag{
				exif::Tag::DateTimeOriginal => {
					let string_time:String =  f.display_value().with_unit(&exif).to_string().replace("-", ":");
					let bytes = string_time.as_str().as_bytes();
					let dt = match exif::DateTime::from_ascii(bytes){
						Ok(byt)=> byt,
						Err(_) => exif::DateTime::from_ascii(b"1970:01:01 00:00:00").unwrap()
					};
					self.date = ( dt.year,dt.month,dt.day);
				}
				exif::Tag::ExposureMode => {

					self.exposure_mode=f.display_value().with_unit(&exif).to_string();
				}
				exif::Tag::GPSLatitude => {
					let result = f.display_value().with_unit(&exif).to_string();
					if !result.is_empty(){
						let data:Vec<&str> = result.split(" ").collect();
						self.latitude.0 = data[0].parse::<f64>().unwrap();
						self.latitude.1 = data[2].parse::<f64>().unwrap();
						self.latitude.2 = data[4].parse::<f64>().unwrap();
					}
				}
				exif::Tag::GPSLatitudeRef => self.latitude.3 = f.display_value().with_unit(&exif).to_string(),
				exif::Tag::GPSLongitude => {
					let result = f.display_value().with_unit(&exif).to_string();
					if !result.is_empty(){
						let data:Vec<&str> = result.split(" ").collect();
						self.longitude.0 = data[0].parse::<f64>().unwrap();
						self.longitude.1 = data[2].parse::<f64>().unwrap();
						self.longitude.2 = data[4].parse::<f64>().unwrap();
					}
				}
				exif::Tag::GPSLongitudeRef => self.longitude.3 = f.display_value().with_unit(&exif).to_string(),
				_ => (),
			}
		}
		if !self.exposure_mode.is_empty() && self.date.0 != 1970 {
			self.is_photo = true;
		}
		Ok(())
	}
	/**
	 Fonction de traitement des autres fichiers images
	 * file (std::fs::File) le fichier ouvert
	 * 
	 # -> Retourne une erreur en cas d'échec
	 * */
	fn open_other(&mut self,path: String) -> std::io::Result<()>{
		let m = std::fs::metadata(path)?;
		if let Ok(time) = m.created(){
			let dt: chrono::DateTime<Utc> = time.clone().into();
			self.date = ( dt.year() as u16,dt.month() as u8,dt.day() as u8);
			Ok(())
		}
		else {
			Err(Error::from(ErrorKind::Unsupported))
		}
	}
	/**
	 Fonction verifiant la presence du nom du lieu correspondant aux données gps
	 * 
	 # -> Retourne true si présent, sinon false
	 * */
	pub fn is_gps(&self) -> bool{
		if !self.latitude.3.is_empty() && !self.longitude.3.is_empty(){
			if self.latitude.3.len() != 1 || self.longitude.3.len() != 1{
				return false
			}
			return true;
		}
		false
	}
	/**
	 Fonction indiquant le chemin final de destination en fonction des options choisies 
	 * destination (String): répertoire de destination choisie
	 * option (vec(bool)): options choisies dans la fenetre GuiWindow
	 ```
	 	* key 0 année
	 	* key 1 mois
	 	* key 2 jour
	 	* key 3 localisation
	 	* key 4 mois en lettre
	 ```
	 # -> Retourne dans un string le chemin final ou sera copié le fichier
	 * */
	pub fn create_path_dir<'a>(&self,destination: String,option:&Vec<bool>) -> String {
		// 0 => a, 1=> mois, 2=> jours, 3 => localisation, 4=> mois en nom
		let opt:Vec<bool> = option.to_vec();
		let nb_option_true = opt.iter().filter(|o| **o == true).count();
		let path: String = match nb_option_true{
			0 => "/".to_owned() +&self.date.0.to_string() + "/"+ &self.date.1.to_string() + "/"+&self.date.2.to_string(),
			_ => {
				let mut p=String::new();
				
				if self.date.0 == 0{
					println!("pas de date");
				}
				else {
					if option[0]{
						p += &("/".to_owned() + &self.date.0.to_string());
					}
					if option[1]{
						if !option[4]{
							p += &("/".to_owned() + &self.date.1.to_string());
						}
						else{
							p += "/";
							let mois = match self.date.1{
								1=>Text::FileJanvier.as_string(),
								2=>Text::FileFevrier.as_string(),
								3=>Text::FileMars.as_string(),
								4=>Text::FileAvril.as_string(),
								5=>Text::FileMai.as_string(),
								6=>Text::FileJuin.as_string(),
								7=>Text::FileJuillet.as_string(),
								8=>Text::FileAout.as_string(),
								9=>Text::FileSeptembre.as_string(),
								10=>Text::FileOctobre.as_string(),
								11=>Text::FileNovembre.as_string(),
								12=>Text::FileDecembre.as_string(),
								_ =>"".to_owned(),
							};
							p += &mois;
						}
					}
					if option[2]{
						p += &("/".to_owned() + &self.date.2.to_string());
					}
				}
				if option[3]{
					if !self.localisation.contains("None"){
						p += &("/".to_owned() + &self.localisation);
					}
					else {
						p += &("/".to_owned());
					}
				}
				p
			}
		};

		let  chemin: String = destination + &path;
		chemin
	}
	/**
	 Fonction de traitement de la copie du fichier,si delete est vrai, efface le fichier si la copie est réussi
	 * path_dest (String) le chemin absolue de destination de la copie
	 * delete (bool) si vrai efface le fichier source
	 * 
	 # -> Retourne une erreur en cas d'échec
	 * */
	pub fn copy(&self,path_dest:String,option: bool) -> Result<bool,String>{
		return match std::fs::copy(self.path.to_string(), path_dest.to_string()+"/"+&self.id_number.to_string()+"_"+&self.name){
			Ok(..) => {
				if option{
					match std::fs::remove_file(self.path.to_string()){
						Ok(..) => (),
						Err(_e) => return Err(Text::FileErrDeleteSource.as_string()),
					};

				}
				return Ok(true)
			}
			Err(..) => {
				Err(Text::FileErrCopy.as_string())
			}
		}
	}

	// setter
	pub fn set_id_numder(&mut self, i: u64) { self.id_number = i; }
	pub fn set_name(&mut self, i: String) { self.name = i; }
	pub fn set_path(&mut self, i: String) { self.path = i; }
	pub fn set_extension(&mut self, i: String) { self.extension = i; }
	pub fn set_size(&mut self, i: u64) { self.size = i; }
	pub fn set_date(&mut self, i: u16, j: u8, k: u8) { self.date = ( i, j, k); }	
	pub fn set_exposure_mode(&mut self, i: String) { self.exposure_mode = i; }	
	pub fn set_latitude(&mut self, i: f64, j: f64, k:f64, l:String) { self.latitude = (i,j,k,l); }
	pub fn set_longitude(&mut self, i: f64, j: f64, k:f64, l:String) { self.longitude = (i,j,k,l); }
	pub fn set_localisation(&mut self, i: String) { self.localisation = i; }
	pub fn set_is_copy(&mut self, i: bool) { self.is_copy = i; }
	pub fn set_is_photo(&mut self, i: bool) { self.is_photo = i; }
	
	// getter
	pub fn name(&self) -> String { self.name.to_string() }
	pub fn id_number(&self) -> u64 {self.id_number}
	pub fn date(&self) -> (u16,u8,u8) { self.date.clone() }
	pub fn size(&self) -> u64 {	self.size }
	pub fn path(&self) -> String { self.path.to_string() }
	pub fn extension(&self) -> String { self.extension.to_string() }
	pub fn latitude(&self) -> (f64,f64,f64,String) { self.latitude.clone() }
	pub fn longitude(&self) -> (f64,f64,f64,String) { self.longitude.clone() }
	pub fn localisation(&self) -> String { self.localisation.clone() }
	pub fn exposure_mode(&self) -> String { self.exposure_mode.clone() }
	pub fn is_copy(&self) -> bool { self.is_copy }
	pub fn is_photo(&self) -> bool { self.is_photo }
}
