//! # img
//!
//! module for the creation and management of images.

use crate::{misc, utils};
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer};
use std::error;
use std::ops;
use std::path::Path;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Pixel, the smallest part of an image
#[derive(Clone, Copy)]
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
    pub fn new_from_256(r: u16, g: u16, b: u16, a: u16) -> Result<Pixel> {
        if r > 255 || g > 255 || b > 255 || a > 255 {
            return Err(Box::new(misc::ImgConversionError::new("Out of Range")));
        }

        let scale = u16::MAX as f64 / 255.0;
        let r = num::clamp(r as f64 * scale, 0.0, u16::MAX as f64) as u16;
        let g = num::clamp(g as f64 * scale, 0.0, u16::MAX as f64) as u16;
        let b = num::clamp(b as f64 * scale, 0.0, u16::MAX as f64) as u16;
        let a = num::clamp(a as f64 * scale, 0.0, u16::MAX as f64) as u16;

        let pix_out = Pixel { r, g, b, a };
        Ok(pix_out)
    }

    pub fn pixel_to_256(&self) -> (u16, u16, u16, u16) {
        let scale = 255.0 / u16::MAX as f64;
        let r = num::clamp(self.r as f64 * scale, 0.0, u16::MAX as f64) as u16;
        let g = num::clamp(self.g as f64 * scale, 0.0, u16::MAX as f64) as u16;
        let b = num::clamp(self.b as f64 * scale, 0.0, u16::MAX as f64) as u16;
        let a = num::clamp(self.a as f64 * scale, 0.0, u16::MAX as f64) as u16;

        (r, g, b, a)
    }

    pub fn pixel_distance_manhattan(&self, distance_from: &Pixel) -> u16 {
        let res = self - distance_from;
        res.r
            .saturating_add(res.g)
            .saturating_add(res.b)
            .saturating_add(res.a)
    }
}

impl ops::Add for Pixel {
    type Output = Pixel;
    fn add(self, rhs: Pixel) -> Pixel {
        Pixel {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
            a: self.a.saturating_add(rhs.a),
        }
    }
}

impl ops::Sub for &Pixel {
    type Output = Pixel;
    fn sub(self, rhs: &Pixel) -> Self::Output {
        Pixel {
            r: self.r.abs_diff(rhs.r),
            g: self.g.abs_diff(rhs.g),
            b: self.b.abs_diff(rhs.b),
            a: self.a.abs_diff(rhs.a),
        }
    }
}

/// The image struct, it stores the image being processed
pub struct Img {
    pub(crate) image: DynamicImage,
}

/// Opens a file as an Img
pub fn new_from_file(in_file: &Path) -> Result<Img> {
    let file_path = match utils::absolute_path(in_file) {
        Ok(e) => e,
        Err(a) => panic!("Error: {:?}", a),
    };
    let raw = ImageReader::open(file_path.as_path())?
        .decode()?
        .to_rgba16();
    let img: DynamicImage = DynamicImage::ImageRgba16(raw);

    Ok(Img { image: img })
}

/// Creates a new Img with a single set colour
pub fn new_blank(colour: Pixel, width: u32, height: u32) -> Result<Img> {
    let img_buff = ImageBuffer::from_fn(width, height, |_, _| {
        image::Rgba([colour.r, colour.g, colour.b, colour.a])
    });
    let img: DynamicImage = DynamicImage::ImageRgba16(img_buff);

    Ok(Img { image: img })
}

/// saves the given Img to a file
pub fn save(image: Img, out_path: &Path) -> Result<()> {
    let file_path = match utils::absolute_path(out_path) {
        Ok(e) => e,
        Err(a) => panic!("Error: {:?}", a),
    };
    match image.image.save(file_path.as_path()) {
        Ok(_) => (),
        Err(e) => panic!("Error: {:?}", e),
    };
    Ok(())
}

/// gives the dimensions of the inputted image
pub fn dimensions(image: &Img) -> (u32, u32) {
    let buffer = image.image.to_rgba16();
    buffer.dimensions()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters;
    use std::fs;

    #[test]
    fn pixel_from_256() {
        let pixel_as_256 = match Pixel::new_from_256(0, 255, 255, 255) {
            Ok(e) => e,
            Err(e) => panic!("Error: {:?}", e),
        };
        let pixel_raw = Pixel {
            r: 0,
            g: 65535,
            b: 65535,
            a: 65535,
        };
        assert_eq!(pixel_as_256.r, pixel_raw.r);
        assert_eq!(pixel_as_256.g, pixel_raw.g);
        assert_eq!(pixel_as_256.b, pixel_raw.b);
        assert_eq!(pixel_as_256.a, pixel_raw.a);
    }

    #[test]
    #[should_panic]
    fn pixel_from_256_panic() {
        let _ = match Pixel::new_from_256(256, 22222, 16, 5) {
            Ok(e) => e,
            Err(e) => panic!("Error: {:?}", e),
        };
    }

    #[test]
    fn pixel_to_256() {
        let pix = match Pixel::new_from_256(65, 98, 42, 12) {
            Ok(e) => e,
            Err(e) => panic!("Error: {:?}", e),
        }; // create pixel from set of values

        let (r, g, b, a) = pix.pixel_to_256();
        assert_eq!(r, 65);
        assert_eq!(g, 98);
        assert_eq!(b, 42);
        assert_eq!(a, 12);
    }

    #[test]
    fn opening_saving() {
        testing_convert("black_square.png", "black_square.png", |image| image);
    }

    #[test]
    fn black_white() {
        testing_convert(
            "rainbow_gradient.png",
            "rainbow_gradient_bw.png",
            |mut image| {
                match filters::to_black_white(&mut image) {
                    Ok(()) => (),
                    Err(e) => panic!("Error: {:?}", e),
                };
                image
            },
        );
    }

    #[test]
    fn colour_change() {
        let start_colour = Pixel {
            r: 0,
            g: 65535,
            b: 65535,
            a: 65535,
        };
        let end_colour = Pixel {
            r: 65535,
            g: 0,
            b: 0,
            a: 65535,
        };
        testing_convert(
            "rainbow_gradient.png",
            "rainbow_gradient_colour_replaced.png",
            |mut image| {
                match filters::colour_replacement(&mut image, start_colour, end_colour, 30) {
                    Ok(()) => (),
                    Err(e) => panic!("Error: {:?}", e),
                };
                image
            },
        );
    }

    #[test]
    fn dimension() {
        let image = match new_blank(
            Pixel {
                r: 0,
                g: 65535,
                b: 65535,
                a: 65535,
            },
            512,
            512,
        ) {
            Ok(a) => a,
            Err(e) => panic!("Error: {:?}", e),
        };
        assert_eq!(dimensions(&image), (512, 512));
    }

    fn testing_convert(file_name: &str, expected_name: &str, command: impl Fn(Img) -> Img) {
        //testing general closure for reuse
        let file_name = file_name;
        let in_path_str = "./tests/test_images/".to_string() + file_name;
        let out_path_str = "./tests/test_temp/".to_string() + expected_name;
        let expected_path_str = "./tests/test_expected/".to_string() + expected_name;

        let in_path = Path::new(&in_path_str);
        let out_path = Path::new(&out_path_str);
        let expected_path = Path::new(&expected_path_str);

        let mut image = match new_from_file(in_path) {
            Ok(img) => img,
            Err(e) => panic!("Error: {:?}", e),
        };

        image = command(image);

        match save(image, out_path) {
            Ok(_) => (),
            Err(e) => panic!("Error: {:?}", e),
        };

        //check if the output file hash is correct
        let output_hash = match utils::testing::hash_file(out_path) {
            Ok(a) => a,
            Err(e) => panic!("Error: {:?}", e),
        };
        let expected_hash = match utils::testing::hash_file(expected_path) {
            Ok(a) => a,
            Err(e) => panic!("Error: {:?}", e),
        };

        assert_eq!(output_hash, expected_hash);
        let _ignore = fs::remove_file(out_path);
    }

    #[test]
    fn new_blank_test() {
        let out_path = Path::new("./tests/test_temp/new_blank.png");
        let expected_path = Path::new("./tests/test_expected/new_blank.png");

        let image = match new_blank(
            Pixel {
                r: 0,
                g: 65535,
                b: 65535,
                a: 65535,
            },
            512,
            512,
        ) {
            Ok(a) => a,
            Err(e) => panic!("Error: {:?}", e),
        };

        match save(image, out_path) {
            Ok(_) => (),
            Err(e) => panic!("Error: {:?}", e),
        };

        //check if the output file hash is correct
        let output_hash = match utils::testing::hash_file(out_path) {
            Ok(a) => a,
            Err(e) => panic!("Error: {:?}", e),
        };
        let expected_hash = match utils::testing::hash_file(expected_path) {
            Ok(a) => a,
            Err(e) => panic!("Error: {:?}", e),
        };

        assert_eq!(output_hash, expected_hash);
        let _ignore = fs::remove_file(out_path);
    }

    #[test]
    fn pixel_distance_check() {
        let base_pix = Pixel {
            r: 0,
            g: 255,
            b: 100,
            a: 40,
        };

        assert_eq!(
            base_pix.pixel_distance_manhattan(
                &(base_pix
                    + Pixel {
                        r: 0,
                        g: 5,
                        b: 0,
                        a: 0
                    })
            ),
            5
        );
        assert_eq!(
            base_pix.pixel_distance_manhattan(
                &(base_pix
                    + Pixel {
                        r: 3,
                        g: 6,
                        b: 2,
                        a: 7
                    })
            ),
            3 + 6 + 2 + 7
        );
        assert_eq!(
            base_pix.pixel_distance_manhattan(
                &(base_pix
                    + Pixel {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    })
            ),
            0
        );
        assert_eq!(
            base_pix.pixel_distance_manhattan(
                &(base_pix
                    + Pixel {
                        r: 1,
                        g: 0,
                        b: 5,
                        a: 0
                    })
            ),
            1 + 5
        );

        assert_eq!(
            base_pix.pixel_distance_manhattan(
                &(base_pix
                    + Pixel {
                        r: 55555,
                        g: 11231,
                        b: 0,
                        a: 0
                    })
            ),
            55555_u16.saturating_add(11231_u16)
        );
    }
}
