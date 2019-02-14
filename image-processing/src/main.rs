pub extern crate gtk;
mod image;
mod ui;
use ui::App;

fn main() {
  App::new().connect_events().then_execute();
}
