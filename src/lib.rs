#[allow(dead_code)]
pub mod img {
    extern crate image;

    use image:: DynamicImage;
    use image::io::Reader as ImageReader;
    use std::path::Path;
    use std::error;
    use crate::utils::outer::paths;

    // Change the alias to `Box<error::Error>`.
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    pub struct Img {
        image: DynamicImage,
    }

    impl Img {
        pub fn new(in_file: &Path) -> Result<Img> {
            let img: DynamicImage;

            let file_path = match paths::absolute_path(in_file) {
                Ok(e) => e,
                Err(a) => panic!("Error: {:?}", a)
            };
            img = ImageReader::open(file_path.as_path())?.decode()?;

            Ok(Img {
                image: img,
            })
        }

        pub fn to_file(&self, out_path: &Path) -> Result<()> {
            let file_path = match paths::absolute_path(out_path) {
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
}

pub(self) mod utils;
