pub mod image_rgb8;

#[doc(inline)]
pub use image_rgb8::ImageRGB8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let mut img = ImageRGB8::from_png("test_image2.png").unwrap();
        img.clear();
        img.draw_rectangle(5, 5, 21, 11, [255, 255, 255], 0, 1.0);
        img.to_png("test_image3.png").unwrap();
    }
}
