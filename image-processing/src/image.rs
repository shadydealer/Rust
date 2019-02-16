pub extern crate image;
use image::image::{
  GenericImageView,
  GenericImage,
  DynamicImage,
  ImageError,
  Pixel
};
use std::path::PathBuf;
use failure::{err_msg, Error};


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


#[cfg(test)]
mod tests {
  use super::*;
  extern crate rand;
  use image::tests::rand::{Rng};
  use image::tests::rand::distributions::Alphanumeric;
  
  #[test]
  fn open_raises_error_if_it_cant_find_the_file() {
    let rstr: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(30)
    .collect();

    let path = PathBuf::from(rstr);
    assert!(Image::open(&path).is_err(), "file should not exist");
  }

  #[test]
  fn save_image_raises_error_if_the_specified_path_doesnt_have_a_specifier() {
    let path = PathBuf::from("nobody cares");
    let dynamic_image = DynamicImage::new_rgb8(0,0);
    let image = Image::new(&path, &dynamic_image);

    let invalid_path = PathBuf::from("../tests/fixtures/unequalized");
    assert!(image.save_image(Some(&invalid_path)).is_err(), "file should not exist");
  }

  #[test]
  fn calculates_the_histogram_correcty() {
    let path = PathBuf::from("nobody cares");
    let dynamic_image = DynamicImage::new_rgb8(1,3);
    let mut image = Image::new(&path, &dynamic_image);

    let mut valid_hist: ColorIntensityBuckets = [0; MAX_COLOR_INTENSITY_USIZE + 1];
    for i in 0..3 {
      image.dynamic_image.put_pixel(0,i as u32, Pixel::from_channels(i as u8,i as u8,i as u8,i as u8));
      valid_hist[i as usize] +=1;
    }
    let result_hist = image.calculate_histogram();

    assert_eq!(result_hist[0], valid_hist[0]);
    assert_eq!(result_hist[1], valid_hist[1]);
    assert_eq!(result_hist[2], valid_hist[2]);
  }

  #[test]
  fn calculates_the_cumulative_distribution_correcty() {
    let path = PathBuf::from("nobody cares");
    let dynamic_image = DynamicImage::new_rgb8(1,3);
    let mut image = Image::new(&path, &dynamic_image);

    let mut valid_distributions = [0; 3];
    for i in 0..3 {
      image.dynamic_image.put_pixel(0,i as u32, Pixel::from_channels(i as u8,i as u8,i as u8,i as u8));
      valid_distributions[i as usize] += i + 1;
    }
    let hist = image.calculate_histogram();
    let result_cd = image.calculate_cumulative_distributions(hist);

    assert_eq!(result_cd[0], valid_distributions[0]);
    assert_eq!(result_cd[1], valid_distributions[1]);
    assert_eq!(result_cd[2], valid_distributions[2]);
  }

  #[test]
  fn equalizes_the_histogram_correcty() {
    let path = PathBuf::from("nobody cares");
    let dynamic_image = DynamicImage::new_rgb8(1,3);
    let mut image = Image::new(&path, &dynamic_image);

    for i in 0..3 {
      image.dynamic_image.put_pixel(0,i as u32, Pixel::from_channels(i as u8,i as u8,i as u8,i as u8));
    }
    let image_eq_hist = image.equalize_histogram();
    for (_,_,rgb) in image_eq_hist.pixels() {
      println!("{:?}",rgb);
    }
    assert_eq!(image_eq_hist.get_pixel(0,0)[0], 0);
    assert_eq!(image_eq_hist.get_pixel(0,1)[0], 85);
    assert_eq!(image_eq_hist.get_pixel(0,2)[0], 170);
  }
}
