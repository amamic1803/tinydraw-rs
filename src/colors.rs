//! A module about the colors

use std::fmt::Display;
use std::slice;

#[cfg(feature = "image")]
use image::ColorType as ImageColorType;

/// An enum that holds the image type information
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ColorType {
    /// An image with 8-bit grayscale pixels
    GRAY8,
    /// An image with 8-bit grayscale pixels + 8-bit alpha channel
    GRAYA8,
    /// An image with 16-bit grayscale pixels
    GRAY16,
    /// An image with 16-bit grayscale pixels + 16-bit alpha channel
    GRAYA16,
    /// An image with 8-bit RGB pixels
    RGB8,
    /// An image with 8-bit RGB pixels + 8-bit alpha channel
    RGBA8,
    /// An image with 16-bit RGB pixels
    RGB16,
    /// An image with 16-bit RGB pixels + 16-bit alpha channel
    RGBA16,
}
impl ColorType {
    /// Return the number of bytes per pixel
    #[inline]
    pub const fn bytes_per_pixel(&self) -> usize {
        match self {
            ColorType::GRAY8 => 1,
            ColorType::GRAYA8 => 2,
            ColorType::GRAY16 => 2,
            ColorType::GRAYA16 => 4,
            ColorType::RGB8 => 3,
            ColorType::RGBA8 => 4,
            ColorType::RGB16 => 6,
            ColorType::RGBA16 => 8,
        }
    }
}
impl Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorType::GRAY8 => write!(f, "GRAY8"),
            ColorType::GRAYA8 => write!(f, "GRAYA8"),
            ColorType::GRAY16 => write!(f, "GRAY16"),
            ColorType::GRAYA16 => write!(f, "GRAYA16"),
            ColorType::RGB8 => write!(f, "RGB8"),
            ColorType::RGBA8 => write!(f, "RGBA8"),
            ColorType::RGB16 => write!(f, "RGB16"),
            ColorType::RGBA16 => write!(f, "RGBA16"),
        }
    }
}
impl From<Color> for ColorType {
    fn from(color: Color) -> Self {
        match color {
            Color::GRAY8(_) => ColorType::GRAY8,
            Color::GRAYA8(_) => ColorType::GRAYA8,
            Color::GRAY16(_) => ColorType::GRAY16,
            Color::GRAYA16(_) => ColorType::GRAYA16,
            Color::RGB8(_) => ColorType::RGB8,
            Color::RGBA8(_) => ColorType::RGBA8,
            Color::RGB16(_) => ColorType::RGB16,
            Color::RGBA16(_) => ColorType::RGBA16,
        }
    }
}
#[cfg(feature = "image")]
impl From<ImageColorType> for ColorType {
    fn from(color: ImageColorType) -> Self {
        match color {
            ImageColorType::L8 => ColorType::GRAY8,
            ImageColorType::La8 => ColorType::GRAYA8,
            ImageColorType::L16 => ColorType::GRAY16,
            ImageColorType::La16 => ColorType::GRAYA16,
            ImageColorType::Rgb8 => ColorType::RGB8,
            ImageColorType::Rgba8 => ColorType::RGBA8,
            ImageColorType::Rgb16 => ColorType::RGB16,
            ImageColorType::Rgba16 => ColorType::RGBA16,
            _ => panic!("Unsupported color type"),
        }
    }
}
#[cfg(feature = "image")]
impl From<ColorType> for ImageColorType {
    fn from(color: ColorType) -> Self {
        match color {
            ColorType::GRAY8 => ImageColorType::L8,
            ColorType::GRAYA8 => ImageColorType::La8,
            ColorType::GRAY16 => ImageColorType::L16,
            ColorType::GRAYA16 => ImageColorType::La16,
            ColorType::RGB8 => ImageColorType::Rgb8,
            ColorType::RGBA8 => ImageColorType::Rgba8,
            ColorType::RGB16 => ImageColorType::Rgb16,
            ColorType::RGBA16 => ImageColorType::Rgba16,
        }
    }
}

/// An enum that holds the color information
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Color {
    /// The 8-bit grayscale color
    GRAY8(u8),
    /// The 8-bit grayscale color + 8-bit alpha channel
    GRAYA8([u8; 2]),
    /// The 16-bit grayscale color
    GRAY16(u16),
    /// The 16-bit grayscale color + 16-bit alpha channel
    GRAYA16([u16; 2]),
    /// The 8-bit RGB color
    RGB8([u8; 3]),
    /// The 8-bit RGB color + 8-bit alpha channel
    RGBA8([u8; 4]),
    /// The 16-bit RGB color
    RGB16([u16; 3]),
    /// The 16-bit RGB color + 16-bit alpha channel
    RGBA16([u16; 4]),
}
impl Color {
    /// Return the number of bytes per pixel
    #[inline]
    pub fn bytes_per_pixel(&self) -> usize {
        ColorType::from(*self).bytes_per_pixel()
    }

    /// Returns the slice of bytes of the color.
    /// The bytes of u16 are represented in native endianness.
    /// If the u16 is constructed back from the bytes,
    /// it should be done using [u16::from_ne_bytes()] to ensure the correct value.
    pub const fn as_bytes(&self) -> &[u8] {
        match self {
            Color::GRAY8(color) => slice::from_ref(color),
            Color::GRAYA8(color) => color as &[u8],
            Color::GRAY16(color) => unsafe { slice::from_raw_parts(color as *const u16 as *const u8, 2) },
            Color::GRAYA16(color) => unsafe { slice::from_raw_parts(color.as_ptr() as *const u8, 4) },
            Color::RGB8(color) => color as &[u8],
            Color::RGBA8(color) => color as &[u8],
            Color::RGB16(color) => unsafe { slice::from_raw_parts(color.as_ptr() as *const u8, 6) },
            Color::RGBA16(color) => unsafe { slice::from_raw_parts(color.as_ptr() as *const u8, 8) },
        }
    }

    /// Create a color from the bytes.
    /// The bytes of u16 should be represented in native endianness.
    /// The length of the bytes should match the color type.
    /// # Returns
    /// The color
    /// # Panics
    /// If the number of bytes does not match the color type
    pub const fn from_bytes(color_type: ColorType, bytes: &[u8]) -> Self {
        if color_type.bytes_per_pixel() != bytes.len() {
            panic!("Invalid number of bytes for the color type");
        }
        match color_type {
            ColorType::GRAY8 => Color::GRAY8(bytes[0]),
            ColorType::GRAYA8 => Color::GRAYA8([bytes[0], bytes[1]]),
            ColorType::GRAY16 => Color::GRAY16(u16::from_ne_bytes([bytes[0], bytes[1]])),
            ColorType::GRAYA16 => Color::GRAYA16([u16::from_ne_bytes([bytes[0], bytes[1]]), u16::from_ne_bytes([bytes[2], bytes[3]])]),
            ColorType::RGB8 => Color::RGB8([bytes[0], bytes[1], bytes[2]]),
            ColorType::RGBA8 => Color::RGBA8([bytes[0], bytes[1], bytes[2], bytes[3]]),
            ColorType::RGB16 => Color::RGB16([
                u16::from_ne_bytes([bytes[0], bytes[1]]),
                u16::from_ne_bytes([bytes[2], bytes[3]]),
                u16::from_ne_bytes([bytes[4], bytes[5]]),
            ]),
            ColorType::RGBA16 => Color::RGBA16([
                u16::from_ne_bytes([bytes[0], bytes[1]]),
                u16::from_ne_bytes([bytes[2], bytes[3]]),
                u16::from_ne_bytes([bytes[4], bytes[5]]),
                u16::from_ne_bytes([bytes[6], bytes[7]])
            ]),
        }
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::GRAY8(value) => write!(f, "GRAY8({:?})", value),
            Color::GRAYA8(value) => write!(f, "GRAYA8({:?})", value),
            Color::GRAY16(value) => write!(f, "GRAY16({:?})", value),
            Color::GRAYA16(value) => write!(f, "GRAYA16({:?})", value),
            Color::RGB8(value) => write!(f, "RGB8({:?})", value),
            Color::RGBA8(value) => write!(f, "RGBA8({:?})", value),
            Color::RGB16(value) => write!(f, "RGB16({:?})", value),
            Color::RGBA16(value) => write!(f, "RGBA16({:?})", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_type_bytes_per_pixel() {
        assert_eq!(ColorType::GRAY8.bytes_per_pixel(), 1);
        assert_eq!(ColorType::GRAYA8.bytes_per_pixel(), 2);
        assert_eq!(ColorType::GRAY16.bytes_per_pixel(), 2);
        assert_eq!(ColorType::GRAYA16.bytes_per_pixel(), 4);
        assert_eq!(ColorType::RGB8.bytes_per_pixel(), 3);
        assert_eq!(ColorType::RGBA8.bytes_per_pixel(), 4);
        assert_eq!(ColorType::RGB16.bytes_per_pixel(), 6);
        assert_eq!(ColorType::RGBA16.bytes_per_pixel(), 8);
    }

    #[test]
    fn test_color_bytes_per_pixel() {
        assert_eq!(Color::GRAY8(0).bytes_per_pixel(), 1);
        assert_eq!(Color::GRAYA8([0, 1]).bytes_per_pixel(), 2);
        assert_eq!(Color::GRAY16(0).bytes_per_pixel(), 2);
        assert_eq!(Color::GRAYA16([0, 1]).bytes_per_pixel(), 4);
        assert_eq!(Color::RGB8([0, 1, 2]).bytes_per_pixel(), 3);
        assert_eq!(Color::RGBA8([0, 1, 2, 3]).bytes_per_pixel(), 4);
        assert_eq!(Color::RGB16([0, 1, 2]).bytes_per_pixel(), 6);
        assert_eq!(Color::RGBA16([0, 1, 2, 3]).bytes_per_pixel(), 8);
    }

    #[test]
    fn test_color_type_display() {
        assert_eq!(format!("{}", ColorType::GRAY8), "GRAY8");
        assert_eq!(format!("{}", ColorType::GRAYA8), "GRAYA8");
        assert_eq!(format!("{}", ColorType::GRAY16), "GRAY16");
        assert_eq!(format!("{}", ColorType::GRAYA16), "GRAYA16");
        assert_eq!(format!("{}", ColorType::RGB8), "RGB8");
        assert_eq!(format!("{}", ColorType::RGBA8), "RGBA8");
        assert_eq!(format!("{}", ColorType::RGB16), "RGB16");
        assert_eq!(format!("{}", ColorType::RGBA16), "RGBA16");
    }

    #[test]
    fn test_color_display() {
        assert_eq!(format!("{}", Color::GRAY8(0)), "GRAY8(0)");
        assert_eq!(format!("{}", Color::GRAYA8([0, 1])), "GRAYA8([0, 1])");
        assert_eq!(format!("{}", Color::GRAY16(0)), "GRAY16(0)");
        assert_eq!(format!("{}", Color::GRAYA16([0, 1])), "GRAYA16([0, 1])");
        assert_eq!(format!("{}", Color::RGB8([0, 1, 2])), "RGB8([0, 1, 2])");
        assert_eq!(format!("{}", Color::RGBA8([0, 1, 2, 3])), "RGBA8([0, 1, 2, 3])");
        assert_eq!(format!("{}", Color::RGB16([0, 1, 2])), "RGB16([0, 1, 2])");
        assert_eq!(format!("{}", Color::RGBA16([0, 1, 2, 3])), "RGBA16([0, 1, 2, 3])");
    }

    #[test]
    fn test_color_type_from_color() {
        assert_eq!(ColorType::from(Color::GRAY8(0)), ColorType::GRAY8);
        assert_eq!(ColorType::from(Color::GRAYA8([0, 1])), ColorType::GRAYA8);
        assert_eq!(ColorType::from(Color::GRAY16(0)), ColorType::GRAY16);
        assert_eq!(ColorType::from(Color::GRAYA16([0, 1])), ColorType::GRAYA16);
        assert_eq!(ColorType::from(Color::RGB8([0, 1, 2])), ColorType::RGB8);
        assert_eq!(ColorType::from(Color::RGBA8([0, 1, 2, 3])), ColorType::RGBA8);
        assert_eq!(ColorType::from(Color::RGB16([0, 1, 2])), ColorType::RGB16);
        assert_eq!(ColorType::from(Color::RGBA16([0, 1, 2, 3])), ColorType::RGBA16);
    }
    
    #[test]
    #[cfg(feature = "image")]
    fn test_color_type_from_image_color_type() {
        assert_eq!(ColorType::from(ImageColorType::L8), ColorType::GRAY8);
        assert_eq!(ColorType::from(ImageColorType::La8), ColorType::GRAYA8);
        assert_eq!(ColorType::from(ImageColorType::L16), ColorType::GRAY16);
        assert_eq!(ColorType::from(ImageColorType::La16), ColorType::GRAYA16);
        assert_eq!(ColorType::from(ImageColorType::Rgb8), ColorType::RGB8);
        assert_eq!(ColorType::from(ImageColorType::Rgba8), ColorType::RGBA8);
        assert_eq!(ColorType::from(ImageColorType::Rgb16), ColorType::RGB16);
        assert_eq!(ColorType::from(ImageColorType::Rgba16), ColorType::RGBA16);
    }
    
    #[test]
    #[cfg(feature = "image")]
    fn test_color_type_into_image_color_type() {
        assert_eq!(ImageColorType::from(ColorType::GRAY8), ImageColorType::L8);
        assert_eq!(ImageColorType::from(ColorType::GRAYA8), ImageColorType::La8);
        assert_eq!(ImageColorType::from(ColorType::GRAY16), ImageColorType::L16);
        assert_eq!(ImageColorType::from(ColorType::GRAYA16), ImageColorType::La16);
        assert_eq!(ImageColorType::from(ColorType::RGB8), ImageColorType::Rgb8);
        assert_eq!(ImageColorType::from(ColorType::RGBA8), ImageColorType::Rgba8);
        assert_eq!(ImageColorType::from(ColorType::RGB16), ImageColorType::Rgb16);
        assert_eq!(ImageColorType::from(ColorType::RGBA16), ImageColorType::Rgba16);
    }
    
    #[test]
    fn test_color_as_bytes() {
        assert_eq!(Color::GRAY8(0).as_bytes(), &[0]);
        assert_eq!(Color::GRAYA8([0, 1]).as_bytes(), &[0, 1]);
        assert_eq!(Color::GRAY16(1).as_bytes(), &1u16.to_ne_bytes());
        assert_eq!(Color::GRAYA16([1, 2]).as_bytes(), &[1u16.to_ne_bytes(), 2u16.to_ne_bytes()].concat());
        assert_eq!(Color::RGB8([0, 1, 2]).as_bytes(), &[0, 1, 2]);
        assert_eq!(Color::RGBA8([0, 1, 2, 3]).as_bytes(), &[0, 1, 2, 3]);
        assert_eq!(Color::RGB16([1, 2, 3]).as_bytes(), &[1u16.to_ne_bytes(), 2u16.to_ne_bytes(), 3u16.to_ne_bytes()].concat());
        assert_eq!(Color::RGBA16([1, 2, 3, 4]).as_bytes(), &[1u16.to_ne_bytes(), 2u16.to_ne_bytes(), 3u16.to_ne_bytes(), 4u16.to_ne_bytes()].concat());
    }
}
