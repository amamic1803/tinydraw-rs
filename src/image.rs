//! A module that contains the [Image] struct and related functions.

// standard library imports
use crate::colors::{Color, ColorType};
use crate::error::Error;
use std::fmt::Display;

/// A struct that holds an image
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Image {
    /// The image pixel data
    pub(crate) data: Vec<u8>,
    /// The width of the image
    pub(crate) width: usize,
    /// The height of the image
    pub(crate) height: usize,
    /// The color type of the image
    pub(crate) color_type: ColorType,
    /// The background color of the image, None if not set
    pub(crate) background_color: Option<Color>,
}
impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Image {{ width: {}, height: {}, color_type: {}, background_color: {}, data: {:?} }}",
            self.width,
            self.height,
            self.color_type,
            self.background_color.map(|c| c.to_string()).unwrap_or_else(|| "None".to_string()),
            self.data
        )
    }
}

impl Image {
    /// Creates a new image with the given width, height, and filled with the background color.
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `background_color` - The background color of the image.
    /// # Returns
    /// * The new image.
    #[allow(clippy::uninit_vec)]
    pub fn new(width: usize, height: usize, background_color: Color) -> Self {
        // create uninitialized data vector
        let len = width * height * background_color.bytes_per_pixel();
        let mut data = Vec::with_capacity(len);
        unsafe {
            data.set_len(len);
        }

        // create image
        let mut image = Self {
            data,
            width,
            height,
            color_type: ColorType::from(background_color),
            background_color: Some(background_color),
        };

        // call clear to fill the image with the background color (initialize data)
        image.clear();

        // return the image
        image
    }

    /// Returns the width of the image.
    /// # Returns
    /// * The width of the image.
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the image.
    /// # Returns
    /// * The height of the image.
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the type of the image.
    /// # Returns
    /// * The type of the image.
    #[inline]
    pub fn color_type(&self) -> ColorType {
        self.color_type
    }

    /// Returns the bytes of the image as a native endian slice.
    /// # Returns
    /// * The bytes of the image.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Returns the bytes of the image as a mutable native endian slice.
    /// # Returns
    /// * The bytes of the image.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Returns the background color of the image.
    /// # Returns
    /// * The background color of the image.
    #[inline]
    pub fn background_color(&self) -> Option<Color> {
        self.background_color
    }

    /// Sets the background color of the image.
    /// # Arguments
    /// * `color` - The new background color.
    /// # Returns
    /// * [Ok] if the background color was set successfully.
    /// * [Err] if there was an error.
    /// # Errors
    /// * [Error::WrongColor] if the color is not compatible with the image type.
    #[inline]
    pub fn set_background_color(&mut self, color: Color) -> Result<(), Error> {
        if ColorType::from(color) == self.color_type {
            self.background_color = Some(color);
            Ok(())
        } else {
            Err(Error::WrongColor)
        }
    }

    /// Reset the image to the background color.
    /// If the background color is not set, this is a no-op.
    pub fn clear(&mut self) {
        if let Some(color) = self.background_color {
            let color_slice = color.as_bytes();
            for i in (0..self.data.len()).step_by(color_slice.len()) {
                self.data[i..(color_slice.len() + i)].copy_from_slice(color_slice);
            }
        }
    }

    /// Fill the image with the given color.
    /// # Arguments
    /// * `color` - The color to fill the image with.
    /// # Returns
    /// * [Ok] if the image was filled successfully.
    /// * [Err] if there was an error.
    /// # Errors
    /// * [Error::WrongColor] if the color is not compatible with the image type.
    pub fn fill_image(&mut self, color: Color) -> Result<(), Error> {
        if ColorType::from(color) == self.color_type {
            let color_slice = color.as_bytes();
            for i in (0..self.data.len()).step_by(color_slice.len()) {
                self.data[i..(color_slice.len() + i)].copy_from_slice(color_slice);
            }
            Ok(())
        } else {
            Err(Error::WrongColor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let width = 100;
        let height = 100;
        let image = Image::new(width, height, Color::GRAY8(255));

        assert_eq!(image.width, width);
        assert_eq!(image.height, height);
        assert_eq!(image.color_type, ColorType::GRAY8);
        assert_eq!(image.background_color, Some(Color::GRAY8(255)));
        assert_eq!(image.data, vec![255; width * height]);
    }

    #[test]
    fn test_getters() {
        let width = 100;
        let height = 100;
        let mut image = Image::new(width, height, Color::GRAY8(255));

        assert_eq!(image.width(), width);
        assert_eq!(image.width(), image.width);
        assert_eq!(image.height(), height);
        assert_eq!(image.height(), image.height);
        assert_eq!(image.color_type(), ColorType::GRAY8);
        assert_eq!(image.color_type(), image.color_type);
        assert_eq!(image.as_bytes(), &vec![255; width * height]);
        assert_eq!(image.as_bytes(), &image.data);
        assert_eq!(image.as_bytes_mut(), &mut vec![255; width * height]);
        assert_eq!(image.background_color(), Some(Color::GRAY8(255)));
        assert_eq!(image.background_color(), image.background_color);
    }

    #[test]
    fn test_background_color() {
        let mut image = Image::new(100, 100, Color::GRAY8(255));
        assert_eq!(image.background_color(), Some(Color::GRAY8(255)));
        assert_eq!(image.set_background_color(Color::RGB8([255, 0, 0])), Err(Error::WrongColor));
        assert_eq!(image.background_color(), Some(Color::GRAY8(255)));
        assert_eq!(image.set_background_color(Color::GRAY8(0)), Ok(()));
        assert_eq!(image.background_color(), Some(Color::GRAY8(0)));
        image.background_color = None;
        assert_eq!(image.background_color(), None);
        assert_eq!(image.set_background_color(Color::GRAYA8([0, 0])), Err(Error::WrongColor));
        assert_eq!(image.background_color(), None);
        assert_eq!(image.set_background_color(Color::GRAY8(0)), Ok(()));
        assert_eq!(image.background_color(), Some(Color::GRAY8(0)));
    }

    #[test]
    fn test_clear() {
        let mut image = Image::new(100, 100, Color::GRAY8(255));
        image.clear();
        assert_eq!(image.data, vec![255; 100 * 100]);
        image.background_color = None;
        image.clear();
        assert_eq!(image.data, vec![255; 100 * 100]);
        image.background_color = Some(Color::GRAY8(0));
        image.clear();
        assert_eq!(image.data, vec![0; 100 * 100]);
    }

    #[test]
    fn test_fill() {
        let mut image = Image::new(100, 100, Color::GRAY8(255));
        image.fill_image(Color::GRAY8(0)).unwrap();
        assert_eq!(image.data, vec![0; 100 * 100]);
        image.fill_image(Color::GRAY8(255)).unwrap();
        assert_eq!(image.data, vec![255; 100 * 100]);
        assert_eq!(image.fill_image(Color::RGB8([255, 0, 0])), Err(Error::WrongColor));
    }
}
