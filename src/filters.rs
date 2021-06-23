use std::error;
use crate::img;
use image::Pixel as Pix;
use image::DynamicImage;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


/// Basic black and white filter that uses an estimated luminance colour
/// ```ignore
/// let image = new_blank(Pixel {r: 0, g: 65535, b: 65535}, 512, 512);
/// image = filters::to_black_white(image);
/// save(image, Path::new("square.png"));
///```

pub fn to_black_white(image: &mut img::Img) -> Result<()> {

    let result = per_pixel(image, |(_x, _y), pixel| {
        // https://stackoverflow.com/a/596243
        let end_value = (0.299*pixel.r as f32 + 0.587*pixel.g as f32 + 0.114*pixel.b as f32) as u16;
        img::Pixel{r: end_value, g: end_value, b: end_value, a: pixel.a}
    });

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(e)
    }
}


/// Allows you to write a custom filter for an image
/// ```ignore
/// let image = new_blank(Pixel {r: 0, g: 65535, b: 65535}, 512, 512);
/// image = filters::per_pixel(image, |(_x, _y), pixel| {
///    // https://stackoverflow.com/a/596243
///    let end_value = (0.299*pixel.r as f32 + 0.587*pixel.g as f32 + 0.114*pixel.b as f32) as u16;
///    img::Pixel{r: end_value, g: end_value, b: end_value, a: pixel.a}
/// });
/// ```
pub fn per_pixel(mut img: &mut img::Img, func: fn((u32, u32), img::Pixel) -> img::Pixel) -> Result<()> {
    //convert image to imagebuffer
    let mut buffer = img.image.to_rgba16();

    buffer.enumerate_pixels_mut().for_each(|(x, y, p)| {
        //let r, g, b, a;
        let (r, g, b, a) = p.channels4();
        let inputs = img::Pixel {r, g, b, a};

        // https://stackoverflow.com/a/596243
        let output = func((x, y), inputs);

        *p = Pix::from_channels(output.r, output.g, output.b, output.a);


    });
    //convert back to image and save
    img.image = DynamicImage::ImageRgba16(buffer);
    Ok(())
}
