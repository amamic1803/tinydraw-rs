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
        let mut img2 = ImageRGB8::from_bytes(200, 200, img.to_bytes()).unwrap();
        img2.draw_line(10, 10, 50, 150, [255, 255, 255]);
        img2.to_png("test_image3.png").unwrap();
    }
}
