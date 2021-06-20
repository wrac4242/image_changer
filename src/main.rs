extern crate image;

use std::path::Path;


use image_changer::img::{Img, };

fn main() {
    let in_path = Path::new("./tests/test_images/rainbow_gradient.png");
    let out_path = Path::new("./tests/test_temp/rainbow_gradient.png");

    let image = crate::Img::new(in_path);

    let mut image = match image {
        Ok(img) => img,
        Err(e) => panic!("Error: {:?}", e)
    };

    println!("foo");
    println!("Error: {:?}", image.to_black_white());

    image.to_file(out_path);
}
