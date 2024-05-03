//! IO functions for images.

use crate::colors::{Color, ColorType};
use crate::error::Error;
use crate::image::Image;

#[cfg(feature = "image")]
use std::{fs::remove_file, path::Path};

#[cfg(feature = "image")]
use image::{io::Reader as ImageReader, save_buffer, ColorType as ImageColorType, DynamicImage};

impl Image {
    /// Creates a new image from the given bytes.
    /// # Arguments
    /// * ```width``` - The width of the image.
    /// * ```height``` - The height of the image.
    /// * ```color_type``` - The color type of the image.
    /// * ```bytes``` - The bytes of the image.
    /// # Returns
    /// * [Result] which holds new [Image] or [Err] with [Error].
    /// # Errors
    /// * [Error::InvalidSize] - If the size of the bytes is not equal to width * height * bytes per pixel.
    pub fn from_bytes(width: usize, height: usize, color_type: ColorType, bytes: &[u8]) -> Result<Image, Error> {
        // check for valid size
        if bytes.len() != width * height * color_type.bytes_per_pixel() || bytes.is_empty() {
            return Err(Error::InvalidSize);
        }

        // check if all pixels have the same color
        let mut same_data = true;
        let bytes_per_pixel = color_type.bytes_per_pixel();
        for i in 0..bytes_per_pixel {
            for j in (i..(bytes.len() - bytes_per_pixel)).step_by(bytes_per_pixel) {
                if bytes[j] != bytes[j + bytes_per_pixel] {
                    same_data = false;
                    break;
                }
            }
        }

        // if they do, store the background color
        let background_color = if same_data {
            Some(match color_type {
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
                    u16::from_ne_bytes([bytes[6], bytes[7]]),
                ]),
            })
        } else {
            None
        };

        Ok(Self {
            width,
            height,
            color_type,
            data: bytes.to_vec(),
            background_color,
        })
    }

    /// Creates a new image from the given file. Requires the ```image``` feature.
    /// # Arguments
    /// * ```path``` - The path to the file.
    /// # Returns
    /// * [Result] which holds the new [Image] or [Box<dyn std::error::Error>].
    #[cfg(feature = "image")]
    pub fn from_file<F: AsRef<Path>>(path: F) -> Result<Image, Box<dyn std::error::Error>> {
        let image: DynamicImage = ImageReader::open(path)?.decode()?;
        Ok(Self::from_bytes(image.width() as usize, image.height() as usize, image.color().into(), image.as_bytes())?)
    }

    /// Writes the image to the given file. Requires the ```image``` feature.
    /// # Arguments
    /// * ```path``` - The path to the file.
    /// * ```overwrite``` - Whether to overwrite the file if it already exists.
    /// # Returns
    /// * [Result] which holds [Ok] or [Box<dyn std::error::Error>].
    #[cfg(feature = "image")]
    pub fn to_file<F: AsRef<Path>>(&self, path: F, overwrite: bool) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        if path.is_file() {
            if overwrite {
                remove_file(path)?;
            } else {
                return Err(Box::new(Error::FileExists));
            }
        }
        save_buffer(path, self.as_bytes(), self.width as u32, self.height as u32, ImageColorType::from(self.color_type))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::colors::{Color, ColorType};
    use crate::image::Image;
    #[cfg(feature = "image")]
    use tempfile::tempdir;

    #[test]
    fn io_bytes() {
        let image = Image::new(100, 100, Color::GRAY8(255));
        let image2 = Image::from_bytes(100, 100, ColorType::GRAY8, image.as_bytes()).unwrap();

        assert_eq!(image, image2);
    }

    #[cfg(feature = "image")]
    fn test_file_io(color: Color) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("img.png");

        let image = Image::new(100, 100, color);
        image.to_file(&path, true).unwrap();
        let image2 = Image::from_file(&path).unwrap();

        assert_eq!(image, image2);
    }

    #[test]
    #[cfg(feature = "image")]
    fn io_gray8() {
        test_file_io(Color::GRAY8(u8::MAX - 1));
    }

    #[test]
    #[cfg(feature = "image")]
    fn io_graya8() {
        test_file_io(Color::GRAYA8([u8::MAX - 1, u8::MAX - 2]));
    }

    #[test]
    #[cfg(feature = "image")]
    fn io_gray16() {
        test_file_io(Color::GRAY16(u16::MAX - 1));
    }

    #[test]
    #[cfg(feature = "image")]
    fn io_graya16() {
        test_file_io(Color::GRAYA16([u16::MAX - 1, u16::MAX - 2]));
    }

    #[test]
    #[cfg(feature = "image")]
    fn io_rgb8() {
        test_file_io(Color::RGB8([u8::MAX - 1, u8::MAX - 2, u8::MAX - 3]));
    }

    #[test]
    #[cfg(feature = "image")]
    fn io_rgba8() {
        test_file_io(Color::RGBA8([u8::MAX - 1, u8::MAX - 2, u8::MAX - 3, u8::MAX - 4]));
    }

    #[test]
    #[cfg(feature = "image")]
    fn io_rgb16() {
        test_file_io(Color::RGB16([u16::MAX - 1, u16::MAX - 2, u16::MAX - 3]));
    }

    #[test]
    #[cfg(feature = "image")]
    fn io_rgba16() {
        test_file_io(Color::RGBA16([u16::MAX - 1, u16::MAX - 2, u16::MAX - 3, u16::MAX - 4]));
    }
}
