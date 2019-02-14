use gtk::*;
use std::path::PathBuf;

pub struct OpenDialog{
  pub open_dialog: FileChooserDialog
}

impl OpenDialog {
    pub fn new(path: Option<PathBuf>) -> Self {
        let open_dialog = FileChooserDialog::new(
                                                 Some("Open"),
                                                 Some(&Window::new(WindowType::Popup)),
                                                 FileChooserAction::Open,
                                                 );

        open_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_dialog.add_button("Open", ResponseType::Ok.into());

        path.map(|p| open_dialog.set_current_folder(p));

        Self {
          open_dialog: open_dialog
        }
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.open_dialog.run() == ResponseType::Ok.into() {
            self.open_dialog.get_filename()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self) { self.open_dialog.destroy(); }
}