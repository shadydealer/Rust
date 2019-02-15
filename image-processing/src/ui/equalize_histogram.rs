use failure::{format_err, Error};
use std::error::Error as OtherError;
use std::sync::RwLock;
use gtk::*;

use ui::image_container::render_image;
use image::Image as MyImage;

pub fn equalize_histogram(image_container: &Image,
                          current_file: &RwLock<Option<MyImage>>,
                          ) -> Result<(), Error> {

	let image_data =
	match current_file.try_read() {
		Ok(guard) => match *guard {
			Some(ref image) => Some((image.equalize_histogram(), image.get_image_path())),
			None => None
		},
		Err(error) => return Err(format_err!("{}", error.description()))
	};

	// if there's no file open, then image data's value will be None
	if let Some((equalized_dynamic_image, image_path)) = image_data {
		let mut new_image = MyImage::new(&image_path, &equalized_dynamic_image);
		render_image(&image_container, &new_image);
		*current_file.write().unwrap() = Some(new_image);
	}
	Ok(())
}