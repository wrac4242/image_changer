extern crate image;

use image:: DynamicImage;
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
        img = ImageReader::open(file_path.as_path())?.decode()?;

        Ok(Img {
            image: img,
        })
    }

    pub fn to_file(&self, out_path: &Path) -> Result<()> {
        let file_path = match utils::absolute_path(out_path) {
            Ok(e) => e,
            Err(a) => panic!("Error: {:?}", a)
        };
        self.image.save(file_path.as_path())?;
        Ok(())
    }
}

// takes in a closure and applies it for each pixel
fn filter_on_each_pixel() -> Img {
    todo!()
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

        let image = Img::new(in_path);

        let image = match image {
            Ok(img) => img,
            Err(e) => panic!("Error: {:?}", e)
        };

        match image.to_file(out_path) {
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
