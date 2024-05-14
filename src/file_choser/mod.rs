use gtk::{prelude::{DialogExtManual, IsA, WidgetExt}, traits::FileChooserExt, FileChooserAction, FileChooserDialog, ResponseType, Window};

pub fn new_load(parent: Option <&impl IsA<Window>>) -> FileChooserDialog{
  let chooser = base(parent.unwrap(), "Charger le fichier", FileChooserAction::Open, 
    &[("Charger", ResponseType::Ok), ("Annuler", ResponseType::Cancel)]);
  let file_filter: gtk::FileFilter = gtk::FileFilter::new();
  file_filter.add_pattern("*.tps");
  chooser.set_filter(&file_filter);
  
  chooser
}
pub fn new_save(parent: Option <&impl IsA<Window>>) -> FileChooserDialog {
  let chooser = base(parent.unwrap(), "Sauvegarder la progression", FileChooserAction::Save,
   &[("Annuler",ResponseType::Cancel),("Sauvegarder",ResponseType::Ok)]);
  chooser.set_widget_name("dialog");
  chooser

}
pub fn new_select(parent: Option <&impl IsA<Window>>, title: &str,btn_val: &str,btn_cancel: &str) -> gtk::FileChooserDialog{
    let rep = base(parent.unwrap(), title, FileChooserAction::SelectFolder,
     &[(btn_cancel,ResponseType::Cancel),(btn_val,ResponseType::Ok)]);
    rep.set_widget_name("dialog");
    rep.set_create_folders(true);
    rep
}
fn base(parent :&impl IsA<Window>, title: &str, action: FileChooserAction, buttons: &[(&str, ResponseType)]) -> FileChooserDialog{
    let chooser = gtk::FileChooserDialog::builder()
    .use_header_bar(1)
    .modal(true)
    .action(action)
    .title(title)
    .transient_for(parent)
    .build();
    chooser.add_buttons(buttons);
    chooser
}