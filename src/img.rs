//! # img
//!
//! module for the creation and management of images.


use image::{DynamicImage, ImageBuffer};
use image::io::Reader as ImageReader;
use std::path::Path;
use std::error;
use crate::{utils, misc};

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Pixel, the smallest part of an image
pub struct Pixel {
    pub(crate) r: u16,
    pub(crate) g: u16,
    pub(crate) b: u16,
    pub(crate) a: u16,
}

impl Pixel {
    /// creates pixels from 0-256 colour ranges
    /// ```
    /// use image_changer::img::*;
    /// let pixel = Pixel::new_from_256(0, 255, 255, 255);
    /// ```
    ///## errors
    /// `Out of Range`: one of the inputs is greater than 255
    pub fn new_from_256(r: u16, g: u16, b: u16, a: u16) -> Result<Pixel>{
        if r > 255 || g > 255 || b > 255 || a > 255 {
            return Err(Box::new(misc::MyError::new("Out of Range")))
        }

        let scale = (u16::MAX as f64 / 255.0) as f64;
        let r = num::clamp(r as f64 *scale, 0.0, u16::MAX as f64) as u16;
        let g = num::clamp(g as f64 *scale, 0.0, u16::MAX as f64) as u16;
        let b = num::clamp(b as f64 *scale, 0.0, u16::MAX as f64) as u16;
        let a = num::clamp(a as f64 *scale, 0.0, u16::MAX as f64) as u16;

        let pix_out = Pixel {r, g, b, a};
        Ok(pix_out)
    }

    pub fn pixel_to_256(&self) -> (u16, u16, u16, u16) {
        let scale = (255.0 / u16::MAX as f64) as f64;
        let r = num::clamp(self.r as f64 *scale, 0.0, u16::MAX as f64) as u16;
        let g = num::clamp(self.g as f64 *scale, 0.0, u16::MAX as f64) as u16;
        let b = num::clamp(self.b as f64 *scale, 0.0, u16::MAX as f64) as u16;
        let a = num::clamp(self.a as f64 *scale, 0.0, u16::MAX as f64) as u16;

        (r, g, b, a)
    }
}

/// The image struct, it stores the image being processed
pub struct Img {
    pub(crate) image: DynamicImage,
}

/// Opens a file as an Img
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

/// Creates a new Img with a single set colour
pub fn new_blank(colour: Pixel, width: u32, height: u32) -> Result<Img> {
    let img: DynamicImage;
    let img_buff = ImageBuffer::from_fn(width, height, |_, _| {
        image::Rgba([colour.r, colour.g, colour.b, colour.a])
    });
    img = DynamicImage::ImageRgba16(img_buff);

    Ok(Img {
        image: img,
    })
}

/// saves the given Img to a file
pub fn save(image: Img, out_path: &Path) -> Result<()> {
    let file_path = match utils::absolute_path(out_path) {
        Ok(e) => e,
        Err(a) => panic!("Error: {:?}", a)
    };
    match image.image.save(file_path.as_path()) {
        Ok(_) => (),
        Err(e) => panic!("Error: {:?}", e)
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::filters;

    #[test]
    fn pixel_from_256(){
        let pixel_as_256 = match Pixel::new_from_256(0, 255, 255, 255) {
            Ok(e) => e,
            Err(e) => panic!("Error: {:?}", e)
        };
        let pixel_raw = Pixel {r: 0, g: 65535, b: 65535, a: 65535};
        assert_eq!(pixel_as_256.r, pixel_raw.r);
        assert_eq!(pixel_as_256.g, pixel_raw.g);
        assert_eq!(pixel_as_256.b, pixel_raw.b);
        assert_eq!(pixel_as_256.a, pixel_raw.a);
    }

    #[test]
    #[should_panic]
    fn pixel_from_256_panic(){
        let _ = match Pixel::new_from_256(256, 22222, 16, 5) {
            Ok(e) => e,
            Err(e) => panic!("Error: {:?}", e)
        };
    }

    #[test]
    fn pixel_to_256(){
        let pix = match Pixel::new_from_256(65, 98, 42, 12) {
            Ok(e) => e,
            Err(e) => panic!("Error: {:?}", e)
        }; // create pixel from set of values

        let (r, g, b, a) = pix.pixel_to_256();
        assert_eq!(r, 65);
        assert_eq!(g, 98);
        assert_eq!(b, 42);
        assert_eq!(a, 12);
    }

    #[test]
    fn opening_saving() {
        testing_convert("black_square.png", "black_square.png", | image | image);
    }

    #[test]
    fn black_white() {
        testing_convert("rainbow_gradient.png", "rainbow_gradient_bw.png", | mut image | {
            match filters::to_black_white(&mut image) {
                Ok(()) => (),
                Err(e) => panic!("Error: {:?}", e)
            };
            image
        });
    }

    fn testing_convert(file_name: &str, expected_name: &str, command: fn(Img) -> Img) {
        //testing general closure for reuse
        let file_name = file_name.clone();
        let in_path_str = "./tests/test_images/".to_string() + file_name.clone();
        let out_path_str = "./tests/test_temp/".to_string() + expected_name.clone();
        let expected_path_str = "./tests/test_expected/".to_string() + expected_name.clone();

        let in_path = Path::new(&in_path_str);
        let out_path = Path::new(&out_path_str);
        let expected_path = Path::new(&expected_path_str);

        let mut image = match new_from_file(in_path) {
            Ok(img) => img,
            Err(e) => panic!("Error: {:?}", e)
        };

        image = command(image);

        match save(image, out_path) {
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

    #[test]
    fn new_blank_test() {
        let out_path = Path::new("./tests/test_temp/new_blank.png");
        let expected_path = Path::new("./tests/test_expected/new_blank.png");

        let image = match new_blank(Pixel {r: 0, g: 65535, b: 65535, a: 65535}, 512, 512) {
            Ok(a) => a,
            Err(e) => panic!("Error: {:?}", e)
        };

        match save(image, out_path) {
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
