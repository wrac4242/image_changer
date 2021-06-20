extern crate image;

use image::{DynamicImage, Pixel};
use image::io::Reader as ImageReader;
use std::path::Path;
use std::error;
use crate::utils;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Img {
    image: DynamicImage,
}

impl Img {
    pub fn new(in_file: &Path) -> Result<Img> {
        let img: DynamicImage;
        let file_path = match utils::absolute_path(in_file) {
        Ok(e) => e,
            Err(a) => panic!("Error: {:?}", a)
        };
        let raw = ImageReader::open(file_path.as_path())?.decode()?.to_rgba16();
        img = DynamicImage::ImageRgba16(raw);

        Ok(Img {
            image: img,
        })
    }

    pub fn save(&self, out_path: &Path) -> Result<()> {
        let file_path = match utils::absolute_path(out_path) {
            Ok(e) => e,
            Err(a) => panic!("Error: {:?}", a)
        };
        self.image.save(file_path.as_path())?;
        Ok(())
    }

    pub fn to_black_white(&mut self) -> Result<()> {

        let result = per_pixel(self, |(_x, _y), (r, g, b, a)| {
            // https://stackoverflow.com/a/596243
            let end_value = (0.299*r as f32 + 0.587*g as f32 + 0.114*b as f32) as u16;
            (end_value, end_value, end_value, a)
        });

        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(e)
        }
    }

}

// takes in (x, y),   (r, g, b, a)
fn per_pixel(mut img: &mut Img, func: fn((u32, u32), (u16, u16, u16, u16))-> (u16, u16, u16, u16)) -> Result<()> {
    //convert image to imagebuffer
    let mut buffer = img.image.to_rgba16();

    buffer.enumerate_pixels_mut().for_each(|(x, y, p)| {
        //let r, g, b, a;
        let inputs = p.channels4();
        // each of these are u16s

        // https://stackoverflow.com/a/596243
        let (r, g, b, a) = func((x, y), inputs);

        *p = Pixel::from_channels(r, g, b, a);


    });
    //convert back to image and save
    img.image = DynamicImage::ImageRgba16(buffer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn opening_saving() {
        let in_path = Path::new("./tests/test_images/black_square.png");
        let out_path = Path::new("./tests/test_temp/black_square.png");
        let expected_path = Path::new("./tests/test_expected/black_square.png");

        let image = match Img::new(in_path) {
            Ok(img) => img,
            Err(e) => panic!("Error: {:?}", e)
        };

        match image.save(out_path) {
            Ok(_) => (),
            Err(e) => panic!("Error: {:?}", e)
        };

        //check if the output file hash is correct
        let output_hash = match utils::testing::hash_file(out_path) {
            Ok(a) => a,
            Err(e) => panic!("Error: {:?}", e)
        };
        let expected_hash = match utils::testing::hash_file(expected_path) {
            Ok(a) => a,
            Err(e) => panic!("Error: {:?}", e)
        };

        assert_eq!(output_hash, expected_hash);
        let _ignore = fs::remove_file(out_path);
    }
}
