pub extern crate image;
use image::image::{
  GenericImageView,
  GenericImage,
  DynamicImage,
  ImageError,
  Pixel
 }

use std::path::PathBuf;

const MAX_COLOR_INTENSITY_U8: u8 = 255;
const MAX_COLOR_INTENSITY_USIZE: usize = 255;

pub struct Image {
  image_path: PathBuf,
  dynamic_image: DynamicImage
}

impl Image {
  pub fn new(image_path: &str) -> Result<Self, ImageError> {
    return match image::open(image_path) {
      Err(error) => Err(error),
      Ok(image) => Ok(Self {
        image_path: String::from(image_path),
        dynamic_image: image
      })
    }
  }

  pub fn equalize_histogram(&self) -> DynamicImage {

    let (width, height) = self.dynamic_image.dimensions();
    let mut new_image_buffer: DynamicImage =
      DynamicImage::new_rgb8(width, height);
    
    let histogram = self.calculate_histogram();
    let cumulative_distributions = self.calculate_cumulative_distributions(histogram);

    let cdf_min =
      match cumulative_distributions.iter().find(|&&cd| cd != 0usize) {
        Some(val) => *val,
        None => 0usize
      };

    for (c, r, mut rgb) in self.dynamic_image.pixels() {
      let cd = cumulative_distributions[rgb[0] as usize];
      let val = ((((cd - cdf_min) as f32) / ((width * height)as f32)) * (MAX_COLOR_INTENSITY_U8 as f32) )as u8;
      new_image_buffer.put_pixel(c,r, Pixel::from_channels(val,val,val,rgb[3]));
    }

    new_image_buffer.save(&self.image_path).unwrap();
    new_image_buffer
  }

  fn calculate_histogram(&self) -> [usize; MAX_COLOR_INTENSITY_USIZE + 1] {
    let mut gray_level_distribution = [0; MAX_COLOR_INTENSITY_USIZE + 1];

    for (_, _, rgb) in self.dynamic_image.pixels() {
      gray_level_distribution[rgb[0] as usize] += 1;
    }
    gray_level_distribution
  }

  fn calculate_cumulative_distributions(&self, histogram:[usize; MAX_COLOR_INTENSITY_USIZE + 1]) -> [usize; MAX_COLOR_INTENSITY_USIZE + 1] {
    let mut cumulative_distributions = [0; MAX_COLOR_INTENSITY_USIZE + 1];

    let mut accum = 0;

    for (i, pixel_count) in histogram.iter().enumerate() {
      accum += pixel_count;
      cumulative_distributions[i] = accum;
    }

    cumulative_distributions
  }
}
