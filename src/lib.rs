pub mod image_rgb;

use crate::image_rgb::ImageRGB;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let mut img = ImageRGB::new(200, 200, [255, 0, 255]);
        img.clear();
        img.to_png("test_image.png");
    }
}
