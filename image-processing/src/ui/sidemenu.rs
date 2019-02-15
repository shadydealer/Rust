use gtk::*;

pub struct SideMenu {
	pub container: Box,
	pub equalize_histogram: Button,
	pub edge_detection: Button
}

impl SideMenu {
	pub fn new() -> Self {
		let padding_between_children = 2;
		let container = Box::new(Orientation::Vertical, padding_between_children);

		let equalize_histogram = SideMenu::initialize_equalize_histogram_button(&container);
		let edge_detection = SideMenu::initialize_edge_detection_button(&container);


		Self {
			container,
			equalize_histogram,
			edge_detection
		}
	}

	fn initialize_equalize_histogram_button(container: &Box) -> Button {
		let padding_between_children = 0;
		let equalize_histogram_button = Button::new_with_label("equalize histogram");

		equalize_histogram_button.set_halign(Align::Center);

		container.pack_start(&equalize_histogram_button, false, false, padding_between_children);
		equalize_histogram_button
	}

	fn initialize_edge_detection_button(container: &Box) -> Button {
		let padding_between_children = 0;
		let edge_detection_button = Button::new_with_label("edge detection");

		edge_detection_button.set_halign(Align::Center);

		container.pack_start(&edge_detection_button, false, false, padding_between_children);
		edge_detection_button
	}
}