use gtk::*;

pub struct ImageContainer {
	pub container: Box
}

impl ImageContainer {
	pub fn new() -> Self {
		let padding_between_children = 0;
		let container = Box::new(Orientation::Vertical, padding_between_children);

		Self {
			container
		}
	}
}