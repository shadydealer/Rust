use gtk::*;
use super::App;

pub struct ConnectedApp {
	app: App
}

impl ConnectedApp {

	pub fn new(app: App) -> Self {
		Self {
			app
		}
	}

	pub fn then_execute(self) -> () {
		self.app.window.show_all();
		gtk::main();
	}
}