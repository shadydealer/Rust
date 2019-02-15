pub extern crate failure;
pub extern crate gtk;
pub extern crate gdk_pixbuf;
mod image;
mod ui;
use ui::App;

fn main() {
  App::new().connect_events().then_execute();
}
