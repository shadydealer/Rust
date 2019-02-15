use std::process;
use std::sync::{Arc, RwLock};

use gtk::*;

use super::{ 
 Header,
 Content,
 ConnectedApp,
 open::open,
 save::save,
 equalize_histogram::equalize_histogram
};

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

    pub fn connect_events(self) -> ConnectedApp {

      let current_file = Arc::new(RwLock::new(None));

      {
        let save = &self.header.save;
        let save_as = &self.header.save_as;

      // Connect all of the events that this UI will act upon.
      self.open_file(current_file.clone());
      self.save_event(&save, current_file.clone(), false);
      self.save_event(&save_as, current_file.clone(), true);
      self.equalize_histogram(current_file.clone());
    }

    ConnectedApp::new(self)
  }

  fn equalize_histogram(&self, current_file: Arc<RwLock<Option<Image>>>) {

    let image_container = self.content.image_container.image_widget.clone();

    let equalize_histogram_button = &self.content.side_menu.equalize_histogram;
    equalize_histogram_button.connect_clicked(move |_| {
      match equalize_histogram(&image_container, &current_file) {
        Err(error) => println!("{:?}", error),
        Ok(()) => ()
      }
    });
  }

  fn open_file(&self, current_file: Arc<RwLock<Option<Image>>>) {

    let headerbar = self.header.container.clone();
    let image_container = self.content.image_container.image_widget.clone();

    self.header.open.connect_clicked(move |_| {
      match open(&headerbar, &image_container, &current_file) {
        Err(error) => println!("{:?}", error),
        Ok(()) => ()
      }
    });
  }

  fn save_event( &self,
                actual_button: &Button,
                current_file: Arc<RwLock<Option<Image>>>,
                save_as: bool,
                ) {
    let headerbar = self.header.container.clone();
    let actual_button = actual_button.clone();

    actual_button.connect_clicked( move |_| {
      match save(&headerbar, &current_file, save_as) {
        Err(error) => println!("{:?}", error),
        Ok(()) => ()
      }
    });
  }
}