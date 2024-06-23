mod imp;
use gtk::{
        glib::{self, clone}, prelude::FrameExt, subclass::prelude::ObjectSubclassIsExt, traits::{ButtonExt, GtkWindowExt}, Window
    };
use crate::{app::{HEIGHT, WIDTH}, header_bar, text::Text};

glib::wrapper! {
    pub struct HelpGui(ObjectSubclass<imp::HelpGui>) 
        @extends gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}
impl HelpGui{
    /// Fonction créant une nouvelle fenetre d'aide
    pub fn new(_title: &str,parent: &Window) -> Self {
        let mut hg: HelpGui = glib::Object::builder()
            .property("default-width", WIDTH-20)
            .property("default-height",HEIGHT-60)
            .property("transient-for", &parent)
            .property("hide-on-close", true)
            .build();
        hg.init_text();
        let this = hg.clone();
        hg.set_titlebar(Some(&header_bar::new(&Text::HelpGuiTitle.as_string(), move || {this.close()})));
        hg
    }
    fn init_text(&mut self){
        let imp = self.imp();
        imp.welcome.set_label(&Text::HelpGuiWelcome.as_string());
        imp.working.set_label(Some(&Text::GuiWorkDir.as_string()));
        imp.working_description.set_label(&Text::HelpGuiWorkingDescrition.as_string());
        imp.working_source.set_label(&(Text::HelpGuiWorkingSource.as_string()+":"));
        imp.working_source_description.set_label(&Text::HelpGuiWorkingSourceDescription.as_string());
        imp.working_destination.set_label(&(Text::HelpGuiWorkingDestination.as_string()+":"));
        imp.working_destination_description.set_label(&Text::HelpGuiWorkingDestinationDescription.as_string());
        imp.working_file_found.set_label(&Text::GuiFileFound.as_string());
        imp.working_file_found_description.set_label(&Text::HelpGuiWorkingFileFoundDescription.as_string());
        imp.working_file_found_description2.set_label(&Text::HelpGuiWorkingFileFoundDescription2.as_string());
        imp.working_file_found_description3.set_label(&Text::HelpGuiWorkingFileFoundDescription3.as_string());
        imp.working_info.set_label(&(Text::HelpGuiWorkingInfo.as_string()+":"));
        imp.working_info_description.set_label(&Text::HelpGuiWorkingInfoDescription.as_string());
        imp.file_at_found.set_label(Some(&Text::HelpGuiFileAtFound.as_string()));
        imp.file_at_found_description.set_label(&Text::HelpGuiFileAtFoundDescription.as_string());
        imp.arbos.set_label(Some(&Text::GuiSortingTree.as_string()));
        imp.arbo_description.set_label(&Text::HelpGuiArboDescription.as_string());
        imp.arbo_annee.set_label(&(Text::GuiYear.as_string()+":"));
        imp.arbo_annee_description.set_label(&Text::HelpGuiArboYearDescription.as_string());
        imp.arbo_mois.set_label(&(Text::GuiMonth.as_string()+":"));
        imp.arbo_mois_description.set_label(&Text::HelpGuiArboMonthDescription.as_string());
        imp.arbo_jours.set_label(&(Text::GuiDay.as_string()+":"));
        imp.arbo_jours_description.set_label(&Text::HelpGuiArboDayDescription.as_string());
        imp.arbo_lieu.set_label(&(Text::GuiLieu.as_string()+":"));
        imp.arbo_lieu_description.set_label(&Text::HelpGuiArboLieuDescription.as_string());
        imp.arbo_letter.set_label(&(Text::GuiLetter.as_string()+":"));
        imp.arbo_letter_description.set_label(&Text::HelpGuiArboLetterDescription.as_string());
        imp.traitement.set_label(Some(&Text::GuiTraitement.as_string()));
        imp.traitement_description.set_label(&Text::HelpGuiTraitementDescription.as_string());
        imp.traitement_renomer.set_label(&(Text::GuiRename.as_string()+":"));
        imp.traitement_renomer_description.set_label(&Text::HelpGuiTraitementRenomerDescription.as_string());
        imp.traitement_delete.set_label(&(Text::GuiDelete.as_string()+":"));
        imp.traitement_delete_description.set_label(&Text::HelpGuiTraitementDeleteDescription.as_string());
        imp.traitement_gestion.set_label(&(Text::GuiChooseOnDemand.as_string()+":"));
        imp.traitement_gestion_description.set_label(&Text::HelpGuiTraitementGestionDescription.as_string());
        imp.traitement_gestion_description2.set_label(&Text::HelpGuiTraitementGestionDescription2.as_string());
    }
    /// function de la gestion du click sur ok
    fn init_ok(&self){
        self.imp().on_help_gui.connect_clicked(clone!(@weak self as this=>move |_| {
            this.close();
        }));
    }
}