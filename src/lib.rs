extern crate image;

use image::{GenericImageView, DynamicImage};
use image::io::Reader as ImageReader;
use image::error::ImageError;
use std::path::Path;
use std::io;

pub struct Img {
    image: DynamicImage,
}

impl Img {
    pub fn new(in_file: &Path) -> Result<Img, ImageError> {
        let img: DynamicImage;

        img = ImageReader::open(in_file)?.decode()?;

        Ok(Img {
            image: img,
        })
    }

    pub fn to_file(&self, out_path: &Path) -> Result<(), ImageError> {
        &self.image.save(out_path)?;
        Ok(())
    }
}

// takes in a closure and applies it for each pixel
fn filter_on_each_pixel() -> Img {
    todo!()
}
