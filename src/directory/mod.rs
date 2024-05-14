use std::fs::DirBuilder;
use std::path::Path;

/**
 Fonction vérifiant que le chemin `p` correspond à un répertoire
 # -> retourne `True` si le chemin est un répertoire sinon false
 * */
pub fn is_dir(p: &Path) -> bool {
	    if !p.to_str().unwrap().is_empty(){
	        if p.is_dir() {
	           return  true;
	        }
	    }
	    false
}
/**
 Fonction créant le repertoire depuis `path`
 * path : chemin du répertoire à créer
 * 
 # -> retourne Ok(true) si la création à reussi sinon Err(e)
 * */
pub fn create_dir(path: String) -> Result<bool, String> {
	match DirBuilder::new()
            .recursive(true)
            .create(&path){
            	Ok(())=>return Ok(true),
            	Err(e) => {
            		return Err("Créate_dir".to_owned()+&e.to_string());
            	}
	}
}
