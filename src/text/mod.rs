use tr::tr;

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
       match self{
            //commun
            Text::Load => tr!("Load"),
            Text::Next => tr!("Next"),
            Text::Quitter => tr!("Exit"),
            Text::Cancel => tr!("Cancel"),
            Text::Choice => tr!("Choose"),
            Text::Valid => tr!("Validate"),
            Text::Rename => tr!("Rename"),
            Text::Save => tr!("Save"),
            Text::Copy => tr!("Copy"),
            Text::DataWait => tr!("waiting for data"),
            //mod app
            Text::AppHomeTitle => tr!(" : Home"),
            Text::AppConfigTitle => tr!(" : Configuration"),
            Text::AppLoadTitle => tr!(" : Load a file"),
            //mod gui
            Text::GuiEntrySource => tr!("Where are your images located?"),
            Text::GuiEntryDestination => tr!("Where do you want to copy them?"),
            Text::GuiTooltipSource => tr!("Selecting the source directory"),
            Text::GuiTooltipDestination => tr!("Selecting the destination directory"),
            Text::GuiOnlyPhoto => tr!("Only the photographs"),
            Text::GuiDay => tr!("Day"),
            Text::GuiMonth => tr!("Month"),
            Text::GuiYear => tr!("Year"),
            Text::GuiLieu => tr!("Places"),
            Text::GuiDelete => tr!("Remove sources"),
            Text::GuiRename => tr!("Rename sources"),
            Text::GuiChooseOnDemand => tr!("Choose as you go"),
            Text::GuiLetter => tr!("Month in letter"),
            Text::GuiDemarrerMove => tr!("Move"),
            Text::GuiDemarrerMoveGestion => tr!("Move and manage"),
            Text::GuiDemarrerCopy => tr!("Copy"),
            Text::GuiDemarrerGestion => tr!("Manage"),
            Text::GuiFileFound => tr!("files found: "),
            Text::GuiFileAtFound => tr!("File to find"),
            Text::GuiWorkDir => tr!("Working directory"),
            Text::GuiView => tr!("Preview"),
            Text::GuiSortingTree => tr!("Sorting tree"),
            Text::GuiTraitement => tr!("Treatment"),
            Text::GuiDirValid => tr!("choose the file types to find."),
            Text::GuiFileTypeOk => tr!("Verified, please click the search icon."),
            Text::GuiFileTypeNone => tr!("Type of file to search: not chosen"),
            Text::GuiSelWait => tr!("awaiting selection"),
            Text::GuiErrorDest => tr!("waiting for copy directory"),
            Text::GuiErrorDestBis => tr!("waiting for destination directory"),
            Text::GuiErrorSel => tr!("the destination must be different from the target"),
            Text::GuiReadyCopy => tr!("choose your tree structure and your treatments."),
            Text::GuiScanningProgress => tr!("Scan in progress, please wait..."),
            Text::GuiChercher => tr!("Search for files in directory"),
            Text::GuiAboutShortDescription => tr!("Sorting program for photos"),
            // mod file
            Text::FileErrOpen => tr!("Error opening incorrect path"),
            Text::FileErrNoExif => tr!("still no data after creating new EXIF data"),
            Text::FileErrNoWriteExif => tr!("Unable to create exif data"),
            Text::FileErrDeleteSource => tr!("Unable to delete source files"),
            Text::FileErrCopy => tr!("Problem while copying"),
            Text::FileJanvier => tr!("January"),
            Text::FileFevrier => tr!("February"),
            Text::FileMars => tr!("March"),
            Text::FileAvril => tr!("April"),
            Text::FileMai => tr!("May"),
            Text::FileJuin => tr!("June"),
            Text::FileJuillet => tr!("July"),
            Text::FileAout => tr!("August"),
            Text::FileSeptembre => tr!("September"),
            Text::FileOctobre => tr!("October"),
            Text::FileNovembre => tr!("Nomenver"),
            Text::FileDecembre => tr!("December"),
            // mod file_choser
            Text::FileChoserSaveProgress => tr!("Save progress"),
            Text::FileChoserSave => tr!("Save"),
            // mod gest_file
            Text::GestFilesNotPhoto => tr!("not_photo"),
            // mod help_gui
            Text::HelpGuiTitle => tr!("help of TriPhoto"),
            Text::HelpGuiWelcome => tr!("Welcome to the program help"),
            Text::HelpGuiWorkingDescrition => tr!("This section allows you to choose the directories used by the program to work."),
            Text::HelpGuiWorkingSource => tr!("Source"),
            Text::HelpGuiWorkingSourceDescription => tr!("Directory containing the photos to copy."),
            Text::HelpGuiWorkingDestination => tr!("Destination"),
            Text::HelpGuiWorkingDestinationDescription => tr!("Directory where the photos will be copied."),
            Text::HelpGuiWorkingFileFoundDescription => tr!("Indicates the number of files found,"),
            Text::HelpGuiWorkingFileFoundDescription2 => tr!("the size of files to copy and"),
            Text::HelpGuiWorkingFileFoundDescription3 => tr!("the size of the destination directory"),
            Text::HelpGuiWorkingInfo => tr!("Information"),
            Text::HelpGuiWorkingInfoDescription => tr!("Indicates the status of the program."),
            Text::HelpGuiFileAtFound => tr!("File at found"),
            Text::HelpGuiFileAtFoundDescription => tr!("This section allows you to select the type of image file to search for."),
            Text::HelpGuiArboDescription => tr!("This section allows you to select the desired tree structure for sorting photos,
if the file is not a photo, a \"not_photo\" directory will be created."),
            Text::HelpGuiArboYearDescription => tr!("Creates a directory based on the year the photo was taken."),
            Text::HelpGuiArboMonthDescription => tr!("Creates a directory based on the month the photo was taken."),
            Text::HelpGuiArboDayDescription => tr!("Creates a directory based on the day the photo was taken."),
            Text::HelpGuiArboLieuDescription => tr!("Creates a directory based on the shooting location."),
            Text::HelpGuiArboLetterDescription => tr!("Creates the directories of the month in letter."),
            Text::HelpGuiTraitementDescription => tr!("This section allows you to select the actions to be carried out after the end of the sorting."),
            Text::HelpGuiTraitementRenomerDescription => tr!("Allows you to rename sorted files."),
            Text::HelpGuiTraitementDeleteDescription => tr!("Allows you to delete sources."),
            Text::HelpGuiTraitementGestionDescription => tr!("Allows you to select the action"),
            Text::HelpGuiTraitementGestionDescription2 => tr!("to perform in real time."),
            // mod rename
            Text::RenameNewName => tr!("New name"),
            Text::RenameKeep => tr!("Keep"),
            // mod popup_accueil
            Text::PopupAccueilHomeMessage => tr!("Welcome,

You will be able to sort your \"Photos\", according to their shooting dates and their geolocations (if available when shooting).

You can also choose to rename the photographs, as well as delete the contents of the source directory.
            "),
            // mod working
            Text::WorkingTitle => tr!("Copying, please wait..."),
            // Text::WorkingProgressStatusWait => todo!(),
            Text::WorkingDataNone => tr!("None"),
            Text::WorkingChemin => tr!("Path"),
            Text::WorkingDate => tr!("Date"),
            Text::WorkingLieu => tr!("Location"),
            Text::WorkingStatus => tr!("Status"),
            Text::WorkingSaveAndQuit => tr!("Save and Quit"),
            Text::WorkingQuitMessageTitle => tr!("Exit the program"),
            Text::WorkingQuitMessageMess => tr!("You are going to leave the program,
 the listing of files to copy/rename will be lost,
 I advise you to save to exit."),
            Text::WorkingInitWarningTitle => tr!("Critical Warning"),
            Text::WorkingInitWarningMess => tr!("You have chosen to delete
 old files this action is irreversible,
 Deleted files will not be recoverable."),
            Text::WorkingInitWarningBtn => tr!("I understand"),
            Text::WorkingChosseOnDemandTitle => tr!(" : Choice for files"),
            Text::WorkingChooseOnDemandMess => tr!("You have chosen to manage files as you go"),
            Text::WorkingFinishedCopyTitle => tr!(" : Copy finished"),
            Text::WorkingFinishedCopyMess => tr!("File copying is complete"),
            Text::WorkingRenameTitle => tr!("Rename file"),
            Text::WorkingSenderStatusInit => tr!("Initialization"),
            Text::WorkingChooseOnDemandDelete => tr!("Delete"),
            Text::WorkingChooseOnDemandStatus => tr!("Personalized management in progress..."),
            Text::WorkingChooseOnDemandDeleteStatus => tr!("Deletion in progress..."),
            Text::WorkingChooseOndemandRenameStatus => tr!("Renaming in progress..."),
            Text::WorkingChooseOnDemandCopyStatus => tr!("Copying in progress..."),
            Text::WorkingEndCopyStatus => tr!("Copied"),
            Text::WorkingEndChooseOnDemandStatus => tr!("Choice made"),
            Text::WorkingEndRenameStatus => tr!("Renamed"),
            Text::WorkingUnknow => tr!("Unknown"),
        }
    }
}
