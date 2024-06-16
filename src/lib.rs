//!	# tri_photo
//!
//!	Cette application permet de trier des fichiers photos et de les hiérachiser dans une arborescence
//! du type Années-mois-jours-localisation. Il peut copier ou déplacer les fichiers trouvés. Il est
//! possible de renomer chaque fichier un à un.
//!
//! Si il y a trop de fichier, une option de sauvegarde du travail en cours est disponible permettant
//! de reprendre plus tard
//!
//! Le programme est excecuté via une interface graphique.
//!
//!	# Information des modules
//!
//! * [app] gére les paramêtres de l'application passé via la ligne de commande, 
//!		    les options de recherche, de tri, et de post traitement.
//!	* [gui] gére l'interface graphique du programme
//!		* [header_bar] gestion de la header bar du programme
//!		* [popup_accueil] gestion du démmarage de l'interface graphique
//!		* [working] interface de suivi de copi
//!		* [rename] interface de gestion du renomage de fichier
//!		* [gif_picture] gére l'affichage des image de type gif (l'inteface graphique en utilise)
//!	* [file] représente les données du fichier (nom, date de création, etc)
//!	* [gest_files] gestion des intéraction avec les fichiers
//!	* [geo] gére la localisation d'une photo lorsque c'est possible
//!	* [directory] gére les fonctions sur répertoire

pub mod app;
pub mod file;
pub mod directory;
pub mod gest_files;
pub mod geo;
pub mod gui;
pub mod popup_accueil;
pub mod gresources;
pub mod working;
pub mod file_choser;
pub mod header_bar;
pub mod text;