use gtk::*;
use gtk::ImageExt;
use gdk_pixbuf::Pixbuf;
use gdk_pixbuf::Colorspace;

use image::Image as MyImage;
use image::image::{GenericImageView};

pub struct ImageContainer {
	pub image_widget: Image
}

impl ImageContainer {
	pub fn new() -> Self {
		let image_widget = Image::new();
		Self {
			image_widget
		}
	}
}

pub fn render_image(image_widget: &Image, image: &MyImage) {

	let dynamic_image = image.get_dynamic_image();
	let pixels = dynamic_image.raw_pixels();
	let (width, height) = dynamic_image.dimensions();

	let pixbuf = Pixbuf::new_from_vec(
	                                  pixels,
	                                  Colorspace::Rgb,
	                                  false,
	                                  8,
	                                  width as i32,
	                                  height as i32,
	                                  3*width as i32);	
	image_widget.set_halign(Align::Start);
	image_widget.set_from_pixbuf(&pixbuf);
}
