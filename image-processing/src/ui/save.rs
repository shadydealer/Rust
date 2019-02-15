use failure::{err_msg, format_err, Error};
use std::error::Error as OtherError;
use std::sync::RwLock;
use gtk::*;
use image::image::DynamicImage;
use image::Image;
use super::dialogs::save_dialog::SaveDialog;

pub enum SaveAction {
	New(Image),
	Saved,
	Canceled
}

fn save_image(image: Option<& Image>, dynamic_image: Option<DynamicImage>) -> Result<SaveAction, Error> {

	// if there's no file open, then there wont be a dynamic_image
	if let Some(dynamic_image) = dynamic_image {

		// user clicked the 'Save' button
		if let Some(image) = image {
			if let Err(error) = image.save_image(None){
				return Err(err_msg(error));
			}
			return Ok(SaveAction::Saved);
		}

		// user clicked the 'Save As' button
		let save_dialog = SaveDialog::new(None);
		if let Some(new_path) = save_dialog.run() {

			let mut new_image = Image::new(&new_path, &dynamic_image);

			if let Err(error) = new_image.save_image(None) {
				return Err(err_msg(error));
			}
			return Ok(SaveAction::New(new_image));
		}
	}
	// user canceled the dialog
	Ok(SaveAction::Canceled)
}

pub fn save(headerbar: &HeaderBar,
            current_file: &RwLock<Option<Image>>,
            save_as: bool,
            ) -> Result<(), Error> {

	let dynamic_image = match current_file.try_read() {
		Ok(guard) => match *guard {
			Some(ref image) => Some(image.get_dynamic_image()),
			None => None
		},
		Err(error) => return Err(format_err!("{}", error.description()))
	};

	// user has clicked the  'Save As' button
	let result =
		if save_as {
			save_image(None, dynamic_image)
		}
		else {
			match current_file.try_read() {
				Ok(guard) => match *guard {
					Some(ref image) => save_image(Some(image), dynamic_image),
					None => Ok(SaveAction::Canceled)
				},
				Err(error) => return Err(format_err!("{}", error.description()))
			}
		};

	match result {
		Ok(SaveAction::Saved) => {
			Ok(())
		},
		Ok(SaveAction::New(image)) => {

			let image_path = image.get_image_path();

			headerbar.set_title(image_path.to_str());

			match Image::open(&image_path){
				Ok(mut image) => *current_file.write().unwrap() = Some(image),
				Err(error) => return Err(err_msg(error))
			}
			Ok(())
		},
		Err(error) => return Err(err_msg(error)),
		_ => Ok(())
	}
}
