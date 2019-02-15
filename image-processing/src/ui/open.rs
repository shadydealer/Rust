use failure::{err_msg, format_err, Error};
use std::error::Error as OtherError;
use std::sync::RwLock;
use gtk::*;

use ui::image_container::render_image;
use image::Image as MyImage;
use super::dialogs::open_dialog::OpenDialog;

pub fn open (headerbar: &HeaderBar,
             image_container: &Image,
             current_file: &RwLock<Option<MyImage>>,
             ) -> Result<(), Error> {


	let open_dialog = OpenDialog::new({
		match current_file.try_read() {
			Ok(guard) => match * guard {
				Some(ref image) => image.get_dir(),
				None => None
			},
			Err(error) => return Err(format_err!("{}", error.description()))
		}

	});

	if let Some(file_path) = open_dialog.run() {
		match MyImage::open(&file_path) {
			Ok(mut image) => {
				headerbar.set_title(file_path.to_str());
				render_image(&image_container, &image);
				*current_file.write().unwrap() = Some(image);
			},
			Err(error) => return Err(err_msg(error))
		}
	}
	Ok(())
}