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
        img.draw_rectangle(38, 35, 165, 150, [255, 255, 255], 10, 0.75);
        img.to_png("test_image3.png").unwrap();
    }
}
