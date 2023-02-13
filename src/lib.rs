extern crate core;

pub mod image;

#[doc(inline)]
pub use image::ImageRGB8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let mut img = ImageRGB8::new(200, 200, [25, 200, 25]);
        // img.draw_circle(100, 100, 11, [255, 255, 255], 0, 1.0);
        img.draw_circle(100, 100, 25, [255, 255, 255], 0, 0.5);
        // img.draw_line(50, 50, 125, 150, [255, 255, 255], 1, 1.0);
        // img.draw_rectangle(25,  25, 175, 175, [255, 255, 255], 5, 0.5);
        // img.clear();
        img.to_png("test_image3.png").unwrap();
    }
}
