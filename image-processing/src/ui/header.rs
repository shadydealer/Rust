use gtk::*;

pub struct Header {
	pub container: HeaderBar,
	pub open: Button,
	pub save: Button,
	pub save_as: Button,

}

impl Header {
	pub fn new() -> Self{
		
		let container = HeaderBar::new();
		container.set_title("Insert Witty Name Here");
		container.set_show_close_button(true);


		let open = Button::new_with_mnemonic("_Open");
		let save = Button::new_with_mnemonic("_Save");
		let save_as = Button::new_with_mnemonic("Save _As");
		container.pack_start(&open);
		container.pack_end(&save_as);
		container.pack_end(&save);

		Self {
			container,
			open,
			save,
			save_as
		}
	}
}