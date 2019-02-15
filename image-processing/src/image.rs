pub extern crate image;
use std::path::PathBuf;
use failure::{err_msg, Error};

use image::image::{
  GenericImageView,
  GenericImage,
  DynamicImage,
  ImageError,
  Pixel
};

const MAX_COLOR_INTENSITY_U8: u8 = 255;
const MAX_COLOR_INTENSITY_USIZE: usize = 255;
type ColorIntensityBuckets = [usize; MAX_COLOR_INTENSITY_USIZE + 1];

#[derive(Clone)]
pub struct Image {
  image_path: PathBuf,
  dynamic_image: DynamicImage
}

impl Image {
  pub fn new(image_path: &PathBuf, dynamic_image: &DynamicImage) -> Self {
    Self {
      image_path: image_path.to_path_buf(),
      dynamic_image: dynamic_image.clone()
    }
  }

  pub fn open(image_path: &PathBuf) -> Result<Self, ImageError> {
    return match image::open(image_path) {
      Err(error) => Err(error),
      Ok(dynamic_image) => Ok(Self {
        image_path: image_path.to_path_buf(),
        dynamic_image
      })
    }
  }

  pub fn save_image(& self, path: Option<&PathBuf>) -> Result<PathBuf, Error> {

    let path = match path {
      Some(path) => &path,
      None => &self.image_path
    };

    match self.dynamic_image.save(&path) {
      Ok(_) => return Ok(path.to_path_buf()),
      Err(error) => return Err(err_msg(error))
    }
  }

  pub fn get_dir(&self) -> Option<PathBuf> {
    self.image_path.parent().map(|p| p.to_path_buf())
  }

  pub fn get_image_path(&self) -> PathBuf {self.image_path.clone()}


  pub fn get_dynamic_image(&self) -> DynamicImage {
    self.dynamic_image.clone()
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

    new_image_buffer
  }

  fn calculate_histogram(&self) -> ColorIntensityBuckets {
    let mut gray_level_distribution = [0; MAX_COLOR_INTENSITY_USIZE + 1];

    for (_, _, rgb) in self.dynamic_image.pixels() {
      gray_level_distribution[rgb[0] as usize] += 1;
    }
    gray_level_distribution
  }

  fn calculate_cumulative_distributions(&self, histogram:[usize; MAX_COLOR_INTENSITY_USIZE + 1]) -> ColorIntensityBuckets {
    let mut cumulative_distributions = [0; MAX_COLOR_INTENSITY_USIZE + 1];

    let mut accum = 0;

    for (i, pixel_count) in histogram.iter().enumerate() {
      accum += pixel_count;
      cumulative_distributions[i] = accum;
    }

    cumulative_distributions
  }

}
