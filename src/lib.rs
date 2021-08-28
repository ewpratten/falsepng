use image::{DynamicImage, GenericImageView, Pixel, Rgba, RgbaImage};
use num_traits::Zero;

/// Given an image, returns a new image with the same dimensions, but with the background replaced by a fake transparency mesh.
pub fn falsify_png(img: &DynamicImage) -> RgbaImage {
    let mut output = RgbaImage::new(img.width(), img.height());
    img.pixels()
        .map(|(x, y, pixel)| {
            let mut out_pixel = pixel;
            // Only operate on pixels with no alpha
            if pixel.to_rgba().channels()[3].is_zero() {
                // Handle deciding if we are in a grey block or not. (5x5px)
                if (x % 10 < 5 && y % 10 < 5) || (x % 10 >= 5 && y % 10 >= 5) {
                    out_pixel = Rgba::from_channels(239, 239, 239, 255);
                } else {
                    out_pixel = Rgba::from_channels(255, 255, 255, 255);
                }
            }
            (x, y, out_pixel)
        })
        .for_each(|(x, y, pixel)| output.get_pixel_mut(x, y).0 = pixel.to_rgba().0);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn falsify_png_test() {
        let img = image::open("test.png").unwrap();
        let falsified = falsify_png(&img);
        falsified.save("testout.png").unwrap();
    }
}
