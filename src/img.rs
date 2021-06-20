extern crate image;

use image::{DynamicImage, Pixel};
use image::io::Reader as ImageReader;
use std::path::Path;
use std::error;
use crate::utils;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct Pixelinternal {
    r: u16,
    g: u16,
    b: u16,
    a: u16,
}

pub struct Img {
    image: DynamicImage,
}

impl Img {
    pub fn new_from_file(in_file: &Path) -> Result<Img> {
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

        let result = per_pixel(self, |(_x, _y), pixel| {
            // https://stackoverflow.com/a/596243
            let end_value = (0.299*pixel.r as f32 + 0.587*pixel.g as f32 + 0.114*pixel.b as f32) as u16;
            Pixelinternal{r: end_value, g: end_value, b: end_value, a: pixel.a}
        });

        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(e)
        }
    }

}

// takes in (x, y),   (r, g, b, a)
fn per_pixel(mut img: &mut Img, func: fn((u32, u32), Pixelinternal) -> Pixelinternal) -> Result<()> {
    //convert image to imagebuffer
    let mut buffer = img.image.to_rgba16();

    buffer.enumerate_pixels_mut().for_each(|(x, y, p)| {
        //let r, g, b, a;
        let (r, g, b, a) = p.channels4();
        // each of these are u16s
        let inputs = Pixelinternal {r, g, b, a};

        // https://stackoverflow.com/a/596243
        let output = func((x, y), inputs);

        *p = Pixel::from_channels(output.r, output.g, output.b, output.a);


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
    fn opening_saving() {
        testing("black_square.png", "black_square.png", | image | image);
    }

    #[test]
    fn black_white() {
        testing("rainbow_gradient.png", "rainbow_gradient_bw.png", | mut image | {
            match image.to_black_white() {
                Ok(()) => (),
                Err(e) => panic!("Error: {:?}", e)
            };
            image
        });
    }

    fn testing(file_name: &str, expected_name: &str, command: fn(Img) -> Img) {
        //testing general closure for reuse
        let file_name = file_name.clone();
        let in_path_str = "./tests/test_images/".to_string() + file_name.clone();
        let out_path_str = "./tests/test_temp/".to_string() + expected_name.clone();
        let expected_path_str = "./tests/test_expected/".to_string() + expected_name.clone();

        let in_path = Path::new(&in_path_str);
        let out_path = Path::new(&out_path_str);
        let expected_path = Path::new(&expected_path_str);

        let mut image = match Img::new_from_file(in_path) {
            Ok(img) => img,
            Err(e) => panic!("Error: {:?}", e)
        };

        image = command(image);

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
