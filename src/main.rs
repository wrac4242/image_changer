
use image_changer::img::{Img, };
use std::path::Path;


//to be made into a test later
fn main() {
    let image = Img::new(Path::new("./tests/test_images/black_square.png"));

    let image = match image {
        Ok(img) => img,
        Err(e) => panic!("Error: {:?}", e)
    };

    println!("{:?}", image.to_file(Path::new("./tests/test_temp/black_square.png")));

    //check if the output file hash is correct
}
