pub mod image_rgb8;

#[doc(inline)]
pub use image_rgb8::ImageRGB8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let mut img = ImageRGB8::new(200, 200, [25, 200, 25]);
        img.clear();
        img.draw_circle(50, 50, 30, [255, 255, 255], 10, 1.0);
        // img.draw_line(25, 25, 150, 100, [255, 255, 255]);
        img.to_png("test_image3.png").unwrap();
    }
}
