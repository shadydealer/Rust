use std::process;
use std::sync::{Arc, RwLock};

use gtk::*;
use super::{ Header, Content, ConnectedApp};
use super::dialogs::open_dialog::OpenDialog;

use image::Image;

pub struct App {
	pub window: Window,
	pub header: Header,
  pub content: Content
}

impl App {
	pub fn new() -> App {
    if gtk::init().is_err() {
      eprintln!("failed to initialize GTK Application");
      process::exit(1);
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_default_size(1280, 680);

    let header = Header::new();
    window.set_titlebar(&header.container);

    let header_title =
    match header.container.get_title() {
      Some(title) => title,
   			None => panic!("Failed to optain the header title"), // should never reach this case
      };

      window.set_wmclass(&header_title, &header_title);


      let content = Content::new();
      window.add(&content.container);

      window.connect_delete_event(move |_, _| {
       main_quit();
       Inhibit(false)
     });

      App { window, header, content }
    }

    pub fn connect_events(&self) -> ConnectedApp {

      let current_file = Arc::new(RwLock::new(None));

      {
        let save = self.header.save;
        let save_as = self.header.save_as;

      // Connect all of the events that this UI will act upon.
      self.open_file(current_file.clone());
      // self.save_event(&save, &save, current_file.clone(), false);
      // self.save_event(&save, &save_as, current_file.clone(), true);
    }

    ConnectedApp::new(*self)
  }

  fn open_file(&self, current_file: Arc<RwLock<Option<Image>>>) {
    self.header.open.connect_clicked(move |_| {
      // Create a new open file dialog using the current file's parent
      // directory as the preferred directory, if it's set.
      let open_dialog = OpenDialog::new({
        let lock = current_file.read().unwrap();
        if let Some(ref path) = *lock {
          path.get_dir()
        } else {
          None
        }
      });
    });
  }
}