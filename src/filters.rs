use crate::img::{Img, Pixel};
use image::Pixel as Pix;
use image::{DynamicImage, Rgba};
use std::error;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Basic black and white filter that uses an estimated luminance colour
/// ```ignore
/// let image = new_blank(Pixel {r: 0, g: 65535, b: 65535}, 512, 512);
/// image = filters::to_black_white(image);
/// save(image, Path::new("square.png"));
///```

pub fn to_black_white(image: &mut Img) -> Result<()> {
    let result = per_pixel(image, |(_x, _y), pixel: Pixel| {
        // https://stackoverflow.com/a/596243
        let end_value =
            (0.299 * pixel.r as f32 + 0.587 * pixel.g as f32 + 0.114 * pixel.b as f32) as u16;
        Pixel {
            r: end_value,
            g: end_value,
            b: end_value,
            a: pixel.a,
        }
    });

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}

/// Basic colour replacement filter that uses an estimated distance
/// ```ignore
/// let startColour = Pixel {r: 0, g: 65535, b: 65535, a: 65535};
/// let image = new_blank(startColour, 512, 512);
/// let endColour = Pixel {r: 65535, g: 0, b: 0, a: 65535};
/// image = filters::colour_replacement(image, startColour, endColour, 5);
/// save(image, Path::new("square.png"));
///```

pub fn colour_replacement(
    image: &mut Img,
    colour_from: Pixel,
    colour_too: Pixel,
    uncertainty: u16,
) -> Result<()> {
    let distance = u16::pow(uncertainty, 2);
    let result = per_pixel(image, |(_x, _y), pixel: Pixel| {
        if pixel.pixel_distance_manhattan(&colour_from) <= distance {
            colour_too
        } else {
            pixel
        }
    });

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
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
pub fn per_pixel(img: &mut Img, func: impl Fn((u32, u32), Pixel) -> Pixel) -> Result<()> {
    //convert image to ImageBuffer
    let mut buffer = img.image.to_rgba16();

    buffer.enumerate_pixels_mut().for_each(|(x, y, p)| {
        //let r, g, b, a;
        let pixel_slice: &[u16] = p.channels();
        let inputs = Pixel {
            r: pixel_slice[0],
            g: pixel_slice[1],
            b: pixel_slice[2],
            a: pixel_slice[3],
        };

        // https://stackoverflow.com/a/596243
        let output = func((x, y), inputs);

        *p = Rgba([output.r, output.g, output.b, output.a]);
    });
    //convert back to image and save
    img.image = DynamicImage::ImageRgba16(buffer);
    Ok(())
}
