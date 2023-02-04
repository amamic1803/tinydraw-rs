pub mod image_rgb;

use crate::image_rgb::ImageRGB;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let mut img = ImageRGB::from_png("test_image2.png");
        img.clear();
        img.to_png("test_image.png");
    }
}
