use std::{fs::File, path::Path};
use gettext::Catalog;

pub enum Text {
    /// Text commun
    Load,
    /// texte pour gtkbouton pour suivant
    Next,
    /// texte de gtkbutton pour quitter
    Quitter,
    /// texte de gtkbutton pour annuler dans la filechooser
    Cancel,
    /// texte de gtkbutton pour choisir dans la filechooser
    Choice,
    Valid,
    Rename,
    Save,
    Copy,
    DataWait,

    ///mod app
    /// Titre de la fenetre popup_acceuil
    AppHomeTitle,
    /// texte de la fenetre gui
    AppConfigTitle,
    /// Titre de la fenetre new_load
    AppLoadTitle,

    ///mod file
    FileErrOpen,
    FileErrNoExif,
    FileErrNoWriteExif,
    FileErrDeleteSource,
    FileErrCopy,
    FileJanvier,
    FileFevrier,
    FileMars,
    FileAvril,
    FileMai,
    FileJuin,
    FileJuillet,
    FileAout,
    FileSeptembre,
    FileOctobre,
    FileNovembre,
    FileDecembre,
    ///mod file_choser
    FileChoserSaveProgress,
    FileChoserSave,

    ///mod gest_file
    GestFilesNotPhoto,
    
    ///mod GUI
    /// texte de gtkentry pour la séléction des sources dans la section répertoire de travail
    GuiEntrySource,
    /// texte de gtkentry pour la séléction de la destinations dans la section répertoire de travail
    GuiEntryDestination,
    /// texte de tooltip pour l'icone de rep_source
    GuiTooltipSource,
    /// texte de tooltip pour l'icone de rep_destination
    GuiTooltipDestination,
    /// texte de gtkcheckbutton pour only_photo dans la section fichier à trouver
    GuiOnlyPhoto,
    /// texte de gtkcheckbutton pour day dans la section arborescence du tri
    GuiDay,
    /// texte de gtkcheckbutton pour month dans la section arborescence du tri
    GuiMonth,
    /// texte de gtkcheckbutton pour year dans la section arborescence du tri
    GuiYear,
    /// texte de gtkcheckbutton pour lieu dans la section arborescence du tri
    GuiLieu,
    /// texte de gtkcheckbutton pour delete dans la section traitement
    GuiDelete,
    /// texte de gtkcheckbutton pour rename dans la section traitement
    GuiRename,
    /// texte de gtkcheckbutton pour choose_on_demand dans la section traitement
    GuiChooseOnDemand,
    /// texte de gtkcheckbutton pour letter dans la section arborescence du tri
    GuiLetter,
    /// texte de gtkbutton pour run
    GuiDemarrerMove,
    /// texte de gtkbutton pour run
    GuiDemarrerMoveGestion,
    /// texte de gtkbutton pour run
    GuiDemarrerCopy,
    /// texte de gtkbutton pour run
    GuiDemarrerGestion,
    /// texte de gtklabel pour les fichiers trouvés de la section répertoire de travail
    GuiFileFound,
    /// texte de gtkframe pour le titre de la section fichier à trouver
    GuiFileAtFound,
    /// texte de gtkframe pour le titre de la section répertoire de travail
    GuiWorkDir,
    /// texte de gtklabel pour aperçu dans la section arborescence du tri
    GuiView,
    /// texte de gtkframe pour le titre de la section arborescence du tri
    GuiSortingTree,
    /// texte de gtkframe pour le titre de la section traitement
    GuiTraitement,
    /// texte de gtklabel pour infos dans la section répertoire de travail (fausse bar d'état)
    GuiDirValid,
    /// texte de gtklabel pour infos dans la section répertoire de travail (fausse bar d'état)
    GuiFileTypeOk,
    /// texte de gtklabel pour infos dans la section répertoire de travail (fausse bar d'état)
    GuiFileTypeNone,
    /// texte de gtklabel pour infos dans la section répertoire de travail (fausse bar d'état)
    GuiSelWait,
    /// texte de gtklabel pour infos dans la section répertoire de travail (fausse bar d'état)
    GuiErrorDest,
    /// texte de gtklabel pour infos dans la section répertoire de travail (fausse bar d'état)
    GuiErrorDestBis,
    /// texte de gtklabel pour infos dans la section répertoire de travail (fausse bar d'état)
    GuiErrorSel,
    /// texte de gtklabel pour infos dans la section répertoire de travail (fausse bar d'état)
    GuiReadyCopy,
    /// texte de gtklabel pour infos dans la section répertoire de travail (fausse bar d'état)
    GuiScanningProgress,
    /// texte de gtkbutton pour chercher dans la section fichier à trouver
    GuiChercher,
    /// texte de la description du programme pour gtkabout
    GuiAboutShortDescription,

    /// mod gui_help
    HelpGuiTitle,
    HelpGuiWelcome,
    HelpGuiWorkingDescrition,
    HelpGuiWorkingSource,
    HelpGuiWorkingSourceDescription,
    HelpGuiWorkingDestination,
    HelpGuiWorkingDestinationDescription,
    HelpGuiWorkingFileFoundDescription,
    HelpGuiWorkingFileFoundDescription2,
    HelpGuiWorkingFileFoundDescription3,
    HelpGuiWorkingInfo,
    HelpGuiWorkingInfoDescription,
    HelpGuiFileAtFound,
    HelpGuiFileAtFoundDescription,
    HelpGuiArboDescription,
    HelpGuiArboYearDescription,
    HelpGuiArboMonthDescription,
    HelpGuiArboDayDescription,
    HelpGuiArboLieuDescription,
    HelpGuiArboLetterDescription,
    HelpGuiTraitementDescription,
    HelpGuiTraitementRenomerDescription,
    HelpGuiTraitementDeleteDescription,
    HelpGuiTraitementGestionDescription,
    HelpGuiTraitementGestionDescription2,

    /// mod rename
    RenameNewName,
    RenameKeep,
    /// mod popup_acceuil
    PopupAccueilHomeMessage,
    ///mod working
    WorkingTitle,
    WorkingDataNone,
    WorkingUnknow,
    WorkingChemin,
    WorkingDate,
    WorkingLieu,
    WorkingStatus,
    WorkingSaveAndQuit,
    WorkingQuitMessageTitle,
    WorkingQuitMessageMess,
    WorkingInitWarningTitle,
    WorkingInitWarningMess,
    WorkingInitWarningBtn,
    WorkingChosseOnDemandTitle,
    WorkingChooseOnDemandMess,
    WorkingFinishedCopyTitle,
    WorkingFinishedCopyMess,
    WorkingRenameTitle,
    WorkingSenderStatusInit,
    WorkingChooseOnDemandDelete,
    WorkingChooseOnDemandStatus,
    WorkingChooseOnDemandDeleteStatus,
    WorkingChooseOndemandRenameStatus,
    WorkingChooseOnDemandCopyStatus,
    WorkingEndCopyStatus,
    WorkingEndChooseOnDemandStatus,
    WorkingEndRenameStatus, 
}
impl Text{
    pub fn as_string(&self) -> String{
        let args = env!("LANG");
        let args= args[0..2].to_string();
        let binding = "locale/".to_owned()+&args+"/LC_MESSAGES/tp.mo";
        let path = Path::new(&binding);
        let catalog = match path.exists(){
            true => {
                let f:File = File::open("locale/".to_owned()+&args+"/LC_MESSAGES/tp.mo").expect("error file");
                Catalog::parse(f).expect("not parse this file") 
            },
            false => {
                Catalog::empty()
            }
        };
        let text = match self{
            //commun
            Text::Load => catalog.gettext("Load"),
            Text::Next => catalog.gettext("Next"),
            Text::Quitter => catalog.gettext("Exit"),
            Text::Cancel => catalog.gettext("Cancel"),
            Text::Choice => catalog.gettext("Choose"),
            Text::Valid => catalog.gettext("Validate"),
            Text::Rename => catalog.gettext("Rename"),
            Text::Save => catalog.gettext("Save"),
            Text::Copy => catalog.gettext("Copy"),
            Text::DataWait => catalog.gettext("waiting for data"),
            //mod app
            Text::AppHomeTitle => catalog.gettext(" : Home"),
            Text::AppConfigTitle => catalog.gettext(" : Configuration"),
            Text::AppLoadTitle => catalog.gettext(" : Load a file"),
            //mod gui
            Text::GuiEntrySource => catalog.gettext("Where are your images located?"),
            Text::GuiEntryDestination => catalog.gettext("Where do you want to copy them?"),
            Text::GuiTooltipSource => catalog.gettext("Selecting the source directory"),
            Text::GuiTooltipDestination => catalog.gettext("Selecting the destination directory"),
            Text::GuiOnlyPhoto => catalog.gettext("Only the photographs"),
            Text::GuiDay => catalog.gettext("Day"),
            Text::GuiMonth => catalog.gettext("Month"),
            Text::GuiYear => catalog.gettext("Year"),
            Text::GuiLieu => catalog.gettext("Places"),
            Text::GuiDelete => catalog.gettext("Remove sources"),
            Text::GuiRename => catalog.gettext("Rename sources"),
            Text::GuiChooseOnDemand => catalog.gettext("Choose as you go"),
            Text::GuiLetter => catalog.gettext("Month in letter"),
            Text::GuiDemarrerMove => catalog.gettext("Move"),
            Text::GuiDemarrerMoveGestion => catalog.gettext("Move and manage"),
            Text::GuiDemarrerCopy => catalog.gettext("Copy"),
            Text::GuiDemarrerGestion => catalog.gettext("Manage"),
            Text::GuiFileFound => catalog.gettext("files found: "),
            Text::GuiFileAtFound => catalog.gettext("File to find"),
            Text::GuiWorkDir => catalog.gettext("Working directory"),
            Text::GuiView => catalog.gettext("Preview"),
            Text::GuiSortingTree => catalog.gettext("Sorting tree"),
            Text::GuiTraitement => catalog.gettext("Treatment"),
            Text::GuiDirValid => catalog.gettext("choose the file types to find."),
            Text::GuiFileTypeOk => catalog.gettext("Verified, please click the search icon."),
            Text::GuiFileTypeNone => catalog.gettext("Type of file to search: not chosen"),
            Text::GuiSelWait => catalog.gettext("awaiting selection"),
            Text::GuiErrorDest => catalog.gettext("waiting for copy directory"),
            Text::GuiErrorDestBis => catalog.gettext("waiting for destination directory"),
            Text::GuiErrorSel => catalog.gettext("the destination must be different from the target"),
            Text::GuiReadyCopy => catalog.gettext("choose your tree structure and your treatments."),
            Text::GuiScanningProgress => catalog.gettext("Scan in progress, please wait..."),
            Text::GuiChercher => catalog.gettext("Search for files in directory"),
            Text::GuiAboutShortDescription => catalog.gettext("Sorting program for photos"),
            // mod file
            Text::FileErrOpen => catalog.gettext("Error opening incorrect path"),
            Text::FileErrNoExif => catalog.gettext("still no data after creating new EXIF data"),
            Text::FileErrNoWriteExif => catalog.gettext("Unable to create exif data"),
            Text::FileErrDeleteSource => catalog.gettext("Unable to delete source files"),
            Text::FileErrCopy => catalog.gettext("Problem while copying"),
            Text::FileJanvier => catalog.gettext("January"),
            Text::FileFevrier => catalog.gettext("February"),
            Text::FileMars => catalog.gettext("March"),
            Text::FileAvril => catalog.gettext("April"),
            Text::FileMai => catalog.gettext("May"),
            Text::FileJuin => catalog.gettext("June"),
            Text::FileJuillet => catalog.gettext("July"),
            Text::FileAout => catalog.gettext("August"),
            Text::FileSeptembre => catalog.gettext("September"),
            Text::FileOctobre => catalog.gettext("October"),
            Text::FileNovembre => catalog.gettext("Nomenver"),
            Text::FileDecembre => catalog.gettext("December"),
            // mod file_choser
            Text::FileChoserSaveProgress => catalog.gettext("Save progress"),
            Text::FileChoserSave => catalog.gettext("Save"),
            // mod gest_file
            Text::GestFilesNotPhoto => catalog.gettext("not_photo"),
            // mod help_gui
            Text::HelpGuiTitle => catalog.gettext("help of TriPhoto"),
            Text::HelpGuiWelcome => catalog.gettext("Welcome to the program help"),
            Text::HelpGuiWorkingDescrition => catalog.gettext("This section allows you to choose the directories used by the program to work."),
            Text::HelpGuiWorkingSource => catalog.gettext("Source"),
            Text::HelpGuiWorkingSourceDescription => catalog.gettext("Directory containing the photos to copy."),
            Text::HelpGuiWorkingDestination => catalog.gettext("Destination"),
            Text::HelpGuiWorkingDestinationDescription => catalog.gettext("Directory where the photos will be copied."),
            Text::HelpGuiWorkingFileFoundDescription => catalog.gettext("Indicates the number of files found,"),
            Text::HelpGuiWorkingFileFoundDescription2 => catalog.gettext("the size of files to copy and"),
            Text::HelpGuiWorkingFileFoundDescription3 => catalog.gettext("the size of the destination directory"),
            Text::HelpGuiWorkingInfo => catalog.gettext("Information"),
            Text::HelpGuiWorkingInfoDescription => catalog.gettext("Indicates the status of the program."),
            Text::HelpGuiFileAtFound => catalog.gettext("File at found"),
            Text::HelpGuiFileAtFoundDescription => catalog.gettext("This section allows you to select the type of image file to search for."),
            Text::HelpGuiArboDescription => catalog.gettext("This section allows you to select the desired tree structure for sorting photos,
if the file is not a photo, a \"not_photo\" directory will be created."),
            Text::HelpGuiArboYearDescription => catalog.gettext("Creates a directory based on the year the photo was taken."),
            Text::HelpGuiArboMonthDescription => catalog.gettext("Creates a directory based on the month the photo was taken."),
            Text::HelpGuiArboDayDescription => catalog.gettext("Creates a directory based on the day the photo was taken."),
            Text::HelpGuiArboLieuDescription => catalog.gettext("Creates a directory based on the shooting location."),
            Text::HelpGuiArboLetterDescription => catalog.gettext("Creates the directories of the month in letter."),
            Text::HelpGuiTraitementDescription => catalog.gettext("This section allows you to select the actions to be carried out after the end of the sorting."),
            Text::HelpGuiTraitementRenomerDescription => catalog.gettext("Allows you to rename sorted files."),
            Text::HelpGuiTraitementDeleteDescription => catalog.gettext("Allows you to delete sources."),
            Text::HelpGuiTraitementGestionDescription => catalog.gettext("Allows you to select the action"),
            Text::HelpGuiTraitementGestionDescription2 => catalog.gettext("to perform in real time."),
            // mod rename
            Text::RenameNewName => catalog.gettext("New name"),
            Text::RenameKeep => catalog.gettext("Keep"),
            // mod popup_accueil
            Text::PopupAccueilHomeMessage => catalog.gettext("Welcome,

You will be able to sort your \"Photos\", according to their shooting dates and their geolocations (if available when shooting).

You can also choose to rename the photographs, as well as delete the contents of the source directory.
            "),
            // mod working
            Text::WorkingTitle => catalog.gettext("Copying, please wait..."),
            // Text::WorkingProgressStatusWait => todo!(),
            Text::WorkingDataNone => catalog.gettext("None"),
            Text::WorkingChemin => catalog.gettext("Path"),
            Text::WorkingDate => catalog.gettext("Date"),
            Text::WorkingLieu => catalog.gettext("Location"),
            Text::WorkingStatus => catalog.gettext("Status"),
            Text::WorkingSaveAndQuit => catalog.gettext("Save and Quit"),
            Text::WorkingQuitMessageTitle => catalog.gettext("Exit the program"),
            Text::WorkingQuitMessageMess => catalog.gettext("You are going to leave the program,
 the listing of files to copy/rename will be lost,
 I advise you to save to exit."),
            Text::WorkingInitWarningTitle => catalog.gettext("Critical Warning"),
            Text::WorkingInitWarningMess => catalog.gettext("You have chosen to delete
 old files this action is irreversible,
 Deleted files will not be recoverable."),
            Text::WorkingInitWarningBtn => catalog.gettext("I understand"),
            Text::WorkingChosseOnDemandTitle => catalog.gettext(" : Choice for files"),
            Text::WorkingChooseOnDemandMess => catalog.gettext("You have chosen to manage files as you go"),
            Text::WorkingFinishedCopyTitle => catalog.gettext(" : Copy finished"),
            Text::WorkingFinishedCopyMess => catalog.gettext("File copying is complete"),
            Text::WorkingRenameTitle => catalog.gettext("Rename file"),
            Text::WorkingSenderStatusInit => catalog.gettext("Initialization"),
            Text::WorkingChooseOnDemandDelete => catalog.gettext("Delete"),
            Text::WorkingChooseOnDemandStatus => catalog.gettext("Personalized management in progress..."),
            Text::WorkingChooseOnDemandDeleteStatus => catalog.gettext("Deletion in progress..."),
            Text::WorkingChooseOndemandRenameStatus => catalog.gettext("Renaming in progress..."),
            Text::WorkingChooseOnDemandCopyStatus => catalog.gettext("Copying in progress..."),
            Text::WorkingEndCopyStatus => catalog.gettext("Copied"),
            Text::WorkingEndChooseOnDemandStatus => catalog.gettext("Choice made"),
            Text::WorkingEndRenameStatus => catalog.gettext("Renamed"),
            Text::WorkingUnknow => catalog.gettext("Unknown"),
        };
        text.to_owned()
    }
}