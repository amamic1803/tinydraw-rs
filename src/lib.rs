extern crate core;

pub mod image;

#[doc(inline)]
pub use image::ImageRGB8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawing_1() {
        let background_color: [u8; 3] = [255, 155, 0];
        let mut image: ImageRGB8 = ImageRGB8::new(640, 360, background_color);

        image.draw_line(0, 0, 639, 359, [255, 255, 255], 1, 1.0);
        image.draw_line(0, 359, 639, 0, [255, 255, 255], 1, 1.0);

        image.draw_rectangle(0, 0, 639, 359, [255, 255, 255], 3, 1.0);

        image.draw_ellipse(319, 179, 300, 150, [0, 0, 0], 0, 0.5);

        image.draw_circle(149, 179, 30, [255, 255, 255], 0, 1.0);
        image.draw_circle(149, 179, 20, [0, 0, 0], 0, 1.0);

        image.draw_circle(489, 179, 30, [255, 255, 255], 0, 1.0);
        image.draw_circle(489, 179, 20, [0, 0, 0], 0, 1.0);


        image.draw_ellipse(319, 90, 80, 30, [255, 255, 255], 0, 1.0);
        image.draw_ellipse(319, 90, 60, 20, [0, 0, 0], 0, 1.0);

        image.to_png("image.png").unwrap();
    }
}
