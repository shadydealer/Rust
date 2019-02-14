pub extern crate gtk;
use gtk::WidgetExt;

mod ui;

use ui::App;

fn main() {
  // Initialize GTK before proceeding.

  // Initialize the UI's initial state
  let app = App::new();
  app.connect_events().then_execute();

  // Make all the widgets within the UI visible.
  app.window.show_all();

  // Start the GTK main event loop
  gtk::main();
}
