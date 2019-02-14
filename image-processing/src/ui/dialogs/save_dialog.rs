use gtk::*;
use std::path::PathBuf;

pub struct SaveDialog{
  pub save_dialog: FileChooserDialog
}

impl SaveDialog {
    pub fn new(path: Option<PathBuf>) -> Self {
        let save_dialog = FileChooserDialog::new(
                                                 Some("Save"),
                                                 Some(&Window::new(WindowType::Popup)),
                                                 FileChooserAction::Save,
                                                 );

        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

        path.map(|p| save_dialog.set_current_folder(p));

        Self {
          save_dialog: save_dialog
        }
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.save_dialog.run() == ResponseType::Ok.into() {
            self.save_dialog.get_filename()
        } else {
            None
        }
    }
}

impl Drop for SaveDialog {
    fn drop(&mut self) { self.save_dialog.destroy(); }
}