extern crate image;

use std::path::Path;


use image_changer::img::{Img, };

fn main() {
    let in_path = Path::new("./tests/test_images/rainbow_gradient.png");
    let out_path = Path::new("./tests/test_temp/rainbow_gradient.png");

    let mut image = match crate::Img::new(in_path) {
        Ok(img) => img,
        Err(e) => panic!("Error: {:?}", e)
    };

    match image.to_black_white() {
        Ok(()) => (),
        Err(e) => panic!("Error: {:?}", e)
    };

    match image.save(out_path) {
        Ok(()) => (),
        Err(e) => panic!("Error: {:?}", e)
    };
}
