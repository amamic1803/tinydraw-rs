# tinydraw-rs

**tinydraw** is a small library for 2D drawing in Rust

---

It is a simple crate used for drawing basic, anti-aliased shapes, written in pure Rust.
Supports reading and exporting images as PNG or bytes.

### Available Shapes
- line
- rectangle
- circle
- ellipse

[Documentation](https://docs.rs/tinydraw/latest/tinydraw/ "docs.rs")

### Example
```rust
use tinydraw::ImageRGB8;

fn main() { 
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
```
This code generates the following image:


### Limitations
- circle and ellipse 
  - won't draw if any part of them goes out of the image bounds
  - thickness above 1 doesn't work (but 0 (filled) works!)
- colorspace
  - only RGB  with bit depth of 8 is currently supported

## Dependencies
https://crates.io/crates/bytemuck (reading, exporting bytes)

https://crates.io/crates/png (reading, exporting PNG)

## Links
https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm

https://www.geeksforgeeks.org/anti-aliased-line-xiaolin-wus-algorithm/

https://create.stephan-brumme.com/antialiased-circle/

https://yellowsplash.wordpress.com/2009/10/23/fast-antialiased-circles-and-ellipses-from-xiaolin-wus-concepts/