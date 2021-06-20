use std::error;
use crate::{utils, img};

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


/// Basic black and white filter that uses an estimated luminance colour
/// ```ignore
/// let image = new_blank(Pixel {r: 0, g: 65535, b: 65535}, 512, 512);
/// image.to_black_white();
/// save(image, Path::new("square.png"));
///```

pub fn to_black_white(image: &mut img::Img) -> Result<()> {

    let result = utils::per_pixel(image, |(_x, _y), pixel| {
        // https://stackoverflow.com/a/596243
        let end_value = (0.299*pixel.r as f32 + 0.587*pixel.g as f32 + 0.114*pixel.b as f32) as u16;
        img::Pixel{r: end_value, g: end_value, b: end_value, a: pixel.a}
    });

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(e)
    }
}
