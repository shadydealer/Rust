use gtk::*;
use super::{SideMenu, ImageContainer};

pub struct Content {
	pub container: Box,
	pub image_container: ImageContainer,
	pub side_menu: SideMenu
}

impl Content {
	
	pub fn new() -> Self {

		let padding_between_children = 0;
		let container = Box::new(Orientation::Horizontal, padding_between_children);

		let side_menu = SideMenu::new();
		let image_container = ImageContainer::new();

		container.pack_start(&image_container.container, false, false, padding_between_children as u32);
		container.pack_end(&side_menu.container, false, false, padding_between_children as u32);

		Self {
			container,
			image_container,
			side_menu
		}
	}

}